use crate::config::*;
use crate::errors::AppError;
use crate::models::*;
use chrono::NaiveDateTime;
use sqlx::MySqlPool;

use super::{get_verified_enterprise, get_enterprise_profile_or_err};

pub async fn create_job(db: &MySqlPool, user_id: i64, req: JobPostReq) -> Result<JobPost, AppError> {
    let enterprise = get_verified_enterprise(db, user_id).await?;

    let result = sqlx::query(
        "INSERT INTO job_posts (enterprise_id, title, salary_range, city, education_req, experience_req, description) VALUES (?, ?, ?, ?, ?, ?, ?)"
    ).bind(enterprise.id).bind(&req.title).bind(&req.salary_range).bind(&req.city)
        .bind(&req.education_req).bind(&req.experience_req).bind(&req.description)
        .execute(db).await?;

    let job: JobPost = sqlx::query_as("SELECT * FROM job_posts WHERE id = ?")
        .bind(result.last_insert_id()).fetch_one(db).await?;
    log::info!("Job created: {} by enterprise_id={}", job.title, enterprise.id);
    Ok(job)
}

pub async fn update_job(db: &MySqlPool, user_id: i64, job_id: i64, req: JobPostReq) -> Result<JobPost, AppError> {
    let enterprise = get_verified_enterprise(db, user_id).await?;

    let job: JobPost = sqlx::query_as("SELECT * FROM job_posts WHERE id = ? AND enterprise_id = ?")
        .bind(job_id).bind(enterprise.id).fetch_optional(db).await?
        .ok_or_else(|| AppError::NotFound("职位不存在".into()))?;

    sqlx::query(
        "UPDATE job_posts SET title=?, salary_range=?, city=?, education_req=?, experience_req=?, description=? WHERE id=?"
    ).bind(&req.title).bind(&req.salary_range).bind(&req.city)
        .bind(&req.education_req).bind(&req.experience_req).bind(&req.description).bind(job.id)
        .execute(db).await?;

    let updated: JobPost = sqlx::query_as("SELECT * FROM job_posts WHERE id = ?")
        .bind(job.id).fetch_one(db).await?;
    log::info!("Job updated: id={}", job.id);
    Ok(updated)
}

pub async fn delete_job(db: &MySqlPool, user_id: i64, job_id: i64) -> Result<(), AppError> {
    let enterprise = get_verified_enterprise(db, user_id).await?;

    let result = sqlx::query("DELETE FROM job_posts WHERE id = ? AND enterprise_id = ?")
        .bind(job_id).bind(enterprise.id).execute(db).await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("职位不存在".into()));
    }
    log::info!("Job deleted: id={}", job_id);
    Ok(())
}

pub async fn admin_delete_job(db: &MySqlPool, job_id: i64) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM job_posts WHERE id = ?")
        .bind(job_id).execute(db).await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("职位不存在".into()));
    }
    log::info!("Admin deleted job: id={}", job_id);
    Ok(())
}

pub async fn list_my_jobs(db: &MySqlPool, user_id: i64, query: &PageQuery) -> Result<PageResp<JobPost>, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let size = query.size.unwrap_or(DEFAULT_PAGE_SIZE).min(MAX_PAGE_SIZE);
    let offset = (page - 1) * size;

    let eid = match get_enterprise_profile_or_err(db, user_id).await {
        Ok(e) => e.id,
        Err(_) => 0,
    };

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM job_posts WHERE enterprise_id = ?")
        .bind(eid).fetch_one(db).await?;

    let list: Vec<JobPost> = sqlx::query_as(
        "SELECT * FROM job_posts WHERE enterprise_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
    ).bind(eid).bind(size).bind(offset).fetch_all(db).await?;

    Ok(PageResp { list, total: total.0, page, size })
}

pub async fn list_jobs(db: &MySqlPool, query: &PageQuery) -> Result<PageResp<JobPostVO>, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let size = query.size.unwrap_or(DEFAULT_PAGE_SIZE).min(MAX_PAGE_SIZE);
    let offset = (page - 1) * size;
    let keyword = query.keyword.as_deref().unwrap_or("");
    let like_pattern = format!("%{}%", keyword);
    let city = query.city.as_deref().unwrap_or("");
    let city_pattern = format!("%{}%", city);

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM job_posts j JOIN enterprise_profiles e ON j.enterprise_id = e.id WHERE j.status = 0 AND e.verified = 1 AND (j.title LIKE ? OR j.city LIKE ?) AND j.city LIKE ?"
    ).bind(&like_pattern).bind(&like_pattern).bind(&city_pattern).fetch_one(db).await?;

    let rows: Vec<(i64, i64, String, String, String, String, String, Option<String>, i8, NaiveDateTime, NaiveDateTime, String)> = sqlx::query_as(
        "SELECT j.id, j.enterprise_id, j.title, j.salary_range, j.city, j.education_req, j.experience_req, j.description, j.status, j.created_at, j.updated_at, e.company_name FROM job_posts j JOIN enterprise_profiles e ON j.enterprise_id = e.id WHERE j.status = 0 AND e.verified = 1 AND (j.title LIKE ? OR j.city LIKE ?) AND j.city LIKE ? ORDER BY j.created_at DESC LIMIT ? OFFSET ?"
    ).bind(&like_pattern).bind(&like_pattern).bind(&city_pattern).bind(size).bind(offset).fetch_all(db).await?;

    let list: Vec<JobPostVO> = rows.into_iter().map(|r| JobPostVO {
        post: JobPost {
            id: r.0, enterprise_id: r.1, title: r.2, salary_range: r.3, city: r.4,
            education_req: r.5, experience_req: r.6, description: r.7, status: r.8,
            created_at: r.9, updated_at: r.10,
        },
        company_name: Some(r.11),
        contact_person: None,
        contact_phone: None,
        industry: None,
        address: None,
    }).collect();

    Ok(PageResp { list, total: total.0, page, size })
}

pub async fn get_job_detail(db: &MySqlPool, id: i64) -> Result<JobPostVO, AppError> {
    let row: (i64, i64, String, String, String, String, String, Option<String>, i8, NaiveDateTime, NaiveDateTime, String, String, String, String, String) = sqlx::query_as(
        "SELECT j.id, j.enterprise_id, j.title, j.salary_range, j.city, j.education_req, j.experience_req, j.description, j.status, j.created_at, j.updated_at, e.company_name, e.contact_person, e.contact_phone, e.industry, e.address FROM job_posts j JOIN enterprise_profiles e ON j.enterprise_id = e.id WHERE j.id = ?"
    ).bind(id).fetch_optional(db).await?
        .ok_or_else(|| AppError::NotFound("职位不存在".into()))?;

    Ok(JobPostVO {
        post: JobPost {
            id: row.0, enterprise_id: row.1, title: row.2, salary_range: row.3, city: row.4,
            education_req: row.5, experience_req: row.6, description: row.7, status: row.8,
            created_at: row.9, updated_at: row.10,
        },
        company_name: Some(row.11),
        contact_person: Some(row.12),
        contact_phone: Some(row.13),
        industry: Some(row.14),
        address: Some(row.15),
    })
}
