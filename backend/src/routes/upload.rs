use actix_web::{web, HttpRequest, HttpResponse};
use actix_multipart::Multipart;
use futures_util::StreamExt;
use crate::config::upload_dir;
use crate::errors::AppError;
use crate::middleware::extract_claims;
use crate::models::ApiResp;
use crate::AppState;

/// 通过文件头魔数校验真实文件类型，防止伪装扩展名绕过
fn validate_file_magic(bytes: &[u8]) -> Option<&'static str> {
    if bytes.len() < 4 {
        return None;
    }
    if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
        return Some("jpeg");
    }
    if bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
        return Some("png");
    }
    if bytes.starts_with(&[0x47, 0x49, 0x46, 0x38]) {
        return Some("gif");
    }
    if bytes.len() >= 12 && bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WEBP" {
        return Some("webp");
    }
    // BMP: 'BM' header
    if bytes.starts_with(&[0x42, 0x4D]) {
        return Some("bmp");
    }
    // TIFF: little-endian (II) or big-endian (MM)
    if bytes.len() >= 4 && ((&bytes[0..2] == b"II" && bytes[2] == 0x2A && bytes[3] == 0x00)
        || (&bytes[0..2] == b"MM" && bytes[2] == 0x00 && bytes[3] == 0x2A)) {
        return Some("tiff");
    }
    None
}

pub async fn upload_file(req: HttpRequest, state: web::Data<AppState>, mut payload: Multipart) -> Result<HttpResponse, AppError> {
    let _claims = extract_claims(&req, &state)?;

    let mut urls: Vec<String> = Vec::new();
    while let Some(Ok(mut field)) = payload.next().await {
        let content_disposition = field.content_disposition().cloned();
        let filename = content_disposition.as_ref().and_then(|cd| cd.get_filename()).unwrap_or("file");

        let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();
        if !["jpg", "jpeg", "png", "gif", "webp", "bmp", "tiff", "tif"].contains(&ext.as_str()) {
            return Err(AppError::BadRequest("仅支持 jpg/jpeg/png/gif/webp/bmp/tiff 格式图片".into()));
        }

        let dir = upload_dir();
        let upload_path = std::path::Path::new(&dir);
        if !upload_path.exists() {
            std::fs::create_dir_all(upload_path).map_err(|e| AppError::Internal(format!("创建上传目录失败: {}", e)))?;
        }

        let mut file_bytes = Vec::new();
        while let Some(Ok(chunk)) = field.next().await {
            if file_bytes.len() + chunk.len() > 5 * 1024 * 1024 {
                return Err(AppError::BadRequest("文件大小不能超过 5MB".into()));
            }
            file_bytes.extend_from_slice(&chunk);
        }

        let real_type = validate_file_magic(&file_bytes)
            .ok_or_else(|| AppError::BadRequest("文件内容不是有效的图片格式（魔数校验失败）".into()))?;

        let new_filename = format!("{}_{}.{}", chrono::Utc::now().format("%Y%m%d%H%M%S"), uuid::Uuid::new_v4().to_string().split('-').next().unwrap_or("x"), real_type);

        let filepath = upload_path.join(&new_filename);
        std::fs::write(&filepath, &file_bytes).map_err(|e| AppError::Internal(format!("保存文件失败: {}", e)))?;

        urls.push(format!("/uploads/{}", new_filename));
    }

    if urls.is_empty() {
        return Err(AppError::BadRequest("未上传任何文件".into()));
    }

    Ok(HttpResponse::Ok().json(ApiResp::ok(urls)))
}
