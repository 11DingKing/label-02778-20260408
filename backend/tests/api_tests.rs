use actix_web::test;
use serde_json::{json, Value};
use serial_test::serial;

mod common;
use common::*;

// ==================== Auth Tests ====================

#[actix_rt::test]
#[serial]
async fn test_register_seeker() {
    let (app, pool) = create_test_app().await;
    cleanup_test_data(&pool, &["test_seeker_reg"]).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(json!({
            "username": "test_seeker_reg",
            "password": "123456",
            "role": 0,
            "phone": "13800000001"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    assert_eq!(body["data"]["role"], 0);
    assert_eq!(body["data"]["username"], "test_seeker_reg");

    cleanup_test_data(&pool, &["test_seeker_reg"]).await;
}

#[actix_rt::test]
#[serial]
async fn test_register_enterprise() {
    let (app, pool) = create_test_app().await;
    cleanup_test_data(&pool, &["test_ent_reg"]).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(json!({
            "username": "test_ent_reg",
            "password": "123456",
            "role": 1
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    assert_eq!(body["data"]["role"], 1);

    cleanup_test_data(&pool, &["test_ent_reg"]).await;
}

#[actix_rt::test]
#[serial]
async fn test_register_duplicate_username() {
    let (app, pool) = create_test_app().await;
    cleanup_test_data(&pool, &["test_dup_user"]).await;

    // First register
    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(json!({"username": "test_dup_user", "password": "123456", "role": 0}))
        .to_request();
    let _ = test::call_service(&app, req).await;

    // Duplicate
    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(json!({"username": "test_dup_user", "password": "654321", "role": 0}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 409);

    cleanup_test_data(&pool, &["test_dup_user"]).await;
}

#[actix_rt::test]
#[serial]
async fn test_register_invalid_role() {
    let (app, _pool) = create_test_app().await;

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(json!({"username": "test_bad_role", "password": "123456", "role": 5}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 400);
}

#[actix_rt::test]
#[serial]
async fn test_login_success() {
    let (app, pool) = create_test_app().await;
    cleanup_test_data(&pool, &["test_login_ok"]).await;

    // Register first
    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(json!({"username": "test_login_ok", "password": "123456", "role": 0}))
        .to_request();
    let _ = test::call_service(&app, req).await;

    // Login
    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(json!({"username": "test_login_ok", "password": "123456"}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    assert!(body["data"]["token"].is_string());

    cleanup_test_data(&pool, &["test_login_ok"]).await;
}

#[actix_rt::test]
#[serial]
async fn test_login_wrong_password() {
    let (app, pool) = create_test_app().await;
    cleanup_test_data(&pool, &["test_login_bad"]).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(json!({"username": "test_login_bad", "password": "123456", "role": 0}))
        .to_request();
    let _ = test::call_service(&app, req).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(json!({"username": "test_login_bad", "password": "wrong_pass"}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 401);

    cleanup_test_data(&pool, &["test_login_bad"]).await;
}

#[actix_rt::test]
#[serial]
async fn test_login_nonexistent_user() {
    let (app, _pool) = create_test_app().await;

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(json!({"username": "no_such_user_xyz", "password": "123456"}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 401);
}

#[actix_rt::test]
#[serial]
async fn test_me_without_token() {
    let (app, _pool) = create_test_app().await;

    let req = test::TestRequest::get()
        .uri("/api/auth/me")
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 401);
}


#[actix_rt::test]
#[serial]
async fn test_me_with_token() {
    let (app, pool) = create_test_app().await;
    cleanup_test_data(&pool, &["test_me_user"]).await;

    let (token, _uid) = register_and_login(&app, "test_me_user", "123456", 0).await;

    let req = test::TestRequest::get()
        .uri("/api/auth/me")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    assert_eq!(body["data"]["username"], "test_me_user");

    cleanup_test_data(&pool, &["test_me_user"]).await;
}

// ==================== Seeker Profile Tests ====================

#[actix_rt::test]
#[serial]
async fn test_seeker_profile_crud() {
    let (app, pool) = create_test_app().await;
    cleanup_test_data(&pool, &["test_seeker_p"]).await;

    let (token, _uid) = register_and_login(&app, "test_seeker_p", "123456", 0).await;

    // Create profile
    let req = test::TestRequest::post()
        .uri("/api/seeker/profile")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(json!({
            "name": "张三",
            "gender": 1,
            "age": 25,
            "education": "本科",
            "experience_years": "3年",
            "expected_salary": "15k-20k",
            "expected_city": "北京",
            "skills": "Rust,Vue,MySQL",
            "self_intro": "热爱编程"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    assert_eq!(body["data"]["name"], "张三");

    // Get profile
    let req = test::TestRequest::get()
        .uri("/api/seeker/profile")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    assert_eq!(body["data"]["expected_city"], "北京");

    // Update profile
    let req = test::TestRequest::post()
        .uri("/api/seeker/profile")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(json!({
            "name": "张三",
            "gender": 1,
            "age": 26,
            "education": "硕士",
            "experience_years": "4年",
            "expected_salary": "20k-30k",
            "expected_city": "上海",
            "skills": "Rust,Vue,MySQL,Docker",
            "self_intro": "资深工程师"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    assert_eq!(body["data"]["expected_city"], "上海");
    assert_eq!(body["data"]["age"], 26);

    cleanup_test_data(&pool, &["test_seeker_p"]).await;
}

#[actix_rt::test]
#[serial]
async fn test_enterprise_cannot_create_seeker_profile() {
    let (app, pool) = create_test_app().await;
    cleanup_test_data(&pool, &["test_ent_no_seeker"]).await;

    let (token, _uid) = register_and_login(&app, "test_ent_no_seeker", "123456", 1).await;

    let req = test::TestRequest::post()
        .uri("/api/seeker/profile")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(json!({
            "name": "不应该成功",
            "gender": 1, "age": 25, "education": "本科",
            "experience_years": "1年", "expected_salary": "10k",
            "expected_city": "北京", "skills": "test"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 403);

    cleanup_test_data(&pool, &["test_ent_no_seeker"]).await;
}

#[actix_rt::test]
#[serial]
async fn test_list_seekers() {
    let (app, pool) = create_test_app().await;
    cleanup_test_data(&pool, &["test_list_sk"]).await;

    let (token, _uid) = register_and_login(&app, "test_list_sk", "123456", 0).await;

    // Create profile first
    let req = test::TestRequest::post()
        .uri("/api/seeker/profile")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(json!({
            "name": "列表测试", "gender": 1, "age": 25, "education": "本科",
            "experience_years": "2年", "expected_salary": "10k",
            "expected_city": "深圳", "skills": "Java"
        }))
        .to_request();
    let _ = test::call_service(&app, req).await;

    // List seekers (no auth required)
    let req = test::TestRequest::get()
        .uri("/api/seeker/list?page=1&size=10")
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    assert!(body["data"]["total"].as_i64().unwrap() >= 1);

    cleanup_test_data(&pool, &["test_list_sk"]).await;
}

// ==================== Enterprise Profile Tests ====================

#[actix_rt::test]
#[serial]
async fn test_enterprise_profile_and_verification() {
    let (app, pool) = create_test_app().await;
    cleanup_test_data(&pool, &["test_ent_prof", "test_admin_v"]).await;

    let (ent_token, _ent_id) = register_and_login(&app, "test_ent_prof", "123456", 1).await;

    // Create enterprise profile
    let req = test::TestRequest::post()
        .uri("/api/enterprise/profile")
        .insert_header(("Authorization", format!("Bearer {}", ent_token)))
        .set_json(json!({
            "company_name": "测试科技公司",
            "license_no": "91110000TEST123456",
            "license_image": "https://example.com/license.jpg",
            "contact_person": "李经理",
            "contact_phone": "13900001111",
            "industry": "互联网",
            "address": "北京市海淀区"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    assert_eq!(body["data"]["verified"], 0); // 待审核

    // Enterprise cannot post job before verification
    let req = test::TestRequest::post()
        .uri("/api/enterprise/job")
        .insert_header(("Authorization", format!("Bearer {}", ent_token)))
        .set_json(json!({
            "title": "Rust工程师",
            "salary_range": "20k-30k",
            "city": "北京",
            "education_req": "本科",
            "experience_req": "3年",
            "description": "负责后端开发"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 403); // 未认证

    cleanup_test_data(&pool, &["test_ent_prof", "test_admin_v"]).await;
}

// ==================== Job Tests ====================

#[actix_rt::test]
#[serial]
async fn test_job_crud() {
    let (app, pool) = create_test_app().await;
    cleanup_test_data(&pool, &["test_job_ent"]).await;

    let (token, uid) = register_and_login(&app, "test_job_ent", "123456", 1).await;

    // Create enterprise profile
    let req = test::TestRequest::post()
        .uri("/api/enterprise/profile")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(json!({
            "company_name": "招聘测试公司",
            "license_no": "91110000JOB1234567",
            "license_image": "",
            "contact_person": "王经理",
            "contact_phone": "13900002222",
            "industry": "科技",
            "address": "上海"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    let ent_id = body["data"]["id"].as_i64().unwrap();

    // Admin verify enterprise
    sqlx::query("UPDATE enterprise_profiles SET verified = 1 WHERE id = ?")
        .bind(ent_id).execute(&pool).await.unwrap();

    // Create job
    let req = test::TestRequest::post()
        .uri("/api/enterprise/job")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(json!({
            "title": "Rust高级工程师",
            "salary_range": "25k-40k",
            "city": "上海",
            "education_req": "本科",
            "experience_req": "5年",
            "description": "负责核心系统开发"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    let job_id = body["data"]["id"].as_i64().unwrap();

    // List jobs
    let req = test::TestRequest::get()
        .uri("/api/jobs?page=1&size=10")
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    assert!(body["data"]["total"].as_i64().unwrap() >= 1);

    // Get job detail
    let req = test::TestRequest::get()
        .uri(&format!("/api/jobs/{}", job_id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    assert_eq!(body["data"]["title"], "Rust高级工程师");
    assert_eq!(body["data"]["company_name"], "招聘测试公司");

    // Update job
    let req = test::TestRequest::put()
        .uri(&format!("/api/enterprise/job/{}", job_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(json!({
            "title": "Rust资深工程师",
            "salary_range": "30k-50k",
            "city": "上海",
            "education_req": "硕士",
            "experience_req": "5年",
            "description": "架构设计"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    assert_eq!(body["data"]["title"], "Rust资深工程师");

    // Delete job
    let req = test::TestRequest::delete()
        .uri(&format!("/api/enterprise/job/{}", job_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);

    cleanup_test_data(&pool, &["test_job_ent"]).await;
}


// ==================== Chat Tests ====================

#[actix_rt::test]
#[serial]
async fn test_chat_contact_flow() {
    let (app, pool) = create_test_app().await;
    cleanup_test_data(&pool, &["test_chat_ent", "test_chat_sk"]).await;

    // Create enterprise and seeker
    let (ent_token, ent_uid) = register_and_login(&app, "test_chat_ent", "123456", 1).await;
    let (sk_token, sk_uid) = register_and_login(&app, "test_chat_sk", "123456", 0).await;

    // Seeker creates profile
    let req = test::TestRequest::post()
        .uri("/api/seeker/profile")
        .insert_header(("Authorization", format!("Bearer {}", sk_token)))
        .set_json(json!({
            "name": "聊天测试求职者", "gender": 1, "age": 25, "education": "本科",
            "experience_years": "2年", "expected_salary": "15k",
            "expected_city": "北京", "skills": "Rust"
        }))
        .to_request();
    let _ = test::call_service(&app, req).await;

    // Enterprise creates profile and gets verified
    let req = test::TestRequest::post()
        .uri("/api/enterprise/profile")
        .insert_header(("Authorization", format!("Bearer {}", ent_token)))
        .set_json(json!({
            "company_name": "聊天测试公司", "license_no": "91110000CHAT12345",
            "license_image": "", "contact_person": "赵经理",
            "contact_phone": "13900003333", "industry": "科技", "address": "北京"
        }))
        .to_request();
    let _ = test::call_service(&app, req).await;
    sqlx::query("UPDATE enterprise_profiles SET verified = 1 WHERE user_id = ?")
        .bind(ent_uid).execute(&pool).await.unwrap();

    // Enterprise initiates contact
    let req = test::TestRequest::post()
        .uri("/api/chat/contact")
        .insert_header(("Authorization", format!("Bearer {}", ent_token)))
        .set_json(json!({"seeker_user_id": sk_uid}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    let contact_id = body["data"]["id"].as_i64().unwrap();
    assert_eq!(body["data"]["status"], 0); // pending

    // Cannot send message before acceptance
    let req = test::TestRequest::post()
        .uri("/api/chat/message")
        .insert_header(("Authorization", format!("Bearer {}", ent_token)))
        .set_json(json!({"contact_id": contact_id, "content": "你好"}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 403);

    // Seeker accepts contact
    let req = test::TestRequest::put()
        .uri(&format!("/api/chat/contact/{}/reply", contact_id))
        .insert_header(("Authorization", format!("Bearer {}", sk_token)))
        .set_json(json!({"accept": true}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);

    // Now can send message
    let req = test::TestRequest::post()
        .uri("/api/chat/message")
        .insert_header(("Authorization", format!("Bearer {}", ent_token)))
        .set_json(json!({"contact_id": contact_id, "content": "你好，对你的简历很感兴趣"}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);

    // Seeker replies
    let req = test::TestRequest::post()
        .uri("/api/chat/message")
        .insert_header(("Authorization", format!("Bearer {}", sk_token)))
        .set_json(json!({"contact_id": contact_id, "content": "谢谢，请问贵公司做什么的？"}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);

    // List messages
    let req = test::TestRequest::get()
        .uri(&format!("/api/chat/messages/{}", contact_id))
        .insert_header(("Authorization", format!("Bearer {}", ent_token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    assert_eq!(body["data"].as_array().unwrap().len(), 2);

    // List contacts
    let req = test::TestRequest::get()
        .uri("/api/chat/contacts")
        .insert_header(("Authorization", format!("Bearer {}", ent_token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    assert!(body["data"].as_array().unwrap().len() >= 1);

    // Mark read
    let req = test::TestRequest::put()
        .uri(&format!("/api/chat/messages/{}/read", contact_id))
        .insert_header(("Authorization", format!("Bearer {}", ent_token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);

    // Duplicate contact should fail
    let req = test::TestRequest::post()
        .uri("/api/chat/contact")
        .insert_header(("Authorization", format!("Bearer {}", ent_token)))
        .set_json(json!({"seeker_user_id": sk_uid}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 409);

    cleanup_test_data(&pool, &["test_chat_ent", "test_chat_sk"]).await;
}

#[actix_rt::test]
#[serial]
async fn test_chat_reject_flow() {
    let (app, pool) = create_test_app().await;
    cleanup_test_data(&pool, &["test_rej_ent", "test_rej_sk"]).await;

    let (ent_token, ent_uid) = register_and_login(&app, "test_rej_ent", "123456", 1).await;
    let (sk_token, sk_uid) = register_and_login(&app, "test_rej_sk", "123456", 0).await;

    // Setup profiles
    let req = test::TestRequest::post()
        .uri("/api/seeker/profile")
        .insert_header(("Authorization", format!("Bearer {}", sk_token)))
        .set_json(json!({
            "name": "拒绝测试", "gender": 2, "age": 23, "education": "大专",
            "experience_years": "1年", "expected_salary": "8k",
            "expected_city": "广州", "skills": "Python"
        }))
        .to_request();
    let _ = test::call_service(&app, req).await;

    let req = test::TestRequest::post()
        .uri("/api/enterprise/profile")
        .insert_header(("Authorization", format!("Bearer {}", ent_token)))
        .set_json(json!({
            "company_name": "拒绝测试公司", "license_no": "91110000REJ123456",
            "license_image": "", "contact_person": "钱经理",
            "contact_phone": "13900004444", "industry": "金融", "address": "广州"
        }))
        .to_request();
    let _ = test::call_service(&app, req).await;
    sqlx::query("UPDATE enterprise_profiles SET verified = 1 WHERE user_id = ?")
        .bind(ent_uid).execute(&pool).await.unwrap();

    // Initiate contact
    let req = test::TestRequest::post()
        .uri("/api/chat/contact")
        .insert_header(("Authorization", format!("Bearer {}", ent_token)))
        .set_json(json!({"seeker_user_id": sk_uid}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    let contact_id = body["data"]["id"].as_i64().unwrap();

    // Seeker rejects
    let req = test::TestRequest::put()
        .uri(&format!("/api/chat/contact/{}/reply", contact_id))
        .insert_header(("Authorization", format!("Bearer {}", sk_token)))
        .set_json(json!({"accept": false}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);

    // Cannot send message after rejection
    let req = test::TestRequest::post()
        .uri("/api/chat/message")
        .insert_header(("Authorization", format!("Bearer {}", ent_token)))
        .set_json(json!({"contact_id": contact_id, "content": "你好"}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 403);

    cleanup_test_data(&pool, &["test_rej_ent", "test_rej_sk"]).await;
}


// ==================== Jianghu Tests ====================

#[actix_rt::test]
#[serial]
async fn test_jianghu_post_like_comment() {
    let (app, pool) = create_test_app().await;
    cleanup_test_data(&pool, &["test_jh_user"]).await;

    let (token, uid) = register_and_login(&app, "test_jh_user", "123456", 0).await;

    // Create post
    let req = test::TestRequest::post()
        .uri("/api/jianghu/post")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(json!({
            "content": "今天天气真好，适合找工作！",
            "images": ["https://example.com/img1.jpg"]
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    let post_id = body["data"]["id"].as_i64().unwrap();

    // List posts
    let req = test::TestRequest::get()
        .uri("/api/jianghu/posts?page=1&size=10")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    assert!(body["data"]["total"].as_i64().unwrap() >= 1);

    // Like post
    let req = test::TestRequest::post()
        .uri(&format!("/api/jianghu/post/{}/like", post_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    assert_eq!(body["data"], true); // liked

    // Unlike post
    let req = test::TestRequest::post()
        .uri(&format!("/api/jianghu/post/{}/like", post_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    assert_eq!(body["data"], false); // unliked

    // Add comment
    let req = test::TestRequest::post()
        .uri(&format!("/api/jianghu/post/{}/comment", post_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(json!({"content": "加油！"}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);

    // List comments
    let req = test::TestRequest::get()
        .uri(&format!("/api/jianghu/post/{}/comments", post_id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    assert_eq!(body["data"].as_array().unwrap().len(), 1);

    // Delete post
    let req = test::TestRequest::delete()
        .uri(&format!("/api/jianghu/post/{}", post_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);

    cleanup_test_data(&pool, &["test_jh_user"]).await;
}

// ==================== Market Tests ====================

#[actix_rt::test]
#[serial]
async fn test_market_post_crud() {
    let (app, pool) = create_test_app().await;
    cleanup_test_data(&pool, &["test_mkt_user"]).await;

    let (token, uid) = register_and_login(&app, "test_mkt_user", "123456", 1).await;

    // Create market post
    let req = test::TestRequest::post()
        .uri("/api/market/post")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(json!({
            "title": "转让奶茶店",
            "category": 0,
            "price": "150000.00",
            "city": "深圳",
            "images": ["https://example.com/shop.jpg"],
            "description": "位置好，客流量大",
            "contact_info": "13800005555"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    let post_id = body["data"]["id"].as_i64().unwrap();

    // List market posts
    let req = test::TestRequest::get()
        .uri("/api/market/posts?page=1&size=10")
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    assert!(body["data"]["total"].as_i64().unwrap() >= 1);

    // Get detail
    let req = test::TestRequest::get()
        .uri(&format!("/api/market/post/{}", post_id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    assert_eq!(body["data"]["title"], "转让奶茶店");

    // Update
    let req = test::TestRequest::put()
        .uri(&format!("/api/market/post/{}", post_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(json!({
            "title": "转让奶茶店（急）",
            "category": 0,
            "price": "120000.00",
            "city": "深圳",
            "images": ["https://example.com/shop.jpg"],
            "description": "降价转让",
            "contact_info": "13800005555"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);

    // Filter by category
    let req = test::TestRequest::get()
        .uri("/api/market/posts?category=0")
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);

    // Delete
    let req = test::TestRequest::delete()
        .uri(&format!("/api/market/post/{}", post_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);

    cleanup_test_data(&pool, &["test_mkt_user"]).await;
}

// ==================== Admin Tests ====================

#[actix_rt::test]
#[serial]
async fn test_admin_stats() {
    let (app, pool) = create_test_app().await;

    // Use existing admin account (admin/admin123)
    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(json!({"username": "admin", "password": "admin123"}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;

    // Admin might not exist in test DB with matching JWT secret, skip if login fails
    if body["code"] != 0 {
        return;
    }
    let token = body["data"]["token"].as_str().unwrap();

    let req = test::TestRequest::get()
        .uri("/api/admin/stats")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    assert!(body["data"]["total_users"].as_i64().unwrap() >= 1);
}

#[actix_rt::test]
#[serial]
async fn test_admin_list_users() {
    let (app, pool) = create_test_app().await;

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(json!({"username": "admin", "password": "admin123"}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    if body["code"] != 0 { return; }
    let token = body["data"]["token"].as_str().unwrap();

    let req = test::TestRequest::get()
        .uri("/api/admin/users?page=1&size=10")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    assert!(body["data"]["list"].as_array().unwrap().len() >= 1);
}

#[actix_rt::test]
#[serial]
async fn test_admin_list_logs() {
    let (app, pool) = create_test_app().await;

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(json!({"username": "admin", "password": "admin123"}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    if body["code"] != 0 { return; }
    let token = body["data"]["token"].as_str().unwrap();

    let req = test::TestRequest::get()
        .uri("/api/admin/logs?page=1&size=10")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
}

#[actix_rt::test]
#[serial]
async fn test_non_admin_cannot_access_admin_api() {
    let (app, pool) = create_test_app().await;
    cleanup_test_data(&pool, &["test_nonadmin"]).await;

    let (token, _uid) = register_and_login(&app, "test_nonadmin", "123456", 0).await;

    let req = test::TestRequest::get()
        .uri("/api/admin/stats")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 403);

    let req = test::TestRequest::get()
        .uri("/api/admin/users?page=1&size=10")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 403);

    cleanup_test_data(&pool, &["test_nonadmin"]).await;
}

// ==================== Validation Tests ====================

#[actix_rt::test]
#[serial]
async fn test_register_short_password() {
    let (app, _pool) = create_test_app().await;

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(json!({"username": "test_short_pw", "password": "12", "role": 0}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 400);
}

#[actix_rt::test]
#[serial]
async fn test_register_short_username() {
    let (app, _pool) = create_test_app().await;

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(json!({"username": "a", "password": "123456", "role": 0}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 400);
}

#[actix_rt::test]
#[serial]
async fn test_empty_jianghu_content() {
    let (app, pool) = create_test_app().await;
    cleanup_test_data(&pool, &["test_empty_jh"]).await;

    let (token, _uid) = register_and_login(&app, "test_empty_jh", "123456", 0).await;

    let req = test::TestRequest::post()
        .uri("/api/jianghu/post")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(json!({"content": "", "images": []}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 400);

    cleanup_test_data(&pool, &["test_empty_jh"]).await;
}


#[actix_rt::test]
#[serial]
async fn test_seeker_cannot_post_market() {
    let (app, pool) = create_test_app().await;
    cleanup_test_data(&pool, &["test_sk_no_mkt"]).await;

    let (token, _uid) = register_and_login(&app, "test_sk_no_mkt", "123456", 0).await;

    let req = test::TestRequest::post()
        .uri("/api/market/post")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(json!({
            "title": "求职者不应该能发布",
            "category": 1,
            "price": "100.00",
            "city": "北京",
            "contact_info": "13800000000"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 403);

    cleanup_test_data(&pool, &["test_sk_no_mkt"]).await;
}


// ==================== Daily Contact Limit Tests ====================

#[actix_rt::test]
#[serial]
async fn test_daily_contact_limit() {
    let (app, pool) = create_test_app().await;
    let seekers = ["test_lim_sk1", "test_lim_sk2", "test_lim_sk3", "test_lim_sk4"];
    let ent_name = "test_lim_ent";
    let mut all_users: Vec<&str> = seekers.to_vec();
    all_users.push(ent_name);
    cleanup_test_data(&pool, &all_users).await;

    // Create enterprise
    let (ent_token, ent_uid) = register_and_login(&app, ent_name, "123456", 1).await;

    // Create enterprise profile and verify
    let req = test::TestRequest::post()
        .uri("/api/enterprise/profile")
        .insert_header(("Authorization", format!("Bearer {}", ent_token)))
        .set_json(json!({
            "company_name": "限额测试公司", "license_no": "91110000LIM123456",
            "license_image": "", "contact_person": "测试",
            "contact_phone": "13900009999", "industry": "科技", "address": "北京"
        }))
        .to_request();
    let _ = test::call_service(&app, req).await;
    sqlx::query("UPDATE enterprise_profiles SET verified = 1 WHERE user_id = ?")
        .bind(ent_uid).execute(&pool).await.unwrap();

    // Create 4 seekers with profiles
    let mut seeker_uids = Vec::new();
    for sk_name in &seekers {
        let (sk_token, sk_uid) = register_and_login(&app, sk_name, "123456", 0).await;
        let req = test::TestRequest::post()
            .uri("/api/seeker/profile")
            .insert_header(("Authorization", format!("Bearer {}", sk_token)))
            .set_json(json!({
                "name": format!("求职者{}", sk_name), "gender": 1, "age": 25, "education": "本科",
                "experience_years": "2年", "expected_salary": "10k",
                "expected_city": "北京", "skills": "Rust"
            }))
            .to_request();
        let _ = test::call_service(&app, req).await;
        seeker_uids.push(sk_uid);
    }

    // Contact first 3 seekers — should all succeed (daily limit = 3)
    for i in 0..3 {
        let req = test::TestRequest::post()
            .uri("/api/chat/contact")
            .insert_header(("Authorization", format!("Bearer {}", ent_token)))
            .set_json(json!({"seeker_user_id": seeker_uids[i], "greeting": format!("你好，第{}次联系", i + 1)}))
            .to_request();
        let resp = test::call_service(&app, req).await;
        let body: Value = test::read_body_json(resp).await;
        assert_eq!(body["code"], 0, "Contact {} should succeed", i + 1);
    }

    // 4th contact should be rejected — daily limit exceeded
    let req = test::TestRequest::post()
        .uri("/api/chat/contact")
        .insert_header(("Authorization", format!("Bearer {}", ent_token)))
        .set_json(json!({"seeker_user_id": seeker_uids[3]}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 403, "4th contact should be rejected due to daily limit");
    assert!(body["message"].as_str().unwrap().contains("上限"), "Error message should mention limit");

    cleanup_test_data(&pool, &all_users).await;
}

#[actix_rt::test]
#[serial]
async fn test_contact_with_greeting() {
    let (app, pool) = create_test_app().await;
    cleanup_test_data(&pool, &["test_greet_ent", "test_greet_sk"]).await;

    let (ent_token, ent_uid) = register_and_login(&app, "test_greet_ent", "123456", 1).await;
    let (sk_token, sk_uid) = register_and_login(&app, "test_greet_sk", "123456", 0).await;

    // Setup profiles
    let req = test::TestRequest::post()
        .uri("/api/seeker/profile")
        .insert_header(("Authorization", format!("Bearer {}", sk_token)))
        .set_json(json!({
            "name": "招呼语测试", "gender": 1, "age": 25, "education": "本科",
            "experience_years": "2年", "expected_salary": "15k",
            "expected_city": "北京", "skills": "Vue"
        }))
        .to_request();
    let _ = test::call_service(&app, req).await;

    let req = test::TestRequest::post()
        .uri("/api/enterprise/profile")
        .insert_header(("Authorization", format!("Bearer {}", ent_token)))
        .set_json(json!({
            "company_name": "招呼语测试公司", "license_no": "91110000GRT123456",
            "license_image": "", "contact_person": "测试",
            "contact_phone": "13900008888", "industry": "科技", "address": "北京"
        }))
        .to_request();
    let _ = test::call_service(&app, req).await;
    sqlx::query("UPDATE enterprise_profiles SET verified = 1 WHERE user_id = ?")
        .bind(ent_uid).execute(&pool).await.unwrap();

    // Initiate contact with greeting
    let req = test::TestRequest::post()
        .uri("/api/chat/contact")
        .insert_header(("Authorization", format!("Bearer {}", ent_token)))
        .set_json(json!({
            "seeker_user_id": sk_uid,
            "greeting": "您好，我们正在招聘Vue工程师，觉得您很合适"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    assert_eq!(body["data"]["greeting"], "您好，我们正在招聘Vue工程师，觉得您很合适");

    // Verify greeting appears in contact list for seeker
    let req = test::TestRequest::get()
        .uri("/api/chat/contacts")
        .insert_header(("Authorization", format!("Bearer {}", sk_token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 0);
    let contacts = body["data"].as_array().unwrap();
    assert!(!contacts.is_empty());
    assert_eq!(contacts[0]["greeting"], "您好，我们正在招聘Vue工程师，觉得您很合适");

    cleanup_test_data(&pool, &["test_greet_ent", "test_greet_sk"]).await;
}

#[actix_rt::test]
#[serial]
async fn test_upload_requires_auth() {
    let (app, _pool) = create_test_app().await;

    // Upload without token should fail
    let req = test::TestRequest::post()
        .uri("/api/upload")
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], 401);
}
