use crate::config::*;
use crate::errors::AppError;
use crate::models::*;
use sqlx::MySqlPool;

pub async fn upsert_seeker_profile(db: &MySqlPool, user_id: i64, req: SeekerProfileReq) -> Result<JobSeekerProfile, AppError> {
    let existing: Option<JobSeekerProfile> = sqlx::query_as(
        "SELECT * FROM job_seeker_profiles WHERE user_id = ?"
    ).bind(user_id).fetch_optional(db).await?;

    if existing.is_some() {
        sqlx::query(
            "UPDATE job_seeker_profiles SET name=?, gender=?, age=?, education=?, experience_years=?, expected_salary=?, expected_city=?, skills=?, self_intro=? WHERE user_id=?"
        ).bind(&req.name).bind(req.gender).bind(req.age).bind(&req.education)
            .bind(&req.experience_years).bind(&req.expected_salary).bind(&req.expected_city)
            .bind(&req.skills).bind(&req.self_intro).bind(user_id)
            .execute(db).await?;
    } else {
        sqlx::query(
            "INSERT INTO job_seeker_profiles (user_id, name, gender, age, education, experience_years, expected_salary, expected_city, skills, self_intro) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        ).bind(user_id).bind(&req.name).bind(req.gender).bind(req.age).bind(&req.education)
            .bind(&req.experience_years).bind(&req.expected_salary).bind(&req.expected_city)
            .bind(&req.skills).bind(&req.self_intro)
            .execute(db).await?;
    }

    let profile: JobSeekerProfile = sqlx::query_as("SELECT * FROM job_seeker_profiles WHERE user_id = ?")
        .bind(user_id).fetch_one(db).await?;
    log::info!("Seeker profile upserted for user_id={}", user_id);
    Ok(profile)
}

pub async fn get_seeker_profile(db: &MySqlPool, user_id: i64) -> Result<Option<JobSeekerProfile>, AppError> {
    let profile = sqlx::query_as("SELECT * FROM job_seeker_profiles WHERE user_id = ?")
        .bind(user_id).fetch_optional(db).await?;
    Ok(profile)
}

pub async fn list_seekers(db: &MySqlPool, query: &PageQuery) -> Result<PageResp<JobSeekerProfile>, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let size = query.size.unwrap_or(DEFAULT_PAGE_SIZE).min(MAX_PAGE_SIZE);
    let offset = (page - 1) * size;
    let keyword = query.keyword.as_deref().unwrap_or("");
    let like_pattern = format!("%{}%", keyword);

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM job_seeker_profiles WHERE status = 0 AND (name LIKE ? OR skills LIKE ? OR expected_city LIKE ?)"
    ).bind(&like_pattern).bind(&like_pattern).bind(&like_pattern).fetch_one(db).await?;

    let list: Vec<JobSeekerProfile> = sqlx::query_as(
        "SELECT * FROM job_seeker_profiles WHERE status = 0 AND (name LIKE ? OR skills LIKE ? OR expected_city LIKE ?) ORDER BY updated_at DESC LIMIT ? OFFSET ?"
    ).bind(&like_pattern).bind(&like_pattern).bind(&like_pattern)
        .bind(size).bind(offset).fetch_all(db).await?;

    Ok(PageResp { list, total: total.0, page, size })
}

pub async fn get_seeker_detail(db: &MySqlPool, id: i64) -> Result<SeekerDetailVO, AppError> {
    sqlx::query_as(
        "SELECT p.*, u.phone FROM job_seeker_profiles p JOIN users u ON u.id = p.user_id WHERE p.id = ? AND p.status = 0"
    )
        .bind(id).fetch_optional(db).await?
        .ok_or_else(|| AppError::NotFound("求职者不存在".into()))
}
