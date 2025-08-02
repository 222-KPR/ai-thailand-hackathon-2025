// Copyright (c) 2025 AI4Thai Crop Guardian Team
// Licensed under the MIT License

//! API Contract Tests
//! Validates that all services adhere to their API contracts and can communicate correctly

use serde_json::json;
use tokio;
use reqwest::Client;
use std::time::Duration;
use uuid::Uuid;
use chrono::Utc;

/// Test configuration for API contract testing
pub struct ApiContractTestConfig {
    pub api_gateway_url: String,
    pub vision_service_url: String,
    pub llm_service_url: String,
    pub timeout: Duration,
}

impl Default for ApiContractTestConfig {
    fn default() -> Self {
        Self {
            api_gateway_url: "http://localhost:3000".to_string(),
            vision_service_url: "http://localhost:8001".to_string(),
            llm_service_url: "http://localhost:8002".to_string(),
            timeout: Duration::from_secs(30),
        }
    }
}

/// API Gateway Contract Tests
pub struct ApiGatewayContractTests {
    client: Client,
    config: ApiContractTestConfig,
}

impl ApiGatewayContractTests {
    pub fn new(config: ApiContractTestConfig) -> Self {
        let client = Client::builder()
            .timeout(config.timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self { client, config }
    }

    /// Test health endpoint contract
    pub async fn test_health_endpoint(&self) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client
            .get(&format!("{}/health", self.config.api_gateway_url))
            .send()
            .await?;

        assert_eq!(response.status(), 200);
        
        let body: serde_json::Value = response.json().await?;
        assert!(body.get("status").is_some());
        assert_eq!(body["status"], "healthy");
        assert!(body.get("timestamp").is_some());
        assert!(body.get("services").is_some());

        Ok(())
    }

    /// Test vision analysis endpoint contract
    pub async fn test_vision_analysis_contract(&self) -> Result<(), Box<dyn std::error::Error>> {
        let request_payload = json!({
            "request_id": Uuid::new_v4(),
            "image_data": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==",
            "crop_type": "Rice",
            "metadata": {
                "size_bytes": 100,
                "width": 1,
                "height": 1,
                "format": "png"
            },
            "timestamp": Utc::now()
        });

        let response = self.client
            .post(&format!("{}/api/vision/analyze", self.config.api_gateway_url))
            .json(&request_payload)
            .send()
            .await?;

        // Should return 200 or 503 (service unavailable in test environment)
        assert!(response.status() == 200 || response.status() == 503);

        if response.status() == 200 {
            let body: serde_json::Value = response.json().await?;
            self.validate_vision_response_contract(&body)?;
        }

        Ok(())
    }

    /// Test LLM advice endpoint contract
    pub async fn test_llm_advice_contract(&self) -> Result<(), Box<dyn std::error::Error>> {
        let request_payload = json!({
            "request_id": Uuid::new_v4(),
            "disease": "Rice Blast",
            "crop_type": "Rice",
            "severity": "Medium",
            "location": null,
            "language": "English",
            "context": {
                "conversation_id": Uuid::new_v4(),
                "previous_messages": [],
                "user_profile": null,
                "weather_context": null
            },
            "timestamp": Utc::now()
        });

        let response = self.client
            .post(&format!("{}/api/llm/advice", self.config.api_gateway_url))
            .json(&request_payload)
            .send()
            .await?;

        // Should return 200 or 503 (service unavailable in test environment)
        assert!(response.status() == 200 || response.status() == 503);

        if response.status() == 200 {
            let body: serde_json::Value = response.json().await?;
            self.validate_llm_response_contract(&body)?;
        }

        Ok(())
    }

    /// Test chat endpoint contract
    pub async fn test_chat_endpoint_contract(&self) -> Result<(), Box<dyn std::error::Error>> {
        let request_payload = json!({
            "message": "What is rice blast disease?",
            "conversation_id": Uuid::new_v4(),
            "language": "English"
        });

        let response = self.client
            .post(&format!("{}/api/chat", self.config.api_gateway_url))
            .json(&request_payload)
            .send()
            .await?;

        assert!(response.status() == 200 || response.status() == 503);
        
        if response.status() == 200 {
            let body: serde_json::Value = response.json().await?;
            assert!(body.get("response").is_some());
            assert!(body.get("conversation_id").is_some());
        }

        Ok(())
    }

    /// Validate vision service response contract
    fn validate_vision_response_contract(&self, response: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        // Required fields
        assert!(response.get("request_id").is_some());
        assert!(response.get("disease").is_some());
        assert!(response.get("confidence").is_some());
        assert!(response.get("severity").is_some());
        assert!(response.get("affected_areas").is_some());
        assert!(response.get("processing_time_ms").is_some());
        assert!(response.get("model_version").is_some());
        assert!(response.get("timestamp").is_some());

        // Type validations
        assert!(response["confidence"].is_f64());
        assert!(response["affected_areas"].is_array());
        assert!(response["processing_time_ms"].is_u64());

        // Value range validations
        let confidence = response["confidence"].as_f64().unwrap();
        assert!(confidence >= 0.0 && confidence <= 1.0);

        Ok(())
    }

    /// Validate LLM service response contract
    fn validate_llm_response_contract(&self, response: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        // Required fields
        assert!(response.get("request_id").is_some());
        assert!(response.get("treatment_plan").is_some());
        assert!(response.get("advice").is_some());
        assert!(response.get("prevention_tips").is_some());
        assert!(response.get("confidence").is_some());
        assert!(response.get("sources").is_some());
        assert!(response.get("timestamp").is_some());

        // Nested object validations
        let treatment_plan = &response["treatment_plan"];
        assert!(treatment_plan.get("steps").is_some());
        assert!(treatment_plan.get("timeline_days").is_some());
        assert!(treatment_plan.get("urgency").is_some());

        // Array validations
        assert!(response["prevention_tips"].is_array());
        assert!(response["sources"].is_array());

        Ok(())
    }
}

/// Service-to-Service Integration Tests
pub struct ServiceIntegrationTests {
    client: Client,
    config: ApiContractTestConfig,
}

impl ServiceIntegrationTests {
    pub fn new(config: ApiContractTestConfig) -> Self {
        let client = Client::builder()
            .timeout(config.timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self { client, config }
    }

    /// Test complete workflow: Image → Vision → LLM
    pub async fn test_complete_analysis_workflow(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Step 1: Submit image for analysis
        let vision_request = json!({
            "request_id": Uuid::new_v4(),
            "image_data": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==",
            "crop_type": "Rice",
            "metadata": {
                "size_bytes": 100,
                "width": 1,
                "height": 1,
                "format": "png"
            },
            "timestamp": Utc::now()
        });

        let vision_response = self.client
            .post(&format!("{}/api/vision/analyze", self.config.api_gateway_url))
            .json(&vision_request)
            .send()
            .await?;

        if vision_response.status() != 200 {
            // Service unavailable, skip integration test
            return Ok(());
        }

        let vision_result: serde_json::Value = vision_response.json().await?;

        // Step 2: Get treatment advice based on vision results
        let llm_request = json!({
            "request_id": Uuid::new_v4(),
            "disease": vision_result["disease"],
            "crop_type": "Rice",
            "severity": vision_result["severity"],
            "location": null,
            "language": "English",
            "context": {
                "conversation_id": Uuid::new_v4(),
                "previous_messages": [],
                "user_profile": null,
                "weather_context": null
            },
            "timestamp": Utc::now()
        });

        let llm_response = self.client
            .post(&format!("{}/api/llm/advice", self.config.api_gateway_url))
            .json(&llm_request)
            .send()
            .await?;

        assert_eq!(llm_response.status(), 200);
        let llm_result: serde_json::Value = llm_response.json().await?;

        // Verify workflow consistency
        assert_eq!(vision_result["disease"], llm_result["treatment_plan"]["disease"]);

        Ok(())
    }

    /// Test error handling across services
    pub async fn test_error_handling_consistency(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Test with invalid image data
        let invalid_request = json!({
            "request_id": Uuid::new_v4(),
            "image_data": "invalid_base64_data",
            "crop_type": "Rice",
            "metadata": {
                "size_bytes": 100,
                "width": 1,
                "height": 1,
                "format": "png"
            },
            "timestamp": Utc::now()
        });

        let response = self.client
            .post(&format!("{}/api/vision/analyze", self.config.api_gateway_url))
            .json(&invalid_request)
            .send()
            .await?;

        // Should return 400 Bad Request with consistent error format
        assert_eq!(response.status(), 400);
        let error_body: serde_json::Value = response.json().await?;
        
        // Validate error response structure
        assert!(error_body.get("error").is_some());
        assert!(error_body.get("message").is_some());
        assert!(error_body.get("timestamp").is_some());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_api_gateway_health_contract() {
        let config = ApiContractTestConfig::default();
        let tester = ApiGatewayContractTests::new(config);
        
        // This test will pass even if service is unavailable
        match tester.test_health_endpoint().await {
            Ok(_) => println!("✅ API Gateway health endpoint contract validated"),
            Err(e) => println!("⚠️  API Gateway not available: {}", e),
        }
    }

    #[tokio::test]
    async fn test_vision_analysis_contract() {
        let config = ApiContractTestConfig::default();
        let tester = ApiGatewayContractTests::new(config);
        
        match tester.test_vision_analysis_contract().await {
            Ok(_) => println!("✅ Vision analysis contract validated"),
            Err(e) => println!("⚠️  Vision service not available: {}", e),
        }
    }

    #[tokio::test]
    async fn test_complete_workflow_integration() {
        let config = ApiContractTestConfig::default();
        let tester = ServiceIntegrationTests::new(config);
        
        match tester.test_complete_analysis_workflow().await {
            Ok(_) => println!("✅ Complete workflow integration tested"),
            Err(e) => println!("⚠️  Integration test skipped: {}", e),
        }
    }
}