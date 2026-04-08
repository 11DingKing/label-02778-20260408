use actix_web::{web, HttpRequest, HttpResponse};
use validator::Validate;
use crate::errors::AppError;
use crate::middleware::{extract_claims, require_role};
use crate::models::*;
use crate::services;
use crate::AppState;
use super::get_ip;

pub async fn upsert_seeker_profile(req: HttpRequest, state: web::Data<AppState>, body: web::Json<SeekerProfileReq>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    require_role(&claims, &[0])?;
    body.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    let profile = services::upsert_seeker_profile(&state.db, claims.sub, body.into_inner()).await?;
    services::log_operation(&state.db, claims.sub, "更新求职档案", &format!("profile_id={}", profile.id), &get_ip(&req)).await;
    Ok(HttpResponse::Ok().json(ApiResp::ok(profile)))
}

pub async fn get_seeker_profile(req: HttpRequest, state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    let profile = services::get_seeker_profile(&state.db, claims.sub).await?;
    Ok(HttpResponse::Ok().json(ApiResp::ok(profile)))
}

pub async fn list_seekers(state: web::Data<AppState>, query: web::Query<PageQuery>) -> Result<HttpResponse, AppError> {
    let result = services::list_seekers(&state.db, &query).await?;
    Ok(HttpResponse::Ok().json(ApiResp::ok(result)))
}

pub async fn get_seeker_detail(state: web::Data<AppState>, path: web::Path<i64>) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let profile = services::get_seeker_detail(&state.db, id).await?;
    Ok(HttpResponse::Ok().json(ApiResp::ok(profile)))
}
