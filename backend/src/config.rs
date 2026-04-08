/// 每日企业联系求职者上限（可通过 DAILY_CONTACT_LIMIT 环境变量覆盖）
pub fn daily_contact_limit() -> i32 {
    std::env::var("DAILY_CONTACT_LIMIT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3)
}

/// 上传文件目录（可通过 UPLOAD_DIR 环境变量覆盖）
pub fn upload_dir() -> String {
    std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "uploads".to_string())
}

/// JWT 过期时间（小时）
pub const JWT_EXPIRATION_HOURS: i64 = 72;

/// 分页默认值
pub const DEFAULT_PAGE_SIZE: u32 = 20;
pub const MAX_PAGE_SIZE: u32 = 100;
