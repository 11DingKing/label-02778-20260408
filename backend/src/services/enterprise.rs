use crate::errors::AppError;
use crate::models::*;
use sqlx::MySqlPool;

pub async fn upsert_enterprise_profile(db: &MySqlPool, user_id: i64, req: EnterpriseProfileReq) -> Result<EnterpriseProfile, AppError> {
    let existing: Option<EnterpriseProfile> = sqlx::query_as(
        "SELECT * FROM enterprise_profiles WHERE user_id = ?"
    ).bind(user_id).fetch_optional(db).await?;

    if existing.is_some() {
        sqlx::query(
            "UPDATE enterprise_profiles SET company_name=?, license_no=?, license_image=?, contact_person=?, contact_phone=?, industry=?, address=?, verified=0 WHERE user_id=?"
        ).bind(&req.company_name).bind(&req.license_no).bind(&req.license_image)
            .bind(&req.contact_person).bind(&req.contact_phone).bind(&req.industry)
            .bind(&req.address).bind(user_id)
            .execute(db).await?;
    } else {
        sqlx::query(
            "INSERT INTO enterprise_profiles (user_id, company_name, license_no, license_image, contact_person, contact_phone, industry, address) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        ).bind(user_id).bind(&req.company_name).bind(&req.license_no).bind(&req.license_image)
            .bind(&req.contact_person).bind(&req.contact_phone).bind(&req.industry).bind(&req.address)
            .execute(db).await?;
    }

    let profile: EnterpriseProfile = sqlx::query_as("SELECT * FROM enterprise_profiles WHERE user_id = ?")
        .bind(user_id).fetch_one(db).await?;
    log::info!("Enterprise profile upserted for user_id={}", user_id);
    Ok(profile)
}

pub async fn get_enterprise_profile(db: &MySqlPool, user_id: i64) -> Result<Option<EnterpriseProfile>, AppError> {
    let profile = sqlx::query_as("SELECT * FROM enterprise_profiles WHERE user_id = ?")
        .bind(user_id).fetch_optional(db).await?;
    Ok(profile)
}
