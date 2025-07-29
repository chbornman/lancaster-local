use chrono::{Utc, NaiveTime, Duration, NaiveDate};
use sqlx::postgres::PgPoolOptions;
use rand::Rng;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    dotenv::dotenv().ok();
    
    println!("🌱 Starting Lancaster Community Platform seed script...");
    println!("⏰ Started at: {}", Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));
    
    // Database connection
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("📍 Connecting to database: {}", database_url.split('@').last().unwrap_or("unknown"));
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| {
            eprintln!("❌ Failed to connect to database: {}", e);
            e
        })?;
    
    println!("✅ Connected to database successfully");
    
    // Clear existing data
    println!("\n🧹 Clearing existing data...");
    
    let event_trans_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM event_translations").fetch_one(&pool).await?;
    let post_trans_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM post_translations").fetch_one(&pool).await?;
    let events_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM events").fetch_one(&pool).await?;
    let posts_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM posts").fetch_one(&pool).await?;
    
    if event_trans_count.0 > 0 || post_trans_count.0 > 0 || events_count.0 > 0 || posts_count.0 > 0 {
        println!("  Found existing data:");
        if event_trans_count.0 > 0 { println!("    - {} event translations", event_trans_count.0); }
        if post_trans_count.0 > 0 { println!("    - {} post translations", post_trans_count.0); }
        if events_count.0 > 0 { println!("    - {} events", events_count.0); }
        if posts_count.0 > 0 { println!("    - {} posts", posts_count.0); }
        
        println!("  Deleting all existing data...");
        sqlx::query("DELETE FROM event_translations").execute(&pool).await?;
        sqlx::query("DELETE FROM post_translations").execute(&pool).await?;
        sqlx::query("DELETE FROM events").execute(&pool).await?;
        sqlx::query("DELETE FROM posts").execute(&pool).await?;
        println!("  ✓ All existing data cleared");
    } else {
        println!("  No existing data found");
    }
    
    // Seed supported languages if not exists
    println!("\n🌍 Ensuring languages are configured...");
    let lang_start = Instant::now();
    seed_languages(&pool).await?;
    println!("  ⏱️  Languages configured in {:.2}s", lang_start.elapsed().as_secs_f64());
    
    // Seed posts
    println!("\n📝 Creating posts...");
    let posts_start = Instant::now();
    seed_posts(&pool).await?;
    println!("  ⏱️  Posts created in {:.2}s", posts_start.elapsed().as_secs_f64());
    
    // Seed events  
    println!("\n📅 Creating events...");
    let events_start = Instant::now();
    seed_events(&pool).await?;
    println!("  ⏱️  Events created in {:.2}s", events_start.elapsed().as_secs_f64());
    
    let elapsed = start_time.elapsed();
    println!("\n✨ Seed completed successfully!");
    println!("⏱️  Total time: {:.2}s", elapsed.as_secs_f64());
    println!("🏁 Finished at: {}", Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));
    
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
        println!("  ✓ Inserted 9 default languages");
    } else {
        println!("  ✓ Languages already configured ({} languages found)", count.0);
    }
    
    Ok(())
}

async fn seed_posts(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let mut posts_created = 0;
    let mut translations_created = 0;
    
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
    
    println!("  Processing {} posts...", posts.len());
    
    // Insert posts and translations
    for (idx, post) in posts.iter().enumerate() {
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
        
        posts_created += 1;
        
        if (idx + 1) % 5 == 0 {
            println!("    Processed {}/{} posts...", idx + 1, posts.len());
        }
        
        // Add translations for published posts
        if post.published {
            // Translation mappings based on original language
            let translations: Vec<(&str, String, Option<String>, &str)> = match post.original_language {
                "en" => {
                    // Provide specific translations based on the post title
                    if post.title.contains("Community Garden") {
                        vec![
                            ("es", "El Ayuntamiento de Lancaster Anuncia Nueva Iniciativa de Jardín Comunitario".to_string(), 
                             Some("El Ayuntamiento de Lancaster se complace en anunciar una nueva iniciativa de jardín comunitario en el corazón del centro. Este proyecto tiene como objetivo unir a los residentes mientras promueve la agricultura urbana sostenible.".to_string()), "ltr"),
                            ("ar", "مجلس مدينة لانكستر يعلن عن مبادرة حديقة مجتمعية جديدة".to_string(),
                             Some("يسر مجلس مدينة لانكستر أن يعلن عن مبادرة حديقة مجتمعية جديدة في قلب وسط المدينة. يهدف هذا المشروع إلى جمع السكان معًا مع تعزيز الزراعة الحضرية المستدامة.".to_string()), "rtl"),
                            ("he", "מועצת העיר לנקסטר מכריזה על יוזמת גינה קהילתית חדשה".to_string(),
                             Some("מועצת העיר לנקסטר גאה להכריז על יוזמת גינה קהילתית חדשה בלב מרכז העיר. פרויקט זה נועד לחבר בין התושבים תוך קידום חקלאות עירונית בת קיימא.".to_string()), "rtl"),
                            ("fr", "Le Conseil Municipal de Lancaster Annonce une Nouvelle Initiative de Jardin Communautaire".to_string(),
                             Some("Le conseil municipal de Lancaster est heureux d'annoncer une nouvelle initiative de jardin communautaire au cœur du centre-ville. Ce projet vise à rassembler les résidents tout en promouvant l'agriculture urbaine durable.".to_string()), "ltr"),
                            ("de", "Stadtrat von Lancaster kündigt neue Gemeinschaftsgarten-Initiative an".to_string(),
                             Some("Der Stadtrat von Lancaster freut sich, eine neue Gemeinschaftsgarten-Initiative im Herzen der Innenstadt anzukündigen. Dieses Projekt zielt darauf ab, die Bewohner zusammenzubringen und gleichzeitig nachhaltige städtische Landwirtschaft zu fördern.".to_string()), "ltr"),
                            ("zh", "兰开斯特市议会宣布新的社区花园计划".to_string(),
                             Some("兰开斯特市议会很高兴地宣布在市中心启动一项新的社区花园计划。该项目旨在将居民聚集在一起，同时促进可持续的城市农业。".to_string()), "ltr"),
                            ("fa", "شورای شهر لنکستر طرح باغ اجتماعی جدید را اعلام کرد".to_string(),
                             Some("شورای شهر لنکستر با خوشحالی طرح باغ اجتماعی جدیدی را در قلب مرکز شهر اعلام می‌کند. این پروژه با هدف گرد هم آوردن ساکنان و ترویج کشاورزی شهری پایدار طراحی شده است.".to_string()), "rtl"),
                            ("ur", "لنکاسٹر سٹی کونسل نے نئی کمیونٹی گارڈن کی پہل کا اعلان کیا".to_string(),
                             Some("لنکاسٹر سٹی کونسل شہر کے مرکز میں ایک نئی کمیونٹی گارڈن کی پہل کا اعلان کرتے ہوئے خوشی محسوس کر رہی ہے۔ اس منصوبے کا مقصد رہائشیوں کو اکٹھا کرنا اور پائیدار شہری زراعت کو فروغ دینا ہے۔".to_string()), "rtl"),
                        ]
                    } else if post.title.contains("Central Market") {
                        vec![
                            ("es", "El Mercado Central Celebra 290 Años de Servicio".to_string(),
                             Some("El Mercado Central de Lancaster, uno de los mercados de agricultores en funcionamiento continuo más antiguos de Estados Unidos, celebra su 290 aniversario este año.".to_string()), "ltr"),
                            ("ar", "السوق المركزي يحتفل بـ 290 عامًا من الخدمة".to_string(),
                             Some("يحتفل سوق لانكستر المركزي، أحد أقدم أسواق المزارعين العاملة باستمرار في الولايات المتحدة، بالذكرى السنوية الـ 290 هذا العام.".to_string()), "rtl"),
                            ("he", "השוק המרכזי חוגג 290 שנות שירות".to_string(),
                             Some("השוק המרכזי של לנקסטר, אחד משווקי האיכרים הפועלים ברציפות הוותיקים ביותר בארצות הברית, חוגג השנה 290 שנה.".to_string()), "rtl"),
                            ("fr", "Le Marché Central Célèbre 290 Ans de Service".to_string(),
                             Some("Le marché central de Lancaster, l'un des plus anciens marchés fermiers en activité continue aux États-Unis, célèbre son 290e anniversaire cette année.".to_string()), "ltr"),
                            ("de", "Zentralmarkt feiert 290 Jahre Dienst".to_string(),
                             Some("Der Lancaster Central Market, einer der ältesten kontinuierlich betriebenen Bauernmärkte in den Vereinigten Staaten, feiert dieses Jahr sein 290-jähriges Jubiläum.".to_string()), "ltr"),
                            ("zh", "中央市场庆祝服务290周年".to_string(),
                             Some("兰开斯特中央市场是美国最古老的持续运营的农贸市场之一，今年庆祝其290周年纪念。".to_string()), "ltr"),
                            ("fa", "بازار مرکزی ۲۹۰ سال خدمت را جشن می‌گیرد".to_string(),
                             Some("بازار مرکزی لنکستر، یکی از قدیمی‌ترین بازارهای کشاورزان با فعالیت مداوم در ایالات متحده، امسال ۲۹۰مین سالگرد خود را جشن می‌گیرد.".to_string()), "rtl"),
                            ("ur", "سینٹرل مارکیٹ 290 سال کی خدمت کا جشن منا رہی ہے".to_string(),
                             Some("لنکاسٹر سینٹرل مارکیٹ، امریکہ میں مسلسل چلنے والی قدیم ترین کسان منڈیوں میں سے ایک، اس سال اپنی 290ویں سالگرہ منا رہی ہے۔".to_string()), "rtl"),
                        ]
                    } else if post.title.contains("Symphony Orchestra") {
                        vec![
                            ("es", "La Orquesta Sinfónica de Lancaster Anuncia Serie de Conciertos Gratuitos de Verano".to_string(),
                             Some("La Orquesta Sinfónica de Lancaster se complace en anunciar su serie anual de conciertos gratuitos de verano en Buchanan Park. Todos los domingos por la noche en julio y agosto, los residentes pueden disfrutar de música clásica bajo las estrellas.".to_string()), "ltr"),
                            ("ar", "أوركسترا لانكستر السيمفونية تعلن عن سلسلة حفلات صيفية مجانية".to_string(),
                             Some("تسر أوركسترا لانكستر السيمفونية أن تعلن عن سلسلة حفلاتها الصيفية المجانية السنوية في حديقة بوكانان. كل مساء أحد في يوليو وأغسطس، يمكن للمقيمين الاستمتاع بالموسيقى الكلاسيكية تحت النجوم.".to_string()), "rtl"),
                            ("he", "התזמורת הסימפונית של לנקסטר מכריזה על סדרת קונצרטים חינמיים בקיץ".to_string(),
                             Some("התזמורת הסימפונית של לנקסטר שמחה להכריז על סדרת הקונצרטים החינמיים השנתית שלה בפארק בוקנן. בכל ערב ראשון ביולי ואוגוסט, התושבים יכולים ליהנות ממוסיקה קלאסית תחת הכוכבים.".to_string()), "rtl"),
                            ("fr", "L'Orchestre Symphonique de Lancaster Annonce une Série de Concerts Gratuits d'Été".to_string(),
                             Some("L'Orchestre symphonique de Lancaster est ravi d'annoncer sa série annuelle de concerts gratuits d'été au parc Buchanan. Tous les dimanches soirs de juillet et août, les résidents peuvent profiter de musique classique sous les étoiles.".to_string()), "ltr"),
                            ("de", "Lancaster Symphonieorchester kündigt kostenlose Sommerkonzertreihe an".to_string(),
                             Some("Das Lancaster Symphony Orchestra freut sich, seine jährliche kostenlose Sommerkonzertreihe im Buchanan Park anzukündigen. Jeden Sonntagabend im Juli und August können die Bewohner klassische Musik unter den Sternen genießen.".to_string()), "ltr"),
                            ("zh", "兰开斯特交响乐团宣布免费夏季音乐会系列".to_string(),
                             Some("兰开斯特交响乐团很高兴地宣布在布坎南公园举办年度免费夏季音乐会系列。七月和八月的每个周日晚上，居民可以在星空下欣赏古典音乐。".to_string()), "ltr"),
                            ("fa", "ارکستر سمفونی لنکستر سری کنسرت‌های رایگان تابستانی را اعلام می‌کند".to_string(),
                             Some("ارکستر سمفونی لنکستر با خوشحالی سری کنسرت‌های رایگان تابستانی سالانه خود را در پارک بوکانان اعلام می‌کند. هر یکشنبه شب در ماه‌های جولای و آگوست، ساکنان می‌توانند از موسیقی کلاسیک زیر ستارگان لذت ببرند.".to_string()), "rtl"),
                            ("ur", "لنکاسٹر سمفنی آرکیسٹرا مفت موسم گرما کنسرٹ سیریز کا اعلان".to_string(),
                             Some("لنکاسٹر سمفنی آرکیسٹرا بکانن پارک میں اپنی سالانہ مفت موسم گرما کنسرٹ سیریز کا اعلان کرتے ہوئے خوشی محسوس کر رہا ہے۔ جولائی اور اگست میں ہر اتوار کی شام، رہائشی ستاروں کے نیچے کلاسیکی موسیقی سے لطف اندوز ہو سکتے ہیں۔".to_string()), "rtl"),
                        ]
                    } else if post.title.contains("Food Bank") {
                        vec![
                            ("es", "Oportunidades de Voluntariado en el Banco de Alimentos de Lancaster".to_string(),
                             Some("El Banco de Alimentos de Lancaster busca voluntarios para su programa ampliado de distribución de comidas. Con la creciente demanda de servicios, necesitamos miembros de la comunidad para ayudar a clasificar donaciones, empacar cajas de alimentos y asistir con la distribución.".to_string()), "ltr"),
                            ("ar", "فرص التطوع في بنك طعام لانكستر".to_string(),
                             Some("يبحث بنك طعام لانكستر عن متطوعين لبرنامجه الموسع لتوزيع الوجبات. مع تزايد الطلب على الخدمات، نحتاج إلى أفراد من المجتمع للمساعدة في فرز التبرعات وتعبئة صناديق الطعام والمساعدة في التوزيع.".to_string()), "rtl"),
                            ("he", "הזדמנויות התנדבות בבנק המזון של לנקסטר".to_string(),
                             Some("בנק המזון של לנקסטר מחפש מתנדבים לתוכנית חלוקת הארוחות המורחבת שלו. עם הביקוש הגובר לשירותים, אנו זקוקים לחברי קהילה שיסייעו במיון תרומות, אריזת קופסאות מזון וסיוע בחלוקה.".to_string()), "rtl"),
                            ("fr", "Opportunités de Bénévolat à la Banque Alimentaire de Lancaster".to_string(),
                             Some("La banque alimentaire de Lancaster recherche des bénévoles pour son programme élargi de distribution de repas. Avec la demande croissante de services, nous avons besoin de membres de la communauté pour aider à trier les dons, emballer des boîtes de nourriture et aider à la distribution.".to_string()), "ltr"),
                            ("de", "Freiwilligenmöglichkeiten bei der Lancaster Food Bank".to_string(),
                             Some("Die Lancaster Food Bank sucht Freiwillige für ihr erweitertes Essensverteilungsprogramm. Mit der steigenden Nachfrage nach Dienstleistungen benötigen wir Gemeindemitglieder, die beim Sortieren von Spenden, Verpacken von Lebensmittelboxen und bei der Verteilung helfen.".to_string()), "ltr"),
                            ("zh", "兰开斯特食品银行的志愿者机会".to_string(),
                             Some("兰开斯特食品银行正在为其扩展的餐食分发计划寻找志愿者。随着服务需求的增加，我们需要社区成员帮助分拣捐赠物品、打包食品盒和协助分发。".to_string()), "ltr"),
                            ("fa", "فرصت‌های داوطلبی در بانک غذای لنکستر".to_string(),
                             Some("بانک غذای لنکستر برای برنامه گسترده توزیع وعده‌های غذایی خود به دنبال داوطلبان است. با افزایش تقاضا برای خدمات، ما به اعضای جامعه نیاز داریم که در مرتب‌سازی کمک‌ها، بسته‌بندی جعبه‌های غذایی و کمک در توزیع یاری کنند.".to_string()), "rtl"),
                            ("ur", "لنکاسٹر فوڈ بینک میں رضاکارانہ مواقع".to_string(),
                             Some("لنکاسٹر فوڈ بینک اپنے وسیع کھانے کی تقسیم پروگرام کے لیے رضاکار تلاش کر رہا ہے۔ خدمات کی بڑھتی ہوئی مانگ کے ساتھ، ہمیں کمیونٹی ممبرز کی ضرورت ہے جو عطیات کی چھانٹی، کھانے کے ڈبوں کی پیکنگ اور تقسیم میں مدد کریں۔".to_string()), "rtl"),
                        ]
                    } else if post.title.contains("Trails") {
                        vec![
                            ("es", "El Sistema de Parques del Condado Añade 50 Millas de Nuevos Senderos".to_string(),
                             Some("Parques y Recreación del Condado de Lancaster ha completado una ambiciosa expansión de su sistema de senderos, añadiendo 50 millas de nuevos senderos para caminatas y ciclismo. Los nuevos senderos conectan parques existentes y áreas naturales.".to_string()), "ltr"),
                            ("ar", "نظام حدائق المقاطعة يضيف 50 ميلاً من المسارات الجديدة".to_string(),
                             Some("أكملت حدائق ومنتزهات مقاطعة لانكستر توسعاً طموحاً لنظام مساراتها، مضيفة 50 ميلاً من مسارات المشي وركوب الدراجات الجديدة. تربط المسارات الجديدة المتنزهات الحالية والمناطق الطبيعية.".to_string()), "rtl"),
                            ("he", "מערכת הפארקים של המחוז מוסיפה 50 מייל של שבילים חדשים".to_string(),
                             Some("פארקים ונופש של מחוז לנקסטר השלימו הרחבה שאפתנית של מערכת השבילים שלהם, והוסיפו 50 מייל של שבילי הליכה ורכיבה חדשים. השבילים החדשים מחברים פארקים קיימים ואזורים טבעיים.".to_string()), "rtl"),
                            ("fr", "Le Système de Parcs du Comté Ajoute 50 Miles de Nouveaux Sentiers".to_string(),
                             Some("Les parcs et loisirs du comté de Lancaster ont achevé une expansion ambitieuse de leur système de sentiers, ajoutant 50 miles de nouveaux sentiers de randonnée et de vélo. Les nouveaux sentiers relient les parcs existants et les espaces naturels.".to_string()), "ltr"),
                            ("de", "Das County Park System fügt 50 Meilen neue Wanderwege hinzu".to_string(),
                             Some("Lancaster County Parks and Recreation hat eine ehrgeizige Erweiterung seines Wegesystems abgeschlossen und 50 Meilen neue Wander- und Radwege hinzugefügt. Die neuen Wege verbinden bestehende Parks und Naturgebiete.".to_string()), "ltr"),
                            ("zh", "县公园系统新增50英里新步道".to_string(),
                             Some("兰开斯特县公园和娱乐部门完成了其步道系统的雄心勃勃的扩建，新增了50英里的徒步和自行车道。新步道连接了现有的公园和自然区域。".to_string()), "ltr"),
                            ("fa", "سیستم پارک‌های منطقه ۵۰ مایل مسیر جدید اضافه می‌کند".to_string(),
                             Some("پارک‌ها و تفریحات شهرستان لنکستر توسعه جاه‌طلبانه سیستم مسیرهای خود را تکمیل کرده و ۵۰ مایل مسیر پیاده‌روی و دوچرخه‌سواری جدید اضافه کرده است. مسیرهای جدید پارک‌ها و مناطق طبیعی موجود را به هم متصل می‌کنند.".to_string()), "rtl"),
                            ("ur", "کاؤنٹی پارک سسٹم نے 50 میل نئی پگڈنڈیاں شامل کیں".to_string(),
                             Some("لنکاسٹر کاؤنٹی پارکس اینڈ ریکریشن نے اپنے ٹریل سسٹم کی بڑی توسیع مکمل کی ہے، 50 میل نئی پیدل چلنے اور سائیکلنگ کی پگڈنڈیاں شامل کی ہیں۔ نئی پگڈنڈیاں موجودہ پارکوں اور قدرتی علاقوں کو جوڑتی ہیں۔".to_string()), "rtl"),
                        ]
                    } else if post.title.contains("Health Screenings") {
                        vec![
                            ("es", "Exámenes de Salud Gratuitos en el Centro Comunitario".to_string(),
                             Some("Lancaster General Health ofrecerá exámenes de salud gratuitos que incluyen controles de presión arterial, pruebas de glucosa y cálculos de IMC. Profesionales de la salud estarán disponibles para responder preguntas.".to_string()), "ltr"),
                            ("ar", "فحوصات صحية مجانية في المركز المجتمعي".to_string(),
                             Some("ستقدم صحة لانكستر العامة فحوصات صحية مجانية تشمل فحوصات ضغط الدم واختبارات الجلوكوز وحسابات مؤشر كتلة الجسم. سيكون المتخصصون في الرعاية الصحية متاحين للإجابة على الأسئلة.".to_string()), "rtl"),
                            ("he", "בדיקות בריאות חינמיות במרכז הקהילתי".to_string(),
                             Some("בריאות כללית לנקסטר תציע בדיקות בריאות חינמיות כולל בדיקות לחץ דם, בדיקות גלוקוז וחישובי BMI. אנשי מקצוע בתחום הבריאות יהיו זמינים לענות על שאלות.".to_string()), "rtl"),
                            ("fr", "Dépistages de Santé Gratuits au Centre Communautaire".to_string(),
                             Some("Lancaster General Health offrira des dépistages de santé gratuits comprenant des contrôles de tension artérielle, des tests de glucose et des calculs d'IMC. Des professionnels de la santé seront disponibles pour répondre aux questions.".to_string()), "ltr"),
                            ("de", "Kostenlose Gesundheitsuntersuchungen im Gemeindezentrum".to_string(),
                             Some("Lancaster General Health wird kostenlose Gesundheitsuntersuchungen anbieten, einschließlich Blutdruckkontrollen, Glukosetests und BMI-Berechnungen. Gesundheitsfachkräfte stehen zur Verfügung, um Fragen zu beantworten.".to_string()), "ltr"),
                            ("zh", "社区中心免费健康检查".to_string(),
                             Some("兰开斯特综合健康将提供免费健康检查，包括血压检查、葡萄糖测试和BMI计算。医疗保健专业人员将在现场回答问题。".to_string()), "ltr"),
                            ("fa", "معاینات رایگان سلامت در مرکز اجتماعی".to_string(),
                             Some("بهداشت عمومی لنکستر معاینات رایگان سلامت شامل چک فشار خون، آزمایش قند خون و محاسبات BMI ارائه خواهد داد. متخصصان مراقبت‌های بهداشتی برای پاسخ به سؤالات در دسترس خواهند بود.".to_string()), "rtl"),
                            ("ur", "کمیونٹی سینٹر میں مفت صحت کی جانچ".to_string(),
                             Some("لنکاسٹر جنرل ہیلتھ مفت صحت کی جانچ پیش کرے گی جس میں بلڈ پریشر چیک، گلوکوز ٹیسٹ اور BMI کیلکولیشن شامل ہیں۔ صحت کی دیکھ بھال کے پیشہ ور افراد سوالات کے جواب دینے کے لیے دستیاب ہوں گے۔".to_string()), "rtl"),
                        ]
                    } else {
                        // Generic translations for other posts
                        vec![
                            ("es", format!("ES: {}", post.title), post.content.map(|c| format!("ES: {}", c)), "ltr"),
                            ("ar", format!("AR: {}", post.title), post.content.map(|c| format!("AR: {}", c)), "rtl"),
                            ("he", format!("HE: {}", post.title), post.content.map(|c| format!("HE: {}", c)), "rtl"),
                            ("fr", format!("FR: {}", post.title), post.content.map(|c| format!("FR: {}", c)), "ltr"),
                            ("de", format!("DE: {}", post.title), post.content.map(|c| format!("DE: {}", c)), "ltr"),
                            ("zh", format!("ZH: {}", post.title), post.content.map(|c| format!("ZH: {}", c)), "ltr"),
                            ("fa", format!("FA: {}", post.title), post.content.map(|c| format!("FA: {}", c)), "rtl"),
                            ("ur", format!("UR: {}", post.title), post.content.map(|c| format!("UR: {}", c)), "rtl"),
                        ]
                    }
                },
                "es" => {
                    if post.title.contains("Alfabetización Digital") {
                        vec![
                            ("en", "New Digital Literacy Program for Seniors".to_string(), 
                             Some("The Lancaster Public Library is proud to introduce free digital literacy classes designed specifically for seniors. Learn basic computer skills, internet safety, and how to connect with loved ones through video calls.".to_string()), "ltr"),
                            ("ar", "برنامج جديد لمحو الأمية الرقمية لكبار السن".to_string(),
                             Some("تفخر مكتبة لانكستر العامة بتقديم دروس مجانية لمحو الأمية الرقمية مصممة خصيصًا لكبار السن. تعلم مهارات الكمبيوتر الأساسية وأمان الإنترنت وكيفية التواصل مع أحبائك من خلال مكالمات الفيديو.".to_string()), "rtl"),
                            ("he", "תוכנית חדשה לאוריינות דיגיטלית למבוגרים".to_string(),
                             Some("הספרייה הציבורית של לנקסטר גאה להציג שיעורי אוריינות דיגיטלית בחינם המיועדים במיוחד למבוגרים. למדו כישורי מחשב בסיסיים, בטיחות באינטרנט וכיצד להתחבר ליקיריכם באמצעות שיחות וידאו.".to_string()), "rtl"),
                            ("fr", "Nouveau Programme d'Alphabétisation Numérique pour les Aînés".to_string(),
                             Some("La bibliothèque publique de Lancaster est fière de présenter des cours gratuits d'alphabétisation numérique conçus spécifiquement pour les aînés. Apprenez les compétences informatiques de base, la sécurité sur Internet et comment vous connecter avec vos proches par appels vidéo.".to_string()), "ltr"),
                            ("de", "Neues Programm zur digitalen Alphabetisierung für Senioren".to_string(),
                             Some("Die öffentliche Bibliothek von Lancaster ist stolz darauf, kostenlose Kurse zur digitalen Alphabetisierung anzubieten, die speziell für Senioren konzipiert wurden. Lernen Sie grundlegende Computerkenntnisse, Internetsicherheit und wie Sie sich über Videoanrufe mit Ihren Lieben verbinden können.".to_string()), "ltr"),
                            ("zh", "老年人数字扫盲新计划".to_string(),
                             Some("兰开斯特公共图书馆自豪地推出专为老年人设计的免费数字扫盲课程。学习基本的电脑技能、互联网安全以及如何通过视频通话与亲人联系。".to_string()), "ltr"),
                            ("fa", "برنامه جدید سواد دیجیتال برای سالمندان".to_string(),
                             Some("کتابخانه عمومی لنکستر با افتخار کلاس‌های رایگان سواد دیجیتال را که مخصوص سالمندان طراحی شده است ارائه می‌دهد. مهارت‌های اساسی کامپیوتر، امنیت اینترنت و نحوه اتصال با عزیزان از طریق تماس‌های ویدیویی را بیاموزید.".to_string()), "rtl"),
                            ("ur", "بزرگوں کے لیے ڈیجیٹل خواندگی کا نیا پروگرام".to_string(),
                             Some("لنکاسٹر پبلک لائبریری فخر سے بزرگوں کے لیے خصوصی طور پر ڈیزائن کی گئی مفت ڈیجیٹل خواندگی کی کلاسیں پیش کر رہی ہے۔ بنیادی کمپیوٹر کی مہارتیں، انٹرنیٹ کی حفاظت اور ویڈیو کالز کے ذریعے اپنے پیاروں سے رابطہ کرنے کا طریقہ سیکھیں۔".to_string()), "rtl"),
                        ]
                    } else if post.title.contains("Festival Internacional") {
                        vec![
                            ("en", "International Food Festival Downtown".to_string(),
                             Some("Mark your calendars for Lancaster's third annual International Food Festival, celebrating our community's cultural diversity. Over 40 vendors will offer cuisines from around the world, with live music and cultural performances throughout the day.".to_string()), "ltr"),
                            ("ar", "مهرجان الطعام الدولي في وسط المدينة".to_string(),
                             Some("ضعوا علامة في تقاويمكم للمهرجان الدولي الثالث للطعام في لانكستر، احتفالاً بالتنوع الثقافي لمجتمعنا. سيقدم أكثر من 40 بائعًا أطباقًا من جميع أنحاء العالم، مع موسيقى حية وعروض ثقافية طوال اليوم.".to_string()), "rtl"),
                            ("he", "פסטיבל האוכל הבינלאומי במרכז העיר".to_string(),
                             Some("סמנו ביומנים שלכם את פסטיבל האוכל הבינלאומי השלישי השנתי של לנקסטר, החוגג את הגיוון התרבותי של הקהילה שלנו. למעלה מ-40 דוכנים יציעו מטבחים מרחבי העולם, עם מוזיקה חיה והופעות תרבותיות לאורך כל היום.".to_string()), "rtl"),
                            ("fr", "Festival International de Cuisine au Centre-Ville".to_string(),
                             Some("Marquez vos calendriers pour le troisième Festival International de Cuisine annuel de Lancaster, célébrant la diversité culturelle de notre communauté. Plus de 40 vendeurs offriront des cuisines du monde entier, avec de la musique live et des performances culturelles toute la journée.".to_string()), "ltr"),
                            ("de", "Internationales Food Festival in der Innenstadt".to_string(),
                             Some("Markieren Sie Ihre Kalender für Lancasters drittes jährliches Internationales Food Festival, das die kulturelle Vielfalt unserer Gemeinschaft feiert. Über 40 Anbieter werden Küchen aus aller Welt anbieten, mit Live-Musik und kulturellen Darbietungen den ganzen Tag über.".to_string()), "ltr"),
                            ("zh", "市中心国际美食节".to_string(),
                             Some("请在您的日历上标记兰开斯特第三届年度国际美食节，庆祝我们社区的文化多样性。超40家商贩将提供来自世界各地的美食，全天都有现场音乐和文化表演。".to_string()), "ltr"),
                            ("fa", "جشنواره بین‌المللی غذا در مرکز شهر".to_string(),
                             Some("تقویم خود را برای سومین جشنواره سالانه بین‌المللی غذای لنکستر علامت‌گذاری کنید که تنوع فرهنگی جامعه ما را جشن می‌گیرد. بیش از ۴۰ فروشنده غذاهایی از سراسر جهان را عرضه خواهند کرد، با موسیقی زنده و اجراهای فرهنگی در طول روز.".to_string()), "rtl"),
                            ("ur", "شہر کے مرکز میں بین الاقوامی فوڈ فیسٹیول".to_string(),
                             Some("لنکاسٹر کے تیسرے سالانہ بین الاقوامی فوڈ فیسٹیول کے لیے اپنے کیلنڈروں میں نشان لگائیں، جو ہماری کمیونٹی کے ثقافتی تنوع کا جشن مناتا ہے۔ 40 سے زیادہ وینڈرز دنیا بھر کے کھانے پیش کریں گے، دن بھر لائیو میوزک اور ثقافتی پرفارمنس کے ساتھ۔".to_string()), "rtl"),
                        ]
                    } else {
                        vec![
                            ("en", format!("EN: {}", post.title), post.content.map(|c| format!("EN: {}", c)), "ltr"),
                            ("ar", format!("AR: {}", post.title), post.content.map(|c| format!("AR: {}", c)), "rtl"),
                            ("he", format!("HE: {}", post.title), post.content.map(|c| format!("HE: {}", c)), "rtl"),
                            ("fr", format!("FR: {}", post.title), post.content.map(|c| format!("FR: {}", c)), "ltr"),
                            ("de", format!("DE: {}", post.title), post.content.map(|c| format!("DE: {}", c)), "ltr"),
                            ("zh", format!("ZH: {}", post.title), post.content.map(|c| format!("ZH: {}", c)), "ltr"),
                            ("fa", format!("FA: {}", post.title), post.content.map(|c| format!("FA: {}", c)), "rtl"),
                            ("ur", format!("UR: {}", post.title), post.content.map(|c| format!("UR: {}", c)), "rtl"),
                        ]
                    }
                },
                "ar" => {
                    if post.title.contains("برنامج") {
                        vec![
                            ("en", "New Refugee Support Program in Lancaster".to_string(),
                             Some("Lancaster Cultural Diversity Center is pleased to announce a comprehensive new support program for refugees and new immigrants. The program includes English language classes, job skills training, and community integration assistance.".to_string()), "ltr"),
                            ("es", "Nuevo Programa de Apoyo a Refugiados en Lancaster".to_string(),
                             Some("El Centro de Diversidad Cultural de Lancaster se complace en anunciar un nuevo programa integral de apoyo para refugiados y nuevos inmigrantes. El programa incluye clases de inglés, capacitación en habilidades laborales y asistencia para la integración comunitaria.".to_string()), "ltr"),
                            ("he", "תוכנית תמיכה חדשה לפליטים בלנקסטר".to_string(),
                             Some("מרכז הגיוון התרבותי של לנקסטר שמח להכריז על תוכנית תמיכה מקיפה חדשה לפליטים ומהגרים חדשים. התוכנית כוללת שיעורי אנגלית, הכשרה לכישורי עבודה וסיוע בהשתלבות בקהילה.".to_string()), "rtl"),
                            ("fr", "Nouveau Programme de Soutien aux Réfugiés à Lancaster".to_string(),
                             Some("Le Centre de Diversité Culturelle de Lancaster est heureux d'annoncer un nouveau programme de soutien complet pour les réfugiés et les nouveaux immigrants. Le programme comprend des cours d'anglais, une formation aux compétences professionnelles et une aide à l'intégration communautaire.".to_string()), "ltr"),
                            ("de", "Neues Flüchtlingshilfsprogramm in Lancaster".to_string(),
                             Some("Das Lancaster Cultural Diversity Center freut sich, ein umfassendes neues Unterstützungsprogramm für Flüchtlinge und neue Einwanderer anzukündigen. Das Programm umfasst Englischkurse, Berufsausbildung und Unterstützung bei der Integration in die Gemeinschaft.".to_string()), "ltr"),
                            ("zh", "兰开斯特新难民支持计划".to_string(),
                             Some("兰开斯特文化多样性中心很高兴地宣布为难民和新移民提供全面的新支持计划。该计划包括英语课程、职业技能培训和社区融入协助。".to_string()), "ltr"),
                            ("fa", "برنامه جدید حمایت از پناهندگان در لنکستر".to_string(),
                             Some("مرکز تنوع فرهنگی لنکستر مفتخر است که برنامه حمایتی جامع جدیدی را برای پناهندگان و مهاجران تازه وارد اعلام کند. این برنامه شامل کلاس‌های زبان انگلیسی، آموزش مهارت‌های شغلی و کمک در ادغام با جامعه می‌باشد.".to_string()), "rtl"),
                            ("ur", "لنکاسٹر میں پناہ گزینوں کے لیے نیا امدادی پروگرام".to_string(),
                             Some("لنکاسٹر کلچرل ڈائیورسٹی سینٹر پناہ گزینوں اور نئے تارکین وطن کے لیے ایک جامع نیا امدادی پروگرام کا اعلان کرتے ہوئے خوشی محسوس کر رہا ہے۔ اس پروگرام میں انگریزی زبان کی کلاسیں، پیشہ ورانہ مہارتوں کی تربیت اور کمیونٹی میں انضمام میں مدد شامل ہے۔".to_string()), "rtl"),
                        ]
                    } else {
                        vec![
                            ("en", "Arabic Cooking Classes at Community Center".to_string(),
                             Some("Join us for a series of authentic Arabic cooking classes every Saturday. We'll learn together how to prepare traditional dishes from various Arab countries. Classes are open to everyone and require no prior experience.".to_string()), "ltr"),
                            ("es", "Clases de Cocina Árabe en el Centro Comunitario".to_string(),
                             Some("Únanse a nosotros para una serie de clases auténticas de cocina árabe todos los sábados. Aprenderemos juntos a preparar platos tradicionales de varios países árabes. Las clases están abiertas para todos y no requieren experiencia previa.".to_string()), "ltr"),
                            ("he", "שיעורי בישול ערבי במרכז הקהילתי".to_string(),
                             Some("הצטרפו אלינו לסדרת שיעורי בישול ערבי אותנטי בכל יום שבת. נלמד יחד כיצד להכין מאכלים מסורתיים ממדינות ערביות שונות. השיעורים פתוחים לכולם ואינם דורשים ניסיון קודם.".to_string()), "rtl"),
                            ("fr", "Cours de Cuisine Arabe au Centre Communautaire".to_string(),
                             Some("Rejoignez-nous pour une série de cours de cuisine arabe authentique tous les samedis. Nous apprendrons ensemble à préparer des plats traditionnels de divers pays arabes. Les cours sont ouverts à tous et ne nécessitent aucune expérience préalable.".to_string()), "ltr"),
                            ("de", "Arabische Kochkurse im Gemeindezentrum".to_string(),
                             Some("Begleiten Sie uns zu einer Reihe authentischer arabischer Kochkurse jeden Samstag. Wir lernen gemeinsam, traditionelle Gerichte aus verschiedenen arabischen Ländern zuzubereiten. Die Kurse stehen allen offen und erfordern keine Vorerfahrung.".to_string()), "ltr"),
                            ("zh", "社区中心的阿拉伯烹饪课程".to_string(),
                             Some("加入我们每周六的正宗阿拉伯烹饪课程。我们将一起学习如何准备来自不同阿拉伯国家的传统菜肴。课程对所有人开放，不需要任何经验。".to_string()), "ltr"),
                            ("fa", "کلاس‌های آشپزی عربی در مرکز اجتماعی".to_string(),
                             Some("به ما برای یک سری کلاس‌های آشپزی عربی اصیل هر شنبه بپیوندید. با هم نحوه تهیه غذاهای سنتی از کشورهای مختلف عربی را یاد می‌گیریم. کلاس‌ها برای همه باز است و نیازی به تجربه قبلی ندارد.".to_string()), "rtl"),
                            ("ur", "کمیونٹی سینٹر میں عربی کھانا پکانے کی کلاسیں".to_string(),
                             Some("ہر ہفتہ مصدقہ عربی کھانا پکانے کی کلاسوں کے لیے ہمارے ساتھ شامل ہوں۔ ہم مل کر مختلف عرب ممالک کے روایتی پکوان تیار کرنا سیکھیں گے۔ کلاسیں سب کے لیے کھلی ہیں اور کسی سابقہ تجربے کی ضرورت نہیں ہے۔".to_string()), "rtl"),
                        ]
                    }
                },
                "fr" => {
                    if post.title.contains("Français") {
                        vec![
                            ("en", "Free French Classes at the Library".to_string(),
                             Some("The Lancaster French community is happy to offer free French classes for all levels. Classes are held every Wednesday evening at the public library. A great opportunity to learn a new language and meet new people.".to_string()), "ltr"),
                            ("es", "Clases Gratuitas de Francés en la Biblioteca".to_string(),
                             Some("La comunidad francesa de Lancaster se complace en ofrecer clases gratuitas de francés para todos los niveles. Las clases se realizan todos los miércoles por la noche en la biblioteca pública. Una excelente oportunidad para aprender un nuevo idioma y conocer gente nueva.".to_string()), "ltr"),
                            ("ar", "دروس اللغة الفرنسية المجانية في المكتبة".to_string(),
                             Some("يسر المجتمع الفرنسي في لانكستر أن يقدم دروسًا مجانية في اللغة الفرنسية لجميع المستويات. تُعقد الدروس كل مساء أربعاء في المكتبة العامة. فرصة رائعة لتعلم لغة جديدة ومقابلة أشخاص جدد.".to_string()), "rtl"),
                            ("he", "שיעורי צרפתית חינם בספרייה".to_string(),
                             Some("הקהילה הצרפתית של לנקסטר שמחה להציע שיעורי צרפתית חינם לכל הרמות. השיעורים מתקיימים בכל יום רביעי בערב בספרייה הציבורית. הזדמנות מצוינת ללמוד שפה חדשה ולהכיר אנשים חדשים.".to_string()), "rtl"),
                            ("de", "Kostenlose Französischkurse in der Bibliothek".to_string(),
                             Some("Die französische Gemeinde von Lancaster freut sich, kostenlose Französischkurse für alle Niveaus anzubieten. Die Kurse finden jeden Mittwochabend in der öffentlichen Bibliothek statt. Eine großartige Gelegenheit, eine neue Sprache zu lernen und neue Leute kennenzulernen.".to_string()), "ltr"),
                            ("zh", "图书馆免费法语课程".to_string(),
                             Some("兰开斯特法语社区很高兴为所有级别提供免费法语课程。课程在每周三晚上在公共图书馆举行。这是学习新语言和结识新朋友的绝佳机会。".to_string()), "ltr"),
                            ("fa", "کلاس‌های رایگان فرانسه در کتابخانه".to_string(),
                             Some("جامعه فرانسوی لنکستر مفتخر است که کلاس‌های رایگان فرانسه را برای همه سطوح ارائه دهد. کلاس‌ها هر چهارشنبه شب در کتابخانه عمومی برگزار می‌شوند. فرصتی عالی برای یادگیری زبان جدید و ملاقات با افراد جدید.".to_string()), "rtl"),
                            ("ur", "لائبریری میں مفت فرانسیسی کلاسیں".to_string(),
                             Some("لنکاسٹر کی فرانسیسی کمیونٹی تمام سطحوں کے لیے مفت فرانسیسی کلاسیں پیش کرتے ہوئے خوشی محسوس کر رہی ہے۔ کلاسیں ہر بدھ کی شام پبلک لائبریری میں ہوتی ہیں۔ نئی زبان سیکھنے اور نئے لوگوں سے ملنے کا ایک بہترین موقع۔".to_string()), "rtl"),
                        ]
                    } else {
                        vec![
                            ("en", "Monthly Flea Market at Central Park".to_string(),
                             Some("Join us for our monthly flea market at Lancaster Central Park. Find unique treasures, antiques, and local artisan creations. Vendors are welcome - register online to reserve your spot.".to_string()), "ltr"),
                            ("es", "Mercado de Pulgas Mensual en el Parque Central".to_string(),
                             Some("Únase a nosotros para nuestro mercado de pulgas mensual en el Parque Central de Lancaster. Encuentre tesoros únicos, antigüedades y creaciones de artesanos locales. Los vendedores son bienvenidos: regístrese en línea para reservar su lugar.".to_string()), "ltr"),
                            ("ar", "سوق البرغوث الشهري في الحديقة المركزية".to_string(),
                             Some("انضموا إلينا في سوق البرغوث الشهري في حديقة لانكستر المركزية. اعثروا على كنوز فريدة والتحف وإبداعات الحرفيين المحليين. نرحب بالبائعين - سجلوا عبر الإنترنت لحجز مكانكم.".to_string()), "rtl"),
                            ("he", "שוק הפשפשים החודשי בפארק המרכזי".to_string(),
                             Some("הצטרפו אלינו לשוק הפשפשים החודשי שלנו בפארק המרכזי של לנקסטר. מצאו אוצרות ייחודיים, עתיקות ויצירות של אומנים מקומיים. מוכרים מוזמנים - הירשמו באינטרנט כדי לשמור על המקום שלכם.".to_string()), "rtl"),
                            ("de", "Monatlicher Flohmarkt im Central Park".to_string(),
                             Some("Besuchen Sie unseren monatlichen Flohmarkt im Lancaster Central Park. Finden Sie einzigartige Schätze, Antiquitäten und lokale Kunsthandwerkskreationen. Verkäufer sind willkommen - registrieren Sie sich online, um Ihren Platz zu reservieren.".to_string()), "ltr"),
                            ("zh", "中央公园每月跳蚤市场".to_string(),
                             Some("加入我们在兰开斯特中央公园的每月跳蚤市场。找到独特的宝贝、古董和当地手工艺品。欢迎商贩 - 在线注册以预留您的摊位。".to_string()), "ltr"),
                            ("fa", "بازار کهنه‌فروشی ماهانه در پارک مرکزی".to_string(),
                             Some("به بازار کهنه‌فروشی ماهانه ما در پارک مرکزی لنکستر بپیوندید. گنجینه‌های منحصر به فرد، عتیقه‌ها و آثار هنری صنعتگران محلی را پیدا کنید. فروشندگان پذیرفته می‌شوند - برای رزرو جایگاه خود آنلاین ثبت نام کنید.".to_string()), "rtl"),
                            ("ur", "سینٹرل پارک میں ماہانہ فلی مارکیٹ".to_string(),
                             Some("لنکاسٹر سینٹرل پارک میں ہماری ماہانہ فلی مارکیٹ میں شامل ہوں۔ منفرد خزانے، نوادرات اور مقامی دستکاری کی تخلیقات تلاش کریں۔ فروشندگان کا خیرمقدم ہے - اپنی جگہ محفوظ کرنے کے لیے آن لائن رجسٹر کریں۔".to_string()), "rtl"),
                        ]
                    }
                },
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
                
                translations_created += 1;
            }
        }
    }
    
    let post_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM posts").fetch_one(pool).await?;
    let published_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM posts WHERE published = true").fetch_one(pool).await?;
    let unpublished_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM posts WHERE published = false").fetch_one(pool).await?;
    let trans_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM post_translations").fetch_one(pool).await?;
    
    println!("  ✓ Created {} posts ({} published, {} awaiting review)", 
        post_count.0, published_count.0, unpublished_count.0);
    println!("  ✓ Created {} post translations (avg {:.1} per published post)", 
        trans_count.0, trans_count.0 as f64 / published_count.0 as f64);
    
    Ok(())
}

async fn seed_events(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let mut events_created = 0;
    let mut translations_created = 0;
    
    // Lancaster-themed events - mixing some relative dates with specific July 2025 dates
    let events_data = vec![
        // Published events - July 2025 specific events
        (
            "Lancaster Farmers Market",
            "info@lancastermarket.org",
            "Weekly Farmers Market - Independence Week Special",
            Some("Shop fresh, locally grown produce, artisanal goods, and handmade crafts at Lancaster's premier farmers market. Special patriotic themed vendors for Independence Day week!"),
            NaiveDate::from_ymd_opt(2025, 7, 3).unwrap(),
            Some("08:00"),
            Some("Lancaster Central Market, 23 N Market St"),
            Some("market"),
            true,
            None,
            true,
        ),
        (
            "City of Lancaster",
            "events@cityoflancasterpa.gov",
            "4th of July Fireworks Spectacular",
            Some("Join us for Lancaster's biggest fireworks display! Food trucks, live music, and family activities start at 6 PM. Fireworks begin at dusk. Bring blankets and chairs!"),
            NaiveDate::from_ymd_opt(2025, 7, 4).unwrap(),
            Some("18:00"),
            Some("Buchanan Park, Race St & Buchanan Ave"),
            Some("community"),
            true,
            None,
            true,
        ),
        (
            "Lancaster Symphony",
            "tickets@lancastersymphony.org",
            "Summer Pops: Music of John Williams",
            Some("Enjoy the iconic film music of John Williams under the stars! Featuring themes from Star Wars, Jurassic Park, Harry Potter, and more. Bring a picnic!"),
            NaiveDate::from_ymd_opt(2025, 7, 12).unwrap(),
            Some("19:30"),
            Some("Buchanan Park Amphitheater"),
            Some("music"),
            true,
            None,
            true,
        ),
        (
            "Lancaster Running Club",
            "info@lancasterrunning.org",
            "Beat the Heat 5K Run/Walk",
            Some("Early morning summer 5K through shaded paths of County Park. Post-race refreshments and ice cream social included!"),
            NaiveDate::from_ymd_opt(2025, 7, 19).unwrap(),
            Some("07:00"),
            Some("Lancaster County Park, 1050 Rockford Rd"),
            Some("sports"),
            false,
            Some("https://www.lancasterrunning.org/summer5k"),
            true,
        ),
        (
            "Lancaster Diversity Coalition",
            "info@lancasterdiversity.org",
            "International Food & Culture Festival",
            Some("Celebrate Lancaster's cultural diversity! Enjoy food from 20+ countries, traditional performances, craft vendors, and children's activities."),
            NaiveDate::from_ymd_opt(2025, 7, 26).unwrap(),
            Some("11:00"),
            Some("Penn Square & King Street"),
            Some("cultural"),
            true,
            None,
            true,
        ),
        // Regular relative date events
        (
            "First Friday Lancaster",
            "events@firstfridaylancaster.com", 
            "First Friday Arts Walk",
            Some("Explore Lancaster's vibrant arts scene on the first Friday of every month. Galleries stay open late with special exhibitions."),
            NaiveDate::from_ymd_opt(2025, 8, 1).unwrap(), // August First Friday
            Some("17:00"),
            Some("Downtown Lancaster Arts District"),
            Some("arts"),
            true,
            None,
            true,
        ),
        (
            "Lancaster Library System",
            "programs@lancasterlibrary.org",
            "Summer Reading Challenge Finale",
            Some("Celebrate the end of our summer reading challenge! Prizes, games, face painting, and a special appearance by children's author Sarah Johnson."),
            NaiveDate::from_ymd_opt(2025, 7, 31).unwrap(),
            Some("14:00"),
            Some("Lancaster Public Library, 125 N Duke St"),
            Some("education"),
            true,
            None,
            true,
        ),
        // Unpublished events
        (
            "Community Volunteers",
            "volunteer@lancasterhelps.org",
            "Park Cleanup Day",
            Some("Help keep Lancaster beautiful! Join fellow volunteers for a community park cleanup. Supplies provided."),
            NaiveDate::from_ymd_opt(2025, 7, 13).unwrap(),
            Some("09:00"),
            Some("Long's Park, 1441 Harrisburg Pike"),
            Some("community"),
            true,
            None,
            false,
        ),
        (
            "Lancaster Youth Soccer",
            "info@lancasteryouthsoccer.org",
            "Summer Soccer Camp Registration Opens",
            Some("Register your child for our popular summer soccer camp! Ages 6-14, all skill levels welcome. Professional coaching in a fun environment."),
            NaiveDate::from_ymd_opt(2025, 7, 21).unwrap(),
            Some("09:00"),
            Some("Lancaster Soccer Complex, 2895 Willow Street Pike"),
            Some("sports"),
            false,
            Some("https://www.lancasteryouthsoccer.org/camp"),
            false,
        ),
    ];
    
    println!("  Processing {} events...", events_data.len());
    
    for (idx, (organizer_name, organizer_email, title, description, event_date, event_time, location, category, is_free, ticket_url, published)) in events_data.into_iter().enumerate() {
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
                location, category, is_free, ticket_url, original_language,
                text_direction, published, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
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
        .bind(ticket_url)
        .bind(original_language)
        .bind(text_direction)
        .bind(published)
        .bind(created_at)
        .fetch_one(pool)
        .await?;
        
        events_created += 1;
        
        // Add simulated translations for published events
        if published {
            let translations = match title {
                "Weekly Farmers Market - Independence Week Special" => {
                    vec![
                        ("es", "Mercado de Agricultores Semanal - Especial Semana de la Independencia".to_string(), 
                         Some("Compre productos frescos cultivados localmente, productos artesanales y artesanías hechas a mano en el principal mercado de agricultores de Lancaster. ¡Vendedores con temática patriótica especial para la semana del Día de la Independencia!".to_string()), "ltr"),
                        ("ar", "سوق المزارعين الأسبوعي - خاص بأسبوع الاستقلال".to_string(),
                         Some("تسوق للحصول على منتجات طازجة مزروعة محليًا والسلع الحرفية والحرف اليدوية في سوق المزارعين الرئيسي في لانكستر. بائعون ذوو طابع وطني خاص لأسبوع عيد الاستقلال!".to_string()), "rtl"),
                        ("he", "שוק איכרים שבועי - מיוחד לשבוע העצמאות".to_string(),
                         Some("קנו תוצרת טרייה שגדלה באזור, מוצרים אומנותיים ומלאכות יד בשוק האיכרים המוביל של לנקסטר. דוכנים בנושא פטריוטי מיוחד לשבוע יום העצמאות!".to_string()), "rtl"),
                        ("fr", "Marché Fermier Hebdomadaire - Spécial Semaine de l'Indépendance".to_string(),
                         Some("Achetez des produits frais cultivés localement, des produits artisanaux et de l'artisanat fait main au principal marché fermier de Lancaster. Vendeurs à thème patriotique spécial pour la semaine de la fête de l'Indépendance!".to_string()), "ltr"),
                        ("de", "Wöchentlicher Bauernmarkt - Unabhängigkeitswoche Spezial".to_string(),
                         Some("Kaufen Sie frische, lokal angebaute Produkte, handwerkliche Waren und handgefertigte Kunsthandwerke auf Lancasters führendem Bauernmarkt. Spezielle patriotisch gestaltete Verkäufer für die Unabhängigkeitswoche!".to_string()), "ltr"),
                        ("zh", "每周农贸市场 - 独立周特别活动".to_string(),
                         Some("在兰开斯特主要的农贸市场购买新鲜的本地种植产品、手工制品和手工艺品。独立日周特别爱国主题摊贩！".to_string()), "ltr"),
                        ("fa", "بازار کشاورزان هفتگی - ویژه هفته استقلال".to_string(),
                         Some("محصولات تازه محلی، کالاهای صنایع دستی و صنایع دستی را در بازار کشاورزان پیشرو لنکستر خریداری کنید. فروشندگان با موضوع میهن‌پرستانه ویژه برای هفته روز استقلال!".to_string()), "rtl"),
                        ("ur", "ہفتہ وار کسان بازار - آزادی ہفتہ خصوصی".to_string(),
                         Some("لنکاسٹر کی سرکردہ کسان منڈی میں تازہ، مقامی طور پر اگائی گئی پیداوار، دستکاری کی اشیاء اور ہاتھ سے بنی دستکاری خریدیں۔ یوم آزادی کے ہفتے کے لیے خصوصی حب الوطنی کے موضوع والے فروخت کنندگان!".to_string()), "rtl"),
                    ]
                },
                "4th of July Fireworks Spectacular" => {
                    vec![
                        ("es", "Espectacular de Fuegos Artificiales del 4 de Julio".to_string(),
                         Some("¡Únase a nosotros para la exhibición de fuegos artificiales más grande de Lancaster! Camiones de comida, música en vivo y actividades familiares comienzan a las 6 PM. Los fuegos artificiales comienzan al anochecer. ¡Traigan mantas y sillas!".to_string()), "ltr"),
                        ("ar", "عرض الألعاب النارية المذهل في الرابع من يوليو".to_string(),
                         Some("انضم إلينا لأكبر عرض للألعاب النارية في لانكستر! شاحنات الطعام والموسيقى الحية والأنشطة العائلية تبدأ في الساعة 6 مساءً. تبدأ الألعاب النارية عند الغسق. أحضروا البطانيات والكراسي!".to_string()), "rtl"),
                        ("he", "מופע זיקוקים מרהיב ב-4 ביולי".to_string(),
                         Some("הצטרפו אלינו למופע הזיקוקים הגדול ביותר של לנקסטר! משאיות אוכל, מוזיקה חיה ופעילויות משפחתיות מתחילות ב-18:00. הזיקוקים מתחילים בשקיעה. הביאו שמיכות וכיסאות!".to_string()), "rtl"),
                        ("fr", "Spectacle de Feux d'Artifice du 4 Juillet".to_string(),
                         Some("Rejoignez-nous pour le plus grand spectacle de feux d'artifice de Lancaster! Food trucks, musique live et activités familiales commencent à 18h. Les feux d'artifice commencent au crépuscule. Apportez des couvertures et des chaises!".to_string()), "ltr"),
                        ("de", "4. Juli Feuerwerk Spektakel".to_string(),
                         Some("Begleiten Sie uns zu Lancasters größtem Feuerwerk! Food Trucks, Live-Musik und Familienaktivitäten beginnen um 18 Uhr. Das Feuerwerk beginnt in der Dämmerung. Bringen Sie Decken und Stühle mit!".to_string()), "ltr"),
                        ("zh", "7月4日烟花盛会".to_string(),
                         Some("加入我们观看兰开斯特最大的烟花表演！餐车、现场音乐和家庭活动从下午6点开始。烟花在黄昏时分开始。请带上毯子和椅子！".to_string()), "ltr"),
                        ("fa", "نمایش فوق‌العاده آتش‌بازی چهارم جولای".to_string(),
                         Some("به ما بپیوندید برای بزرگترین نمایش آتش‌بازی لنکستر! کامیون‌های غذا، موسیقی زنده و فعالیت‌های خانوادگی از ساعت 6 بعدازظهر شروع می‌شود. آتش‌بازی در غروب شروع می‌شود. پتو و صندلی بیاورید!".to_string()), "rtl"),
                        ("ur", "4 جولائی آتش بازی کا شاندار نظارہ".to_string(),
                         Some("لنکاسٹر کی سب سے بڑی آتش بازی کے نمائش میں ہمارے ساتھ شامل ہوں! فوڈ ٹرکس، لائیو موسیقی اور خاندانی سرگرمیاں شام 6 بجے شروع ہوتی ہیں۔ آتش بازی غروب آفتاب کے وقت شروع ہوتی ہے۔ کمبل اور کرسیاں لائیں!".to_string()), "rtl"),
                    ]
                },
                "Summer Pops: Music of John Williams" => {
                    vec![
                        ("es", "Pops de Verano: Música de John Williams".to_string(),
                         Some("¡Disfruta de la icónica música cinematográfica de John Williams bajo las estrellas! Con temas de Star Wars, Jurassic Park, Harry Potter y más. ¡Trae un picnic!".to_string()), "ltr"),
                        ("ar", "موسيقى الصيف الشعبية: موسيقى جون ويليامز".to_string(),
                         Some("استمتع بموسيقى الأفلام الأيقونية لجون ويليامز تحت النجوم! يضم موضوعات من حرب النجوم وحديقة جوراسيك وهاري بوتر والمزيد. أحضر نزهة!".to_string()), "rtl"),
                        ("he", "פופ קיץ: המוזיקה של ג'ון וויליאמס".to_string(),
                         Some("תיהנו ממוזיקת הסרטים האייקונית של ג'ון וויליאמס תחת הכוכבים! כולל נושאים ממלחמת הכוכבים, פארק היורה, הארי פוטר ועוד. הביאו פיקניק!".to_string()), "rtl"),
                        ("fr", "Pops d'Été: Musique de John Williams".to_string(),
                         Some("Profitez de la musique de film emblématique de John Williams sous les étoiles! Avec des thèmes de Star Wars, Jurassic Park, Harry Potter et plus. Apportez un pique-nique!".to_string()), "ltr"),
                        ("de", "Sommer-Pops: Musik von John Williams".to_string(),
                         Some("Genießen Sie die ikonische Filmmusik von John Williams unter den Sternen! Mit Themen aus Star Wars, Jurassic Park, Harry Potter und mehr. Bringen Sie ein Picknick mit!".to_string()), "ltr"),
                        ("zh", "夏季流行音乐会：约翰·威廉姆斯的音乐".to_string(),
                         Some("在星空下欣赏约翰·威廉姆斯的标志性电影音乐！包括《星球大战》、《侏罗纪公园》、《哈利·波特》等主题曲。带上野餐！".to_string()), "ltr"),
                        ("fa", "موسیقی پاپ تابستانی: موسیقی جان ویلیامز".to_string(),
                         Some("از موسیقی فیلم نمادین جان ویلیامز زیر ستارگان لذت ببرید! شامل تم‌هایی از جنگ ستارگان، پارک ژوراسیک، هری پاتر و بیشتر. پیک‌نیک بیاورید!".to_string()), "rtl"),
                        ("ur", "سمر پاپس: جان ولیمز کی موسیقی".to_string(),
                         Some("ستاروں کے نیچے جان ولیمز کی مشہور فلمی موسیقی سے لطف اندوز ہوں! اسٹار وارز، جراسک پارک، ہیری پوٹر اور مزید کے موضوعات شامل ہیں۔ پکنک لائیں!".to_string()), "rtl"),
                    ]
                },
                "Beat the Heat 5K Run/Walk" => {
                    vec![
                        ("es", "Carrera/Caminata 5K Vence el Calor".to_string(),
                         Some("5K matutino de verano a través de senderos sombreados del Parque del Condado. ¡Refrigerios después de la carrera y reunión social con helados incluidos!".to_string()), "ltr"),
                        ("ar", "سباق/مشي 5K تغلب على الحرارة".to_string(),
                         Some("سباق 5K صيفي في الصباح الباكر عبر المسارات المظللة في حديقة المقاطعة. يشمل المرطبات بعد السباق واجتماع اجتماعي مع الآيس كريم!".to_string()), "rtl"),
                        ("he", "ריצה/הליכה 5K נצח את החום".to_string(),
                         Some("5K קיצי בבוקר המוקדם דרך שבילים מוצלים בפארק המחוז. כולל כיבוד לאחר המרוץ ומפגש חברתי עם גלידה!".to_string()), "rtl"),
                        ("fr", "Course/Marche 5K Battez la Chaleur".to_string(),
                         Some("5K d'été tôt le matin à travers les sentiers ombragés du parc du comté. Rafraîchissements après la course et social de crème glacée inclus!".to_string()), "ltr"),
                        ("de", "Beat the Heat 5K Lauf/Spaziergang".to_string(),
                         Some("Frühmorgendlicher Sommer-5K durch schattige Wege des County Parks. Erfrischungen nach dem Rennen und Eis-Social inklusive!".to_string()), "ltr"),
                        ("zh", "战胜炎热5公里跑步/步行".to_string(),
                         Some("清晨夏季5公里穿越县公园阴凉小径。包括赛后茶点和冰淇淋社交活动！".to_string()), "ltr"),
                        ("fa", "دو/پیاده‌روی ۵ کیلومتری بر گرما غلبه کنید".to_string(),
                         Some("۵ کیلومتر تابستانی صبح زود از میان مسیرهای سایه‌دار پارک شهرستان. شامل نوشیدنی‌های پس از مسابقه و گردهمایی بستنی!".to_string()), "rtl"),
                        ("ur", "گرمی کو شکست دیں 5K دوڑ/واک".to_string(),
                         Some("کاؤنٹی پارک کے سایہ دار راستوں سے صبح سویرے موسم گرما کی 5K۔ دوڑ کے بعد ریفریشمنٹ اور آئس کریم سوشل شامل ہے!".to_string()), "rtl"),
                    ]
                },
                "First Friday Arts Walk" => {
                    vec![
                        ("es", "Paseo de Arte del Primer Viernes".to_string(),
                         Some("Explore la vibrante escena artística de Lancaster el primer viernes de cada mes. Las galerías permanecen abiertas hasta tarde con exposiciones especiales.".to_string()), "ltr"),
                        ("ar", "جولة الفنون في أول جمعة".to_string(),
                         Some("استكشف مشهد الفنون النابض بالحياة في لانكستر في أول جمعة من كل شهر. تبقى المعارض مفتوحة حتى وقت متأخر مع معارض خاصة.".to_string()), "rtl"),
                        ("he", "סיור אמנות ביום שישי הראשון".to_string(),
                         Some("חקרו את סצנת האמנות התוססת של לנקסטר ביום שישי הראשון של כל חודש. הגלריות נשארות פתוחות עד מאוחר עם תערוכות מיוחדות.".to_string()), "rtl"),
                        ("fr", "Promenade Artistique du Premier Vendredi".to_string(),
                         Some("Explorez la scène artistique vibrante de Lancaster le premier vendredi de chaque mois. Les galeries restent ouvertes tard avec des expositions spéciales.".to_string()), "ltr"),
                        ("de", "Kunstspaziergang am ersten Freitag".to_string(),
                         Some("Erkunden Sie Lancasters lebendige Kunstszene am ersten Freitag jeden Monats. Galerien bleiben mit Sonderausstellungen bis spät geöffnet.".to_string()), "ltr"),
                        ("zh", "第一个周五艺术步行".to_string(),
                         Some("在每月的第一个周五探索兰开斯特充满活力的艺术场景。画廊将举办特别展览，开放至晚。".to_string()), "ltr"),
                        ("fa", "قدم زدن هنری اولین جمعه".to_string(),
                         Some("صحنه هنری پرجنب و جوش لنکستر را در اولین جمعه هر ماه کشف کنید. گالری‌ها با نمایشگاه‌های ویژه تا دیروقت باز می‌مانند.".to_string()), "rtl"),
                        ("ur", "پہلے جمعہ کی فن واک".to_string(),
                         Some("ہر مہینے کے پہلے جمعہ کو لنکاسٹر کی زندگی سے بھرپور آرٹ سین کی تلاش کریں۔ گیلریاں خصوصی نمائشوں کے ساتھ دیر تک کھلی رہتی ہیں۔".to_string()), "rtl"),
                    ]
                },
                "Beethoven's 9th Symphony" => {
                    vec![
                        ("es", "Novena Sinfonía de Beethoven".to_string(),
                         Some("Experimente el poder y la majestuosidad de la sinfonía final de Beethoven, con la Orquesta y Coro Sinfónicos de Lancaster.".to_string()), "ltr"),
                        ("ar", "السيمفونية التاسعة لبيتهوفن".to_string(),
                         Some("اختبر قوة وعظمة سيمفونية بيتهوفن الأخيرة، مع أوركسترا وجوقة لانكستر السيمفونية.".to_string()), "rtl"),
                        ("he", "הסימפוניה התשיעית של בטהובן".to_string(),
                         Some("חוו את העוצמה וההוד של הסימפוניה האחרונה של בטהובן, עם התזמורת הסימפונית והמקהלה של לנקסטר.".to_string()), "rtl"),
                        ("fr", "9e Symphonie de Beethoven".to_string(),
                         Some("Vivez la puissance et la majesté de la dernière symphonie de Beethoven, avec l'Orchestre symphonique et le Chœur de Lancaster.".to_string()), "ltr"),
                        ("de", "Beethovens 9. Sinfonie".to_string(),
                         Some("Erleben Sie die Kraft und Majestät von Beethovens letzter Sinfonie mit dem Lancaster Symphony Orchestra und Chor.".to_string()), "ltr"),
                        ("zh", "贝多芬第九交响曲".to_string(),
                         Some("与兰开斯特交响乐团和合唱团一起体验贝多芬最后交响曲的力量和威严。".to_string()), "ltr"),
                        ("fa", "سمفونی نهم بتهوون".to_string(),
                         Some("قدرت و شکوه آخرین سمفونی بتهوون را با ارکستر و کُر سمفونی لنکستر تجربه کنید.".to_string()), "rtl"),
                        ("ur", "بیٹھوون کی نویں سمفنی".to_string(),
                         Some("لنکاسٹر سمفنی آرکیسٹرا اور کوائر کے ساتھ بیٹھوون کی آخری سمفنی کی طاقت اور شان و شوکت کا تجربہ کریں۔".to_string()), "rtl"),
                    ]
                },
                "Red Rose 5K Run/Walk" => {
                    vec![
                        ("es", "Carrera/Caminata Red Rose 5K".to_string(),
                         Some("Únase a cientos de corredores y caminantes para el 5K favorito de Lancaster a través del centro histórico y el parque del condado.".to_string()), "ltr"),
                        ("ar", "سباق/مشي الوردة الحمراء 5K".to_string(),
                         Some("انضم إلى مئات العدائين والمشاة في سباق 5K المفضل في لانكستر عبر وسط المدينة التاريخي وحديقة المقاطعة.".to_string()), "rtl"),
                        ("he", "ריצה/הליכה 5K של הוורד האדום".to_string(),
                         Some("הצטרפו למאות רצים והולכים ל-5K האהוב על לנקסטר דרך מרכז העיר ההיסטורי ופארק המחוז.".to_string()), "rtl"),
                        ("fr", "Course/Marche Red Rose 5K".to_string(),
                         Some("Rejoignez des centaines de coureurs et de marcheurs pour le 5K préféré de Lancaster à travers le centre-ville historique et le parc du comté.".to_string()), "ltr"),
                        ("de", "Red Rose 5K Lauf/Spaziergang".to_string(),
                         Some("Schließen Sie sich Hunderten von Läufern und Spaziergängern bei Lancasters Lieblings-5K durch die historische Innenstadt und den County Park an.".to_string()), "ltr"),
                        ("zh", "红玫瑰五公里跑步/步行".to_string(),
                         Some("加入数百名跑步者和步行者，参加兰开斯特最受欢迎的五公里赛事，穿过历史悠久的市中心和县公园。".to_string()), "ltr"),
                        ("fa", "دو/پیاده‌روی ۵ کیلومتری رُز سرخ".to_string(),
                         Some("به صدها دونده و پیاده‌رو در محبوب‌ترین مسابقه ۵ کیلومتری لنکستر از طریق مرکز تاریخی شهر و پارک منطقه بپیوندید.".to_string()), "rtl"),
                        ("ur", "ریڈ روز 5K دوڑ/واک".to_string(),
                         Some("لنکاسٹر کی پسندیدہ 5K میں سینکڑوں دوڑنے والوں اور چلنے والوں کے ساتھ شامل ہوں جو تاریخی شہر کے مرکز اور کاؤنٹی پارک سے گزرتی ہے۔".to_string()), "rtl"),
                    ]
                },
                _ => {
                    vec![
                        ("es", format!("ES: {}", title), description.map(|d| format!("ES: {}", d)), "ltr"),
                    ("ar", format!("AR: {}", title), description.map(|d| format!("AR: {}", d)), "rtl"),
                    ("he", format!("HE: {}", title), description.map(|d| format!("HE: {}", d)), "rtl"),
                    ("fr", format!("FR: {}", title), description.map(|d| format!("FR: {}", d)), "ltr"),
                    ("de", format!("DE: {}", title), description.map(|d| format!("DE: {}", d)), "ltr"),
                    ("zh", format!("ZH: {}", title), description.map(|d| format!("ZH: {}", d)), "ltr"),
                    ("fa", format!("FA: {}", title), description.map(|d| format!("FA: {}", d)), "rtl"),
                    ("ur", format!("UR: {}", title), description.map(|d| format!("UR: {}", d)), "rtl"),
                ]
                }
            };
            
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
                
                translations_created += 1;
            }
        }
    }
    
    let event_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM events").fetch_one(pool).await?;
    let published_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM events WHERE published = true").fetch_one(pool).await?;
    let unpublished_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM events WHERE published = false").fetch_one(pool).await?;
    let trans_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM event_translations").fetch_one(pool).await?;
    
    println!("  ✓ Created {} events ({} published, {} awaiting review)", 
        event_count.0, published_count.0, unpublished_count.0);
    println!("  ✓ Created {} event translations (avg {:.1} per published event)", 
        trans_count.0, trans_count.0 as f64 / published_count.0 as f64);
    
    Ok(())
}