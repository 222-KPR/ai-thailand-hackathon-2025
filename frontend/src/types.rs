use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Language support
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    Thai,
    English,
}

// Crop types supported by the backend
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CropType {
    Rice,
    Cassava,
    Durian,
    Mango,
    Rubber,
}

impl std::fmt::Display for CropType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CropType::Rice => write!(f, "rice"),
            CropType::Cassava => write!(f, "cassava"),
            CropType::Durian => write!(f, "durian"),
            CropType::Mango => write!(f, "mango"),
            CropType::Rubber => write!(f, "rubber"),
        }
    }
}

// Disease severity levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiseaseSeverity {
    Low,
    Moderate,
    High,
    Critical,
}

// Vision Service Response (matches backend/vision-service)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionResponse {
    pub disease: String,
    pub confidence: f64,
    pub severity: String,
    pub treatment_steps: Vec<TreatmentStep>,
    pub model_version: String,
    pub analysis_timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreatmentStep {
    pub step: i32,
    pub description: String,
    pub timing: String,
}

// Queue Worker Responses (matches backend/queue-worker)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobResponse {
    pub job_id: String,
    pub status: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobStatus {
    pub job_id: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub result: Option<PestDetectionResult>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PestDetectionResult {
    pub detected_pests: Vec<String>,
    pub message: String,
    pub confidence_scores: std::collections::HashMap<String, f64>,
    pub total_detections: i32,
    pub treatment_advice: Option<String>,
}

// Request structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionRequest {
    pub image_data: String,
    pub crop_type: String,
    pub confidence_threshold: f64,
}

// Chat message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: Uuid,
    pub content: String,
    pub is_user: bool,
    pub timestamp: DateTime<Utc>,
    pub language: Language,
}

// Health response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub timestamp: DateTime<Utc>,
}

// API Error types
#[derive(Debug, Clone)]
pub enum ApiError {
    NetworkError(String),
    ParseError(String),
    ServiceUnavailable,
    InvalidRequest(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::NetworkError(msg) => write!(f, "Network error: {msg}"),
            ApiError::ParseError(msg) => write!(f, "Parse error: {msg}"),
            ApiError::ServiceUnavailable => write!(f, "Service unavailable"),
            ApiError::InvalidRequest(msg) => write!(f, "Invalid request: {msg}"),
        }
    }
}

impl std::error::Error for ApiError {}
