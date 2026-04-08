use crate::config::*;
use crate::errors::AppError;
use crate::middleware::Claims;
use crate::models::*;
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::MySqlPool;

pub async fn register(db: &MySqlPool, req: RegisterReq) -> Result<UserVO, AppError> {
    log::info!("Register attempt: username={}, role={}", req.username, req.role);
    if req.role != 0 && req.role != 1 {
        log::warn!("Register rejected: invalid role={}", req.role);
        return Err(AppError::BadRequest("角色只能是求职者(0)或企业(1)".into()));
    }
    let exists: Option<(i64,)> = sqlx::query_as("SELECT id FROM users WHERE username = ?")
        .bind(&req.username).fetch_optional(db).await?;
    if exists.is_some() {
        log::warn!("Register rejected: username={} already exists", req.username);
        return Err(AppError::Conflict("用户名已存在".into()));
    }
    let hash = bcrypt::hash(&req.password, 12)?;
    let phone = req.phone.unwrap_or_default();
    let result = sqlx::query(
        "INSERT INTO users (username, password_hash, phone, role) VALUES (?, ?, ?, ?)"
    ).bind(&req.username).bind(&hash).bind(&phone).bind(req.role)
        .execute(db).await?;

    let user: User = sqlx::query_as("SELECT * FROM users WHERE id = ?")
        .bind(result.last_insert_id()).fetch_one(db).await?;
    log::info!("User registered successfully: id={}, username={}, role={}", user.id, user.username, user.role);
    Ok(user.into())
}

pub async fn login(db: &MySqlPool, jwt_secret: &str, req: LoginReq) -> Result<LoginResp, AppError> {
    log::info!("Login attempt: username={}", req.username);
    let user: User = sqlx::query_as("SELECT * FROM users WHERE username = ?")
        .bind(&req.username).fetch_optional(db).await?
        .ok_or_else(|| {
            log::warn!("Login failed: username={} not found", req.username);
            AppError::Unauthorized("用户名或密码错误".into())
        })?;

    if user.status == 1 {
        log::warn!("Login rejected: user {} is disabled", req.username);
        return Err(AppError::Forbidden("账号已被禁用".into()));
    }
    if !bcrypt::verify(&req.password, &user.password_hash)? {
        log::warn!("Login failed: wrong password for username={}", req.username);
        return Err(AppError::Unauthorized("用户名或密码错误".into()));
    }

    let exp = (Utc::now() + chrono::Duration::hours(JWT_EXPIRATION_HOURS)).timestamp() as usize;
    let claims = Claims {
        sub: user.id,
        role: user.role,
        username: user.username.clone(),
        exp,
    };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_bytes()))?;
    log::info!("Login successful: user_id={}, username={}", user.id, user.username);
    Ok(LoginResp { token, user: user.into() })
}
