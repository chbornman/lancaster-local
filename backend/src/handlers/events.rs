use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::json;

use crate::{
    db::SharedState,
    models::{CreateEventRequest, Event, EventWithTranslation},
    services::translation::detect_text_direction,
};

#[derive(Deserialize)]
pub struct GetEventsQuery {
    pub lang: Option<String>,
    pub month: Option<String>, // Format: YYYY-MM
    pub category: Option<String>,
}

pub async fn get_events(
    State(state): State<SharedState>,
    Query(params): Query<GetEventsQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let lang = params.lang.unwrap_or_else(|| "en".to_string());
    
    let mut query = r#"
        SELECT 
            e.id,
            e.organizer_name,
            COALESCE(et.title, e.title) as title,
            COALESCE(et.description, e.description) as description,
            e.title as original_title,
            e.description as original_description,
            e.event_date,
            e.event_time,
            e.location,
            e.category,
            e.is_free,
            e.ticket_price,
            e.ticket_url,
            e.original_language,
            e.text_direction as original_text_direction,
            COALESCE(et.text_direction, e.text_direction) as text_direction,
            CASE WHEN et.id IS NOT NULL AND e.original_language != $1 THEN true ELSE false END as is_translated,
            e.created_at
        FROM events e
        LEFT JOIN event_translations et ON e.id = et.event_id AND et.language_code = $1
        WHERE e.published = true
    "#.to_string();

    let mut bind_count = 1;
    let mut conditions = vec![];

    if let Some(month) = &params.month {
        bind_count += 1;
        conditions.push(format!("DATE_TRUNC('month', e.event_date) = DATE_TRUNC('month', ${}::date)", bind_count));
    }

    if let Some(category) = &params.category {
        bind_count += 1;
        conditions.push(format!("e.category = ${}", bind_count));
    }

    if !conditions.is_empty() {
        query.push_str(" AND ");
        query.push_str(&conditions.join(" AND "));
    }

    query.push_str(" ORDER BY e.event_date ASC, e.event_time ASC");

    let mut q = sqlx::query_as::<_, EventWithTranslation>(&query).bind(&lang);

    if let Some(month) = &params.month {
        let date = NaiveDate::parse_from_str(&format!("{}-01", month), "%Y-%m-%d")
            .map_err(|_| StatusCode::BAD_REQUEST)?;
        q = q.bind(date);
    }

    if let Some(category) = &params.category {
        q = q.bind(category);
    }

    let events = q.fetch_all(&state.pool).await.map_err(|e| {
        tracing::error!("Failed to fetch events: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(json!({
        "events": events
    })))
}

pub async fn create_event(
    State(state): State<SharedState>,
    Json(payload): Json<CreateEventRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let language = payload.language.clone().unwrap_or_else(|| "en".to_string());
    let text_direction = payload.text_direction.clone().unwrap_or_else(|| {
        detect_text_direction(&payload.title, &language)
    });

    let event_date = NaiveDate::parse_from_str(&payload.event_date, "%Y-%m-%d")
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let event_time = payload
        .event_time
        .as_ref()
        .map(|t| chrono::NaiveTime::parse_from_str(t, "%H:%M:%S").ok())
        .flatten();

    let event = sqlx::query_as::<_, Event>(
        r#"
        INSERT INTO events (
            organizer_name, organizer_email, title, description,
            event_date, event_time, location, category,
            is_free, ticket_price, ticket_url,
            original_language, text_direction
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        RETURNING *
        "#,
    )
    .bind(&payload.organizer_name)
    .bind(&payload.organizer_email)
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(event_date)
    .bind(event_time)
    .bind(&payload.location)
    .bind(&payload.category)
    .bind(payload.is_free)
    .bind(payload.ticket_price)
    .bind(&payload.ticket_url)
    .bind(&language)
    .bind(&text_direction)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create event: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(json!({
        "event": event,
        "message": "Event submitted successfully and is awaiting moderation"
    })))
}

pub async fn publish_event(
    State(state): State<SharedState>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Update event to published
    sqlx::query("UPDATE events SET published = true WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to publish event: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Trigger translation job
    if let Some(api_key) = &state.google_api_key {
        let pool = std::sync::Arc::new(state.pool.clone());
        let api_key = api_key.clone();
        tokio::spawn(async move {
            crate::services::background::translate_event(pool, id, api_key).await;
        });
    }

    Ok(Json(json!({
        "message": "Event published successfully"
    })))
}