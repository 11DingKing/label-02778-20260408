use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpRequest, web,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::task::{Context, Poll};
use std::rc::Rc;
use crate::errors::AppError;
use crate::AppState;
use crate::config::{RATE_LIMIT_BUCKET_CAPACITY, RATE_LIMIT_REFILL_RATE_PER_SECOND};
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i64,
    pub role: i8,
    pub username: String,
    pub exp: usize,
}

#[derive(Debug, Clone)]
pub struct RateLimitMiddleware;

impl<S, B> Transform<S, ServiceRequest> for RateLimitMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RateLimitMiddlewareService<S>;
    type InitError = ();
    type Future = LocalBoxFuture<'static, Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        Box::pin(async move {
            Ok(RateLimitMiddlewareService {
                service: Rc::new(service),
            })
        })
    }
}

pub struct RateLimitMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RateLimitMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        
        Box::pin(async move {
            let app_state = req.app_data::<web::Data<AppState>>();
            
            if let Some(state) = app_state {
                let claims = extract_claims_from_req(&req, state);
                let ip = req.connection_info().realip_remote_addr().unwrap_or("unknown").to_string();
                
                let key = if let Some(ref claims) = claims {
                    format!("{}:{}", ip, claims.sub)
                } else {
                    ip
                };

                let bucket = state.rate_limiter.entry(key.clone())
                    .or_insert_with(|| {
                        std::sync::Arc::new(parking_lot::Mutex::new(crate::TokenBucket::new(RATE_LIMIT_BUCKET_CAPACITY)))
                    })
                    .clone();

                let mut bucket = bucket.lock();
                let now = Instant::now();
                let elapsed = now.duration_since(bucket.last_refill).as_secs_f64();
                
                let new_tokens = elapsed * RATE_LIMIT_REFILL_RATE_PER_SECOND;
                bucket.tokens = (bucket.tokens + new_tokens).min(RATE_LIMIT_BUCKET_CAPACITY as f64);
                bucket.last_refill = now;

                if bucket.tokens < 1.0 {
                    return Err(AppError::TooManyRequests("请求过于频繁，请稍后再试".to_string()).into());
                }

                bucket.tokens -= 1.0;
            }

            service.call(req).await
        })
    }
}

fn extract_claims_from_req(req: &ServiceRequest, state: &web::Data<AppState>) -> Option<Claims> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())?;

    let token = auth_header.strip_prefix("Bearer ")?;

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ).ok().map(|d| d.claims)
}

pub fn extract_claims(req: &HttpRequest, state: &web::Data<AppState>) -> Result<Claims, AppError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("缺少认证信息".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("认证格式错误".to_string()))?;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(token_data.claims)
}

pub fn require_role(claims: &Claims, allowed_roles: &[i8]) -> Result<(), AppError> {
    if !allowed_roles.contains(&claims.role) {
        return Err(AppError::Forbidden("权限不足".to_string()));
    }
    Ok(())
}
