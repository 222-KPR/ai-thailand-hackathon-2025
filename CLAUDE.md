# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 🏗️ Project Architecture

AI4Thai Crop Guardian is a microservices application with a Rust backend and Python AI services:

- **API Gateway** (Rust/Axum): Central entry point at `api-gateway/` - handles authentication, routing, and external API integration
- **Vision Service** (Python/FastAPI): Computer vision for disease detection at `ai-services/vision-service/`
- **LLM Service** (Python/FastAPI): Agricultural advisory using language models at `ai-services/llm-service/`
- **Frontend** (Yew WebAssembly): Progressive web app at `frontend/`
- **Shared** (Rust): Common types and utilities at `shared/`
- **Queue Worker** (Rust): Background job processing at `queue-worker/`

The project uses a Rust workspace with PostgreSQL and Redis for data storage and caching.

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

# LLM service  
cd llm-service && python -m uvicorn app:app --reload --port 8002
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
- LLM Service: 8002
- PostgreSQL: 5432
- Redis: 6379

### Docker Services
```bash
docker-compose up -d postgres redis  # Infrastructure only
docker-compose up -d                 # All services
```

## 📁 Code Organization

### Rust Services Structure
Each Rust service follows this pattern:
```
src/
├── main.rs           # Entry point
├── lib.rs            # Library exports
├── config/           # Configuration modules
├── handlers/         # HTTP handlers
├── middleware/       # Custom middleware
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

## 🗃️ Database & Migrations

- PostgreSQL database with migrations in `api-gateway/migrations/`
- Redis for caching and queue management
- Database URL: `postgresql://postgres:password@localhost:5432/ai4thai`

## 🤖 AI Services Architecture

The system supports two deployment modes:
1. **External AI Services**: Uses AI4Thai API for quick demos
2. **Local AI Services**: Runs HuggingFace models locally for development

Vision service handles crop disease detection, LLM service provides agricultural advice in Thai language.