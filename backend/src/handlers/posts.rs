use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    db::SharedState,
    models::{CreatePostRequest, Post, PostWithTranslation},
    services::translation::detect_text_direction,
};

#[derive(Deserialize)]
pub struct GetPostsQuery {
    pub lang: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

pub async fn get_posts(
    State(state): State<SharedState>,
    Query(params): Query<GetPostsQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let lang = params.lang.unwrap_or_else(|| "en".to_string());
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);
    let offset = (page - 1) * limit;

    // Get posts with translations
    let posts = sqlx::query_as::<_, PostWithTranslation>(
        r#"
        SELECT 
            p.id,
            p.author_name,
            COALESCE(pt.title, p.title) as title,
            COALESCE(pt.content, p.content) as content,
            p.title as original_title,
            p.content as original_content,
            p.link_url,
            p.image_url,
            p.post_type,
            p.original_language,
            p.text_direction as original_text_direction,
            COALESCE(pt.text_direction, p.text_direction) as text_direction,
            CASE WHEN pt.id IS NOT NULL AND p.original_language != $1 THEN true ELSE false END as is_translated,
            p.created_at
        FROM posts p
        LEFT JOIN post_translations pt ON p.id = pt.post_id AND pt.language_code = $1
        WHERE p.published = true
        ORDER BY p.created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(&lang)
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch posts: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Get total count
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM posts WHERE published = true")
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "posts": posts,
        "pagination": {
            "page": page,
            "limit": limit,
            "total": total,
            "total_pages": (total as f64 / limit as f64).ceil() as i64
        }
    })))
}

pub async fn create_post(
    State(state): State<SharedState>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let language = payload.language.clone().unwrap_or_else(|| "en".to_string());
    let text_direction = payload.text_direction.clone().unwrap_or_else(|| {
        detect_text_direction(&payload.title, &language)
    });

    let post = sqlx::query_as::<_, Post>(
        r#"
        INSERT INTO posts (
            author_name, author_email, title, content, link_url, 
            image_url, post_type, original_language, text_direction
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#,
    )
    .bind(&payload.author_name)
    .bind(&payload.author_email)
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(&payload.link_url)
    .bind(&payload.image_url)
    .bind(&payload.post_type)
    .bind(&language)
    .bind(&text_direction)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create post: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(json!({
        "post": post,
        "message": "Post submitted successfully and is awaiting moderation"
    })))
}

pub async fn publish_post(
    State(state): State<SharedState>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Update post to published
    sqlx::query("UPDATE posts SET published = true, updated_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to publish post: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Trigger translation job
    if let Some(api_key) = &state.google_api_key {
        let pool = std::sync::Arc::new(state.pool.clone());
        let api_key = api_key.clone();
        tokio::spawn(async move {
            crate::services::background::translate_post(pool, id, api_key).await;
        });
    }

    Ok(Json(json!({
        "message": "Post published successfully"
    })))
}