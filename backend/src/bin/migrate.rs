use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    
    println!("Starting database migration...");
    
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    // Create connection pool
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await?;
    
    // Run migrations
    println!("Running migrations from /app/migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    
    println!("Migration completed successfully!");
    
    Ok(())
}