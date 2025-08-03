# AI4Thai AI Services 🤖

> Advanced AI services for agricultural pest detection and disease identification using modern Python tooling

## 🎯 Overview

The AI Services component provides:

- **Vision Service**: Real-time pest detection and plant disease identification
- **Queue Worker**: Background job processing and image data management
- **Modern Development Stack**: uv, ruff, pytest, and comprehensive tooling

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  Vision Service │    │  Queue Worker   │    │     Redis       │
│   (Port 2001)   │    │   (Port 2003)   │    │   (Port 6379)   │
│                 │    │                 │    │                 │
│ • YOLO11s Pest  │◄──►│ • Async Jobs    │◄──►│ • Job Queue     │
│ • LLaVA Disease │    │ • Image Storage │    │ • Cache         │
│ • Real-time API │    │ • Celery Worker │    │ • Session Store │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## 🚀 Quick Start

### Prerequisites

- Python 3.11+
- [uv](https://github.com/astral-sh/uv) (fast Python package manager)
- Docker & Docker Compose (for Redis and deployment)
- CUDA-capable GPU (optional, for accelerated inference)

### Development Setup

```bash
# Clone and navigate to ai-services
cd ai-services

# Run automated setup
./setup-dev.sh

# Start Redis (required for queue worker)
docker run -d -p 6379:6379 redis:alpine

# Start Vision Service (Terminal 1)
cd vision-service
make dev

# Start Queue Worker (Terminal 2)
cd queue-worker
make dev

# Start Celery Worker (Terminal 3)
cd queue-worker
make worker
```

### Testing the Services

```bash
# Test Vision Service
curl -X POST "http://localhost:2001/health"

# Test Queue Worker
curl -X POST "http://localhost:2003/health"

# Upload image for pest detection
curl -X POST "http://localhost:2001/detect/pests" \
  -F "image=@path/to/your/crop_image.jpg" \
  -F "confidence_threshold=0.3"
```

## 🛠️ Development

### Modern Python Tooling

This project uses modern Python development tools:

- **[uv](https://github.com/astral-sh/uv)**: Ultra-fast Python package installer and resolver
- **[ruff](https://github.com/astral-sh/ruff)**: Extremely fast Python linter and formatter
- **pytest**: Testing framework with async support
- **pyproject.toml**: Modern Python project configuration

### Code Quality

```bash
# Run linting for all services
./lint-all.sh check

# Auto-fix linting issues
./lint-all.sh fix

# Run tests for specific service
cd vision-service && make test
cd queue-worker && make test
```

### Available Commands

Each service includes a Makefile with development commands:

```bash
# Show available commands
make help

# Environment setup
make install      # Production dependencies
make dev-install  # Development dependencies

# Code quality
make lint         # Run ruff linter
make format       # Format code with ruff
make check        # Full lint + format check

# Testing
make test         # Run pytest with coverage
make test-fast    # Run pytest without coverage

# Development
make dev          # Start development server
make run          # Start production server
```

## 📁 Project Structure

```
ai-services/
├── vision-service/           # Computer vision AI service
│   ├── app.py               # FastAPI application
│   ├── services/            # AI model services
│   │   ├── pest_detection.py    # YOLO11s pest detection
│   │   └── disease_detection.py # LLaVA disease detection
│   ├── memory_manager.py    # GPU memory optimization
│   ├── pyproject.toml       # uv project configuration
│   ├── Makefile            # Development commands
│   └── Dockerfile          # Container configuration
│
├── queue-worker/            # Background job processing
│   ├── app.py              # FastAPI application
│   ├── tasks.py            # Celery task definitions
│   ├── pyproject.toml      # uv project configuration
│   ├── Makefile           # Development commands
│   └── Dockerfile         # Container configuration
│
├── deployment/             # Docker Compose configurations
├── setup-dev.sh           # Automated development setup
├── lint-all.sh           # Project-wide linting
└── README.md             # This file
```

## 🧠 AI Models

### Vision Service Models

1. **Pest Detection**: YOLO11s
   - Source: `underdogquality/yolo11s-pest-detection`
   - Purpose: Real-time agricultural pest identification
   - Performance: Sub-3 second inference on H100

2. **Disease Detection**: LLaVA-v1.5-7B
   - Source: `YuchengShi/LLaVA-v1.5-7B-Plant-Leaf-Diseases-Detection`
   - Purpose: Plant disease identification and analysis
   - Features: Thai language support, treatment recommendations

### Model Optimization

- **Memory Management**: Automatic GPU memory optimization for 16GB VRAM
- **Batch Processing**: Optimized batch sizes for H100 architecture
- **Model Caching**: Intelligent model loading and caching strategies
- **Async Processing**: Non-blocking inference with queue-based processing

## 🐳 Docker Deployment

### Development

```bash
# Build services
docker-compose -f deployment/docker-compose.yml build

# Start all services
docker-compose -f deployment/docker-compose.yml up -d

# View logs
docker-compose -f deployment/docker-compose.yml logs -f
```

### Production

```bash
# Use production-optimized builds
docker build --target production -t ai4thai-vision-service vision-service/
docker build --target production -t ai4thai-queue-worker queue-worker/

# For H100 optimization
docker build --target h100-optimized -t ai4thai-vision-h100 vision-service/
```

## 📊 API Documentation

### Vision Service (Port 2001)

```bash
# Health checks
GET  /health                    # Basic health check
GET  /health/detailed          # Detailed health with model status
GET  /health/disease           # Disease detection service health
GET  /health/memory            # Memory usage statistics

# Pest Detection
POST /detect/pests             # Detect pests in image
POST /analyze                  # Alias for pest detection

# Disease Detection
POST /detect/disease           # Detect plant diseases

# Comprehensive Analysis
POST /analyze/comprehensive    # Combined pest + disease detection

# Service Information
GET  /info                     # Service capabilities and models
GET  /                         # Service overview
```

### Queue Worker (Port 2003)

```bash
# Health and Status
GET  /health                   # Health check
GET  /info                     # Service information

# Async Job Processing
POST /analyze/pest             # Queue pest detection job
POST /analyze/disease          # Queue disease detection job
POST /analyze/comprehensive    # Queue comprehensive analysis

# Job Management
GET  /jobs/{job_id}           # Get job status and results
DELETE /jobs/{job_id}         # Cancel pending job

# Image Storage
POST /images/store            # Store image for later processing

# Monitoring
GET  /queue/stats             # Queue statistics
GET  /images/stats            # Image storage statistics

# Maintenance
POST /maintenance/cleanup     # Trigger cleanup
```

## 🔧 Configuration

### Environment Variables

```bash
# Vision Service
CUDA_VISIBLE_DEVICES=0        # GPU selection
MODEL_MAX_LENGTH=512          # Max input length
MAX_BATCH_SIZE=1             # Batch size for inference
GRADIENT_CHECKPOINTING=true   # Memory optimization

# Queue Worker
REDIS_URL=redis://localhost:6379/0
CELERY_BROKER_URL=redis://localhost:6379/0
CELERY_RESULT_BACKEND=redis://localhost:6379/0
VISION_SERVICE_URL=http://localhost:2001
```

### Performance Tuning

For H100 16GB VRAM optimization:

```bash
# GPU Memory Management
export PYTORCH_CUDA_ALLOC_CONF=max_split_size_mb:2048
export TORCH_CUDA_ARCH_LIST="9.0"
export CUDA_LAUNCH_BLOCKING=0

# Model Optimization
export USE_FLASH_ATTENTION=true
export GRADIENT_CHECKPOINTING=true
export MODEL_MAX_LENGTH=512
```

## 🧪 Testing

### Running Tests

```bash
# Test all services
cd vision-service && make test
cd queue-worker && make test

# Run specific test categories
cd vision-service && uv run pytest -m "not slow"
cd queue-worker && uv run pytest -m integration
```

### Test Categories

- **Unit Tests**: Fast, isolated component tests
- **Integration Tests**: Service integration testing
- **Slow Tests**: Full model inference tests (marked as `slow`)

## 🚨 Troubleshooting

### Common Issues

**CUDA Out of Memory**:
```bash
# Reduce batch size and enable memory optimization
export MAX_BATCH_SIZE=1
export GRADIENT_CHECKPOINTING=true
export PYTORCH_CUDA_ALLOC_CONF=max_split_size_mb:1024
```

**Model Loading Errors**:
```bash
# Clear model cache and restart
rm -rf ~/.cache/huggingface/
make clean && make dev
```

**Redis Connection Issues**:
```bash
# Start Redis container
docker run -d -p 6379:6379 redis:alpine

# Check Redis connection
redis-cli ping
```

### Performance Monitoring

```bash
# Monitor GPU usage
nvidia-smi -l 1

# Monitor memory usage
curl http://localhost:2001/health/memory

# Monitor queue statistics
curl http://localhost:2003/queue/stats
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Set up development environment (`./setup-dev.sh`)
4. Make changes and test (`make test`)
5. Run linting (`./lint-all.sh check`)
6. Commit changes (`git commit -m 'Add amazing feature'`)
7. Push to branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## 📄 License

MIT License - see [LICENSE](../LICENSE) file for details.

---

Made with ❤️ for Thai farmers by KPR team for AI Thailand Hackathon 2025
