# AI4Thai Crop Guardian - Microservices Architecture

## 🏗️ System Design Overview

### **Architecture Pattern**: Domain-Driven Microservices
- **API Gateway**: Rust (Axum) - High-performance routing and business logic
- **AI Services**: Python (FastAPI) - ML model serving and inference
- **External APIs**: Third-party TTS/ASR services
- **Data Layer**: PostgreSQL + Redis for persistence and caching

## 📐 Service Architecture Diagram

```
┌─────────────────────┐
│     Frontend        │
│   (Yew WASM PWA)    │
└──────────┬──────────┘
           │ HTTPS/WSS
           ▼
┌─────────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   API Gateway       │    │  Vision Service │    │   LLM Service   │
│   (Rust/Axum)       │───▶│  (Python/FastAPI)│    │ (Python/FastAPI)│
│                     │    │                 │    │                 │
│ • Authentication    │    │ • Crop Disease  │    │ • Farm Advisory │
│ • Rate Limiting     │    │ • Image Processing│  │ • Treatment Plans│
│ • Request Routing   │    │ • Model Inference│   │ • Thai Language │
│ • WebSocket         │    │                 │    │                 │
│ • File Upload       │    │                 │    │                 │
└─────────┬───────────┘    └─────────────────┘    └─────────────────┘
          │                          │                       │
          ▼                          ▼                       ▼
┌─────────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│    PostgreSQL       │    │     Redis       │    │  External APIs  │
│                     │    │                 │    │                 │
│ • User Data         │    │ • Job Queue     │    │ • TTS Service   │
│ • Chat History      │    │ • Caching       │    │ • ASR Service   │
│ • Diagnosis Records │    │ • Session Store │    │ • Weather API   │
└─────────────────────┘    └─────────────────┘    └─────────────────┘
```

## 🔧 Service Responsibilities

### **API Gateway (Rust - Port 3000)**
**Domain**: User Management, Orchestration, Communication
```rust
// Core responsibilities
├── Authentication & Authorization
├── Request/Response Transformation  
├── Service Discovery & Load Balancing
├── Rate Limiting & Circuit Breaker
├── WebSocket Chat Management
├── File Upload/Download
├── External API Integration (TTS/ASR)
├── Caching Strategy
└── Error Handling & Logging
```

### **Vision Service (Python - Port 8001)**
**Domain**: Computer Vision & Image Analysis
```python
# Core responsibilities
├── Crop Disease Detection
├── Image Preprocessing & Validation
├── ML Model Inference
├── Confidence Score Calculation
├── Batch Processing Support
├── Model Version Management
└── Health Monitoring
```

### **LLM Service (Python - Port 8002)**
**Domain**: Natural Language Processing & Advisory
```python
# Core responsibilities  
├── Farming Advisory Generation
├── Treatment Recommendation
├── Thai Language Processing
├── Context-Aware Responses
├── Prompt Engineering
├── Response Validation
└── Cost Optimization
```

## 📁 Updated Project Structure

```
ai4thai-crop-guardian/
├── README.md
├── docker-compose.yml
├── docker-compose.dev.yml
├── .env.example
├── docs/
│   ├── api/
│   │   ├── gateway-api.md
│   │   ├── vision-service.md
│   │   └── llm-service.md
│   └── deployment/
│       ├── local-setup.md
│       └── production.md
├── scripts/
│   ├── setup.sh
│   ├── dev-start.sh
│   └── test-all.sh
│
├── api-gateway/                    # Rust API Gateway
│   ├── Cargo.toml
│   ├── Dockerfile
│   ├── .dockerignore
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── config/
│   │   │   ├── mod.rs
│   │   │   ├── database.rs
│   │   │   ├── redis.rs
│   │   │   └── services.rs         # Service discovery config
│   │   ├── handlers/
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs
│   │   │   ├── chat.rs
│   │   │   ├── diagnosis.rs
│   │   │   ├── files.rs
│   │   │   └── health.rs
│   │   ├── services/
│   │   │   ├── mod.rs
│   │   │   ├── vision_client.rs    # Vision service client
│   │   │   ├── llm_client.rs       # LLM service client
│   │   │   ├── tts_client.rs       # External TTS API
│   │   │   ├── asr_client.rs       # External ASR API
│   │   │   └── queue_service.rs
│   │   ├── middleware/
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs
│   │   │   ├── rate_limit.rs
│   │   │   ├── cors.rs
│   │   │   └── circuit_breaker.rs
│   │   ├── models/
│   │   │   ├── mod.rs
│   │   │   ├── user.rs
│   │   │   ├── chat.rs
│   │   │   ├── diagnosis.rs
│   │   │   └── api_contracts.rs    # Service API contracts
│   │   └── utils/
│   │       ├── mod.rs
│   │       ├── validation.rs
│   │       └── errors.rs
│   ├── migrations/
│   └── tests/
│       ├── integration/
│       └── unit/
│
├── ai-services/
│   ├── shared/                     # Shared Python utilities
│   │   ├── __init__.py
│   │   ├── logging_config.py
│   │   ├── health_check.py
│   │   └── metrics.py
│   │
│   ├── vision-service/             # Computer Vision Service
│   │   ├── Dockerfile
│   │   ├── requirements.txt
│   │   ├── .dockerignore
│   │   ├── app.py                  # FastAPI application
│   │   ├── config.py
│   │   ├── models/
│   │   │   ├── __init__.py
│   │   │   ├── crop_disease_model.py
│   │   │   └── model_loader.py
│   │   ├── preprocessing/
│   │   │   ├── __init__.py
│   │   │   ├── image_processor.py
│   │   │   └── validators.py
│   │   ├── api/
│   │   │   ├── __init__.py
│   │   │   ├── routes.py
│   │   │   └── schemas.py
│   │   ├── services/
│   │   │   ├── __init__.py
│   │   │   ├── inference_service.py
│   │   │   └── batch_processor.py
│   │   └── tests/
│   │       ├── test_api.py
│   │       ├── test_models.py
│   │       └── fixtures/
│   │
│   └── llm-service/                # LLM Advisory Service
│       ├── Dockerfile
│       ├── requirements.txt
│       ├── .dockerignore
│       ├── app.py                  # FastAPI application
│       ├── config.py
│       ├── prompts/
│       │   ├── __init__.py
│       │   ├── farming_prompts.py
│       │   ├── thai_prompts.py
│       │   └── treatment_prompts.py
│       ├── api/
│       │   ├── __init__.py
│       │   ├── routes.py
│       │   └── schemas.py
│       ├── services/
│       │   ├── __init__.py
│       │   ├── llm_client.py
│       │   ├── prompt_engine.py
│       │   └── response_validator.py
│       ├── utils/
│       │   ├── __init__.py
│       │   ├── thai_processor.py
│       │   └── cost_tracker.py
│       └── tests/
│           ├── test_api.py
│           ├── test_prompts.py
│           └── fixtures/
│
├── frontend/                       # Yew WebAssembly Frontend
│   ├── Cargo.toml
│   ├── Trunk.toml
│   ├── Dockerfile
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── app.rs
│   │   ├── components/
│   │   ├── services/
│   │   │   ├── mod.rs
│   │   │   ├── api_client.rs       # Gateway API client
│   │   │   ├── websocket.rs
│   │   │   └── external_apis.rs    # Direct external API calls
│   │   └── utils/
│   ├── static/
│   └── styles/
│
├── shared/                         # Shared Rust Types
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── models/
│   │   │   ├── mod.rs
│   │   │   ├── diagnosis.rs
│   │   │   ├── chat.rs
│   │   │   └── api_contracts.rs    # Cross-service contracts
│   │   └── utils/
│   └── build.rs
│
└── infrastructure/
    ├── nginx/
    │   ├── nginx.conf
    │   └── ssl/
    ├── monitoring/
    │   ├── prometheus.yml
    │   └── grafana/
    └── scripts/
        ├── backup.sh
        └── deploy.sh
```

## 🔌 API Contracts & Communication

### **Inter-Service Communication**
```rust
// Service-to-service API contracts
#[derive(Serialize, Deserialize)]
pub struct VisionRequest {
    pub image_data: String,     // base64 encoded
    pub crop_type: CropType,
    pub metadata: ImageMetadata,
    pub request_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct VisionResponse {
    pub disease: String,
    pub confidence: f32,
    pub severity: DiseaseSeverity,
    pub affected_areas: Vec<BoundingBox>,
    pub processing_time_ms: u64,
    pub model_version: String,
    pub request_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct LLMRequest {
    pub disease: String,
    pub crop_type: CropType,
    pub severity: DiseaseSeverity,
    pub location: Option<GeoLocation>,
    pub language: Language,
    pub context: ChatContext,
    pub request_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct LLMResponse {
    pub treatment_plan: TreatmentPlan,
    pub advice: String,
    pub prevention_tips: Vec<String>,
    pub estimated_cost: Option<CostEstimate>,
    pub confidence: f32,
    pub request_id: Uuid,
}
```

### **External API Integration**
```rust
// External service clients in API Gateway
#[derive(Clone)]
pub struct ExternalServices {
    pub tts_client: TTSClient,
    pub asr_client: ASRClient,
    pub weather_client: WeatherClient,
}

impl TTSClient {
    pub async fn synthesize_thai(&self, text: &str) -> Result<AudioData> {
        // Call external Thai TTS service
    }
}

impl ASRClient {
    pub async fn transcribe_thai(&self, audio: &[u8]) -> Result<String> {
        // Call external Thai ASR service
    }
}
```

## 🚦 Service Discovery & Load Balancing

### **Service Registry Pattern**
```rust
// In API Gateway
#[derive(Clone)]
pub struct ServiceRegistry {
    vision_service: ServiceEndpoint,
    llm_service: ServiceEndpoint,
    health_checker: HealthChecker,
}

impl ServiceRegistry {
    pub async fn get_healthy_vision_endpoint(&self) -> Result<String> {
        // Return healthy vision service endpoint
        // Implement circuit breaker pattern
    }
    
    pub async fn get_healthy_llm_endpoint(&self) -> Result<String> {
        // Return healthy LLM service endpoint
        // Implement load balancing
    }
}
```

## 📊 Monitoring & Observability

### **Health Check Endpoints**
```python
# In each Python service
@app.get("/health")
async def health_check():
    return {
        "status": "healthy",
        "timestamp": datetime.utcnow(),
        "version": "1.0.0",
        "dependencies": {
            "model_loaded": model_status(),
            "memory_usage": get_memory_usage(),
            "gpu_usage": get_gpu_usage() if gpu_available else None
        }
    }

@app.get("/metrics")
async def metrics():
    return {
        "requests_total": request_counter,
        "requests_per_second": calculate_rps(),
        "average_response_time": avg_response_time,
        "error_rate": error_rate,
        "model_inference_time": avg_inference_time
    }
```

## 🔒 Security & Best Practices

### **Authentication Flow**
```
1. Frontend → API Gateway: JWT Token
2. API Gateway → AI Services: Service-to-Service Auth (API Key)
3. API Gateway → External APIs: API Key/OAuth
```

### **Rate Limiting Strategy**
```rust
// Per-user rate limiting
pub struct RateLimiter {
    user_limits: HashMap<UserId, TokenBucket>,
    service_limits: HashMap<ServiceId, TokenBucket>,
}

// Rate limits:
// - User requests: 100/minute
// - Vision API: 50/minute per user
// - LLM API: 20/minute per user
// - TTS/ASR: 30/minute per user
```

### **Error Handling & Circuit Breaker**
```rust
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Vision service unavailable")]
    VisionServiceDown,
    #[error("LLM service timeout")]
    LLMTimeout,
    #[error("External API rate limit exceeded")]
    ExternalAPIRateLimit,
}

// Circuit breaker implementation
pub struct CircuitBreaker {
    state: CircuitState,
    failure_threshold: u32,
    timeout_duration: Duration,
}
```

## 🎯 Development Benefits

### **Parallel Development**
- **Rust Team**: API Gateway, authentication, WebSocket, external integrations
- **Python Team**: Vision model, LLM service, inference optimization
- **Frontend Team**: WASM UI, camera integration, chat interface

### **Technology Optimization**
- **Rust**: High-performance API routing, memory efficiency, type safety
- **Python**: ML ecosystem, rapid model prototyping, library ecosystem
- **External APIs**: Proven TTS/ASR services, cost-effective, reliable

### **Scalability & Maintenance**
- **Independent Scaling**: Scale vision/LLM services based on load
- **Model Updates**: Deploy new models without affecting API Gateway
- **Service Isolation**: Failures in one service don't affect others
- **Cost Optimization**: Scale expensive AI services independently

This microservices architecture follows industry best practices while optimizing for your hackathon timeline and resource constraints.