# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 🏗️ Project Architecture

AI4Thai Crop Guardian is a microservices application with a Rust backend and Python AI services:

- **API Gateway** (Rust/Axum): Main application backend at `api-gateway/` - handles routing, chat, and external API integration  
- **Frontend** (Yew WebAssembly): Progressive web app at `frontend/`
- **Shared** (Rust): Common types and utilities at `shared/`

**AI Services (deployed separately):**
- **Vision Service** (Python/FastAPI): Computer vision for pest and disease detection at `ai-services/vision-service/`
- **Queue Worker** (Python/Celery): Background vision job processing and image management at `ai-services/queue-worker/`

The project uses a Rust workspace for the main application with Redis for chat storage. AI services are deployed separately and can use either self-hosted models or external AI4Thai APIs.

## 🚀 Development Commands

### Quick Start
```bash
# Demo deployment (external AI services)
./scripts/demo-start.sh

# Full development setup
./scripts/setup-dev.sh

# Start all development services
./scripts/dev-start.sh
```

### Building
```bash
# Build API Gateway
cd api-gateway && cargo build

# Build frontend (requires trunk)
cd frontend && trunk build

# Build all Rust services
cargo build --workspace
```

### Testing
```bash
# All tests (created by setup script)
./scripts/test-all.sh

# Individual service tests
cd api-gateway && cargo test
cd ai-services && source venv/bin/activate && pytest
cd frontend && wasm-pack test --headless --firefox
```

### Frontend Development
```bash
cd frontend
trunk serve --port 8080 --open  # Hot reload development server
trunk build --release           # Production build
trunk clean                     # Clean build artifacts

# First time setup
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-pack
```

### Python AI Services
```bash
cd ai-services
source venv/bin/activate

# Vision service
cd vision-service && python -m uvicorn app:app --reload --port 8001

# Queue worker service  
cd queue-worker && python -m uvicorn app:app --reload --port 8003

# Queue worker background tasks
cd queue-worker && celery -A tasks worker --loglevel=info
```

## 🔧 Key Configuration

### Environment Setup
- Copy `.env.example` to `.env` and configure
- For demo: Set `AI4THAI_API_KEY` for external services
- For development: Set database URLs, service ports, and API keys

### Service Ports
- Frontend: 8080
- API Gateway: 3000
- Vision Service: 8001
- Queue Worker: 8003
- Redis: 6379

### Docker Services
```bash
# Main application (API Gateway + Frontend)
docker-compose up -d

# AI Services (Vision Service + Queue Worker)
cd ai-services/deployment && docker-compose up -d
```

## 📁 Code Organization

### Rust Services Structure
The API Gateway follows this simplified pattern:
```
src/
├── main.rs           # Entry point
├── lib.rs            # Library exports
├── config/           # Configuration modules
├── handlers/         # HTTP handlers  
├── models/           # Data models
├── services/         # Business logic
└── utils/           # Utilities
```

### Python Services Structure
```
├── app.py            # FastAPI application
├── requirements.txt  # Dependencies
├── models/          # ML models and configs
├── services/        # Business logic
├── utils/           # Utilities
└── tests/           # Unit tests
```

### Frontend Structure (Yew WebAssembly)
```
src/
├── main.rs          # App entry point
├── app.rs           # Main app component
├── components/      # Reusable components
├── pages/          # Page components
├── services/       # API clients
└── utils/          # Utilities
```

## 🛠️ Development Tools

### Rust Tools
- `trunk`: Frontend build tool and dev server
- `wasm-pack`: WebAssembly packaging
- `cargo-watch`: Auto-rebuild on changes
- `cargo-tarpaulin`: Code coverage

### Python Tools
- Uses virtual environment at `ai-services/venv/`
- FastAPI with uvicorn for development
- pytest for testing
- Dependencies managed per service

### Required Tools Installation
Run `./scripts/setup-dev.sh` which installs:
- Rust WASM target
- trunk, wasm-pack, cargo-watch
- Python virtual environment with all dependencies

## 🗃️ Data Storage

- Redis for chat storage, vision job queuing, and image caching
- Redis URL: `redis://localhost:6379`

## 🤖 AI Services Architecture

The system supports two deployment modes:
1. **External AI Services**: Uses AI4Thai API for quick demos
2. **Local AI Services**: Runs HuggingFace models locally for development

### Vision Service Features
- **Pest Detection**: Detects agricultural pests using YOLO11s model from `underdogquality/yolo11s-pest-detection`
- **Disease Detection**: Identifies plant diseases using LLaVA vision-language model from `YuchengShi/LLaVA-v1.5-7B-Plant-Leaf-Diseases-Detection`
- **Comprehensive Analysis**: Combined pest and disease detection in parallel
- **Thai Language Support**: Results and recommendations in Thai for local farmers
- **Treatment Recommendations**: AI-powered actionable advice for farmers
- **Real-time Analysis**: Fast async processing with dual model support
- **REST API**: Simple HTTP endpoints for integration

### Queue Worker Features
- **Background Processing**: Celery-based async job processing for vision tasks
- **Image Management**: Validation, preprocessing, and temporary storage with TTL
- **Job Tracking**: Full lifecycle management with status monitoring
- **Parallel Processing**: Supports concurrent pest and disease detection
- **Error Handling**: Automatic retries and comprehensive error reporting
- **Redis Integration**: Uses Redis for job queues and image caching