use actix_web::{web, HttpRequest, HttpResponse};
use validator::Validate;
use crate::errors::AppError;
use crate::middleware::{extract_claims, require_role};
use crate::models::*;
use crate::services;
use crate::AppState;
use super::get_ip;

pub async fn initiate_contact(req: HttpRequest, state: web::Data<AppState>, body: web::Json<ContactReq>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    require_role(&claims, &[1])?;
    let contact = services::initiate_contact(&state.db, claims.sub, body.seeker_user_id, body.greeting.clone()).await?;
    services::log_operation(&state.db, claims.sub, "发起联系", &format!("seeker_user_id={}", body.seeker_user_id), &get_ip(&req)).await;
    Ok(HttpResponse::Ok().json(ApiResp::ok(contact)))
}

pub async fn reply_contact(req: HttpRequest, state: web::Data<AppState>, path: web::Path<i64>, body: web::Json<ReplyReq>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    let contact_id = path.into_inner();
    services::reply_contact(&state.db, claims.sub, contact_id, body.accept).await?;
    let action = if body.accept { "接受联系" } else { "拒绝联系" };
    services::log_operation(&state.db, claims.sub, action, &format!("contact_id={}", contact_id), &get_ip(&req)).await;
    Ok(HttpResponse::Ok().json(ApiResp::<()>::ok_msg("操作成功")))
}

pub async fn list_contacts(req: HttpRequest, state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    let contacts = services::list_contacts(&state.db, claims.sub, claims.role).await?;
    Ok(HttpResponse::Ok().json(ApiResp::ok(contacts)))
}

pub async fn send_message(req: HttpRequest, state: web::Data<AppState>, body: web::Json<SendMessageReq>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    body.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    let msg = services::send_message(&state.db, claims.sub, body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResp::ok(msg)))
}

pub async fn list_messages(req: HttpRequest, state: web::Data<AppState>, path: web::Path<i64>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    let contact_id = path.into_inner();
    let messages = services::list_messages(&state.db, claims.sub, contact_id).await?;
    Ok(HttpResponse::Ok().json(ApiResp::ok(messages)))
}

pub async fn mark_read(req: HttpRequest, state: web::Data<AppState>, path: web::Path<i64>) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &state)?;
    services::mark_read(&state.db, claims.sub, path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResp::<()>::ok_msg("已读")))
}
