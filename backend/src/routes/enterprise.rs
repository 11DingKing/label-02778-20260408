use actix_web::{web, HttpRequest, HttpResponse};
use validator::Validate;
use crate::errors::AppError;
use crate::middleware::{extract_claims, require_role};
use crate::models::*;
use crate::services;
use crate::AppState;
use super::get_ip;

pub async fn upsert_enterprise_profile(req: HttpRequest, state: web::Data<AppState>, body: web::Json<EnterpriseProfileReq>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    require_role(&claims, &[1])?;
    body.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    let profile = services::upsert_enterprise_profile(&state.db, claims.sub, body.into_inner()).await?;
    services::log_operation(&state.db, claims.sub, "更新企业档案", &format!("enterprise_id={}", profile.id), &get_ip(&req)).await;
    Ok(HttpResponse::Ok().json(ApiResp::ok(profile)))
}

pub async fn get_enterprise_profile(req: HttpRequest, state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    let profile = services::get_enterprise_profile(&state.db, claims.sub).await?;
    Ok(HttpResponse::Ok().json(ApiResp::ok(profile)))
}
