use chrono::{Utc, NaiveTime, Duration};
use sqlx::postgres::PgPoolOptions;
use rand::Rng;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    
    println!("🌱 Starting Lancaster Community Platform seed script...");
    
    // Database connection
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    println!("✅ Connected to database");
    
    // Clear existing data
    println!("🧹 Clearing existing data...");
    sqlx::query("DELETE FROM event_translations").execute(&pool).await?;
    sqlx::query("DELETE FROM post_translations").execute(&pool).await?;
    sqlx::query("DELETE FROM events").execute(&pool).await?;
    sqlx::query("DELETE FROM posts").execute(&pool).await?;
    
    // Seed supported languages if not exists
    println!("🌍 Ensuring languages are configured...");
    seed_languages(&pool).await?;
    
    // Seed posts
    println!("📝 Creating posts...");
    seed_posts(&pool).await?;
    
    // Seed events  
    println!("📅 Creating events...");
    seed_events(&pool).await?;
    
    println!("✨ Seed completed successfully!");
    
    Ok(())
}

async fn seed_languages(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    // Check if languages already exist
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM supported_languages")
        .fetch_one(pool)
        .await?;
    
    if count.0 == 0 {
        // Insert default languages
        sqlx::query(r#"
            INSERT INTO supported_languages (code, name, native_name, is_rtl, text_direction, enabled)
            VALUES 
                ('en', 'English', 'English', false, 'ltr', true),
                ('es', 'Spanish', 'Español', false, 'ltr', true),
                ('ar', 'Arabic', 'العربية', true, 'rtl', true),
                ('he', 'Hebrew', 'עברית', true, 'rtl', true),
                ('fr', 'French', 'Français', false, 'ltr', true),
                ('de', 'German', 'Deutsch', false, 'ltr', false)
        "#)
        .execute(pool)
        .await?;
    }
    
    Ok(())
}

async fn seed_posts(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    
    // Posts with different original languages
    struct PostData {
        author_name: &'static str,
        author_email: Option<&'static str>,
        title: &'static str,
        content: Option<&'static str>,
        link_url: Option<&'static str>,
        image_url: Option<&'static str>,
        post_type: &'static str,
        original_language: &'static str,
        published: bool,
    }
    
    let posts = vec![
        // English posts
        PostData {
            author_name: "Sarah Johnson",
            author_email: Some("sarah.johnson@lancaster.gov"),
            title: "Lancaster City Council Announces New Community Garden Initiative",
            content: Some("The Lancaster City Council is excited to announce a new community garden initiative in the heart of downtown. This project aims to bring residents together while promoting sustainable urban agriculture."),
            link_url: None,
            image_url: Some("https://images.unsplash.com/photo-1416879595882-3373a0480b5b?w=800"),
            post_type: "announcement",
            original_language: "en",
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
            original_language: "en",
            published: true,
        },
        // Spanish original posts
        PostData {
            author_name: "Maria Hernandez",
            author_email: Some("mhernandez@spanishcenter.org"),
            title: "Nuevo Programa de Alfabetización Digital para Adultos Mayores",
            content: Some("La Biblioteca Pública de Lancaster se enorgullece de presentar clases gratuitas de alfabetización digital diseñadas específicamente para personas mayores. Aprenda habilidades informáticas básicas, seguridad en Internet y cómo conectarse con sus seres queridos a través de videollamadas."),
            link_url: None,
            image_url: None,
            post_type: "announcement",
            original_language: "es",
            published: true,
        },
        PostData {
            author_name: "Carlos Rodriguez",
            author_email: None,
            title: "Festival Internacional de Comida en el Centro",
            content: Some("Marque sus calendarios para el tercer Festival Internacional de Comida anual de Lancaster, celebrando la diversidad cultural de nuestra comunidad. Más de 40 vendedores ofrecerán cocinas de todo el mundo, con música en vivo y actuaciones culturales durante todo el día."),
            link_url: None,
            image_url: Some("https://images.unsplash.com/photo-1555939594-58d7cb561ad1?w=800"),
            post_type: "announcement",
            original_language: "es",
            published: true,
        },
        // Arabic original posts
        PostData {
            author_name: "أحمد حسن",
            author_email: Some("ahmad@lancasterdiversity.org"),
            title: "برنامج جديد لدعم اللاجئين في لانكستر",
            content: Some("يسر مركز لانكستر للتنوع الثقافي أن يعلن عن برنامج دعم شامل جديد للاجئين والمهاجرين الجدد. يشمل البرنامج دروس اللغة الإنجليزية والتدريب على المهارات الوظيفية والمساعدة في الاندماج المجتمعي."),
            link_url: None,
            image_url: None,
            post_type: "announcement",
            original_language: "ar",
            published: true,
        },
        PostData {
            author_name: "فاطمة الزهراء",
            author_email: None,
            title: "دروس الطبخ العربي في المركز المجتمعي",
            content: Some("انضموا إلينا لسلسلة من دروس الطبخ العربي الأصيل كل يوم سبت. سنتعلم معاً إعداد الأطباق التقليدية من مختلف البلدان العربية. الدروس مفتوحة للجميع ولا تتطلب خبرة سابقة."),
            link_url: None,
            image_url: Some("https://images.unsplash.com/photo-1547592180-85f173990554?w=800"),
            post_type: "article",
            original_language: "ar",
            published: true,
        },
        // French original posts
        PostData {
            author_name: "Jean-Pierre Dubois",
            author_email: Some("jpdubois@lancasterfrench.org"),
            title: "Cours de Français Gratuits à la Bibliothèque",
            content: Some("La communauté francophone de Lancaster est heureuse d'offrir des cours de français gratuits pour tous les niveaux. Les cours ont lieu chaque mercredi soir à la bibliothèque publique. Une excellente opportunité d'apprendre une nouvelle langue et de rencontrer de nouvelles personnes."),
            link_url: None,
            image_url: None,
            post_type: "announcement",
            original_language: "fr",
            published: true,
        },
        PostData {
            author_name: "Marie Laurent",
            author_email: None,
            title: "Marché aux Puces Mensuel au Parc Central",
            content: Some("Rejoignez-nous pour notre marché aux puces mensuel au Parc Central de Lancaster. Trouvez des trésors uniques, des antiquités, et des créations artisanales locales. Les vendeurs sont les bienvenus - inscrivez-vous en ligne pour réserver votre emplacement."),
            link_url: Some("https://www.lancastermarkets.org/fleamarket"),
            image_url: None,
            post_type: "link",
            original_language: "fr",
            published: true,
        },
        // More English posts
        PostData {
            author_name: "Emily Rodriguez",
            author_email: Some("emily.r@email.com"),
            title: "Lancaster Symphony Orchestra Announces Free Summer Concert Series",
            content: Some("The Lancaster Symphony Orchestra is thrilled to announce its annual free summer concert series in Buchanan Park. Every Sunday evening in July and August, residents can enjoy classical music under the stars."),
            link_url: None,
            image_url: Some("https://images.unsplash.com/photo-1493225457124-a3eb161ffa5f?w=800"),
            post_type: "announcement",
            original_language: "en",
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
            original_language: "en",
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
            original_language: "en",
            published: true,
        },
        PostData {
            author_name: "Rachel Green",
            author_email: Some("rgreen@lancasterhealth.org"),
            title: "Free Health Screenings at Community Center",
            content: Some("Lancaster General Health will be offering free health screenings including blood pressure checks, glucose testing, and BMI calculations. Healthcare professionals will be available to answer questions."),
            link_url: None,
            image_url: Some("https://images.unsplash.com/photo-1576091160399-112ba8d25d1d?w=800"),
            post_type: "announcement",
            original_language: "en",
            published: true,
        },
        // Unpublished posts (awaiting review)
        PostData {
            author_name: "Jennifer Brown",
            author_email: Some("jbrown@email.com"),
            title: "Local Business Spotlight: Miller's Homemade Ice Cream",
            content: Some("Miller's Homemade Ice Cream has been a Lancaster favorite for over 40 years. Using locally sourced ingredients and time-honored recipes, this family-owned business continues to delight residents."),
            link_url: None,
            image_url: None,
            post_type: "article",
            original_language: "en",
            published: false,
        },
        PostData {
            author_name: "Robert Lee",
            author_email: Some("rlee@lancastertech.com"),
            title: "Tech Meetup Group Launches in Lancaster",
            content: Some("A new technology meetup group is forming in Lancaster, aimed at connecting local developers, designers, and tech enthusiasts. Monthly meetings will feature guest speakers and workshops."),
            link_url: Some("https://www.meetup.com/lancaster-tech"),
            image_url: None,
            post_type: "link",
            original_language: "en",
            published: false,
        },
        PostData {
            author_name: "Student Climate Coalition",
            author_email: Some("students@climateaction.org"),
            title: "Youth Climate March Planned for Earth Day",
            content: Some("Lancaster's youth are organizing a climate march to demand action on environmental issues. The peaceful march will begin at Penn Square and proceed to the courthouse."),
            link_url: None,
            image_url: Some("https://images.unsplash.com/photo-1569163139599-0f4517e36f51?w=800"),
            post_type: "announcement",
            original_language: "en",
            published: false,
        },
    ];
    
    // Insert posts and translations
    for post in posts {
        // Random time in the past 30 days
        let days_ago = rng.gen_range(0..30);
        let hours_ago = rng.gen_range(0..24);
        let created_at = Utc::now() - Duration::days(days_ago) - Duration::hours(hours_ago);
        
        let text_direction = if post.original_language == "ar" || post.original_language == "he" { 
            "rtl" 
        } else { 
            "ltr" 
        };
        
        // Insert post
        let post_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO posts (
                author_name, author_email, title, content, link_url, image_url, 
                post_type, original_language, text_direction, published, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $11)
            RETURNING id
            "#
        )
        .bind(post.author_name)
        .bind(post.author_email)
        .bind(post.title)
        .bind(post.content)
        .bind(post.link_url)
        .bind(post.image_url)
        .bind(post.post_type)
        .bind(post.original_language)
        .bind(text_direction)
        .bind(post.published)
        .bind(created_at)
        .fetch_one(pool)
        .await?;
        
        // Add translations for published posts
        if post.published {
            // Translation mappings based on original language
            let translations: Vec<(&str, String, Option<String>, &str)> = match post.original_language {
                "en" => vec![
                    ("es", format!("ES: {}", post.title), post.content.map(|c| format!("ES: {}", c)), "ltr"),
                    ("ar", format!("AR: {}", post.title), post.content.map(|c| format!("AR: {}", c)), "rtl"),
                    ("fr", format!("FR: {}", post.title), post.content.map(|c| format!("FR: {}", c)), "ltr"),
                ],
                "es" => vec![
                    ("en", "New Digital Literacy Program for Seniors".to_string(), Some("The Lancaster Public Library is proud to introduce free digital literacy classes designed specifically for seniors. Learn basic computer skills, internet safety, and how to connect with loved ones through video calls.".to_string()), "ltr"),
                    ("ar", format!("AR: {}", post.title), post.content.map(|c| format!("AR: {}", c)), "rtl"),
                    ("fr", format!("FR: {}", post.title), post.content.map(|c| format!("FR: {}", c)), "ltr"),
                ],
                "ar" => vec![
                    ("en", 
                     if post.title.contains("برنامج") { 
                         "New Refugee Support Program in Lancaster".to_string() 
                     } else { 
                         "Arabic Cooking Classes at Community Center".to_string() 
                     }, 
                     if post.title.contains("برنامج") { 
                         Some("Lancaster Cultural Diversity Center is pleased to announce a comprehensive new support program for refugees and new immigrants. The program includes English language classes, job skills training, and community integration assistance.".to_string())
                     } else {
                         Some("Join us for a series of authentic Arabic cooking classes every Saturday. We'll learn together how to prepare traditional dishes from various Arab countries. Classes are open to everyone and require no prior experience.".to_string())
                     }, "ltr"),
                    ("es", format!("ES: {}", post.title), post.content.map(|c| format!("ES: {}", c)), "ltr"),
                    ("fr", format!("FR: {}", post.title), post.content.map(|c| format!("FR: {}", c)), "ltr"),
                ],
                "fr" => vec![
                    ("en", 
                     if post.title.contains("Français") { 
                         "Free French Classes at the Library".to_string() 
                     } else { 
                         "Monthly Flea Market at Central Park".to_string() 
                     }, 
                     if post.title.contains("Français") { 
                         Some("The Lancaster French community is happy to offer free French classes for all levels. Classes are held every Wednesday evening at the public library. A great opportunity to learn a new language and meet new people.".to_string())
                     } else {
                         Some("Join us for our monthly flea market at Lancaster Central Park. Find unique treasures, antiques, and local artisan creations. Vendors are welcome - register online to reserve your spot.".to_string())
                     }, "ltr"),
                    ("es", format!("ES: {}", post.title), post.content.map(|c| format!("ES: {}", c)), "ltr"),
                    ("ar", format!("AR: {}", post.title), post.content.map(|c| format!("AR: {}", c)), "rtl"),
                ],
                _ => vec![],
            };
            
            for (lang_code, trans_title, trans_content, direction) in translations {
                sqlx::query(
                    r#"
                    INSERT INTO post_translations (post_id, language_code, title, content, text_direction)
                    VALUES ($1, $2, $3, $4, $5)
                    ON CONFLICT (post_id, language_code) DO NOTHING
                    "#
                )
                .bind(post_id)
                .bind(lang_code)
                .bind(&trans_title)
                .bind(&trans_content)
                .bind(direction)
                .execute(pool)
                .await?;
            }
        }
    }
    
    let post_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM posts").fetch_one(pool).await?;
    let published_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM posts WHERE published = true").fetch_one(pool).await?;
    let unpublished_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM posts WHERE published = false").fetch_one(pool).await?;
    
    println!("  ✓ Created {} posts ({} published, {} awaiting review)", 
        post_count.0, published_count.0, unpublished_count.0);
    
    Ok(())
}

async fn seed_events(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    
    // Lancaster-themed events
    let events_data = vec![
        // Published events
        (
            "Lancaster Farmers Market",
            "info@lancastermarket.org",
            "Weekly Farmers Market",
            Some("Shop fresh, locally grown produce, artisanal goods, and handmade crafts at Lancaster's premier farmers market."),
            7, // Days from now
            Some("08:00"),
            Some("Lancaster Central Market, 23 N Market St"),
            Some("market"),
            true,
            None,
            None,
            true,
        ),
        (
            "First Friday Lancaster",
            "events@firstfridaylancaster.com", 
            "First Friday Arts Walk",
            Some("Explore Lancaster's vibrant arts scene on the first Friday of every month. Galleries stay open late with special exhibitions."),
            30,
            Some("17:00"),
            Some("Downtown Lancaster Arts District"),
            Some("arts"),
            true,
            None,
            None,
            true,
        ),
        (
            "Lancaster Symphony",
            "tickets@lancastersymphony.org",
            "Beethoven's 9th Symphony",
            Some("Experience the power and majesty of Beethoven's final symphony, featuring the Lancaster Symphony Orchestra and Chorus."),
            14,
            Some("19:30"),
            Some("Fulton Theatre, 12 N Prince St"),
            Some("music"),
            false,
            Some(45.00),
            Some("https://www.lancastersymphony.org/tickets"),
            true,
        ),
        (
            "Lancaster Running Club",
            "info@lancasterrunning.org",
            "Red Rose 5K Run/Walk",
            Some("Join hundreds of runners and walkers for Lancaster's favorite 5K through historic downtown and County Park."),
            21,
            Some("07:00"),
            Some("Buchanan Park, Race St & Buchanan Ave"),
            Some("sports"),
            false,
            Some(35.00),
            Some("https://www.redrose5k.com"),
            true,
        ),
        // Unpublished events
        (
            "Community Volunteers",
            "volunteer@lancasterhelps.org",
            "Park Cleanup Day",
            Some("Help keep Lancaster beautiful! Join fellow volunteers for a community park cleanup. Supplies provided."),
            15,
            Some("09:00"),
            Some("Long's Park, 1441 Harrisburg Pike"),
            Some("community"),
            true,
            None,
            None,
            false,
        ),
    ];
    
    for (organizer_name, organizer_email, title, description, days_from_now, event_time, location, category, is_free, ticket_price, ticket_url, published) in events_data {
        let event_date = (Utc::now() + Duration::days(days_from_now)).date_naive();
        let event_time = event_time.map(|t| NaiveTime::parse_from_str(t, "%H:%M").unwrap());
        
        // Random time in the past for creation
        let created_days_ago = rng.gen_range(5..20);
        let created_at = Utc::now() - Duration::days(created_days_ago);
        
        // Always use English as original language for consistency
        let original_language = "en";
        let text_direction = "ltr";
        
        let event_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO events (
                organizer_name, organizer_email, title, description, event_date, event_time,
                location, category, is_free, ticket_price, ticket_url, original_language,
                text_direction, published, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING id
            "#
        )
        .bind(organizer_name)
        .bind(organizer_email)
        .bind(title)
        .bind(description)
        .bind(event_date)
        .bind(event_time)
        .bind(location)
        .bind(category)
        .bind(is_free)
        .bind(ticket_price)
        .bind(ticket_url)
        .bind(original_language)
        .bind(text_direction)
        .bind(published)
        .bind(created_at)
        .fetch_one(pool)
        .await?;
        
        // Add simulated translations for published events
        if published {
            let translations = vec![
                ("es", format!("ES: {}", title), description.map(|d| format!("ES: {}", d)), "ltr"),
                ("ar", format!("AR: {}", title), description.map(|d| format!("AR: {}", d)), "rtl"),
                ("fr", format!("FR: {}", title), description.map(|d| format!("FR: {}", d)), "ltr"),
            ];
            
            for (lang_code, trans_title, trans_desc, direction) in translations {
                sqlx::query(
                    r#"
                    INSERT INTO event_translations (event_id, language_code, title, description, text_direction)
                    VALUES ($1, $2, $3, $4, $5)
                    ON CONFLICT (event_id, language_code) DO NOTHING
                    "#
                )
                .bind(event_id)
                .bind(lang_code)
                .bind(&trans_title)
                .bind(&trans_desc)
                .bind(direction)
                .execute(pool)
                .await?;
            }
        }
    }
    
    let event_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM events").fetch_one(pool).await?;
    let published_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM events WHERE published = true").fetch_one(pool).await?;
    let unpublished_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM events WHERE published = false").fetch_one(pool).await?;
    
    println!("  ✓ Created {} events ({} published, {} awaiting review)", 
        event_count.0, published_count.0, unpublished_count.0);
    
    Ok(())
}