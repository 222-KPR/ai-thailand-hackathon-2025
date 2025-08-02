use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

/// Configuration for external API services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalApisConfig {
    pub ai4thai: AI4ThaiConfig,
    pub weather: WeatherConfig,
    pub default_timeout_seconds: u64,
    pub default_retry_attempts: u32,
}

impl Default for ExternalApisConfig {
    fn default() -> Self {
        Self {
            ai4thai: AI4ThaiConfig::default(),
            weather: WeatherConfig::default(),
            default_timeout_seconds: 30,
            default_retry_attempts: 3,
        }
    }
}

impl ExternalApisConfig {
    pub fn default_timeout(&self) -> Duration {
        Duration::from_secs(self.default_timeout_seconds)
    }
}

/// AI4Thai API configuration for speech services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AI4ThaiConfig {
    pub api_key: String,
    pub api_url: String,
    pub tts: AI4ThaiTTSConfig,
    pub asr: AI4ThaiASRConfig,
    pub timeout_seconds: u64,
    pub rate_limit_per_minute: u32,
    pub cache_duration_hours: u64,
}

impl Default for AI4ThaiConfig {
    fn default() -> Self {
        Self {
            api_key: env::var("AI4THAI_API_KEY").unwrap_or_default(),
            api_url: env::var("AI4THAI_API_URL")
                .unwrap_or_else(|_| "https://api.ai4thai.org/v1".to_string()),
            tts: AI4ThaiTTSConfig::default(),
            asr: AI4ThaiASRConfig::default(),
            timeout_seconds: 30,
            rate_limit_per_minute: 100,
            cache_duration_hours: 24,
        }
    }
}

impl AI4ThaiConfig {
    pub fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout_seconds)
    }
    
    pub fn cache_duration(&self) -> Duration {
        Duration::from_secs(self.cache_duration_hours * 3600)
    }
    
    pub fn is_configured(&self) -> bool {
        !self.api_key.is_empty() && !self.api_url.is_empty()
    }
    
    pub fn tts_endpoint(&self) -> String {
        format!("{}/speech/text-to-speech", self.api_url)
    }
    
    pub fn asr_endpoint(&self) -> String {
        format!("{}/speech/speech-to-text", self.api_url)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AI4ThaiTTSConfig {
    pub default_voice_th: String,
    pub default_voice_en: String,
    pub supported_voices: Vec<String>,
    pub max_text_length: usize,
    pub audio_format: String,
    pub sample_rate: u32,
    pub speed_range: (f32, f32),
    pub pitch_range: (f32, f32),
}

impl Default for AI4ThaiTTSConfig {
    fn default() -> Self {
        Self {
            default_voice_th: "th-TH-Premwadee".to_string(),
            default_voice_en: "en-US-Jenny".to_string(),
            supported_voices: vec![
                "th-TH-Premwadee".to_string(),
                "th-TH-Niran".to_string(),
                "en-US-Jenny".to_string(),
                "en-US-Guy".to_string(),
            ],
            max_text_length: 5000,
            audio_format: "mp3".to_string(),
            sample_rate: 22050,
            speed_range: (0.5, 2.0),
            pitch_range: (0.5, 2.0),
        }
    }
}

impl AI4ThaiTTSConfig {
    pub fn get_voice_for_language(&self, language: &str) -> &str {
        match language.to_lowercase().as_str() {
            "th" | "thai" => &self.default_voice_th,
            "en" | "english" => &self.default_voice_en,
            _ => &self.default_voice_en,
        }
    }
    
    pub fn is_voice_supported(&self, voice: &str) -> bool {
        self.supported_voices.contains(&voice.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AI4ThaiASRConfig {
    pub supported_languages: Vec<String>,
    pub max_audio_duration_seconds: u64,
    pub max_file_size_mb: u64,
    pub supported_formats: Vec<String>,
    pub sample_rates: Vec<u32>,
    pub enable_punctuation: bool,
    pub enable_timestamps: bool,
    pub language_detection: bool,
}

impl Default for AI4ThaiASRConfig {
    fn default() -> Self {
        Self {
            supported_languages: vec![
                "th".to_string(),
                "th-TH".to_string(),
                "en".to_string(),
                "en-US".to_string(),
            ],
            max_audio_duration_seconds: 300, // 5 minutes
            max_file_size_mb: 25,
            supported_formats: vec![
                "mp3".to_string(),
                "wav".to_string(),
                "webm".to_string(),
                "m4a".to_string(),
                "ogg".to_string(),
            ],
            sample_rates: vec![16000, 22050, 44100, 48000],
            enable_punctuation: true,
            enable_timestamps: true,
            language_detection: true,
        }
    }
}

impl AI4ThaiASRConfig {
    pub fn max_audio_duration(&self) -> Duration {
        Duration::from_secs(self.max_audio_duration_seconds)
    }
    
    pub fn max_file_size_bytes(&self) -> u64 {
        self.max_file_size_mb * 1024 * 1024
    }
    
    pub fn is_supported_language(&self, language: &str) -> bool {
        self.supported_languages.contains(&language.to_lowercase())
    }
    
    pub fn is_supported_format(&self, format: &str) -> bool {
        self.supported_formats.contains(&format.to_lowercase())
    }
    
    pub fn is_supported_sample_rate(&self, sample_rate: u32) -> bool {
        self.sample_rates.contains(&sample_rate)
    }
    
    pub fn get_optimal_sample_rate(&self) -> u32 {
        22050 // Optimal for speech recognition
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherConfig {
    pub provider: WeatherProvider,
    pub api_key: String,
    pub api_url: String,
    pub default_location: String,
    pub cache_duration_minutes: u64,
    pub forecast_days: u8,
    pub rate_limit_per_hour: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeatherProvider {
    OpenWeatherMap,
    WeatherAPI,
    AccuWeather,
}

impl Default for WeatherConfig {
    fn default() -> Self {
        Self {
            provider: WeatherProvider::OpenWeatherMap,
            api_key: env::var("WEATHER_API_KEY").unwrap_or_default(),
            api_url: env::var("WEATHER_API_URL")
                .unwrap_or_else(|_| "https://api.openweathermap.org/data/2.5".to_string()),
            default_location: "Bangkok,TH".to_string(),
            cache_duration_minutes: 30,
            forecast_days: 7,
            rate_limit_per_hour: 1000,
        }
    }
}

impl WeatherConfig {
    pub fn cache_duration(&self) -> Duration {
        Duration::from_secs(self.cache_duration_minutes * 60)
    }
    
    pub fn is_configured(&self) -> bool {
        !self.api_key.is_empty() && !self.api_url.is_empty()
    }
}

/// Rate limiting configuration for external APIs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_size: u32,
    pub window_size_seconds: u64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            burst_size: 10,
            window_size_seconds: 60,
        }
    }
}

impl RateLimitConfig {
    pub fn window_size(&self) -> Duration {
        Duration::from_secs(self.window_size_seconds)
    }
}