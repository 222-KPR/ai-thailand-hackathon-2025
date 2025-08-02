# AI4Thai Crop Guardian 🌾

> AI-powered crop disease detection and advisory system for Thai farmers

## 🎯 Overview

AI4Thai Crop Guardian provides intelligent crop disease detection, multimodal chat interface, and personalized treatment recommendations for Thai farmers using computer vision and AI.

### Key Features

- 🔍 **Smart Disease Detection**: High accuracy crop disease identification using computer vision
- 💬 **Multimodal Chat**: Text, voice, and image-based farmer interaction
- 🌐 **Thai Language Support**: Native Thai language processing and responses
- 📱 **PWA Experience**: Offline-capable progressive web application
- 🚀 **High Performance**: Sub-3 second response times with Rust backend

### Supported Crops

- 🌾 Rice (ข้าว) - Blast, Brown spot, Bacterial blight
- 🥔 Cassava (มันสำปะหลัง) - Mosaic virus, Root rot
- 🥭 Durian (ทุเรียน) - Fusarium wilt, Anthracnose
- 🥭 Mango (มะม่วง) - Anthracnose, Powdery mildew
- 🌳 Rubber (ยางพารา) - Leaf blight, Tapping panel dryness

## 🏗️ Architecture

The system uses a microservices architecture with two deployment modes:

### Main Application (Always Deployed)
```
Frontend PWA         API Gateway         Chat Storage
(Yew WebAssembly) → (Rust/Axum)    →   (Redis)
     :8080              :3000            :6379
```

### AI Services (Deployed Separately)
```
Vision Service       Queue Worker      Background Processing
(Python/FastAPI)   (Python/FastAPI)   (Celery Workers/Beat)
     :2001             :2003                + Redis
                                           :6379
      |                   |                   |
┌─────────────────────────────────────────────────┐
│  Comprehensive Plant Health Analysis Platform  │
├─────────────────────────────────────────────────┤
│ • YOLO11s Pest Detection                       │
│ • LLaVA Disease Identification                 │
│ • Async Job Processing                         │
│ • Thai Language Support                       │
│ • Treatment Recommendations                    │
└─────────────────────────────────────────────────┘
```

## 🚀 Quick Start

### Option 1: Demo with External AI Services
```bash
git clone <repository-url>
cd ai4thai-crop-guardian

# Configure environment
cp .env.example .env
# Edit .env: Set AI4THAI_API_KEY=your_api_key

# Start demo (main application only)
./scripts/demo-start.sh
```
**Access**: Frontend at http://localhost:8080, API at http://localhost:3000

### Option 2: Full Self-hosted Deployment
```bash
# Deploy AI services first
cd ai-services/deployment
docker-compose up -d

# Start main application
cd ../../
./scripts/setup-dev.sh
./scripts/dev-start.sh
```

## 🛠️ Development

### Prerequisites
- Rust 1.70+
- Node.js 18+
- Docker & Docker Compose
- Python 3.9+ (for AI services)

### Setup
```bash
# Install development tools
./scripts/setup-dev.sh

# Start all services
./scripts/dev-start.sh

# Stop services
./scripts/dev-stop.sh
```

### Build Commands
```bash
# Main application
cargo build --workspace
cd frontend && trunk build

# AI services (if self-hosting)
cd ai-services/deployment && docker-compose build
```

### Testing
```bash
./scripts/test-all.sh              # All tests
cd api-gateway && cargo test       # Backend tests
cd frontend && wasm-pack test --headless --firefox  # Frontend tests
cd ai-services && source venv/bin/activate && pytest  # AI services tests
```

## 📁 Project Structure

```
ai4thai-crop-guardian/
├── api-gateway/          # Rust API Gateway (main backend)
├── frontend/             # Yew WebAssembly PWA
├── shared/               # Common Rust types
├── ai-services/          # AI Services (deployed separately)
│   ├── vision-service/   # Computer vision service
│   ├── queue-worker/     # Background job processing
│   └── deployment/       # Docker compose for AI services
├── scripts/              # Development and deployment scripts
└── .gitlab-ci.yml        # CI/CD pipeline
```

## 🔧 Configuration

### Environment Variables
```bash
# Main Application
REDIS_URL=redis://localhost:6379
API_GATEWAY_PORT=3000
FRONTEND_PORT=8080

# AI Services (External Mode)
AI4THAI_API_KEY=your_api_key_here

# AI Services (Self-hosted Mode)
VISION_SERVICE_URL=http://localhost:2001
QUEUE_WORKER_URL=http://localhost:2003
```

### Service Ports
- **Frontend**: 8080
- **API Gateway**: 3000
- **Vision Service**: 2001
- **Queue Worker**: 2003
- **Vision Load Balancer**: 2011
- **Redis**: 6379
- **Prometheus**: 9090
- **Grafana**: 3001

## 🚀 Deployment

### GitLab CI/CD Pipeline

The repository includes a GitLab CI/CD pipeline for automated AI services deployment:

```bash
# Tag and deploy
git tag v1.0.0
git push origin v1.0.0
```

**Pipeline Requirements**:
- GitLab runner with tag `hackathon-siamai2`
- Docker and Docker Compose installed
- 8GB+ RAM, 4+ CPU cores
- Network access to ports 2001-2011, 6379, 9090, 3001

**Pipeline Jobs**:
- `build-ai-services`: Builds Docker images (automatic)
- `deploy-ai-services`: Deploys services (manual)
- `cleanup-ai-services`: Cleanup deployment (manual)
- `health-check`: Service health verification (manual)

### Manual Deployment

```bash
# AI services deployment
./scripts/deploy-ai-services.sh deploy

# Check deployment status
./scripts/deploy-ai-services.sh status

# Health checks
./scripts/deploy-ai-services.sh health

# Cleanup
./scripts/deploy-ai-services.sh cleanup
```

### Deployed Services

| Service | Container Name | Port | Purpose |
|---------|----------------|------|---------|
| Vision Service | `team10-vision-service` | 2001 | Computer vision API |
| Queue Worker | `team10-queue-worker` | 2003 | Background job processing |
| Celery Worker | `team10-celery-worker` | - | Task processor |
| Celery Beat | `team10-celery-beat` | - | Task scheduler |
| Redis | `team10-ai-redis` | 6379 | Cache and job queue |
| Load Balancer | `team10-vision-lb` | 2011 | Vision service LB |
| Prometheus | `team10-ai-prometheus` | 9090 | Metrics collection |
| Grafana | `team10-ai-grafana` | 3001 | Monitoring dashboards |

All services use standardized team10 volumes:
- `team10-root`: Root access for all services
- `team10-data`: Data storage (models, configs, databases)

## 📊 API Reference

### Vision Service API (Port 2001)

#### Pest Detection
- **POST** `/detect/pests` - Detect pests using YOLO11s model
- **POST** `/analyze` - Alias for pest detection
- **Parameters**: `image`, `confidence_threshold` (0.01), `return_details` (false)

#### Disease Detection
- **POST** `/detect/disease` - Detect diseases using LLaVA model
- **Parameters**: `image`, `custom_prompt` (optional)

#### Comprehensive Analysis
- **POST** `/analyze/comprehensive` - Combined pest and disease detection
- **Parameters**: `image`, `pest_confidence`, `pest_details`, `disease_prompt`

#### Health & Info
- **GET** `/health` - Basic health check
- **GET** `/health/detailed` - Detailed health with model status
- **GET** `/info` - Service capabilities and model information
- **GET** `/` - Service overview

### Queue Worker API (Port 2003)

#### Async Processing
- **POST** `/analyze/pest` - Queue pest detection job
- **POST** `/analyze/disease` - Queue disease detection job
- **POST** `/analyze/comprehensive` - Queue comprehensive analysis

#### Job Management
- **GET** `/jobs/{job_id}` - Get job status and results
- **DELETE** `/jobs/{job_id}` - Cancel pending job

#### Monitoring
- **GET** `/queue/stats` - Queue and worker statistics
- **GET** `/images/stats` - Image storage statistics
- **POST** `/maintenance/cleanup` - Trigger cleanup of old data

## 🔍 AI Services

### Vision Service Features
- **Pest Detection**: YOLO11s model (`underdogquality/yolo11s-pest-detection`)
- **Disease Detection**: LLaVA model (`YuchengShi/LLaVA-v1.5-7B-Plant-Leaf-Diseases-Detection`)
- **Thai Language Support**: Results and recommendations in Thai
- **Real-time Analysis**: Fast async processing with dual model support

### Queue Worker Features
- **Background Processing**: Celery-based async job processing
- **Image Management**: Validation, preprocessing, and temporary storage
- **Job Tracking**: Full lifecycle management with status monitoring
- **Error Handling**: Automatic retries and comprehensive error reporting

## 🔍 Monitoring & Health Checks

```bash
# Check running containers
docker ps --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}" | grep team10

# View logs
docker logs team10-vision-service
docker logs team10-queue-worker

# Health checks
curl -f http://localhost:2001/health  # Vision Service
curl -f http://localhost:2003/health  # Queue Worker

# Monitor resources
docker stats $(docker ps --format "{{.Names}}" | grep team10)

# View volumes
docker volume ls | grep team10
```

## 🚨 Troubleshooting

### Common Issues

**Build Failures**:
```bash
docker info                    # Check Docker daemon
df -h                         # Check available space
docker system prune -f        # Clean up Docker
```

**Deployment Failures**:
```bash
docker logs team10-vision-service
docker logs team10-queue-worker
netstat -tulpn | grep -E ':(2001|2003|2011|6379|9090|3001)'
```

**Health Check Failures**:
```bash
curl -v http://localhost:2001/health
curl -v http://localhost:2003/health
docker ps | grep team10
```

## 🤝 Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open Pull Request

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

---

Made with ❤️ for Thai farmers by KPR team for AI Thailand Hackathon 2025
