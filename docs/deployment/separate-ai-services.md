# Separate AI Services Deployment

This guide explains how to deploy AI services (Vision and LLM) separately from the main application using HuggingFace models.

## 🎯 Architecture Overview

### Separate Deployment Architecture

```mermaid
graph TB
    subgraph "Main Application Services"
        Frontend[Frontend PWA<br/>Yew WebAssembly<br/>:8080]
        Gateway[API Gateway<br/>Rust/Axum<br/>:3000]
        Worker[Queue Worker<br/>Rust]
        DB[(PostgreSQL<br/>:5432)]
        Cache[(Redis<br/>:6379)]
    end
    
    subgraph "AI Services Infrastructure"
        subgraph "Vision Service Cluster"
            VS1[Vision Service<br/>HuggingFace Models<br/>:8001]
            VS2[Vision Service 2]
            VS3[Vision Service 3]
            VSLB[Vision Load Balancer<br/>:8011]
        end
        
        subgraph "LLM Service Cluster"
            LLM1[LLM Service<br/>HuggingFace Models<br/>:8002]
            LLM2[LLM Service 2]
            LLM3[LLM Service 3]
            LLMLB[LLM Load Balancer<br/>:8012]
        end
        
        AIRedis[(Redis Cache<br/>AI Services)]
    end
    
    Frontend --> Gateway
    Gateway --> VSLB
    Gateway --> LLMLB
    Worker --> VSLB
    Worker --> LLMLB
    
    VSLB --> VS1
    VSLB --> VS2
    VSLB --> VS3
    
    LLMLB --> LLM1
    LLMLB --> LLM2
    LLMLB --> LLM3
    
    VS1 --> AIRedis
    LLM1 --> AIRedis
    
    style VS1 fill:#e8f5e8
    style LLM1 fill:#e8f5e8
    style VSLB fill:#fff3e0
    style LLMLB fill:#fff3e0
```

## 🚀 Quick Start

### Prerequisites

- Docker and Docker Compose
- NVIDIA Docker (for GPU support)
- HuggingFace account and token
- 16GB+ RAM (32GB+ recommended)
- GPU with 8GB+ VRAM (optional but recommended)

### 1. Deploy AI Services

```bash
# Deploy AI services separately
./scripts/deploy-ai-services.sh basic

# Or with GPU support
./scripts/deploy-ai-services.sh gpu

# Or with full monitoring
./scripts/deploy-ai-services.sh all
```

### 2. Deploy Main Application

```bash
# Configure main application to use local AI services
cp .env.example .env
# Edit .env:
# VISION_SERVICE_URL=http://localhost:8001
# LLM_SERVICE_URL=http://localhost:8002

# Start main application services
docker-compose up -d
```

### 3. Access Services

- **Frontend**: http://localhost:8080
- **API Gateway**: http://localhost:3000
- **Vision Service**: http://localhost:8001
- **LLM Service**: http://localhost:8002

## 🔧 Deployment Modes

### Basic Mode
```bash
./scripts/deploy-ai-services.sh basic
```
- Vision Service
- LLM Service  
- Redis Cache

### GPU Mode
```bash
./scripts/deploy-ai-services.sh gpu
```
- GPU-optimized containers
- CUDA support
- Optimized memory usage

### Full Mode
```bash
./scripts/deploy-ai-services.sh full
```
- Basic services
- Load balancers
- High availability setup

### Monitoring Mode
```bash
./scripts/deploy-ai-services.sh monitoring
```
- Basic services
- Prometheus metrics
- Grafana dashboards

### All Features
```bash
./scripts/deploy-ai-services.sh all
```
- All services
- Load balancers
- Monitoring
- Full production setup

## 🤖 AI Models Configuration

### Vision Service Models

#### Crop Classification
- **Model**: `google/vit-base-patch16-224`
- **Purpose**: Identify crop types from images
- **Input**: 224x224 RGB images
- **Output**: Crop type with confidence score

#### Disease Detection
- **Model**: `microsoft/resnet-50` (fine-tuned)
- **Purpose**: Detect diseases in crop images
- **Input**: 256x256 RGB images
- **Output**: Disease classification with confidence

### LLM Service Models

#### Agricultural Advisor
- **Model**: `microsoft/DialoGPT-large` (fine-tuned)
- **Purpose**: Provide agricultural advice and chat
- **Context**: 512 tokens
- **Languages**: Thai, English

#### Thai Language Processing
- **Model**: `airesearch/wangchanberta-base-att-spm-uncased`
- **Purpose**: Thai language understanding
- **Context**: 256 tokens
- **Specialization**: Thai agricultural terminology

## 📊 Resource Requirements

### Minimum Requirements

| Service | CPU | RAM | GPU | Storage |
|---------|-----|-----|-----|---------|
| Vision Service | 4 cores | 8GB | 4GB VRAM | 20GB |
| LLM Service | 8 cores | 16GB | 8GB VRAM | 30GB |
| Redis Cache | 2 cores | 2GB | - | 5GB |

### Recommended Requirements

| Service | CPU | RAM | GPU | Storage |
|---------|-----|-----|-----|---------|
| Vision Service | 8 cores | 16GB | 8GB VRAM | 50GB |
| LLM Service | 16 cores | 32GB | 16GB VRAM | 100GB |
| Redis Cache | 4 cores | 4GB | - | 10GB |

## 🔧 Configuration

### Environment Variables

#### AI Services Configuration
```bash
# HuggingFace Configuration
HUGGINGFACE_HUB_TOKEN=your_token_here
MODELS_TO_DOWNLOAD=google/vit-base-patch16-224,microsoft/resnet-50

# GPU Configuration
CUDA_VISIBLE_DEVICES=0
BUILD_TARGET=gpu-production

# Resource Limits
VISION_MEMORY_LIMIT=8G
LLM_MEMORY_LIMIT=16G

# Service URLs
VISION_SERVICE_URL=http://localhost:8001
LLM_SERVICE_URL=http://localhost:8002
```

#### Main Application Configuration
```bash
# AI Service Integration
VISION_SERVICE_URL=http://localhost:8001
LLM_SERVICE_URL=http://localhost:8002

# Database
DATABASE_URL=postgresql://postgres:password@localhost:5432/ai4thai
REDIS_URL=redis://localhost:6379
```

### Model Configuration

#### Custom Model Configuration
```yaml
# ai-services/deployment/models.yml
vision_models:
  crop_classifier:
    model_name: "google/vit-base-patch16-224"
    confidence_threshold: 0.8
    batch_size: 4
    
  disease_detector:
    model_name: "microsoft/resnet-50"
    confidence_threshold: 0.7
    batch_size: 2

llm_models:
  agricultural_advisor:
    model_name: "microsoft/DialoGPT-large"
    max_length: 512
    temperature: 0.7
    
  thai_language:
    model_name: "airesearch/wangchanberta-base-att-spm-uncased"
    max_length: 256
    temperature: 0.8
```

## 🔍 Monitoring and Health Checks

### Health Check Endpoints

```bash
# Vision Service
curl http://localhost:8001/health
curl http://localhost:8001/health/models
curl http://localhost:8001/models

# LLM Service
curl http://localhost:8002/health
curl http://localhost:8002/health/models
curl http://localhost:8002/models
```

### Monitoring Dashboards

#### Prometheus Metrics
- **URL**: http://localhost:9090
- **Metrics**: Response times, error rates, model performance
- **Alerts**: Service health, resource usage

#### Grafana Dashboards
- **URL**: http://localhost:3001 (admin/admin)
- **Dashboards**: AI service performance, resource usage
- **Alerts**: Email/Slack notifications

### Log Management

```bash
# View AI service logs
docker-compose -f ai-services/deployment/docker-compose.yml logs -f

# Specific service logs
docker-compose -f ai-services/deployment/docker-compose.yml logs vision-service
docker-compose -f ai-services/deployment/docker-compose.yml logs llm-service
```

## 🔄 Scaling and Load Balancing

### Horizontal Scaling

```bash
# Scale Vision Service
docker-compose -f ai-services/deployment/docker-compose.yml up -d --scale vision-service=3

# Scale LLM Service
docker-compose -f ai-services/deployment/docker-compose.yml up -d --scale llm-service=2
```

### Load Balancer Configuration

#### NGINX Configuration
```nginx
# ai-services/deployment/nginx/vision-nginx.conf
upstream vision_backend {
    server vision-service:8001;
    server vision-service-2:8001;
    server vision-service-3:8001;
}

server {
    listen 80;
    location / {
        proxy_pass http://vision_backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

### Auto-scaling with Docker Swarm

```bash
# Initialize Docker Swarm
docker swarm init

# Deploy as stack with auto-scaling
docker stack deploy -c ai-services/deployment/docker-compose.swarm.yml ai-services

# Scale services
docker service scale ai-services_vision-service=5
docker service scale ai-services_llm-service=3
```

## 🚨 Troubleshooting

### Common Issues

#### Model Download Failures
```bash
# Check HuggingFace token
echo $HUGGINGFACE_HUB_TOKEN

# Manual model download
docker-compose --profile init up model-downloader

# Check model cache
docker volume inspect ai-services_vision_models
```

#### GPU Not Detected
```bash
# Check NVIDIA Docker
docker run --rm --gpus all nvidia/cuda:11.8-base-ubuntu22.04 nvidia-smi

# Check CUDA environment
docker-compose exec vision-service nvidia-smi
```

#### Out of Memory Errors
```bash
# Reduce batch size
export VISION_BATCH_SIZE=1
export LLM_MAX_LENGTH=256

# Increase memory limits
export VISION_MEMORY_LIMIT=16G
export LLM_MEMORY_LIMIT=32G
```

#### Service Connection Issues
```bash
# Check service connectivity
curl -f http://localhost:8001/health
curl -f http://localhost:8002/health

# Check network configuration
docker network ls
docker network inspect ai-services_ai-services-network
```

### Performance Optimization

#### Model Optimization
```bash
# Enable model quantization
export ENABLE_QUANTIZATION=true

# Use optimized models
export CROP_CLASSIFIER_MODEL=google/vit-base-patch16-224-optimized
export DISEASE_DETECTOR_MODEL=microsoft/resnet-50-quantized
```

#### Caching Optimization
```bash
# Increase Redis memory
export REDIS_MAXMEMORY=4gb

# Enable model caching
export ENABLE_MODEL_CACHE=true
export MODEL_CACHE_SIZE=2gb
```

## 📚 API Documentation

### Vision Service API

#### Disease Detection
```bash
curl -X POST http://localhost:8001/detect/disease \
  -F "image=@crop_image.jpg" \
  -F "crop_type=rice" \
  -F "confidence_threshold=0.7"
```

#### Crop Classification
```bash
curl -X POST http://localhost:8001/classify/crop \
  -F "image=@plant_image.jpg" \
  -F "confidence_threshold=0.8"
```

### LLM Service API

#### Chat
```bash
curl -X POST http://localhost:8002/chat \
  -H "Content-Type: application/json" \
  -d '{
    "message": "How do I treat rice blast disease?",
    "language": "th"
  }'
```

#### Agricultural Advice
```bash
curl -X POST http://localhost:8002/advice \
  -H "Content-Type: application/json" \
  -d '{
    "crop_type": "rice",
    "disease": "blast",
    "language": "th"
  }'
```

## 🔐 Security Considerations

### API Security
- API key authentication
- Rate limiting
- Input validation
- Output sanitization

### Model Security
- Model encryption at rest
- Secure model downloads
- Access control
- Audit logging

### Network Security
- Private networks
- SSL/TLS encryption
- Firewall rules
- VPN access

---

This separate deployment approach provides better resource utilization, independent scaling, and easier maintenance of AI services while maintaining full functionality of the AI4Thai Crop Guardian system.
