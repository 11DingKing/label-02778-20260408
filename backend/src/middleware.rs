use actix_web::{HttpRequest, web};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use crate::errors::AppError;
use crate::AppState;
use crate::config::{RATE_LIMIT_BUCKET_CAPACITY, RATE_LIMIT_REFILL_RATE_PER_SECOND};
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i64,       // user_id
    pub role: i8,       // 0=seeker, 1=enterprise, 2=admin
    pub username: String,
    pub exp: usize,
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

pub fn check_rate_limit(
    req: &HttpRequest,
    state: &web::Data<AppState>,
    claims: Option<&Claims>,
) -> Result<(), AppError> {
    let ip = req.connection_info().realip_remote_addr().unwrap_or("unknown").to_string();
    
    let key = if let Some(claims) = claims {
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

    if bucket.tokens >= 1.0 {
        bucket.tokens -= 1.0;
        Ok(())
    } else {
        Err(AppError::TooManyRequests("请求过于频繁，请稍后再试".to_string()))
    }
}
