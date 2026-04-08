use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

// ========== User ==========
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub phone: String,
    pub avatar: String,
    pub role: i8,
    pub status: i8,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserVO {
    pub id: i64,
    pub username: String,
    pub phone: String,
    pub avatar: String,
    pub role: i8,
    pub status: i8,
    pub created_at: NaiveDateTime,
}

impl From<User> for UserVO {
    fn from(u: User) -> Self {
        UserVO {
            id: u.id,
            username: u.username,
            phone: u.phone,
            avatar: u.avatar,
            role: u.role,
            status: u.status,
            created_at: u.created_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterReq {
    #[validate(length(min = 2, max = 50, message = "用户名长度2-50"))]
    pub username: String,
    #[validate(length(min = 6, max = 50, message = "密码长度6-50"))]
    pub password: String,
    pub role: i8, // 0 or 1
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginReq {
    #[validate(length(min = 1, message = "用户名不能为空"))]
    pub username: String,
    #[validate(length(min = 1, message = "密码不能为空"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResp {
    pub token: String,
    pub user: UserVO,
}

// ========== JobSeekerProfile ==========
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct JobSeekerProfile {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub gender: i8,
    pub age: i32,
    pub education: String,
    pub experience_years: String,
    pub expected_salary: String,
    pub expected_city: String,
    pub skills: String,
    pub self_intro: Option<String>,
    pub status: i8,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SeekerProfileReq {
    #[validate(length(min = 1, max = 50, message = "姓名长度1-50"))]
    pub name: String,
    pub gender: i8,
    #[validate(range(min = 16, max = 65, message = "年龄范围16-65"))]
    pub age: i32,
    #[validate(length(min = 1, max = 20, message = "学历不能为空"))]
    pub education: String,
    #[validate(length(min = 1, max = 20, message = "工作年限不能为空"))]
    pub experience_years: String,
    #[validate(length(min = 1, max = 50, message = "期望薪资不能为空"))]
    pub expected_salary: String,
    #[validate(length(min = 1, max = 50, message = "期望城市不能为空"))]
    pub expected_city: String,
    #[validate(length(min = 1, max = 500, message = "技能不能为空，长度1-500"))]
    pub skills: String,
    pub self_intro: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SeekerDetailVO {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub gender: i8,
    pub age: i32,
    pub education: String,
    pub experience_years: String,
    pub expected_salary: String,
    pub expected_city: String,
    pub skills: String,
    pub self_intro: Option<String>,
    pub status: i8,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub phone: String,
}

// ========== EnterpriseProfile ==========
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct EnterpriseProfile {
    pub id: i64,
    pub user_id: i64,
    pub company_name: String,
    pub license_no: String,
    pub license_image: String,
    pub contact_person: String,
    pub contact_phone: String,
    pub industry: String,
    pub address: String,
    pub verified: i8,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Validate)]
pub struct EnterpriseProfileReq {
    #[validate(length(min = 1, max = 100, message = "公司名称长度1-100"))]
    pub company_name: String,
    #[validate(length(min = 1, max = 50, message = "营业执照号长度1-50"))]
    pub license_no: String,
    pub license_image: String,
    #[validate(length(min = 1, max = 50, message = "联系人不能为空，长度1-50"))]
    pub contact_person: String,
    #[validate(length(min = 1, max = 20, message = "联系电话不能为空，长度1-20"))]
    pub contact_phone: String,
    #[validate(length(min = 1, max = 50, message = "行业不能为空，长度1-50"))]
    pub industry: String,
    #[validate(length(min = 1, max = 200, message = "地址不能为空，长度1-200"))]
    pub address: String,
}

// ========== JobPost ==========
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct JobPost {
    pub id: i64,
    pub enterprise_id: i64,
    pub title: String,
    pub salary_range: String,
    pub city: String,
    pub education_req: String,
    pub experience_req: String,
    pub description: Option<String>,
    pub status: i8,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct JobPostVO {
    #[serde(flatten)]
    pub post: JobPost,
    pub company_name: Option<String>,
    pub contact_person: Option<String>,
    pub contact_phone: Option<String>,
    pub industry: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct JobPostReq {
    #[validate(length(min = 1, max = 100, message = "职位名称长度1-100"))]
    pub title: String,
    #[validate(length(min = 1, max = 50, message = "薪资范围不能为空"))]
    pub salary_range: String,
    #[validate(length(min = 1, max = 50, message = "城市不能为空"))]
    pub city: String,
    #[validate(length(min = 1, max = 20, message = "学历要求不能为空"))]
    pub education_req: String,
    #[validate(length(min = 1, max = 20, message = "经验要求不能为空"))]
    pub experience_req: String,
    pub description: Option<String>,
}

// ========== Chat ==========
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChatContact {
    pub id: i64,
    pub enterprise_user_id: i64,
    pub seeker_user_id: i64,
    pub status: i8,
    pub contact_date: chrono::NaiveDate,
    pub greeting: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct ChatContactVO {
    pub id: i64,
    pub enterprise_user_id: i64,
    pub seeker_user_id: i64,
    pub status: i8,
    pub contact_date: chrono::NaiveDate,
    pub greeting: String,
    pub created_at: NaiveDateTime,
    pub other_username: String,
    pub other_avatar: String,
    pub unread_count: i64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChatMessage {
    pub id: i64,
    pub contact_id: i64,
    pub sender_id: i64,
    pub content: String,
    pub is_read: i8,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SendMessageReq {
    pub contact_id: i64,
    #[validate(length(min = 1, message = "消息不能为空"))]
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ContactReq {
    pub seeker_user_id: i64,
    pub greeting: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReplyReq {
    pub accept: bool,
}

// ========== Jianghu ==========
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct JianghuPost {
    pub id: i64,
    pub user_id: Option<i64>,
    pub content: String,
    pub images: String,
    pub like_count: i32,
    pub comment_count: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct JianghuPostVO {
    pub id: i64,
    pub user_id: Option<i64>,
    pub content: String,
    pub images: Vec<String>,
    pub like_count: i32,
    pub comment_count: i32,
    pub created_at: NaiveDateTime,
    pub username: String,
    pub avatar: String,
    pub liked: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct JianghuPostReq {
    #[validate(length(min = 1, message = "内容不能为空"))]
    pub content: String,
    pub images: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct JianghuComment {
    pub id: i64,
    pub post_id: i64,
    pub user_id: i64,
    pub content: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct JianghuCommentVO {
    pub id: i64,
    pub user_id: i64,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub username: String,
    pub avatar: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CommentReq {
    #[validate(length(min = 1, message = "评论不能为空"))]
    pub content: String,
}

// ========== Market ==========
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct MarketPost {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub category: i8,
    pub price: rust_decimal::Decimal,
    pub city: String,
    pub images: String,
    pub description: Option<String>,
    pub contact_info: String,
    pub status: i8,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct MarketPostVO {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub category: i8,
    pub price: rust_decimal::Decimal,
    pub city: String,
    pub images: Vec<String>,
    pub description: Option<String>,
    pub contact_info: String,
    pub status: i8,
    pub created_at: NaiveDateTime,
    pub username: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct MarketPostReq {
    #[validate(length(min = 1, max = 100, message = "标题长度1-100"))]
    pub title: String,
    pub category: i8,
    #[validate(custom(function = "validate_price"))]
    pub price: rust_decimal::Decimal,
    #[validate(length(min = 1, max = 50, message = "城市不能为空，长度1-50"))]
    pub city: String,
    pub images: Option<Vec<String>>,
    pub description: Option<String>,
    #[validate(length(min = 1, max = 100, message = "联系方式不能为空，长度1-100"))]
    pub contact_info: String,
}

fn validate_price(price: &rust_decimal::Decimal) -> Result<(), validator::ValidationError> {
    use rust_decimal::Decimal;
    if *price < Decimal::ZERO {
        let mut err = validator::ValidationError::new("price_negative");
        err.message = Some("价格不能为负数".into());
        return Err(err);
    }
    if *price > Decimal::from(100_000_000) {
        let mut err = validator::ValidationError::new("price_too_large");
        err.message = Some("价格不能超过1亿".into());
        return Err(err);
    }
    Ok(())
}

// ========== Common ==========
#[derive(Debug, Deserialize)]
pub struct PageQuery {
    pub page: Option<u32>,
    pub size: Option<u32>,
    pub keyword: Option<String>,
    pub category: Option<i8>,
    pub city: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PageResp<T: Serialize> {
    pub list: Vec<T>,
    pub total: i64,
    pub page: u32,
    pub size: u32,
}

#[derive(Debug, Serialize)]
pub struct ApiResp<T: Serialize> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResp<T> {
    pub fn ok(data: T) -> Self {
        ApiResp { code: 0, message: "success".to_string(), data: Some(data) }
    }
    pub fn ok_msg(msg: &str) -> ApiResp<()> {
        ApiResp { code: 0, message: msg.to_string(), data: None }
    }
}

// ========== OperationLog ==========
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct OperationLog {
    pub id: i64,
    pub user_id: i64,
    pub action: String,
    pub target: String,
    pub ip: String,
    pub created_at: NaiveDateTime,
}

// ========== Admin Stats ==========
#[derive(Debug, Serialize)]
pub struct AdminStats {
    pub total_users: i64,
    pub total_seekers: i64,
    pub total_enterprises: i64,
    pub total_jobs: i64,
    pub total_jianghu: i64,
    pub total_market: i64,
    pub pending_verify: i64,
}
