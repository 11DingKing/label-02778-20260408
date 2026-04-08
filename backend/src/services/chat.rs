use crate::config::*;
use crate::errors::AppError;
use crate::models::*;
use chrono::Local;
use sqlx::MySqlPool;

use super::{get_user, get_verified_enterprise};

pub async fn initiate_contact(db: &MySqlPool, enterprise_user_id: i64, seeker_user_id: i64, greeting: Option<String>) -> Result<ChatContact, AppError> {
    log::info!("Contact initiation: enterprise_user={} -> seeker_user={}", enterprise_user_id, seeker_user_id);

    if enterprise_user_id == seeker_user_id {
        return Err(AppError::BadRequest("不能联系自己".into()));
    }

    let _seeker_user = get_user(db, seeker_user_id).await?;
    if _seeker_user.role != 0 || _seeker_user.status != 0 {
        return Err(AppError::NotFound("求职者不存在或已被禁用".into()));
    }

    let _enterprise = get_verified_enterprise(db, enterprise_user_id).await?;

    let _seeker: JobSeekerProfile = sqlx::query_as(
        "SELECT * FROM job_seeker_profiles WHERE user_id = ? AND status = 0"
    ).bind(seeker_user_id).fetch_optional(db).await?
        .ok_or_else(|| AppError::NotFound("求职者档案不存在".into()))?;

    let mut tx = db.begin().await?;

    let existing: Option<ChatContact> = sqlx::query_as(
        "SELECT * FROM chat_contacts WHERE enterprise_user_id = ? AND seeker_user_id = ?"
    ).bind(enterprise_user_id).bind(seeker_user_id).fetch_optional(&mut *tx).await?;
    if existing.is_some() {
        tx.rollback().await?;
        return Err(AppError::Conflict("已发起过联系".into()));
    }

    let today = Local::now().date_naive();
    let daily: Option<(i32,)> = sqlx::query_as(
        "SELECT count FROM daily_contact_count WHERE enterprise_user_id = ? AND contact_date = ? FOR UPDATE"
    ).bind(enterprise_user_id).bind(today).fetch_optional(&mut *tx).await?;

    let current_count = daily.map(|d| d.0).unwrap_or(0);
    let limit = daily_contact_limit();
    if current_count >= limit {
        tx.rollback().await?;
        return Err(AppError::Forbidden(format!("今日联系次数已达上限({}次)", limit)));
    }

    let greeting_text = greeting.unwrap_or_default();
    let result = sqlx::query(
        "INSERT INTO chat_contacts (enterprise_user_id, seeker_user_id, contact_date, greeting) VALUES (?, ?, ?, ?)"
    ).bind(enterprise_user_id).bind(seeker_user_id).bind(today).bind(&greeting_text)
        .execute(&mut *tx).await?;

    sqlx::query(
        "INSERT INTO daily_contact_count (enterprise_user_id, contact_date, count) VALUES (?, ?, 1) ON DUPLICATE KEY UPDATE count = count + 1"
    ).bind(enterprise_user_id).bind(today).execute(&mut *tx).await?;

    let contact: ChatContact = sqlx::query_as("SELECT * FROM chat_contacts WHERE id = ?")
        .bind(result.last_insert_id()).fetch_one(&mut *tx).await?;

    tx.commit().await?;
    log::info!("Contact initiated: enterprise={} -> seeker={}", enterprise_user_id, seeker_user_id);
    Ok(contact)
}

pub async fn reply_contact(db: &MySqlPool, user_id: i64, contact_id: i64, accept: bool) -> Result<(), AppError> {
    let contact: ChatContact = sqlx::query_as(
        "SELECT * FROM chat_contacts WHERE id = ? AND seeker_user_id = ?"
    ).bind(contact_id).bind(user_id).fetch_optional(db).await?
        .ok_or_else(|| AppError::NotFound("联系请求不存在".into()))?;

    if contact.status != 0 {
        return Err(AppError::BadRequest("已回复过该请求".into()));
    }

    let new_status: i8 = if accept { 1 } else { 2 };
    sqlx::query("UPDATE chat_contacts SET status = ? WHERE id = ?")
        .bind(new_status).bind(contact_id).execute(db).await?;
    log::info!("Contact {} replied: accept={}", contact_id, accept);
    Ok(())
}

pub async fn list_contacts(db: &MySqlPool, user_id: i64, role: i8) -> Result<Vec<ChatContactVO>, AppError> {
    let contacts: Vec<ChatContact> = if role == 1 {
        sqlx::query_as("SELECT * FROM chat_contacts WHERE enterprise_user_id = ? ORDER BY created_at DESC")
            .bind(user_id).fetch_all(db).await?
    } else {
        sqlx::query_as("SELECT * FROM chat_contacts WHERE seeker_user_id = ? ORDER BY created_at DESC")
            .bind(user_id).fetch_all(db).await?
    };

    let mut result = Vec::new();
    for c in contacts {
        let other_id = if role == 1 { c.seeker_user_id } else { c.enterprise_user_id };
        let other: User = sqlx::query_as("SELECT * FROM users WHERE id = ?")
            .bind(other_id).fetch_one(db).await?;
        let unread: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM chat_messages WHERE contact_id = ? AND sender_id != ? AND is_read = 0"
        ).bind(c.id).bind(user_id).fetch_one(db).await?;

        result.push(ChatContactVO {
            id: c.id,
            enterprise_user_id: c.enterprise_user_id,
            seeker_user_id: c.seeker_user_id,
            status: c.status,
            contact_date: c.contact_date,
            greeting: c.greeting,
            created_at: c.created_at,
            other_username: other.username,
            other_avatar: other.avatar,
            unread_count: unread.0,
        });
    }
    Ok(result)
}

pub async fn send_message(db: &MySqlPool, user_id: i64, req: SendMessageReq) -> Result<ChatMessage, AppError> {
    let contact: ChatContact = sqlx::query_as("SELECT * FROM chat_contacts WHERE id = ?")
        .bind(req.contact_id).fetch_optional(db).await?
        .ok_or_else(|| AppError::NotFound("联系不存在".into()))?;

    if user_id != contact.enterprise_user_id && user_id != contact.seeker_user_id {
        return Err(AppError::Forbidden("无权发送消息".into()));
    }

    if contact.status == 0 && user_id == contact.seeker_user_id {
        sqlx::query("UPDATE chat_contacts SET status = 1 WHERE id = ?")
            .bind(contact.id).execute(db).await?;
        log::info!("Contact {} auto-accepted by seeker reply", contact.id);
    } else if contact.status != 1 {
        return Err(AppError::Forbidden("对方尚未接受联系请求，无法发送消息".into()));
    }

    let result = sqlx::query(
        "INSERT INTO chat_messages (contact_id, sender_id, content) VALUES (?, ?, ?)"
    ).bind(req.contact_id).bind(user_id).bind(&req.content)
        .execute(db).await?;

    let msg: ChatMessage = sqlx::query_as("SELECT * FROM chat_messages WHERE id = ?")
        .bind(result.last_insert_id()).fetch_one(db).await?;
    Ok(msg)
}

pub async fn list_messages(db: &MySqlPool, user_id: i64, contact_id: i64) -> Result<Vec<ChatMessage>, AppError> {
    let contact: ChatContact = sqlx::query_as("SELECT * FROM chat_contacts WHERE id = ?")
        .bind(contact_id).fetch_optional(db).await?
        .ok_or_else(|| AppError::NotFound("联系不存在".into()))?;

    if user_id != contact.enterprise_user_id && user_id != contact.seeker_user_id {
        return Err(AppError::Forbidden("无权查看消息".into()));
    }

    let messages: Vec<ChatMessage> = sqlx::query_as(
        "SELECT * FROM chat_messages WHERE contact_id = ? ORDER BY created_at ASC"
    ).bind(contact_id).fetch_all(db).await?;
    Ok(messages)
}

pub async fn mark_read(db: &MySqlPool, user_id: i64, contact_id: i64) -> Result<(), AppError> {
    sqlx::query("UPDATE chat_messages SET is_read = 1 WHERE contact_id = ? AND sender_id != ?")
        .bind(contact_id).bind(user_id).execute(db).await?;
    Ok(())
}
