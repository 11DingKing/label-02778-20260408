use actix_web::{web, HttpRequest, HttpResponse};
use validator::Validate;
use crate::errors::AppError;
use crate::middleware::extract_claims;
use crate::models::*;
use crate::services;
use crate::AppState;
use super::get_ip;

// 「江湖说，所有人都可以发布」—— 登录用户关联身份，未登录用户以匿名身份发布。
// 浏览动态列表不强制登录（仅影响是否显示"已点赞"状态）。
pub async fn create_jianghu_post(req: HttpRequest, state: web::Data<AppState>, body: web::Json<JianghuPostReq>) -> Result<HttpResponse, AppError> {
    let uid = extract_claims(&req, &state).ok().map(|c| c.sub);
    body.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    let post = services::create_jianghu_post(&state.db, uid, body.into_inner()).await?;
    if let Some(user_id) = uid {
        services::log_operation(&state.db, user_id, "发布江湖说", &format!("post_id={}", post.id), &get_ip(&req)).await;
    }
    Ok(HttpResponse::Ok().json(ApiResp::ok(post)))
}

pub async fn list_jianghu_posts(req: HttpRequest, state: web::Data<AppState>, query: web::Query<PageQuery>) -> Result<HttpResponse, AppError> {
    let uid = extract_claims(&req, &state).ok().map(|c| c.sub);
    let result = services::list_jianghu_posts(&state.db, uid, &query).await?;
    Ok(HttpResponse::Ok().json(ApiResp::ok(result)))
}

pub async fn toggle_like(req: HttpRequest, state: web::Data<AppState>, path: web::Path<i64>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    let liked = services::toggle_like(&state.db, claims.sub, path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResp::ok(liked)))
}

pub async fn add_comment(req: HttpRequest, state: web::Data<AppState>, path: web::Path<i64>, body: web::Json<CommentReq>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    body.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    let comment = services::add_comment(&state.db, claims.sub, path.into_inner(), &body.content).await?;
    Ok(HttpResponse::Ok().json(ApiResp::ok(comment)))
}

pub async fn list_comments(state: web::Data<AppState>, path: web::Path<i64>) -> Result<HttpResponse, AppError> {
    let comments = services::list_comments(&state.db, path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResp::ok(comments)))
}

pub async fn delete_jianghu_post(req: HttpRequest, state: web::Data<AppState>, path: web::Path<i64>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    let post_id = path.into_inner();
    services::delete_jianghu_post(&state.db, claims.sub, post_id, claims.role == 2).await?;
    services::log_operation(&state.db, claims.sub, "删除江湖说", &format!("post_id={}", post_id), &get_ip(&req)).await;
    Ok(HttpResponse::Ok().json(ApiResp::<()>::ok_msg("删除成功")))
}
