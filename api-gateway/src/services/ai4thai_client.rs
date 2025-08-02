use crate::config::external_apis::AI4ThaiConfig;
use crate::{AppError, AppResult};
use reqwest::{multipart, Client};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tracing::{error, info, instrument, warn};

/// AI4Thai API client for speech services
#[derive(Clone)]
pub struct AI4ThaiClient {
    client: Client,
    config: AI4ThaiConfig,
}

impl AI4ThaiClient {
    pub fn new(config: AI4ThaiConfig) -> AppResult<Self> {
        let client = Client::builder()
            .timeout(config.timeout())
            .user_agent("AI4Thai-Crop-Guardian/1.0")
            .build()
            .map_err(|e| AppError::ExternalApi(format!("Failed to create HTTP client: {e}")))?;

        Ok(Self { client, config })
    }

    /// Convert text to speech using AI4Thai TTS API
    #[instrument(skip(self, text), fields(text_length = text.len()))]
    pub async fn text_to_speech(
        &self,
        text: &str,
        language: &str,
        voice: Option<&str>,
        options: Option<TTSOptions>,
    ) -> AppResult<TTSResponse> {
        let start_time = Instant::now();

        // Validate input
        if text.is_empty() {
            return Err(AppError::Validation("Text cannot be empty".to_string()));
        }

        if text.len() > self.config.tts.max_text_length {
            return Err(AppError::Validation(format!(
                "Text length {} exceeds maximum of {}",
                text.len(),
                self.config.tts.max_text_length
            )));
        }

        let voice = voice.unwrap_or_else(|| self.config.tts.get_voice_for_language(language));

        if !self.config.tts.is_voice_supported(voice) {
            return Err(AppError::Validation(format!(
                "Unsupported voice: {voice}"
            )));
        }

        // Prepare request
        let request_body = TTSRequest {
            text: text.to_string(),
            language: language.to_string(),
            voice: voice.to_string(),
            options: options.unwrap_or_default(),
        };

        info!(
            "Sending TTS request to AI4Thai API: {} characters, language: {}, voice: {}",
            text.len(),
            language,
            voice
        );

        // Make API request
        let response = self
            .client
            .post(self.config.tts_endpoint())
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| AppError::ExternalApi(format!("TTS API request failed: {e}")))?;

        let processing_time = start_time.elapsed();

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("TTS API error: {} - {}", status, error_text);
            return Err(AppError::ExternalApi(format!(
                "TTS API returned error {status}: {error_text}"
            )));
        }

        let tts_response: TTSApiResponse = response
            .json()
            .await
            .map_err(|e| AppError::ExternalApi(format!("Failed to parse TTS response: {e}")))?;

        info!(
            "TTS request completed successfully in {:.2}ms",
            processing_time.as_millis()
        );

        Ok(TTSResponse {
            audio_url: tts_response.audio_url,
            audio_data: tts_response.audio_data,
            duration_seconds: tts_response.duration_seconds,
            format: tts_response.format,
            size_bytes: tts_response.size_bytes,
            processing_time_ms: processing_time.as_millis() as u64,
        })
    }

    /// Convert speech to text using AI4Thai ASR API
    #[instrument(skip(self, audio_data), fields(audio_size = audio_data.len()))]
    pub async fn speech_to_text(
        &self,
        audio_data: &[u8],
        language: Option<&str>,
        options: Option<ASROptions>,
    ) -> AppResult<ASRResponse> {
        let start_time = Instant::now();

        // Validate input
        if audio_data.is_empty() {
            return Err(AppError::Validation(
                "Audio data cannot be empty".to_string(),
            ));
        }

        if audio_data.len() > self.config.asr.max_file_size_bytes() as usize {
            return Err(AppError::Validation(format!(
                "Audio file size {} exceeds maximum of {} bytes",
                audio_data.len(),
                self.config.asr.max_file_size_bytes()
            )));
        }

        let options = options.unwrap_or_default();

        // Validate language if specified
        if let Some(lang) = language {
            if !self.config.asr.is_supported_language(lang) {
                return Err(AppError::Validation(format!(
                    "Unsupported language: {lang}"
                )));
            }
        }

        info!(
            "Sending ASR request to AI4Thai API: {} bytes, language: {:?}",
            audio_data.len(),
            language
        );

        // Create multipart form
        let mut form = multipart::Form::new().part(
            "audio",
            multipart::Part::bytes(audio_data.to_vec())
                .file_name("audio.wav")
                .mime_str("audio/wav")?,
        );

        if let Some(lang) = language {
            form = form.text("language", lang.to_string());
        }

        if options.enable_punctuation {
            form = form.text("enable_punctuation", "true");
        }

        if options.enable_timestamps {
            form = form.text("enable_timestamps", "true");
        }

        if self.config.asr.language_detection {
            form = form.text("language_detection", "true");
        }

        // Make API request
        let response = self
            .client
            .post(self.config.asr_endpoint())
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .multipart(form)
            .send()
            .await
            .map_err(|e| AppError::ExternalApi(format!("ASR API request failed: {e}")))?;

        let processing_time = start_time.elapsed();

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("ASR API error: {} - {}", status, error_text);
            return Err(AppError::ExternalApi(format!(
                "ASR API returned error {status}: {error_text}"
            )));
        }

        let asr_response: ASRApiResponse = response
            .json()
            .await
            .map_err(|e| AppError::ExternalApi(format!("Failed to parse ASR response: {e}")))?;

        info!(
            "ASR request completed successfully in {:.2}ms, transcribed {} characters",
            processing_time.as_millis(),
            asr_response.transcription.len()
        );

        Ok(ASRResponse {
            transcription: asr_response.transcription,
            confidence: asr_response.confidence,
            detected_language: asr_response.detected_language,
            timestamps: asr_response.timestamps,
            processing_time_ms: processing_time.as_millis() as u64,
        })
    }

    /// Check if AI4Thai API is healthy
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> bool {
        let health_url = format!("{}/health", self.config.api_url);

        match self
            .client
            .get(&health_url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .timeout(Duration::from_secs(5))
            .send()
            .await
        {
            Ok(response) => {
                let is_healthy = response.status().is_success();
                if is_healthy {
                    info!("AI4Thai API health check passed");
                } else {
                    warn!("AI4Thai API health check failed: {}", response.status());
                }
                is_healthy
            }
            Err(e) => {
                warn!("AI4Thai API health check failed: {}", e);
                false
            }
        }
    }

    /// Get API usage statistics
    #[instrument(skip(self))]
    pub async fn get_usage_stats(&self) -> AppResult<UsageStats> {
        let stats_url = format!("{}/usage/stats", self.config.api_url);

        let response = self
            .client
            .get(&stats_url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()
            .await
            .map_err(|e| AppError::ExternalApi(format!("Failed to get usage stats: {e}")))?;

        if !response.status().is_success() {
            return Err(AppError::ExternalApi(format!(
                "Usage stats API returned error: {}",
                response.status()
            )));
        }

        let stats: UsageStats = response
            .json()
            .await
            .map_err(|e| AppError::ExternalApi(format!("Failed to parse usage stats: {e}")))?;

        Ok(stats)
    }
}

// Request/Response types for AI4Thai API

#[derive(Debug, Serialize)]
struct TTSRequest {
    text: String,
    language: String,
    voice: String,
    options: TTSOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TTSOptions {
    pub speed: f32,
    pub pitch: f32,
    pub volume: f32,
    pub format: String,
    pub sample_rate: u32,
}

impl Default for TTSOptions {
    fn default() -> Self {
        Self {
            speed: 1.0,
            pitch: 1.0,
            volume: 1.0,
            format: "mp3".to_string(),
            sample_rate: 22050,
        }
    }
}

#[derive(Debug, Deserialize)]
struct TTSApiResponse {
    audio_url: Option<String>,
    audio_data: Option<String>, // base64 encoded
    duration_seconds: f32,
    format: String,
    size_bytes: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TTSResponse {
    pub audio_url: Option<String>,
    pub audio_data: Option<String>,
    pub duration_seconds: f32,
    pub format: String,
    pub size_bytes: u64,
    pub processing_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ASROptions {
    pub enable_punctuation: bool,
    pub enable_timestamps: bool,
    pub model: Option<String>,
}

impl Default for ASROptions {
    fn default() -> Self {
        Self {
            enable_punctuation: true,
            enable_timestamps: true,
            model: None,
        }
    }
}

#[derive(Debug, Deserialize)]
struct ASRApiResponse {
    transcription: String,
    confidence: f32,
    detected_language: Option<String>,
    timestamps: Option<Vec<TimestampSegment>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ASRResponse {
    pub transcription: String,
    pub confidence: f32,
    pub detected_language: Option<String>,
    pub timestamps: Option<Vec<TimestampSegment>>,
    pub processing_time_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimestampSegment {
    pub text: String,
    pub start_time: f32,
    pub end_time: f32,
    pub confidence: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsageStats {
    pub tts_requests_today: u32,
    pub asr_requests_today: u32,
    pub tts_characters_today: u32,
    pub asr_minutes_today: f32,
    pub monthly_quota: UsageQuota,
    pub current_usage: UsageQuota,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsageQuota {
    pub tts_characters: u32,
    pub asr_minutes: f32,
    pub requests: u32,
}

// Error handling for multipart form creation
impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::ExternalApi(format!("HTTP request error: {err}"))
    }
}
