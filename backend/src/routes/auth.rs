use actix_web::{web, HttpRequest, HttpResponse};
use validator::Validate;
use crate::errors::AppError;
use crate::middleware::extract_claims;
use crate::models::*;
use crate::services;
use crate::AppState;
use super::get_ip;

pub async fn register(req: HttpRequest, state: web::Data<AppState>, body: web::Json<RegisterReq>) -> Result<HttpResponse, AppError> {
    body.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    let user = services::register(&state.db, body.into_inner()).await?;
    services::log_operation(&state.db, user.id, "用户注册", &format!("username={}, role={}", user.username, user.role), &get_ip(&req)).await;
    Ok(HttpResponse::Ok().json(ApiResp::ok(user)))
}

pub async fn login(req: HttpRequest, state: web::Data<AppState>, body: web::Json<LoginReq>) -> Result<HttpResponse, AppError> {
    body.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    let resp = services::login(&state.db, &state.jwt_secret, body.into_inner()).await?;
    services::log_operation(&state.db, resp.user.id, "用户登录", &format!("username={}", resp.user.username), &get_ip(&req)).await;
    Ok(HttpResponse::Ok().json(ApiResp::ok(resp)))
}

pub async fn me(req: HttpRequest, state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    let user: crate::models::User = sqlx::query_as("SELECT * FROM users WHERE id = ?")
        .bind(claims.sub).fetch_one(&state.db).await?;
    let vo: UserVO = user.into();
    Ok(HttpResponse::Ok().json(ApiResp::ok(vo)))
}

pub async fn refresh(_req: HttpRequest, state: web::Data<AppState>, body: web::Json<RefreshTokenReq>) -> Result<HttpResponse, AppError> {
    body.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    let new_token = services::refresh_token(&state.db, &state.jwt_secret, &body.token).await?;
    Ok(HttpResponse::Ok().json(ApiResp::ok(RefreshTokenResp { token: new_token })))
}
