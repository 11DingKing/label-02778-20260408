mod auth;
mod seeker;
mod enterprise;
mod job;
mod chat;
mod jianghu;
mod market;
mod admin;

use crate::errors::AppError;
use crate::models::*;
use sqlx::MySqlPool;

pub use auth::*;
pub use seeker::*;
pub use enterprise::*;
pub use job::*;
pub use chat::*;
pub use jianghu::*;
pub use market::*;
pub use admin::*;

// ========== Log Service ==========
pub async fn log_operation(db: &MySqlPool, user_id: i64, action: &str, target: &str, ip: &str) {
    log::info!("[OP_LOG] user_id={} action={} target={} ip={}", user_id, action, target, ip);
    if let Err(e) = sqlx::query("INSERT INTO operation_logs (user_id, action, target, ip) VALUES (?, ?, ?, ?)")
        .bind(user_id).bind(action).bind(target).bind(ip)
        .execute(db).await {
        log::error!("Failed to write operation log: {:?}", e);
    }
}

// ========== Common Helpers ==========

/// 获取用户，不存在则返回 NotFound
pub(crate) async fn get_user(db: &MySqlPool, user_id: i64) -> Result<User, AppError> {
    sqlx::query_as("SELECT * FROM users WHERE id = ?")
        .bind(user_id).fetch_optional(db).await?
        .ok_or_else(|| AppError::NotFound("用户不存在".into()))
}

/// 获取已认证的企业档案，未认证或不存在则返回对应错误
pub(crate) async fn get_verified_enterprise(db: &MySqlPool, user_id: i64) -> Result<EnterpriseProfile, AppError> {
    sqlx::query_as("SELECT * FROM enterprise_profiles WHERE user_id = ? AND verified = 1")
        .bind(user_id).fetch_optional(db).await?
        .ok_or_else(|| AppError::Forbidden("企业未认证，无法执行此操作".into()))
}

/// 获取企业档案（不要求已认证），不存在则返回错误
pub(crate) async fn get_enterprise_profile_or_err(db: &MySqlPool, user_id: i64) -> Result<EnterpriseProfile, AppError> {
    sqlx::query_as("SELECT * FROM enterprise_profiles WHERE user_id = ?")
        .bind(user_id).fetch_optional(db).await?
        .ok_or_else(|| AppError::Forbidden("企业信息不存在".into()))
}
