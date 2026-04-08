use actix_web::{test, web, App};
use serde_json::{json, Value};
use sqlx::mysql::MySqlPoolOptions;
use talent_backend::AppState;

pub async fn create_test_app() -> (
    impl actix_web::dev::Service<
        actix_http::Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    >,
    sqlx::MySqlPool,
) {
    dotenv::dotenv().ok();
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or("mysql://talent:talent123@127.0.0.1:3306/talent_db".to_string());

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    let state = web::Data::new(AppState {
        db: pool.clone(),
        jwt_secret: "test_jwt_secret_key_for_testing".to_string(),
    });

    let app = test::init_service(
        App::new()
            .app_data(state)
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                let detail = err.to_string();
                actix_web::error::InternalError::from_response(
                    err,
                    actix_web::HttpResponse::BadRequest()
                        .json(json!({"code": 400, "message": detail})),
                )
                .into()
            }))
            .configure(talent_backend::routes::configure),
    )
    .await;

    (app, pool)
}

/// 注册用户并返回 (token, user_id)
pub async fn register_and_login(
    app: &impl actix_web::dev::Service<
        actix_http::Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    >,
    username: &str,
    password: &str,
    role: i8,
) -> (String, i64) {
    // Register
    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(json!({
            "username": username,
            "password": password,
            "role": role
        }))
        .to_request();
    let resp = test::call_service(app, req).await;
    assert!(resp.status().is_success(), "Register failed for {}", username);

    // Login
    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(json!({
            "username": username,
            "password": password
        }))
        .to_request();
    let resp = test::call_service(app, req).await;
    assert!(resp.status().is_success(), "Login failed for {}", username);
    let body: Value = test::read_body_json(resp).await;
    let token = body["data"]["token"].as_str().unwrap().to_string();
    let user_id = body["data"]["user"]["id"].as_i64().unwrap();
    (token, user_id)
}

/// 清理测试数据
pub async fn cleanup_test_data(pool: &sqlx::MySqlPool, usernames: &[&str]) {
    for username in usernames {
        // 获取 user_id
        let user: Option<(i64,)> =
            sqlx::query_as("SELECT id FROM users WHERE username = ?")
                .bind(username)
                .fetch_optional(pool)
                .await
                .unwrap();

        if let Some((uid,)) = user {
            // 清理关联数据
            let _ = sqlx::query("DELETE FROM chat_messages WHERE contact_id IN (SELECT id FROM chat_contacts WHERE enterprise_user_id = ? OR seeker_user_id = ?)")
                .bind(uid).bind(uid).execute(pool).await;
            let _ = sqlx::query("DELETE FROM chat_contacts WHERE enterprise_user_id = ? OR seeker_user_id = ?")
                .bind(uid).bind(uid).execute(pool).await;
            let _ = sqlx::query("DELETE FROM daily_contact_count WHERE enterprise_user_id = ?")
                .bind(uid).execute(pool).await;
            let _ = sqlx::query("DELETE FROM jianghu_likes WHERE user_id = ?")
                .bind(uid).execute(pool).await;
            let _ = sqlx::query("DELETE FROM jianghu_comments WHERE user_id = ?")
                .bind(uid).execute(pool).await;
            let _ = sqlx::query("DELETE FROM jianghu_posts WHERE user_id = ?")
                .bind(uid).execute(pool).await;
            let _ = sqlx::query("DELETE FROM market_posts WHERE user_id = ?")
                .bind(uid).execute(pool).await;
            let _ = sqlx::query("DELETE FROM job_posts WHERE enterprise_id IN (SELECT id FROM enterprise_profiles WHERE user_id = ?)")
                .bind(uid).execute(pool).await;
            let _ = sqlx::query("DELETE FROM enterprise_profiles WHERE user_id = ?")
                .bind(uid).execute(pool).await;
            let _ = sqlx::query("DELETE FROM job_seeker_profiles WHERE user_id = ?")
                .bind(uid).execute(pool).await;
            let _ = sqlx::query("DELETE FROM operation_logs WHERE user_id = ?")
                .bind(uid).execute(pool).await;
            let _ = sqlx::query("DELETE FROM users WHERE id = ?")
                .bind(uid).execute(pool).await;
        }
    }
}
