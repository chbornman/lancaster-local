use sqlx::PgPool;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

use crate::{
    models::{Post, Event, Language},
    services::translation::TranslationService,
};

pub async fn translate_post(
    pool: Arc<PgPool>,
    post_id: i32,
    api_key: String,
) {
    tracing::info!("Starting translation for post {}", post_id);
    
    let translation_service = TranslationService::new(api_key);
    
    // Get the post
    let post = match sqlx::query_as::<_, Post>(
        "SELECT * FROM posts WHERE id = $1"
    )
    .bind(post_id)
    .fetch_one(pool.as_ref())
    .await {
        Ok(p) => p,
        Err(e) => {
            tracing::error!("Failed to fetch post {}: {:?}", post_id, e);
            return;
        }
    };
    
    // Get enabled languages
    let languages = match sqlx::query_as::<_, Language>(
        "SELECT * FROM supported_languages WHERE enabled = true"
    )
    .fetch_all(pool.as_ref())
    .await {
        Ok(langs) => langs,
        Err(e) => {
            tracing::error!("Failed to fetch languages: {:?}", e);
            return;
        }
    };
    
    // Translate to each language (except original)
    for lang in languages {
        if lang.code == post.original_language {
            continue;
        }
        
        // Small delay to avoid rate limiting
        sleep(Duration::from_millis(100)).await;
        
        // Translate title
        let title_result = translation_service
            .translate_text(&post.title, &lang.code, Some(&post.original_language))
            .await;
            
        let translated_title = match title_result {
            Ok(r) => r.translated_text,
            Err(e) => {
                tracing::error!("Failed to translate title to {}: {:?}", lang.code, e);
                continue;
            }
        };
        
        // Translate content if present
        let translated_content = if let Some(content) = &post.content {
            match translation_service
                .translate_text(content, &lang.code, Some(&post.original_language))
                .await {
                Ok(r) => Some(r.translated_text),
                Err(e) => {
                    tracing::error!("Failed to translate content to {}: {:?}", lang.code, e);
                    None
                }
            }
        } else {
            None
        };
        
        // Store translation
        let _ = sqlx::query(
            r#"
            INSERT INTO post_translations (
                post_id, language_code, title, content, text_direction
            )
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (post_id, language_code) 
            DO UPDATE SET 
                title = EXCLUDED.title,
                content = EXCLUDED.content,
                text_direction = EXCLUDED.text_direction,
                translated_at = NOW()
            "#
        )
        .bind(post_id)
        .bind(&lang.code)
        .bind(&translated_title)
        .bind(&translated_content)
        .bind(&lang.text_direction)
        .execute(pool.as_ref())
        .await;
        
        tracing::info!("Translated post {} to {}", post_id, lang.code);
    }
    
    tracing::info!("Completed translation for post {}", post_id);
}

pub async fn translate_event(
    pool: Arc<PgPool>,
    event_id: i32,
    api_key: String,
) {
    tracing::info!("Starting translation for event {}", event_id);
    
    let translation_service = TranslationService::new(api_key);
    
    // Get the event
    let event = match sqlx::query_as::<_, Event>(
        "SELECT * FROM events WHERE id = $1"
    )
    .bind(event_id)
    .fetch_one(pool.as_ref())
    .await {
        Ok(e) => e,
        Err(e) => {
            tracing::error!("Failed to fetch event {}: {:?}", event_id, e);
            return;
        }
    };
    
    // Get enabled languages
    let languages = match sqlx::query_as::<_, Language>(
        "SELECT * FROM supported_languages WHERE enabled = true"
    )
    .fetch_all(pool.as_ref())
    .await {
        Ok(langs) => langs,
        Err(e) => {
            tracing::error!("Failed to fetch languages: {:?}", e);
            return;
        }
    };
    
    // Translate to each language (except original)
    for lang in languages {
        if lang.code == event.original_language {
            continue;
        }
        
        // Small delay to avoid rate limiting
        sleep(Duration::from_millis(100)).await;
        
        // Translate title
        let title_result = translation_service
            .translate_text(&event.title, &lang.code, Some(&event.original_language))
            .await;
            
        let translated_title = match title_result {
            Ok(r) => r.translated_text,
            Err(e) => {
                tracing::error!("Failed to translate title to {}: {:?}", lang.code, e);
                continue;
            }
        };
        
        // Translate description if present
        let translated_description = if let Some(description) = &event.description {
            match translation_service
                .translate_text(description, &lang.code, Some(&event.original_language))
                .await {
                Ok(r) => Some(r.translated_text),
                Err(e) => {
                    tracing::error!("Failed to translate description to {}: {:?}", lang.code, e);
                    None
                }
            }
        } else {
            None
        };
        
        // Store translation
        let _ = sqlx::query(
            r#"
            INSERT INTO event_translations (
                event_id, language_code, title, description, text_direction
            )
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (event_id, language_code) 
            DO UPDATE SET 
                title = EXCLUDED.title,
                description = EXCLUDED.description,
                text_direction = EXCLUDED.text_direction,
                translated_at = NOW()
            "#
        )
        .bind(event_id)
        .bind(&lang.code)
        .bind(&translated_title)
        .bind(&translated_description)
        .bind(&lang.text_direction)
        .execute(pool.as_ref())
        .await;
        
        tracing::info!("Translated event {} to {}", event_id, lang.code);
    }
    
    tracing::info!("Completed translation for event {}", event_id);
}