use crate::errors::AppError;
use crate::config::*;
use crate::models::*;
use sqlx::MySqlPool;

pub async fn create_jianghu_post(db: &MySqlPool, user_id: Option<i64>, req: JianghuPostReq) -> Result<JianghuPost, AppError> {
    let images_str = req.images.map(|v| v.join(",")).unwrap_or_default();
    let result = sqlx::query(
        "INSERT INTO jianghu_posts (user_id, content, images) VALUES (?, ?, ?)"
    ).bind(user_id).bind(&req.content).bind(&images_str).execute(db).await?;

    let post: JianghuPost = sqlx::query_as("SELECT * FROM jianghu_posts WHERE id = ?")
        .bind(result.last_insert_id()).fetch_one(db).await?;
    log::info!("Jianghu post created: id={} by user_id={:?}", post.id, user_id);
    Ok(post)
}

pub async fn list_jianghu_posts(db: &MySqlPool, current_user_id: Option<i64>, query: &PageQuery) -> Result<PageResp<JianghuPostVO>, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let size = query.size.unwrap_or(DEFAULT_PAGE_SIZE).min(MAX_PAGE_SIZE);
    let offset = (page - 1) * size;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM jianghu_posts")
        .fetch_one(db).await?;

    let posts: Vec<JianghuPost> = sqlx::query_as(
        "SELECT * FROM jianghu_posts ORDER BY created_at DESC LIMIT ? OFFSET ?"
    ).bind(size).bind(offset).fetch_all(db).await?;

    let uid = current_user_id.unwrap_or(0);
    let mut list = Vec::new();
    for p in posts {
        let (username, avatar) = if let Some(poster_id) = p.user_id {
            let user: User = sqlx::query_as("SELECT * FROM users WHERE id = ?")
                .bind(poster_id).fetch_one(db).await?;
            (user.username, user.avatar)
        } else {
            ("匿名用户".to_string(), String::new())
        };
        let liked: Option<(i64,)> = sqlx::query_as(
            "SELECT id FROM jianghu_likes WHERE post_id = ? AND user_id = ?"
        ).bind(p.id).bind(uid).fetch_optional(db).await?;

        let images: Vec<String> = if p.images.is_empty() {
            vec![]
        } else {
            p.images.split(',').map(|s| s.to_string()).collect()
        };

        list.push(JianghuPostVO {
            id: p.id, user_id: p.user_id, content: p.content, images,
            like_count: p.like_count, comment_count: p.comment_count,
            created_at: p.created_at, username, avatar,
            liked: liked.is_some(),
        });
    }

    Ok(PageResp { list, total: total.0, page, size })
}

pub async fn toggle_like(db: &MySqlPool, user_id: i64, post_id: i64) -> Result<bool, AppError> {
    let mut tx = db.begin().await?;

    let existing: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM jianghu_likes WHERE post_id = ? AND user_id = ?"
    ).bind(post_id).bind(user_id).fetch_optional(&mut *tx).await?;

    if let Some((like_id,)) = existing {
        sqlx::query("DELETE FROM jianghu_likes WHERE id = ?").bind(like_id).execute(&mut *tx).await?;
        sqlx::query("UPDATE jianghu_posts SET like_count = like_count - 1 WHERE id = ? AND like_count > 0")
            .bind(post_id).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(false)
    } else {
        sqlx::query("INSERT INTO jianghu_likes (post_id, user_id) VALUES (?, ?)")
            .bind(post_id).bind(user_id).execute(&mut *tx).await?;
        sqlx::query("UPDATE jianghu_posts SET like_count = like_count + 1 WHERE id = ?")
            .bind(post_id).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(true)
    }
}

pub async fn add_comment(db: &MySqlPool, user_id: i64, post_id: i64, content: &str) -> Result<JianghuComment, AppError> {
    let mut tx = db.begin().await?;

    let result = sqlx::query(
        "INSERT INTO jianghu_comments (post_id, user_id, content) VALUES (?, ?, ?)"
    ).bind(post_id).bind(user_id).bind(content).execute(&mut *tx).await?;

    sqlx::query("UPDATE jianghu_posts SET comment_count = comment_count + 1 WHERE id = ?")
        .bind(post_id).execute(&mut *tx).await?;

    let comment: JianghuComment = sqlx::query_as("SELECT * FROM jianghu_comments WHERE id = ?")
        .bind(result.last_insert_id()).fetch_one(&mut *tx).await?;

    tx.commit().await?;
    Ok(comment)
}

pub async fn list_comments(db: &MySqlPool, post_id: i64) -> Result<Vec<JianghuCommentVO>, AppError> {
    let comments: Vec<JianghuComment> = sqlx::query_as(
        "SELECT * FROM jianghu_comments WHERE post_id = ? ORDER BY created_at ASC"
    ).bind(post_id).fetch_all(db).await?;

    let mut list = Vec::new();
    for c in comments {
        let user: User = sqlx::query_as("SELECT * FROM users WHERE id = ?")
            .bind(c.user_id).fetch_one(db).await?;
        list.push(JianghuCommentVO {
            id: c.id, user_id: c.user_id, content: c.content,
            created_at: c.created_at, username: user.username, avatar: user.avatar,
        });
    }
    Ok(list)
}

pub async fn delete_jianghu_post(db: &MySqlPool, user_id: i64, post_id: i64, is_admin: bool) -> Result<(), AppError> {
    let post: JianghuPost = sqlx::query_as("SELECT * FROM jianghu_posts WHERE id = ?")
        .bind(post_id).fetch_optional(db).await?
        .ok_or_else(|| AppError::NotFound("动态不存在".into()))?;

    if !is_admin && post.user_id != Some(user_id) {
        return Err(AppError::Forbidden("无权删除".into()));
    }

    let mut tx = db.begin().await?;
    sqlx::query("DELETE FROM jianghu_likes WHERE post_id = ?").bind(post_id).execute(&mut *tx).await?;
    sqlx::query("DELETE FROM jianghu_comments WHERE post_id = ?").bind(post_id).execute(&mut *tx).await?;
    sqlx::query("DELETE FROM jianghu_posts WHERE id = ?").bind(post_id).execute(&mut *tx).await?;
    tx.commit().await?;
    log::info!("Jianghu post deleted: id={}", post_id);
    Ok(())
}
