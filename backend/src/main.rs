use actix_cors::Cors;
use actix_web::{web, App, HttpServer, HttpResponse, middleware::Logger};
use sqlx::mysql::MySqlPoolOptions;
use std::env;

use talent_backend::AppState;

async fn api_docs() -> HttpResponse {
    let html = include_str!("api_docs.html");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn serve_upload(path: web::Path<String>) -> HttpResponse {
    let filename = path.into_inner();
    // 防止路径穿越
    if filename.contains("..") || filename.contains('/') {
        return HttpResponse::BadRequest().finish();
    }
    let dir = talent_backend::config::upload_dir();
    let filepath = std::path::Path::new(&dir).join(&filename);
    match std::fs::read(&filepath) {
        Ok(data) => {
            let content_type = match filepath.extension().and_then(|e| e.to_str()) {
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("png") => "image/png",
                Some("gif") => "image/gif",
                Some("webp") => "image/webp",
                Some("bmp") => "image/bmp",
                Some("tiff") | Some("tif") => "image/tiff",
                _ => "application/octet-stream",
            };
            HttpResponse::Ok()
                .content_type(content_type)
                .insert_header(("Cache-Control", "public, max-age=86400"))
                .body(data)
        }
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    // 生产安全检查：弱密钥警告
    let weak_patterns = ["CHANGE_ME", "talent_net_secret", "example", "not_for_production"];
    let is_weak = jwt_secret.len() < 32 || weak_patterns.iter().any(|p| jwt_secret.contains(p));
    if is_weak {
        log::warn!("============================================================");
        log::warn!("⚠️  JWT_SECRET 不安全！当前值为示例/弱密钥。");
        log::warn!("⚠️  生产环境请设置 64 字符以上的随机密钥，例如：");
        log::warn!("⚠️  openssl rand -base64 48");
        log::warn!("============================================================");
    }
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    log::info!("Database connected successfully");

    let app_state = web::Data::new(AppState {
        db: pool,
        jwt_secret,
    });

    log::info!("Starting server at {}:{}", host, port);

    HttpServer::new(move || {
        // CORS: 通过 CORS_ORIGIN 环境变量配置允许的域名
        // 未设置时默认允许所有来源（开发/演示环境）
        // 生产环境应设置为具体域名，如 CORS_ORIGIN=https://example.com
        let cors = match env::var("CORS_ORIGIN") {
            Ok(origin) if !origin.is_empty() => {
                let mut cors = Cors::default();
                for o in origin.split(',') {
                    cors = cors.allowed_origin(o.trim());
                }
                cors.allow_any_method().allow_any_header().max_age(3600)
            }
            _ => Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600),
        };

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(app_state.clone())
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                let detail = err.to_string();
                actix_web::error::InternalError::from_response(
                    err,
                    actix_web::HttpResponse::BadRequest().json(
                        serde_json::json!({"code": 400, "message": detail}),
                    ),
                )
                .into()
            }))
            .configure(talent_backend::routes::configure)
            .service(actix_web::web::scope("/uploads").service(actix_web::web::resource("/{filename:.*}").to(serve_upload)))
            .route("/api-docs", web::get().to(api_docs))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
