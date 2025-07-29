use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use base64::{Engine as _, engine::general_purpose};
use chrono::{Duration, Utc};
use serde_json::json;

use crate::{
    db::SharedState,
    models::{AdminLoginRequest, AdminLoginResponse, AdminSession, Event, Post},
};

pub async fn admin_login(
    State(state): State<SharedState>,
    Json(payload): Json<AdminLoginRequest>,
) -> Result<Json<AdminLoginResponse>, StatusCode> {
    // Simple password check
    if payload.password != state.admin_password {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Generate session token
    let token = general_purpose::STANDARD.encode(rand::random::<[u8; 32]>());
    let expires_at = Utc::now() + Duration::hours(24);

    // Store session
    sqlx::query(
        "INSERT INTO admin_sessions (session_token, expires_at) VALUES ($1, $2)"
    )
    .bind(&token)
    .bind(expires_at)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create session: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(AdminLoginResponse { token }))
}

pub async fn admin_logout(
    State(state): State<SharedState>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let token = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    sqlx::query("DELETE FROM admin_sessions WHERE session_token = $1")
        .bind(token)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "message": "Logged out successfully"
    })))
}

pub async fn get_admin_posts(
    State(state): State<SharedState>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Verify admin session
    verify_admin_session(&state, &headers).await?;

    let posts = sqlx::query_as::<_, Post>(
        "SELECT * FROM posts ORDER BY created_at DESC"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch posts: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(json!({
        "posts": posts
    })))
}

pub async fn get_admin_events(
    State(state): State<SharedState>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Verify admin session
    verify_admin_session(&state, &headers).await?;

    let events = sqlx::query_as::<_, Event>(
        "SELECT * FROM events ORDER BY event_date DESC, event_time DESC"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch events: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(json!({
        "events": events
    })))
}

async fn verify_admin_session(
    state: &SharedState,
    headers: &HeaderMap,
) -> Result<(), StatusCode> {
    let token = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let session = sqlx::query_as::<_, AdminSession>(
        "SELECT * FROM admin_sessions WHERE session_token = $1 AND expires_at > NOW()"
    )
    .bind(token)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(())
}