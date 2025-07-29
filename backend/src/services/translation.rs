use anyhow::{anyhow, Result};
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::models::{LanguageDetectionResult, TranslationResult};

lazy_static::lazy_static! {
    static ref RTL_LANGUAGES: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("ar"); // Arabic
        set.insert("he"); // Hebrew
        set.insert("fa"); // Persian/Farsi
        set.insert("ur"); // Urdu
        set.insert("yi"); // Yiddish
        set.insert("ps"); // Pashto
        set.insert("sd"); // Sindhi
        set
    };
}

pub fn detect_text_direction(text: &str, language_code: &str) -> String {
    // First check language code
    if RTL_LANGUAGES.contains(language_code) {
        return "rtl".to_string();
    }

    // Then check for RTL characters in text
    let rtl_regex = regex::Regex::new(r"[\u0591-\u07FF\u200F\u202B\u202E\uFB1D-\uFDFD\uFE70-\uFEFC]").unwrap();
    if rtl_regex.is_match(text) {
        return "rtl".to_string();
    }

    "ltr".to_string()
}

pub fn is_rtl_language(language_code: &str) -> bool {
    RTL_LANGUAGES.contains(language_code)
}

#[derive(Debug, Serialize, Deserialize)]
struct GoogleTranslateRequest {
    q: Vec<String>,
    target: String,
    source: Option<String>,
    format: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GoogleTranslateResponse {
    data: TranslateTextResponseList,
}

#[derive(Debug, Serialize, Deserialize)]
struct TranslateTextResponseList {
    translations: Vec<TranslateTextResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TranslateTextResponse {
    #[serde(rename = "translatedText")]
    translated_text: String,
    #[serde(rename = "detectedSourceLanguage")]
    detected_source_language: Option<String>,
}

pub struct TranslationService {
    api_key: String,
    client: reqwest::Client,
}

impl TranslationService {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    pub async fn translate_text(
        &self,
        text: &str,
        target_lang: &str,
        source_lang: Option<&str>,
    ) -> Result<TranslationResult> {
        let url = format!(
            "https://translation.googleapis.com/language/translate/v2?key={}",
            self.api_key
        );

        let request_body = GoogleTranslateRequest {
            q: vec![text.to_string()],
            target: target_lang.to_string(),
            source: source_lang.map(|s| s.to_string()),
            format: "text".to_string(),
        };

        let response = self
            .client
            .post(&url)
            .json(&request_body)
            .send()
            .await?
            .json::<GoogleTranslateResponse>()
            .await?;

        let translation = response
            .data
            .translations
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("No translation returned"))?;

        let source_language = translation
            .detected_source_language
            .or_else(|| source_lang.map(|s| s.to_string()))
            .unwrap_or_else(|| "unknown".to_string());

        let text_direction = detect_text_direction(&translation.translated_text, target_lang);

        Ok(TranslationResult {
            translated_text: translation.translated_text,
            source_language,
            target_language: target_lang.to_string(),
            text_direction,
            confidence: 1.0,
        })
    }

    pub async fn detect_language(
        &self,
        text: &str,
    ) -> Result<LanguageDetectionResult> {
        let url = format!(
            "https://translation.googleapis.com/language/translate/v2/detect?key={}",
            self.api_key
        );

        #[derive(Serialize)]
        struct DetectRequest {
            q: Vec<String>,
        }

        #[derive(Deserialize)]
        struct DetectResponse {
            data: DetectionsData,
        }

        #[derive(Deserialize)]
        struct DetectionsData {
            detections: Vec<Vec<Detection>>,
        }

        #[derive(Deserialize)]
        struct Detection {
            language: String,
            confidence: f32,
        }

        let request_body = DetectRequest {
            q: vec![text.to_string()],
        };

        let response = self
            .client
            .post(&url)
            .json(&request_body)
            .send()
            .await?
            .json::<DetectResponse>()
            .await?;

        let detection = response
            .data
            .detections
            .into_iter()
            .next()
            .and_then(|d| d.into_iter().next())
            .ok_or_else(|| anyhow!("No language detected"))?;

        let is_rtl = is_rtl_language(&detection.language);
        let text_direction = if is_rtl { "rtl" } else { "ltr" }.to_string();

        Ok(LanguageDetectionResult {
            language: detection.language,
            confidence: detection.confidence,
            is_rtl,
            text_direction,
        })
    }

    pub async fn translate_batch(
        &self,
        texts: Vec<&str>,
        target_lang: &str,
        source_lang: Option<&str>,
    ) -> Result<Vec<TranslationResult>> {
        let url = format!(
            "https://translation.googleapis.com/language/translate/v2?key={}",
            self.api_key
        );

        let request_body = GoogleTranslateRequest {
            q: texts.iter().map(|t| t.to_string()).collect(),
            target: target_lang.to_string(),
            source: source_lang.map(|s| s.to_string()),
            format: "text".to_string(),
        };

        let response = self
            .client
            .post(&url)
            .json(&request_body)
            .send()
            .await?
            .json::<GoogleTranslateResponse>()
            .await?;

        let results = response
            .data
            .translations
            .into_iter()
            .map(|translation| {
                let source_language = translation
                    .detected_source_language
                    .or_else(|| source_lang.map(|s| s.to_string()))
                    .unwrap_or_else(|| "unknown".to_string());

                let text_direction = detect_text_direction(&translation.translated_text, target_lang);

                TranslationResult {
                    translated_text: translation.translated_text,
                    source_language,
                    target_language: target_lang.to_string(),
                    text_direction,
                    confidence: 1.0,
                }
            })
            .collect();

        Ok(results)
    }
}