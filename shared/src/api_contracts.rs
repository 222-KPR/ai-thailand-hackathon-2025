use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// Service-to-service communication types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServiceType {
    Vision,
    LLM,
    TTS,
    ASR,
    Weather,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CropType {
    Rice,
    Cassava,
    Durian,
    Mango,
    Rubber,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DiseaseSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Language {
    Thai,
    English,
}

// ============================================================================
// Vision Service API Contracts
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct VisionRequest {
    pub request_id: Uuid,
    #[validate(length(min = 1))]
    pub image_data: String,     // base64 encoded
    pub crop_type: CropType,
    pub metadata: ImageMetadata,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageMetadata {
    pub size_bytes: u64,
    pub width: u32,
    pub height: u32,
    pub format: String,         // "jpeg", "png", etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionResponse {
    pub request_id: Uuid,
    pub disease: String,
    pub confidence: f32,        // 0.0 - 1.0
    pub severity: DiseaseSeverity,
    pub affected_areas: Vec<BoundingBox>,
    pub processing_time_ms: u64,
    pub model_version: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub confidence: f32,
}

// ============================================================================
// LLM Service API Contracts
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LLMRequest {
    pub request_id: Uuid,
    #[validate(length(min = 1))]
    pub disease: String,
    pub crop_type: CropType,
    pub severity: DiseaseSeverity,
    pub location: Option<GeoLocation>,
    pub language: Language,
    pub context: ChatContext,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatContext {
    pub conversation_id: Uuid,
    pub previous_messages: Vec<ChatMessage>,
    pub user_profile: Option<UserProfile>,
    pub weather_context: Option<WeatherContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMessage {
    pub role: String,           // "user", "assistant", "system"
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub farm_size_rai: Option<f32>,
    pub location: GeoLocation,
    pub experience_years: Option<u32>,
    pub preferred_language: Language,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMResponse {
    pub request_id: Uuid,
    pub treatment_plan: TreatmentPlan,
    pub advice: String,
    pub prevention_tips: Vec<String>,
    pub estimated_cost: Option<CostEstimate>,
    pub confidence: f32,
    pub sources: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreatmentPlan {
    pub steps: Vec<TreatmentStep>,
    pub timeline_days: u32,
    pub urgency: TreatmentUrgency,
    pub organic_alternative: Option<OrganicTreatment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreatmentStep {
    pub step_number: u32,
    pub description: String,
    pub materials_needed: Vec<Material>,
    pub timing: String,         // "immediate", "after 3 days", etc.
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub name: String,
    pub quantity: String,
    pub estimated_cost_baht: Option<f32>,
    pub where_to_buy: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TreatmentUrgency {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganicTreatment {
    pub method: String,
    pub ingredients: Vec<String>,
    pub preparation: String,
    pub effectiveness: f32,     // 0.0 - 1.0 compared to chemical treatment
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostEstimate {
    pub min_baht: f32,
    pub max_baht: f32,
    pub breakdown: Vec<CostItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostItem {
    pub item: String,
    pub cost_baht: f32,
    pub is_optional: bool,
}

// ============================================================================
// External API Contracts
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TTSRequest {
    pub request_id: Uuid,
    #[validate(length(min = 1, max = 5000))]
    pub text: String,
    pub language: Language,
    pub voice_settings: VoiceSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSettings {
    pub voice_id: String,       // "th-female-1", "en-male-1", etc.
    pub speed: f32,             // 0.5 - 2.0
    pub pitch: f32,             // 0.5 - 2.0
    pub volume: f32,            // 0.0 - 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TTSResponse {
    pub request_id: Uuid,
    pub audio_url: String,      // URL to generated audio file
    pub duration_seconds: f32,
    pub format: String,         // "mp3", "wav", etc.
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ASRRequest {
    pub request_id: Uuid,
    #[validate(length(min = 1))]
    pub audio_data: String,     // base64 encoded audio
    pub language: Language,
    pub audio_format: String,   // "mp3", "wav", "webm", etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ASRResponse {
    pub request_id: Uuid,
    pub transcribed_text: String,
    pub confidence: f32,        // 0.0 - 1.0
    pub processing_time_ms: u64,
    pub detected_language: Language,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WeatherRequest {
    pub request_id: Uuid,
    pub location: GeoLocation,
    pub forecast_days: u8,      // 1-7 days
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherResponse {
    pub request_id: Uuid,
    pub current: CurrentWeather,
    pub forecast: Vec<WeatherForecast>,
    pub alerts: Vec<WeatherAlert>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentWeather {
    pub temperature_celsius: f32,
    pub humidity_percent: f32,
    pub rainfall_mm: f32,
    pub wind_speed_kmh: f32,
    pub conditions: String,     // "sunny", "rainy", "cloudy", etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherForecast {
    pub date: String,           // YYYY-MM-DD
    pub temperature_min: f32,
    pub temperature_max: f32,
    pub rainfall_probability: f32, // 0.0 - 1.0
    pub rainfall_mm: f32,
    pub conditions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherAlert {
    pub severity: String,       // "low", "medium", "high", "extreme"
    pub title: String,
    pub description: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherContext {
    pub current_conditions: String,
    pub rainfall_last_24h: f32,
    pub temperature_avg: f32,
    pub humidity_avg: f32,
    pub alerts: Vec<String>,
}

// ============================================================================
// Common Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub address: Option<String>,
    pub province: Option<String>,
    pub district: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    pub service_type: ServiceType,
    pub status: HealthStatus,
    pub response_time_ms: u64,
    pub last_check: DateTime<Utc>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetrics {
    pub service_type: ServiceType,
    pub requests_total: u64,
    pub requests_per_second: f32,
    pub average_response_time_ms: f32,
    pub error_rate: f32,        // 0.0 - 1.0
    pub uptime_percent: f32,    // 0.0 - 100.0
}

// ============================================================================
// Error Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceError {
    pub service_type: ServiceType,
    pub error_code: String,
    pub error_message: String,
    pub request_id: Option<Uuid>,
    pub timestamp: DateTime<Utc>,
    pub retry_after_seconds: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitError {
    pub service_type: ServiceType,
    pub limit: u32,
    pub remaining: u32,
    pub reset_time: DateTime<Utc>,
    pub retry_after_seconds: u32,
}

// ============================================================================
// Integration Response Wrappers
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ServiceError>,
    pub metadata: ResponseMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetadata {
    pub request_id: Uuid,
    pub processing_time_ms: u64,
    pub service_version: String,
    pub timestamp: DateTime<Utc>,
}

impl<T> ServiceResponse<T> {
    pub fn success(data: T, request_id: Uuid, processing_time_ms: u64) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            metadata: ResponseMetadata {
                request_id,
                processing_time_ms,
                service_version: env!("CARGO_PKG_VERSION").to_string(),
                timestamp: Utc::now(),
            },
        }
    }

    pub fn error(error: ServiceError, request_id: Uuid, processing_time_ms: u64) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            metadata: ResponseMetadata {
                request_id,
                processing_time_ms,
                service_version: env!("CARGO_PKG_VERSION").to_string(),
                timestamp: Utc::now(),
            },
        }
    }
}