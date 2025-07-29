use axum::{
    routing::{get, post, delete},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod handlers;
mod models;
mod services;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "lancaster_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Database connection
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    // Redis connection
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let redis_client = redis::Client::open(redis_url).expect("Failed to connect to Redis");

    // Create app state
    let app_state = std::sync::Arc::new(db::AppState {
        pool,
        redis_client,
        admin_password: std::env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set"),
        google_api_key: std::env::var("GOOGLE_TRANSLATE_API_KEY").ok(),
    });

    // Build router
    let app = Router::new()
        // Health check
        .route("/api/health", get(handlers::health_check))
        
        // Language endpoints
        .route("/api/languages", get(handlers::get_supported_languages))
        
        // Post endpoints
        .route("/api/posts", get(handlers::get_posts))
        .route("/api/posts", post(handlers::create_post))
        .route("/api/posts/:id/publish", post(handlers::publish_post))
        
        // Event endpoints
        .route("/api/events", get(handlers::get_events))
        .route("/api/events", post(handlers::create_event))
        .route("/api/events/:id/publish", post(handlers::publish_event))
        
        // Admin endpoints
        .route("/api/admin/login", post(handlers::admin_login))
        .route("/api/admin/logout", post(handlers::admin_logout))
        .route("/api/admin/posts", get(handlers::get_admin_posts))
        .route("/api/admin/events", get(handlers::get_admin_events))
        .route("/api/admin/posts/:id", delete(handlers::delete_post))
        .route("/api/admin/events/:id", delete(handlers::delete_event))
        
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers([AUTHORIZATION, CONTENT_TYPE]),
        )
        .with_state(app_state);

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("Invalid PORT");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}