use chrono::{Utc, NaiveTime, Duration, NaiveDate};
use sqlx::postgres::PgPoolOptions;
use rand::Rng;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    
    println!("🌱 Starting Lancaster Community Platform seed script...");
    
    // Database connection
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("📍 Connecting to database...");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    println!("✅ Connected to database successfully");
    
    // Clear existing data
    println!("\n🧹 Clearing existing data...");
    sqlx::query("DELETE FROM event_translations").execute(&pool).await?;
    sqlx::query("DELETE FROM post_translations").execute(&pool).await?;
    sqlx::query("DELETE FROM events").execute(&pool).await?;
    sqlx::query("DELETE FROM posts").execute(&pool).await?;
    println!("✓ Cleared existing data");
    
    // Seed supported languages if not exists
    println!("\n🌍 Ensuring languages are configured...");
    seed_languages(&pool).await?;
    
    // Seed posts
    println!("\n📝 Creating posts...");
    seed_posts(&pool).await?;
    
    // Seed events  
    println!("\n📅 Creating events...");
    seed_events(&pool).await?;
    
    println!("\n✨ Seed completed successfully!");
    
    Ok(())
}

async fn seed_languages(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    // Check if languages already exist
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM supported_languages")
        .fetch_one(pool)
        .await?;
    
    if count.0 == 0 {
        println!("  No languages found, inserting default languages...");
        // Insert default languages
        sqlx::query(r#"
            INSERT INTO supported_languages (code, name, native_name, is_rtl, text_direction, enabled)
            VALUES 
                ('en', 'English', 'English', false, 'ltr', true),
                ('es', 'Spanish', 'Español', false, 'ltr', true),
                ('ar', 'Arabic', 'العربية', true, 'rtl', true),
                ('he', 'Hebrew', 'עברית', true, 'rtl', true),
                ('fr', 'French', 'Français', false, 'ltr', true),
                ('de', 'German', 'Deutsch', false, 'ltr', true),
                ('zh', 'Chinese', '中文', false, 'ltr', true),
                ('fa', 'Persian', 'فارسی', true, 'rtl', true),
                ('ur', 'Urdu', 'اردو', true, 'rtl', true)
            "#)
            .execute(pool)
            .await?;
        println!("  ✓ Inserted 9 languages");
    } else {
        println!("  ✓ Languages already configured ({} found)", count.0);
    }
    
    Ok(())
}

async fn seed_posts(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let posts_created;
    
    // Simple posts - just create them in English, let the app translate as needed
    struct PostData {
        author_name: &'static str,
        author_email: Option<&'static str>,
        title: &'static str,
        content: Option<&'static str>,
        link_url: Option<&'static str>,
        image_url: Option<&'static str>,
        post_type: &'static str,
        published: bool,
    }
    
    let posts = vec![
        PostData {
            author_name: "Sarah Johnson",
            author_email: Some("sarah.johnson@lancaster.gov"),
            title: "Lancaster City Council Announces New Community Garden Initiative",
            content: Some("The Lancaster City Council is excited to announce a new community garden initiative in the heart of downtown. This project aims to bring residents together while promoting sustainable urban agriculture."),
            link_url: None,
            image_url: Some("https://images.unsplash.com/photo-1416879595882-3373a0480b5b?w=800"),
            post_type: "announcement",
            published: true,
        },
        PostData {
            author_name: "Mike Chen",
            author_email: Some("mike@lancasterlocal.com"),
            title: "Central Market Celebrates 290 Years of Service",
            content: Some("Lancaster Central Market, one of the oldest continuously operating farmers markets in the United States, is celebrating its 290th anniversary this year."),
            link_url: Some("https://www.centralmarketlancaster.com"),
            image_url: Some("https://images.unsplash.com/photo-1488459716781-31db52582fe9?w=800"),
            post_type: "article",
            published: true,
        },
        PostData {
            author_name: "Emily Rodriguez",
            author_email: Some("emily.r@email.com"),
            title: "Lancaster Symphony Orchestra Announces Free Summer Concert Series",
            content: Some("The Lancaster Symphony Orchestra is thrilled to announce its annual free summer concert series in Buchanan Park. Every Sunday evening in July and August, residents can enjoy classical music under the stars."),
            link_url: None,
            image_url: Some("https://images.unsplash.com/photo-1493225457124-a3eb161ffa5f?w=800"),
            post_type: "announcement",
            published: true,
        },
        PostData {
            author_name: "Community Board",
            author_email: Some("board@lancastercommunity.org"),
            title: "Volunteer Opportunities at Lancaster Food Bank",
            content: Some("The Lancaster Food Bank is seeking volunteers for its expanded meal distribution program. With increasing demand for services, we need community members to help sort donations, pack food boxes, and assist with distribution."),
            link_url: Some("https://www.lancasterfoodbank.org/volunteer"),
            image_url: None,
            post_type: "link",
            published: true,
        },
        PostData {
            author_name: "Lisa Thompson",
            author_email: Some("lisa@lancasterparks.org"),
            title: "County Park System Adds 50 Miles of New Trails",
            content: Some("Lancaster County Parks and Recreation has completed an ambitious expansion of its trail system, adding 50 miles of new hiking and biking trails. The new trails connect existing parks and natural areas."),
            link_url: None,
            image_url: Some("https://images.unsplash.com/photo-1551632811-561732d1e306?w=800"),
            post_type: "announcement",
            published: true,
        },
    ];
    
    posts_created = posts.len();
    
    // Insert posts with random dates in the past 30 days
    for post in posts {
        let days_ago = rng.gen_range(0..30);
        let created_at = Utc::now() - Duration::days(days_ago);
        
        sqlx::query(
            r#"
            INSERT INTO posts (
                author_name, author_email, title, content, link_url, image_url, 
                post_type, original_language, text_direction, published, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $11)
            "#
        )
        .bind(post.author_name)
        .bind(post.author_email)
        .bind(post.title)
        .bind(post.content)
        .bind(post.link_url)
        .bind(post.image_url)
        .bind(post.post_type)
        .bind("en") // All seed posts in English
        .bind("ltr")
        .bind(post.published)
        .bind(created_at)
        .execute(pool)
        .await?;
    }
    
    println!("  ✓ Created {} posts", posts_created);
    
    Ok(())
}

async fn seed_events(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let events_created;
    
    struct EventData {
        author_name: &'static str,
        author_email: Option<&'static str>,
        title: &'static str,
        description: &'static str,
        location: Option<&'static str>,
        date_offset: i64, // Days from today
        time: NaiveTime,
        duration_hours: i32,
        event_type: &'static str,
        published: bool,
    }
    
    let events = vec![
        EventData {
            author_name: "Parks Department",
            author_email: Some("events@lancasterparks.org"),
            title: "Summer Movie Night: The Princess Bride",
            description: "Join us for a free outdoor screening of The Princess Bride in Buchanan Park. Bring your blankets and lawn chairs!",
            location: Some("Buchanan Park Amphitheater"),
            date_offset: 5,
            time: NaiveTime::from_hms_opt(20, 0, 0).unwrap(),
            duration_hours: 2,
            event_type: "entertainment",
            published: true,
        },
        EventData {
            author_name: "Lancaster Library",
            author_email: Some("programs@lancasterlibrary.org"),
            title: "Children's Story Time",
            description: "Weekly story time for children ages 3-6. Join us for stories, songs, and crafts!",
            location: Some("Lancaster Public Library - Children's Section"),
            date_offset: 2,
            time: NaiveTime::from_hms_opt(10, 30, 0).unwrap(),
            duration_hours: 1,
            event_type: "education",
            published: true,
        },
        EventData {
            author_name: "City Council",
            author_email: Some("council@lancaster.gov"),
            title: "Town Hall Meeting on Infrastructure",
            description: "Public forum to discuss upcoming infrastructure projects including road repairs and utility upgrades.",
            location: Some("City Hall - Council Chambers"),
            date_offset: 14,
            time: NaiveTime::from_hms_opt(18, 30, 0).unwrap(),
            duration_hours: 2,
            event_type: "government",
            published: true,
        },
        EventData {
            author_name: "Farmers Market",
            author_email: Some("info@lancastermarket.org"),
            title: "Weekly Farmers Market",
            description: "Fresh produce, baked goods, and local crafts. Every Saturday morning at Central Market.",
            location: Some("Lancaster Central Market"),
            date_offset: 3,
            time: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            duration_hours: 4,
            event_type: "market",
            published: true,
        },
        EventData {
            author_name: "Community Center",
            author_email: Some("activities@communitycenter.org"),
            title: "Senior Bingo Night",
            description: "Free bingo night for seniors. Prizes and refreshments provided!",
            location: Some("Lancaster Community Center"),
            date_offset: 7,
            time: NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
            duration_hours: 2,
            event_type: "social",
            published: true,
        },
    ];
    
    events_created = events.len();
    
    // Insert events
    for event in events {
        let event_date = (Utc::now() + Duration::days(event.date_offset)).date_naive();
        let created_at = Utc::now() - Duration::days(rng.gen_range(5..20));
        
        sqlx::query(
            r#"
            INSERT INTO events (
                author_name, author_email, title, description, location,
                date, time, duration_hours, event_type, original_language, 
                text_direction, published, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $13)
            "#
        )
        .bind(event.author_name)
        .bind(event.author_email)
        .bind(event.title)
        .bind(event.description)
        .bind(event.location)
        .bind(event_date)
        .bind(event.time)
        .bind(event.duration_hours)
        .bind(event.event_type)
        .bind("en") // All seed events in English
        .bind("ltr")
        .bind(event.published)
        .bind(created_at)
        .execute(pool)
        .await?;
    }
    
    println!("  ✓ Created {} events", events_created);
    
    Ok(())
}