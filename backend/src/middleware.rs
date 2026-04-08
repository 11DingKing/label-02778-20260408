use actix_web::{HttpRequest, web};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use crate::errors::AppError;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i64,       // user_id
    pub role: i8,       // 0=seeker, 1=enterprise, 2=admin
    pub username: String,
    pub exp: usize,
}

pub fn extract_claims(req: &HttpRequest, state: &web::Data<AppState>) -> Result<Claims, AppError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("缺少认证信息".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("认证格式错误".to_string()))?;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(token_data.claims)
}

pub fn require_role(claims: &Claims, allowed_roles: &[i8]) -> Result<(), AppError> {
    if !allowed_roles.contains(&claims.role) {
        return Err(AppError::Forbidden("权限不足".to_string()));
    }
    Ok(())
}
