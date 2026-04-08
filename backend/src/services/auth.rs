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

pub async fn refresh_token(db: &MySqlPool, jwt_secret: &str, old_token: &str) -> Result<String, AppError> {
    use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

    log::info!("Refresh token attempt");

    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = false;

    let token_data = decode::<Claims>(
        old_token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &validation,
    )?;

    let claims = token_data.claims;
    let now = Utc::now().timestamp() as usize;

    const REFRESH_GRACE_PERIOD_SECONDS: usize = 7 * 24 * 60 * 60;
    if claims.exp + REFRESH_GRACE_PERIOD_SECONDS < now {
        log::warn!("Refresh token rejected: token expired beyond grace period");
        return Err(AppError::Unauthorized("token已过期，请重新登录".into()));
    }

    let user: User = sqlx::query_as("SELECT * FROM users WHERE id = ?")
        .bind(claims.sub).fetch_optional(db).await?
        .ok_or_else(|| {
            log::warn!("Refresh token rejected: user_id={} not found", claims.sub);
            AppError::Unauthorized("用户不存在".into())
        })?;

    if user.status == 1 {
        log::warn!("Refresh token rejected: user_id={} is disabled", claims.sub);
        return Err(AppError::Forbidden("账号已被禁用".into()));
    }

    let exp = (Utc::now() + chrono::Duration::hours(JWT_EXPIRATION_HOURS)).timestamp() as usize;
    let new_claims = Claims {
        sub: user.id,
        role: user.role,
        username: user.username.clone(),
        exp,
    };
    let new_token = encode(&Header::default(), &new_claims, &EncodingKey::from_secret(jwt_secret.as_bytes()))?;
    log::info!("Refresh token successful: user_id={}, username={}", user.id, user.username);
    Ok(new_token)
}
