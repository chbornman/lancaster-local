use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub redis_client: redis::Client,
    pub admin_password: String,
    pub google_api_key: Option<String>,
}

pub type SharedState = Arc<AppState>;