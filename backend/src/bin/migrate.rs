use sqlx::postgres::PgPoolOptions;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    dotenv::dotenv().ok();
    
    println!("🚀 Starting Lancaster Community Platform database migration...");
    
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    println!("📍 Connecting to database: {}", database_url.split('@').last().unwrap_or("unknown"));
    
    // Create connection pool
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await?;
    
    println!("✅ Database connection established");
    
    // Run migrations
    println!("📦 Running migrations from ./migrations directory...");
    
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    
    println!("✅ All migrations completed successfully!");
    
    // Log final state
    let applied: Vec<(i64, String, chrono::DateTime<chrono::Utc>)> = sqlx::query_as(
        "SELECT version, description, installed_on FROM _sqlx_migrations ORDER BY version"
    )
    .fetch_all(&pool)
    .await?;
    
    println!("\n📊 Migration Status:");
    println!("  Total migrations applied: {}", applied.len());
    
    if !applied.is_empty() {
        println!("\n  Applied migrations:");
        for (version, description, _) in applied {
            println!("    ✓ v{}: {}", version, description);
        }
    }
    
    let elapsed = start_time.elapsed();
    println!("\n⏱️  Migration completed in {:.2}s", elapsed.as_secs_f64());
    
    Ok(())
}