use actix_web::{web, HttpRequest, HttpResponse};
use validator::Validate;
use crate::errors::AppError;
use crate::middleware::{extract_claims, require_role};
use crate::models::*;
use crate::services;
use crate::AppState;
use super::get_ip;

pub async fn create_market_post(req: HttpRequest, state: web::Data<AppState>, body: web::Json<MarketPostReq>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    require_role(&claims, &[1])?;
    body.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    let post = services::create_market_post(&state.db, claims.sub, body.into_inner()).await?;
    services::log_operation(&state.db, claims.sub, "发布好市场", &format!("post_id={}", post.id), &get_ip(&req)).await;
    Ok(HttpResponse::Ok().json(ApiResp::ok(post)))
}

pub async fn list_market_posts(state: web::Data<AppState>, query: web::Query<PageQuery>) -> Result<HttpResponse, AppError> {
    let result = services::list_market_posts(&state.db, &query).await?;
    Ok(HttpResponse::Ok().json(ApiResp::ok(result)))
}

pub async fn get_market_detail(state: web::Data<AppState>, path: web::Path<i64>) -> Result<HttpResponse, AppError> {
    let detail = services::get_market_detail(&state.db, path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResp::ok(detail)))
}

pub async fn update_market_post(req: HttpRequest, state: web::Data<AppState>, path: web::Path<i64>, body: web::Json<MarketPostReq>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    require_role(&claims, &[1])?;
    body.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    let id = path.into_inner();
    services::update_market_post(&state.db, claims.sub, id, body.into_inner()).await?;
    services::log_operation(&state.db, claims.sub, "编辑好市场", &format!("post_id={}", id), &get_ip(&req)).await;
    Ok(HttpResponse::Ok().json(ApiResp::<()>::ok_msg("更新成功")))
}

pub async fn delete_market_post(req: HttpRequest, state: web::Data<AppState>, path: web::Path<i64>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    require_role(&claims, &[1, 2])?;
    let id = path.into_inner();
    services::delete_market_post(&state.db, claims.sub, id, claims.role == 2).await?;
    services::log_operation(&state.db, claims.sub, "删除好市场", &format!("post_id={}", id), &get_ip(&req)).await;
    Ok(HttpResponse::Ok().json(ApiResp::<()>::ok_msg("删除成功")))
}
