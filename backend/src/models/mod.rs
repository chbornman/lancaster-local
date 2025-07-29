use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Language {
    pub code: String,
    pub name: String,
    pub native_name: String,
    pub is_rtl: bool,
    pub text_direction: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: i32,
    pub author_name: String,
    pub author_email: Option<String>,
    pub title: String,
    pub content: Option<String>,
    pub link_url: Option<String>,
    pub image_url: Option<String>,
    pub post_type: String,
    pub original_language: String,
    pub text_direction: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub author_name: String,
    pub author_email: Option<String>,
    pub title: String,
    pub content: Option<String>,
    pub link_url: Option<String>,
    pub image_url: Option<String>,
    pub post_type: String,
    pub language: Option<String>,
    pub text_direction: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PostTranslation {
    pub id: i32,
    pub post_id: i32,
    pub language_code: String,
    pub title: String,
    pub content: Option<String>,
    pub text_direction: String,
    pub translated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PostWithTranslation {
    pub id: i32,
    pub author_name: String,
    pub title: String,
    pub content: Option<String>,
    pub original_title: String,
    pub original_content: Option<String>,
    pub link_url: Option<String>,
    pub image_url: Option<String>,
    pub post_type: String,
    pub original_language: String,
    pub original_text_direction: String,
    pub text_direction: String,
    pub is_translated: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Event {
    pub id: i32,
    pub organizer_name: String,
    pub organizer_email: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub event_date: NaiveDate,
    pub event_time: Option<NaiveTime>,
    pub location: Option<String>,
    pub category: Option<String>,
    pub is_free: bool,
    pub ticket_url: Option<String>,
    pub original_language: String,
    pub text_direction: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEventRequest {
    pub organizer_name: String,
    pub organizer_email: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub event_date: String,
    pub event_time: Option<String>,
    pub location: Option<String>,
    pub category: Option<String>,
    pub is_free: bool,
    pub ticket_url: Option<String>,
    pub language: Option<String>,
    pub text_direction: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EventTranslation {
    pub id: i32,
    pub event_id: i32,
    pub language_code: String,
    pub title: String,
    pub description: Option<String>,
    pub text_direction: String,
    pub translated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EventWithTranslation {
    pub id: i32,
    pub organizer_name: String,
    pub title: String,
    pub description: Option<String>,
    pub original_title: String,
    pub original_description: Option<String>,
    pub event_date: NaiveDate,
    pub event_time: Option<NaiveTime>,
    pub location: Option<String>,
    pub category: Option<String>,
    pub is_free: bool,
    pub ticket_url: Option<String>,
    pub original_language: String,
    pub original_text_direction: String,
    pub text_direction: String,
    pub is_translated: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminLoginRequest {
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminLoginResponse {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AdminSession {
    pub id: i32,
    pub session_token: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationResult {
    pub translated_text: String,
    pub source_language: String,
    pub target_language: String,
    pub text_direction: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageDetectionResult {
    pub language: String,
    pub confidence: f32,
    pub is_rtl: bool,
    pub text_direction: String,
}