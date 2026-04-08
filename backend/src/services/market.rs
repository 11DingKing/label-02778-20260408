use crate::config::*;
use crate::errors::AppError;
use crate::models::*;
use chrono::NaiveDateTime;
use sqlx::MySqlPool;

use super::get_user;

pub async fn create_market_post(db: &MySqlPool, user_id: i64, req: MarketPostReq) -> Result<MarketPost, AppError> {
    let user = get_user(db, user_id).await?;
    if user.role != 1 {
        return Err(AppError::Forbidden("仅企业用户可发布商品".into()));
    }

    let images_str = req.images.map(|v| v.join(",")).unwrap_or_default();
    let result = sqlx::query(
        "INSERT INTO market_posts (user_id, title, category, price, city, images, description, contact_info) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    ).bind(user_id).bind(&req.title).bind(req.category).bind(&req.price)
        .bind(&req.city).bind(&images_str).bind(&req.description).bind(&req.contact_info)
        .execute(db).await?;

    let post: MarketPost = sqlx::query_as("SELECT * FROM market_posts WHERE id = ?")
        .bind(result.last_insert_id()).fetch_one(db).await?;
    log::info!("Market post created: id={} by user_id={}", post.id, user_id);
    Ok(post)
}

pub async fn list_market_posts(db: &MySqlPool, query: &PageQuery) -> Result<PageResp<MarketPostVO>, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let size = query.size.unwrap_or(DEFAULT_PAGE_SIZE).min(MAX_PAGE_SIZE);
    let offset = (page - 1) * size;
    let keyword = query.keyword.as_deref().unwrap_or("");
    let like_pattern = format!("%{}%", keyword);

    let mut where_clause = "m.status = 0 AND (m.title LIKE ? OR m.city LIKE ?)".to_string();
    let category = query.category;
    if category.is_some() {
        where_clause.push_str(" AND m.category = ?");
    }

    let total: (i64,) = if let Some(cat) = category {
        sqlx::query_as(&format!("SELECT COUNT(*) FROM market_posts m WHERE {}", where_clause))
            .bind(&like_pattern).bind(&like_pattern).bind(cat).fetch_one(db).await?
    } else {
        sqlx::query_as(&format!("SELECT COUNT(*) FROM market_posts m WHERE {}", where_clause))
            .bind(&like_pattern).bind(&like_pattern).fetch_one(db).await?
    };

    let sql = format!(
        "SELECT m.*, u.username FROM market_posts m JOIN users u ON m.user_id = u.id WHERE {} ORDER BY m.created_at DESC LIMIT ? OFFSET ?",
        where_clause
    );

    let rows: Vec<(i64, i64, String, i8, rust_decimal::Decimal, String, String, Option<String>, String, i8, NaiveDateTime, NaiveDateTime, String)> = if let Some(cat) = category {
        sqlx::query_as(&sql)
            .bind(&like_pattern).bind(&like_pattern).bind(cat).bind(size).bind(offset)
            .fetch_all(db).await?
    } else {
        sqlx::query_as(&sql)
            .bind(&like_pattern).bind(&like_pattern).bind(size).bind(offset)
            .fetch_all(db).await?
    };

    let list: Vec<MarketPostVO> = rows.into_iter().map(|r| {
        let images: Vec<String> = if r.6.is_empty() { vec![] } else { r.6.split(',').map(|s| s.to_string()).collect() };
        MarketPostVO {
            id: r.0, user_id: r.1, title: r.2, category: r.3, price: r.4,
            city: r.5, images, description: r.7, contact_info: r.8,
            status: r.9, created_at: r.10, username: r.12,
        }
    }).collect();

    Ok(PageResp { list, total: total.0, page, size })
}

pub async fn get_market_detail(db: &MySqlPool, id: i64) -> Result<MarketPostVO, AppError> {
    let row: (i64, i64, String, i8, rust_decimal::Decimal, String, String, Option<String>, String, i8, NaiveDateTime, NaiveDateTime, String) = sqlx::query_as(
        "SELECT m.*, u.username FROM market_posts m JOIN users u ON m.user_id = u.id WHERE m.id = ?"
    ).bind(id).fetch_optional(db).await?
        .ok_or_else(|| AppError::NotFound("商品不存在".into()))?;

    let images: Vec<String> = if row.6.is_empty() { vec![] } else { row.6.split(',').map(|s| s.to_string()).collect() };
    Ok(MarketPostVO {
        id: row.0, user_id: row.1, title: row.2, category: row.3, price: row.4,
        city: row.5, images, description: row.7, contact_info: row.8,
        status: row.9, created_at: row.10, username: row.12,
    })
}

pub async fn update_market_post(db: &MySqlPool, user_id: i64, id: i64, req: MarketPostReq) -> Result<(), AppError> {
    let user = get_user(db, user_id).await?;
    if user.role != 1 {
        return Err(AppError::Forbidden("仅企业用户可编辑商品".into()));
    }

    let post: MarketPost = sqlx::query_as("SELECT * FROM market_posts WHERE id = ? AND user_id = ?")
        .bind(id).bind(user_id).fetch_optional(db).await?
        .ok_or_else(|| AppError::NotFound("商品不存在".into()))?;

    let images_str = req.images.map(|v| v.join(",")).unwrap_or_default();
    sqlx::query(
        "UPDATE market_posts SET title=?, category=?, price=?, city=?, images=?, description=?, contact_info=? WHERE id=?"
    ).bind(&req.title).bind(req.category).bind(&req.price).bind(&req.city)
        .bind(&images_str).bind(&req.description).bind(&req.contact_info).bind(post.id)
        .execute(db).await?;
    log::info!("Market post updated: id={}", id);
    Ok(())
}

pub async fn delete_market_post(db: &MySqlPool, user_id: i64, id: i64, is_admin: bool) -> Result<(), AppError> {
    let post: MarketPost = sqlx::query_as("SELECT * FROM market_posts WHERE id = ?")
        .bind(id).fetch_optional(db).await?
        .ok_or_else(|| AppError::NotFound("商品不存在".into()))?;

    if !is_admin && post.user_id != user_id {
        return Err(AppError::Forbidden("无权删除".into()));
    }

    sqlx::query("DELETE FROM market_posts WHERE id = ?").bind(id).execute(db).await?;
    log::info!("Market post deleted: id={}", id);
    Ok(())
}

pub async fn admin_list_market_posts(db: &MySqlPool, query: &PageQuery) -> Result<PageResp<MarketPostVO>, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let size = query.size.unwrap_or(DEFAULT_PAGE_SIZE).min(MAX_PAGE_SIZE);
    let offset = (page - 1) * size;
    let keyword = query.keyword.as_deref().unwrap_or("");
    let like_pattern = format!("%{}%", keyword);

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM market_posts m WHERE m.title LIKE ? OR m.city LIKE ?"
    ).bind(&like_pattern).bind(&like_pattern).fetch_one(db).await?;

    let rows: Vec<(i64, i64, String, i8, rust_decimal::Decimal, String, String, Option<String>, String, i8, NaiveDateTime, NaiveDateTime, String)> =
        sqlx::query_as(
            "SELECT m.*, u.username FROM market_posts m JOIN users u ON m.user_id = u.id WHERE m.title LIKE ? OR m.city LIKE ? ORDER BY m.created_at DESC LIMIT ? OFFSET ?"
        ).bind(&like_pattern).bind(&like_pattern).bind(size).bind(offset).fetch_all(db).await?;

    let list: Vec<MarketPostVO> = rows.into_iter().map(|r| {
        let images: Vec<String> = if r.6.is_empty() { vec![] } else { r.6.split(',').map(|s| s.to_string()).collect() };
        MarketPostVO {
            id: r.0, user_id: r.1, title: r.2, category: r.3, price: r.4,
            city: r.5, images, description: r.7, contact_info: r.8,
            status: r.9, created_at: r.10, username: r.12,
        }
    }).collect();

    Ok(PageResp { list, total: total.0, page, size })
}
