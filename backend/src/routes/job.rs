use actix_web::{web, HttpRequest, HttpResponse};
use validator::Validate;
use crate::errors::AppError;
use crate::middleware::{extract_claims, require_role};
use crate::models::*;
use crate::services;
use crate::AppState;
use super::get_ip;

pub async fn create_job(req: HttpRequest, state: web::Data<AppState>, body: web::Json<JobPostReq>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    require_role(&claims, &[1])?;
    body.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    let job = services::create_job(&state.db, claims.sub, body.into_inner()).await?;
    services::log_operation(&state.db, claims.sub, "发布招聘", &format!("job_id={}", job.id), &get_ip(&req)).await;
    Ok(HttpResponse::Ok().json(ApiResp::ok(job)))
}

pub async fn update_job(req: HttpRequest, state: web::Data<AppState>, path: web::Path<i64>, body: web::Json<JobPostReq>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    require_role(&claims, &[1])?;
    body.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    let job = services::update_job(&state.db, claims.sub, path.into_inner(), body.into_inner()).await?;
    services::log_operation(&state.db, claims.sub, "编辑招聘", &format!("job_id={}", job.id), &get_ip(&req)).await;
    Ok(HttpResponse::Ok().json(ApiResp::ok(job)))
}

pub async fn delete_job(req: HttpRequest, state: web::Data<AppState>, path: web::Path<i64>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    require_role(&claims, &[1])?;
    let job_id = path.into_inner();
    services::delete_job(&state.db, claims.sub, job_id).await?;
    services::log_operation(&state.db, claims.sub, "删除招聘", &format!("job_id={}", job_id), &get_ip(&req)).await;
    Ok(HttpResponse::Ok().json(ApiResp::<()>::ok_msg("删除成功")))
}

pub async fn list_my_jobs(req: HttpRequest, state: web::Data<AppState>, query: web::Query<PageQuery>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    let result = services::list_my_jobs(&state.db, claims.sub, &query).await?;
    Ok(HttpResponse::Ok().json(ApiResp::ok(result)))
}

pub async fn list_jobs(state: web::Data<AppState>, query: web::Query<PageQuery>) -> Result<HttpResponse, AppError> {
    let result = services::list_jobs(&state.db, &query).await?;
    Ok(HttpResponse::Ok().json(ApiResp::ok(result)))
}

pub async fn get_job_detail(state: web::Data<AppState>, path: web::Path<i64>) -> Result<HttpResponse, AppError> {
    let job = services::get_job_detail(&state.db, path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResp::ok(job)))
}
