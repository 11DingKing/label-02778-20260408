use crate::config::*;
use crate::errors::AppError;
use crate::models::*;
use sqlx::MySqlPool;

pub async fn admin_list_users(db: &MySqlPool, query: &PageQuery) -> Result<PageResp<UserVO>, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let size = query.size.unwrap_or(DEFAULT_PAGE_SIZE).min(MAX_PAGE_SIZE);
    let offset = (page - 1) * size;
    let keyword = query.keyword.as_deref().unwrap_or("");
    let like_pattern = format!("%{}%", keyword);

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM users WHERE username LIKE ? OR phone LIKE ?"
    ).bind(&like_pattern).bind(&like_pattern).fetch_one(db).await?;

    let users: Vec<User> = sqlx::query_as(
        "SELECT * FROM users WHERE username LIKE ? OR phone LIKE ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
    ).bind(&like_pattern).bind(&like_pattern).bind(size).bind(offset).fetch_all(db).await?;

    let list: Vec<UserVO> = users.into_iter().map(|u| u.into()).collect();
    Ok(PageResp { list, total: total.0, page, size })
}

pub async fn admin_toggle_user_status(db: &MySqlPool, user_id: i64) -> Result<(), AppError> {
    let user: User = sqlx::query_as("SELECT * FROM users WHERE id = ?")
        .bind(user_id).fetch_optional(db).await?
        .ok_or_else(|| AppError::NotFound("用户不存在".into()))?;

    if user.role == 2 {
        return Err(AppError::Forbidden("不能禁用管理员账号".into()));
    }

    let new_status: i8 = if user.status == 0 { 1 } else { 0 };
    sqlx::query("UPDATE users SET status = ? WHERE id = ?")
        .bind(new_status).bind(user_id).execute(db).await?;
    log::info!("User {} status changed: {} -> {}", user_id, user.status, new_status);
    Ok(())
}

pub async fn admin_list_enterprises(db: &MySqlPool, query: &PageQuery) -> Result<PageResp<EnterpriseProfile>, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let size = query.size.unwrap_or(DEFAULT_PAGE_SIZE).min(MAX_PAGE_SIZE);
    let offset = (page - 1) * size;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM enterprise_profiles")
        .fetch_one(db).await?;

    let list: Vec<EnterpriseProfile> = sqlx::query_as(
        "SELECT * FROM enterprise_profiles ORDER BY FIELD(verified, 0, 2, 1), created_at DESC LIMIT ? OFFSET ?"
    ).bind(size).bind(offset).fetch_all(db).await?;

    Ok(PageResp { list, total: total.0, page, size })
}

pub async fn admin_verify_enterprise(db: &MySqlPool, enterprise_id: i64, verified: i8) -> Result<(), AppError> {
    if verified != 1 && verified != 2 {
        return Err(AppError::BadRequest("审核状态只能是1(通过)或2(拒绝)".into()));
    }
    let result = sqlx::query("UPDATE enterprise_profiles SET verified = ? WHERE id = ?")
        .bind(verified).bind(enterprise_id).execute(db).await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("企业不存在".into()));
    }
    log::info!("Enterprise {} verified: {}", enterprise_id, verified);
    Ok(())
}

pub async fn admin_list_logs(db: &MySqlPool, query: &PageQuery) -> Result<PageResp<OperationLog>, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let size = query.size.unwrap_or(DEFAULT_PAGE_SIZE).min(MAX_PAGE_SIZE);
    let offset = (page - 1) * size;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM operation_logs")
        .fetch_one(db).await?;

    let list: Vec<OperationLog> = sqlx::query_as(
        "SELECT * FROM operation_logs ORDER BY created_at DESC LIMIT ? OFFSET ?"
    ).bind(size).bind(offset).fetch_all(db).await?;

    Ok(PageResp { list, total: total.0, page, size })
}

pub async fn admin_stats(db: &MySqlPool) -> Result<AdminStats, AppError> {
    let total_users: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users").fetch_one(db).await?;
    let total_seekers: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE role = 0").fetch_one(db).await?;
    let total_enterprises: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE role = 1").fetch_one(db).await?;
    let total_jobs: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM job_posts").fetch_one(db).await?;
    let total_jianghu: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM jianghu_posts").fetch_one(db).await?;
    let total_market: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM market_posts").fetch_one(db).await?;
    let pending_verify: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM enterprise_profiles WHERE verified = 0").fetch_one(db).await?;

    Ok(AdminStats {
        total_users: total_users.0,
        total_seekers: total_seekers.0,
        total_enterprises: total_enterprises.0,
        total_jobs: total_jobs.0,
        total_jianghu: total_jianghu.0,
        total_market: total_market.0,
        pending_verify: pending_verify.0,
    })
}
