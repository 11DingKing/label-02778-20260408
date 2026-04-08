use actix_web::{web, HttpRequest, HttpResponse};
use crate::errors::AppError;
use crate::middleware::{extract_claims, require_role};
use crate::models::*;
use crate::services;
use crate::AppState;
use super::get_ip;

pub async fn admin_list_users(req: HttpRequest, state: web::Data<AppState>, query: web::Query<PageQuery>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    require_role(&claims, &[2])?;
    let result = services::admin_list_users(&state.db, &query).await?;
    Ok(HttpResponse::Ok().json(ApiResp::ok(result)))
}

pub async fn admin_toggle_user_status(req: HttpRequest, state: web::Data<AppState>, path: web::Path<i64>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    require_role(&claims, &[2])?;
    let user_id = path.into_inner();
    services::admin_toggle_user_status(&state.db, user_id).await?;
    services::log_operation(&state.db, claims.sub, "切换用户状态", &format!("user_id={}", user_id), &get_ip(&req)).await;
    Ok(HttpResponse::Ok().json(ApiResp::<()>::ok_msg("操作成功")))
}

pub async fn admin_list_enterprises(req: HttpRequest, state: web::Data<AppState>, query: web::Query<PageQuery>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    require_role(&claims, &[2])?;
    let result = services::admin_list_enterprises(&state.db, &query).await?;
    Ok(HttpResponse::Ok().json(ApiResp::ok(result)))
}

#[derive(serde::Deserialize)]
pub struct VerifyReq {
    pub verified: i8,
}

pub async fn admin_verify_enterprise(req: HttpRequest, state: web::Data<AppState>, path: web::Path<i64>, body: web::Json<VerifyReq>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    require_role(&claims, &[2])?;
    let eid = path.into_inner();
    services::admin_verify_enterprise(&state.db, eid, body.verified).await?;
    services::log_operation(&state.db, claims.sub, "审核企业", &format!("enterprise_id={}, verified={}", eid, body.verified), &get_ip(&req)).await;
    Ok(HttpResponse::Ok().json(ApiResp::<()>::ok_msg("审核完成")))
}

pub async fn admin_list_logs(req: HttpRequest, state: web::Data<AppState>, query: web::Query<PageQuery>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    require_role(&claims, &[2])?;
    let result = services::admin_list_logs(&state.db, &query).await?;
    Ok(HttpResponse::Ok().json(ApiResp::ok(result)))
}

pub async fn admin_stats(req: HttpRequest, state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    require_role(&claims, &[2])?;
    let stats = services::admin_stats(&state.db).await?;
    Ok(HttpResponse::Ok().json(ApiResp::ok(stats)))
}

pub async fn admin_list_market(req: HttpRequest, state: web::Data<AppState>, query: web::Query<PageQuery>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    require_role(&claims, &[2])?;
    let result = services::admin_list_market_posts(&state.db, &query).await?;
    Ok(HttpResponse::Ok().json(ApiResp::ok(result)))
}

pub async fn admin_delete_market(req: HttpRequest, state: web::Data<AppState>, path: web::Path<i64>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    require_role(&claims, &[2])?;
    let id = path.into_inner();
    services::delete_market_post(&state.db, claims.sub, id, true).await?;
    services::log_operation(&state.db, claims.sub, "管理员删除市场商品", &format!("post_id={}", id), &get_ip(&req)).await;
    Ok(HttpResponse::Ok().json(ApiResp::<()>::ok_msg("删除成功")))
}

pub async fn admin_list_jianghu(req: HttpRequest, state: web::Data<AppState>, query: web::Query<PageQuery>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    require_role(&claims, &[2])?;
    let result = services::list_jianghu_posts(&state.db, None, &query).await?;
    Ok(HttpResponse::Ok().json(ApiResp::ok(result)))
}

pub async fn admin_delete_jianghu(req: HttpRequest, state: web::Data<AppState>, path: web::Path<i64>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    require_role(&claims, &[2])?;
    let id = path.into_inner();
    services::delete_jianghu_post(&state.db, claims.sub, id, true).await?;
    services::log_operation(&state.db, claims.sub, "管理员删除江湖说", &format!("post_id={}", id), &get_ip(&req)).await;
    Ok(HttpResponse::Ok().json(ApiResp::<()>::ok_msg("删除成功")))
}

pub async fn admin_list_jobs(req: HttpRequest, state: web::Data<AppState>, query: web::Query<PageQuery>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    require_role(&claims, &[2])?;
    let result = services::list_jobs(&state.db, &query).await?;
    Ok(HttpResponse::Ok().json(ApiResp::ok(result)))
}

pub async fn admin_delete_job(req: HttpRequest, state: web::Data<AppState>, path: web::Path<i64>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    require_role(&claims, &[2])?;
    let id = path.into_inner();
    services::admin_delete_job(&state.db, id).await?;
    services::log_operation(&state.db, claims.sub, "管理员删除招聘", &format!("job_id={}", id), &get_ip(&req)).await;
    Ok(HttpResponse::Ok().json(ApiResp::<()>::ok_msg("删除成功")))
}
