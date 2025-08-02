use shared::{
    VisionResponse, LLMResponse, BoundingBox,
    CropType, Language, DiseaseSeverity,
    TreatmentPlan, TreatmentStep, TreatmentUrgency,
    Material, CostEstimate, CostItem
};
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Clone)]
pub enum ApiError {
    NetworkError(String),
    ParseError(String),
    ServiceUnavailable,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::NetworkError(msg) => write!(f, "Network error: {msg}"),
            ApiError::ParseError(msg) => write!(f, "Parse error: {msg}"),
            ApiError::ServiceUnavailable => write!(f, "Service unavailable"),
        }
    }
}

impl std::error::Error for ApiError {}

pub struct ApiService;

impl ApiService {
    pub async fn analyze_image(
        _image_data: String,
        crop_type: CropType,
        _language: Language,
    ) -> Result<VisionResponse, ApiError> {
        // Mock response for development
        let response = VisionResponse {
            request_id: Uuid::new_v4(),
            disease: format!("Mock disease for {crop_type:?}"),
            confidence: 0.85,
            severity: DiseaseSeverity::Medium,
            affected_areas: vec![
                BoundingBox {
                    x: 0.2,
                    y: 0.3,
                    width: 0.4,
                    height: 0.3,
                    confidence: 0.85,
                }
            ],
            processing_time_ms: 1500,
            model_version: "v1.0.0".to_string(),
            timestamp: Utc::now(),
        };
        
        Ok(response)
    }

    pub async fn get_treatment_advice(
        vision_result: &VisionResponse,
        crop_type: CropType,
        language: Language,
    ) -> Result<LLMResponse, ApiError> {
        // Mock response for development
        let treatment_text = match language {
            Language::Thai => format!(
                "การรักษาโรค {} ในพืช {:?}:\n\n1. ใช้สารเคมีป้องกันโรค\n2. ปรับปรุงการระบายน้ำ\n3. ตัดส่วนที่เป็นโรคออก\n4. เฝ้าระวังและติดตามอาการ",
                vision_result.disease, crop_type
            ),
            Language::English => format!(
                "Treatment for {} in {:?}:\n\n1. Apply appropriate fungicide\n2. Improve drainage systems\n3. Remove affected plant parts\n4. Monitor progress daily",
                vision_result.disease, crop_type
            ),
        };

        let response = LLMResponse {
            request_id: Uuid::new_v4(),
            treatment_plan: TreatmentPlan {
                steps: vec![
                    TreatmentStep {
                        step_number: 1,
                        description: "Apply fungicide treatment".to_string(),
                        materials_needed: vec![
                            Material {
                                name: "Copper fungicide".to_string(),
                                quantity: "100ml".to_string(),
                                estimated_cost_baht: Some(150.0),
                                where_to_buy: vec!["Local agricultural store".to_string()],
                            }
                        ],
                        timing: "Immediate".to_string(),
                        warnings: vec!["Wear protective gear".to_string()],
                    }
                ],
                timeline_days: 14,
                urgency: TreatmentUrgency::Medium,
                organic_alternative: None,
            },
            advice: treatment_text,
            prevention_tips: vec![
                "Improve drainage".to_string(),
                "Monitor regularly".to_string(),
            ],
            estimated_cost: Some(CostEstimate {
                min_baht: 100.0,
                max_baht: 200.0,
                breakdown: vec![
                    CostItem {
                        item: "Fungicide".to_string(),
                        cost_baht: 150.0,
                        is_optional: false,
                    }
                ],
            }),
            confidence: 0.80,
            sources: vec!["Agricultural Research Institute".to_string()],
            timestamp: Utc::now(),
        };
        
        Ok(response)
    }

    pub async fn send_chat_message(
        _message: String,
        _conversation_id: Uuid,
        _language: Language,
    ) -> Result<String, ApiError> {
        // Mock chat response
        Ok("This is a mock response from the AI assistant. In a real implementation, this would connect to the LLM service.".to_string())
    }

    pub async fn check_health() -> Result<(), ApiError> {
        // Mock health check - always returns OK for development
        Ok(())
    }
}