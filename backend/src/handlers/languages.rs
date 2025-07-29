use axum::{extract::State, http::StatusCode, Json};
use serde_json::json;

use crate::{db::SharedState, models::Language};

pub async fn get_supported_languages(
    State(state): State<SharedState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let languages = sqlx::query_as::<_, Language>(
        "SELECT code, name, native_name, is_rtl, text_direction, enabled 
         FROM supported_languages 
         WHERE enabled = true 
         ORDER BY name"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch languages: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(json!({
        "languages": languages
    })))
}