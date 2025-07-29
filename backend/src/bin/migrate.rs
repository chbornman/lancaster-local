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
        .await
        .map_err(|e| {
            eprintln!("❌ Failed to connect to database: {}", e);
            e
        })?;
    
    println!("✅ Database connection established");
    
    // Run migrations
    println!("📦 Running migrations from ./migrations directory...");
    
    let migrator = sqlx::migrate!("./migrations");
    let migrations = migrator.iter().collect::<Vec<_>>();
    
    println!("📋 Found {} migration(s) to check", migrations.len());
    
    for migration in &migrations {
        println!("  - {} (v{}): {}", 
            migration.description, 
            migration.version,
            if migration.migration_type.is_up() { "UP" } else { "DOWN" }
        );
    }
    
    match migrator.run(&pool).await {
        Ok(_) => {
            println!("✅ All migrations completed successfully!");
            
            // Log final state
            let applied: Vec<(i64, String, i64)> = sqlx::query_as(
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
        }
        Err(e) => {
            eprintln!("❌ Migration failed: {}", e);
            return Err(Box::new(e));
        }
    }
    
    let elapsed = start_time.elapsed();
    println!("\n⏱️  Migration completed in {:.2}s", elapsed.as_secs_f64());
    
    Ok(())
}