# AI4Thai Crop Guardian - Updated Project Structure

## 🏗️ Enhanced Microservices Architecture

### **Vision Service Design Decision**: Single Service with Model Registry
- **Multiple models in one service** with dynamic loading
- **Model registry** for version management and A/B testing
- **Resource sharing** across models (GPU memory optimization)
- **Load balancing** between model instances

### **External API Integration**: AI4Thai API Services
- **ASR Service**: AI4Thai Speech-to-Text API
- **TTS Service**: AI4Thai Text-to-Speech API
- **Unified authentication** and billing through AI4Thai platform

## 📁 Updated Project Structure

```
ai4thai-crop-guardian/
├── README.md
├── docker-compose.yml
├── docker-compose.dev.yml
├── .env.example
├── .env.ai4thai                    # AI4Thai API credentials
├── docs/
│   ├── api/
│   │   ├── gateway-api.md
│   │   ├── vision-service.md
│   │   ├── llm-service.md
│   │   └── ai4thai-integration.md  # AI4Thai API documentation
│   ├── deployment/
│   │   ├── local-setup.md
│   │   ├── production.md
│   │   └── model-deployment.md     # Vision model deployment guide
│   └── models/
│       ├── model-registry.md       # Model versioning and management
│       └── performance-benchmarks.md
├── scripts/
│   ├── setup.sh
│   ├── dev-start.sh
│   ├── test-all.sh
│   ├── model-download.sh           # Download vision models
│   └── ai4thai-setup.sh            # AI4Thai API setup
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
│   │   │   ├── services.rs         # Service discovery config
│   │   │   └── ai4thai.rs          # AI4Thai API configuration
│   │   ├── handlers/
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs
│   │   │   ├── chat.rs
│   │   │   ├── diagnosis.rs
│   │   │   ├── files.rs
│   │   │   ├── speech.rs           # ASR/TTS endpoints
│   │   │   └── health.rs
│   │   ├── services/
│   │   │   ├── mod.rs
│   │   │   ├── vision_client.rs    # Vision service client
│   │   │   ├── llm_client.rs       # LLM service client
│   │   │   ├── ai4thai_client.rs   # AI4Thai API client
│   │   │   ├── speech_service.rs   # ASR/TTS orchestration
│   │   │   └── queue_service.rs
│   │   ├── middleware/
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs
│   │   │   ├── rate_limit.rs
│   │   │   ├── cors.rs
│   │   │   ├── circuit_breaker.rs
│   │   │   └── ai4thai_auth.rs     # AI4Thai API authentication
│   │   ├── models/
│   │   │   ├── mod.rs
│   │   │   ├── user.rs
│   │   │   ├── chat.rs
│   │   │   ├── diagnosis.rs
│   │   │   ├── speech.rs           # ASR/TTS models
│   │   │   └── api_contracts.rs
│   │   └── utils/
│   │       ├── mod.rs
│   │       ├── validation.rs
│   │       ├── audio_processing.rs # Audio format handling
│   │       └── errors.rs
│   ├── migrations/
│   └── tests/
│       ├── integration/
│       │   ├── ai4thai_integration.rs
│       │   └── vision_service.rs
│       └── unit/
│
├── ai-services/
│   ├── shared/                     # Shared Python utilities
│   │   ├── __init__.py
│   │   ├── logging_config.py
│   │   ├── health_check.py
│   │   ├── metrics.py
│   │   ├── model_registry.py       # Model management utilities
│   │   └── performance_monitor.py
│   │
│   ├── vision-service/             # Enhanced Computer Vision Service
│   │   ├── Dockerfile
│   │   ├── Dockerfile.gpu          # GPU-optimized variant
│   │   ├── requirements.txt
│   │   ├── requirements-gpu.txt    # GPU dependencies
│   │   ├── .dockerignore
│   │   ├── app.py                  # FastAPI application
│   │   ├── config.py
│   │   ├── models/                 # Model Registry System
│   │   │   ├── __init__.py
│   │   │   ├── registry.py         # Model registry and loader
│   │   │   ├── base_model.py       # Abstract base model class
│   │   │   ├── crop_disease/       # Crop disease models
│   │   │   │   ├── __init__.py
│   │   │   │   ├── efficientnet_b0.py    # EfficientNet model
│   │   │   │   ├── resnet50.py           # ResNet model
│   │   │   │   ├── mobilenet_v3.py       # Mobile-optimized model
│   │   │   │   └── ensemble_model.py     # Ensemble of multiple models
│   │   │   ├── plant_detection/    # Plant detection models
│   │   │   │   ├── __init__.py
│   │   │   │   ├── yolo_v8.py            # YOLO object detection
│   │   │   │   └── rcnn_model.py         # R-CNN for detailed detection
│   │   │   └── model_configs/      # Model configuration files
│   │   │       ├── efficientnet_b0.yaml
│   │   │       ├── resnet50.yaml
│   │   │       ├── mobilenet_v3.yaml
│   │   │       └── ensemble.yaml
│   │   ├── preprocessing/
│   │   │   ├── __init__.py
│   │   │   ├── image_processor.py
│   │   │   ├── augmentation.py     # Data augmentation for inference
│   │   │   └── validators.py
│   │   ├── api/
│   │   │   ├── __init__.py
│   │   │   ├── routes.py
│   │   │   ├── model_routes.py     # Model management endpoints
│   │   │   └── schemas.py
│   │   ├── services/
│   │   │   ├── __init__.py
│   │   │   ├── inference_service.py
│   │   │   ├── model_manager.py    # Model loading/unloading
│   │   │   ├── batch_processor.py
│   │   │   ├── model_selector.py   # Dynamic model selection
│   │   │   └── performance_tracker.py
│   │   ├── utils/
│   │   │   ├── __init__.py
│   │   │   ├── gpu_manager.py      # GPU memory management
│   │   │   ├── cache_manager.py    # Model caching
│   │   │   └── benchmark.py        # Model performance benchmarking
│   │   ├── data/
│   │   │   ├── models/             # Downloaded model files
│   │   │   │   ├── efficientnet_b0/
│   │   │   │   ├── resnet50/
│   │   │   │   └── mobilenet_v3/
│   │   │   ├── sample_images/      # Test images
│   │   │   └── benchmarks/         # Performance test results
│   │   └── tests/
│   │       ├── test_api.py
│   │       ├── test_models.py
│   │       ├── test_model_registry.py
│   │       └── fixtures/
│   │
│   └── llm-service/                # Enhanced LLM Advisory Service
│       ├── Dockerfile
│       ├── requirements.txt
│       ├── .dockerignore
│       ├── app.py                  # FastAPI application
│       ├── config.py
│       ├── prompts/
│       │   ├── __init__.py
│       │   ├── farming_prompts.py
│       │   ├── thai_prompts.py
│       │   ├── treatment_prompts.py
│       │   ├── crop_specific/      # Crop-specific prompt templates
│       │   │   ├── rice_prompts.py
│       │   │   ├── cassava_prompts.py
│       │   │   ├── durian_prompts.py
│       │   │   └── common_prompts.py
│       │   └── contexts/           # Context-aware prompts
│       │       ├── weather_context.py
│       │       ├── seasonal_context.py
│       │       └── regional_context.py
│       ├── api/
│       │   ├── __init__.py
│       │   ├── routes.py
│       │   └── schemas.py
│       ├── services/
│       │   ├── __init__.py
│       │   ├── llm_client.py
│       │   ├── prompt_engine.py
│       │   ├── context_builder.py  # Build context from user data
│       │   ├── response_validator.py
│       │   └── thai_processor.py   # Thai language optimization
│       ├── utils/
│       │   ├── __init__.py
│       │   ├── cost_tracker.py
│       │   ├── cache_manager.py    # Response caching
│       │   └── quality_scorer.py   # Response quality assessment
│       └── tests/
│           ├── test_api.py
│           ├── test_prompts.py
│           ├── test_thai_processing.py
│           └── fixtures/
│
├── frontend/                       # Enhanced Yew WebAssembly Frontend
│   ├── Cargo.toml
│   ├── Trunk.toml
│   ├── Dockerfile
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── app.rs
│   │   ├── components/
│   │   │   ├── mod.rs
│   │   │   ├── chat/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── chat_window.rs
│   │   │   │   ├── message.rs
│   │   │   │   ├── input_bar.rs
│   │   │   │   ├── voice_input.rs  # Voice recording component
│   │   │   │   └── audio_player.rs # TTS audio playback
│   │   │   ├── diagnosis/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── result_card.rs
│   │   │   │   ├── confidence_meter.rs
│   │   │   │   ├── treatment_plan.rs
│   │   │   │   ├── model_selector.rs    # Model selection UI
│   │   │   │   └── comparison_view.rs   # Compare multiple models
│   │   │   └── media/
│   │   │       ├── mod.rs
│   │   │       ├── camera_capture.rs
│   │   │       ├── audio_recorder.rs
│   │   │       ├── audio_controls.rs    # Play/pause/speed controls
│   │   │       └── file_uploader.rs
│   │   ├── services/
│   │   │   ├── mod.rs
│   │   │   ├── api_client.rs
│   │   │   ├── websocket.rs
│   │   │   ├── speech_service.rs        # ASR/TTS integration
│   │   │   └── audio_utils.rs           # Audio processing utilities
│   │   └── utils/
│   │       ├── mod.rs
│   │       ├── audio_codecs.rs          # Audio format handling
│   │       └── thai_text.rs             # Thai text processing
│   ├── static/
│   │   ├── index.html
│   │   ├── manifest.json
│   │   ├── sw.js                        # Enhanced service worker
│   │   ├── icons/
│   │   └── audio/                       # Audio assets
│   │       ├── notification.mp3
│   │       └── success.mp3
│   └── styles/
│       ├── main.scss
│       ├── components/
│       ├── themes/
│       └── audio.scss                   # Audio component styles
│
├── shared/                             # Enhanced Shared Rust Types
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── models/
│   │   │   ├── mod.rs
│   │   │   ├── diagnosis.rs
│   │   │   ├── chat.rs
│   │   │   ├── speech.rs               # ASR/TTS models
│   │   │   └── vision_models.rs        # Vision model metadata
│   │   ├── api_contracts.rs
│   │   ├── ai4thai/                    # AI4Thai API types
│   │   │   ├── mod.rs
│   │   │   ├── speech_api.rs
│   │   │   ├── auth.rs
│   │   │   └── billing.rs
│   │   └── utils/
│   │       ├── mod.rs
│   │       └── validation.rs
│   └── build.rs
│
├── infrastructure/                     # Enhanced Infrastructure
│   ├── nginx/
│   │   ├── nginx.conf
│   │   ├── ai4thai-proxy.conf          # AI4Thai API proxy config
│   │   └── ssl/
│   ├── monitoring/
│   │   ├── prometheus.yml
│   │   ├── model-metrics.yml           # Vision model monitoring
│   │   └── grafana/
│   │       ├── dashboards/
│   │       │   ├── api-gateway.json
│   │       │   ├── vision-models.json
│   │       │   └── ai4thai-usage.json
│   │       └── provisioning/
│   ├── models/                         # Model deployment configs
│   │   ├── model-registry.yml
│   │   ├── deployment-configs/
│   │   └── benchmarks/
│   └── scripts/
│       ├── backup.sh
│       ├── deploy.sh
│       ├── model-update.sh             # Model version updates
│       └── ai4thai-health-check.sh
│
└── tools/                              # Development and deployment tools
    ├── model-benchmarking/
    │   ├── benchmark_runner.py
    │   ├── model_comparison.py
    │   └── performance_report.py
    ├── ai4thai-testing/
    │   ├── speech_test.py
    │   ├── api_load_test.py
    │   └── integration_test.py
    └── deployment/
        ├── k8s/                        # Kubernetes manifests
        │   ├── api-gateway.yaml
        │   ├── vision-service.yaml
        │   ├── llm-service.yaml
        │   └── ai4thai-secrets.yaml
        └── helm/                       # Helm charts
            ├── Chart.yaml
            ├── values.yaml
            └── templates/
```

## 🔧 Key Architecture Enhancements

### **1. Vision Service with Model Registry**
```python
# Model Registry Pattern
class ModelRegistry:
    def __init__(self):
        self.models = {}
        self.active_models = {}
        self.performance_metrics = {}
    
    async def load_model(self, model_name: str, model_version: str):
        # Dynamic model loading with GPU memory management
        
    async def get_best_model(self, crop_type: str, image_complexity: float):
        # Intelligent model selection based on requirements
        
    async def ensemble_inference(self, image: np.ndarray, models: List[str]):
        # Ensemble prediction across multiple models
```

### **2. AI4Thai API Integration**
```rust
// AI4Thai API Client
pub struct AI4ThaiClient {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

impl AI4ThaiClient {
    pub async fn speech_to_text(&self, audio: &[u8], language: &str) -> Result<String> {
        // AI4Thai ASR API integration
    }
    
    pub async fn text_to_speech(&self, text: &str, language: &str) -> Result<Vec<u8>> {
        // AI4Thai TTS API integration
    }
}
```

### **3. Enhanced Service Configuration**
```yaml
# docker-compose.yml updates
services:
  vision-service:
    environment:
      - MODEL_REGISTRY_ENABLED=true
      - MODELS_TO_LOAD=efficientnet_b0,resnet50,mobilenet_v3
      - GPU_MEMORY_LIMIT=8GB
      - MODEL_CACHE_SIZE=2GB
      
  api-gateway:
    environment:
      - AI4THAI_API_KEY=${AI4THAI_API_KEY}
      - AI4THAI_API_URL=${AI4THAI_API_URL}
      - SPEECH_CACHE_DURATION=3600
```

## 🎯 Benefits of This Architecture

### **Vision Service Benefits**
- **Resource Efficiency**: Share GPU memory across models
- **A/B Testing**: Compare model performance in real-time
- **Gradual Rollouts**: Deploy new models with traffic splitting
- **Cost Optimization**: Single service vs. multiple deployments

### **AI4Thai Integration Benefits**
- **Native Thai Support**: Optimized for Thai language processing
- **Unified Billing**: Single API account for all speech services
- **Better Performance**: Local infrastructure and optimization
- **Cost Efficiency**: Competitive pricing for Thai market

### **Development Benefits**
- **Parallel Development**: Teams can work on different components
- **Easy Scaling**: Scale individual services based on load
- **Model Flexibility**: Easy to add/remove models
- **Monitoring**: Detailed metrics per model and service

This architecture provides the flexibility for multiple models while maintaining operational simplicity and leveraging AI4Thai's specialized services for the Thai market.