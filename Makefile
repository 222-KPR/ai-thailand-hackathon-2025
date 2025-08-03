# AI4Thai Crop Guardian - Makefile
# Simple command interface for development and deployment

.PHONY: help dev prod build clean test lint format stop health

# Default target
help:
	@echo "🌾 AI4Thai Crop Guardian"
	@echo ""
	@echo "Available commands:"
	@echo "  make dev     - Start development environment"
	@echo "  make prod    - Start production environment"
	@echo "  make build   - Build all services"
	@echo "  make test    - Run all tests"
	@echo "  make lint    - Run code quality checks"
	@echo "  make format  - Format code"
	@echo "  make health  - Check service health"
	@echo "  make stop    - Stop all services"
	@echo "  make clean   - Clean up containers and images"

# Development environment (local)
dev:
	@echo "🚀 Starting AI4Thai Development Environment..."
	@if [ ! -f .env ]; then cp .env.template .env; fi
	@cd backend/vision-service && python3 -m venv venv && source venv/bin/activate && pip install -r requirements.txt && uvicorn app:app --host 127.0.0.1 --port 2001 --reload &
	@cd backend/queue-worker && python3 -m venv venv && source venv/bin/activate && pip install -r requirements.txt && uvicorn app:app --host 127.0.0.1 --port 8001 --reload &
	@cd frontend && trunk serve --address 127.0.0.1 --port 8080 &
	@echo "✅ Services started:"
	@echo "  Frontend:      http://localhost:8080"
	@echo "  Vision Service: http://localhost:2001"
	@echo "  Queue Worker:   http://localhost:8001"

# Production environment (Docker)
prod:
	@echo "🚀 Starting AI4Thai Production Environment..."
	@if [ ! -f .env ]; then cp .env.template .env && sed -i 's/ENVIRONMENT=development/ENVIRONMENT=production/' .env; fi
	@docker-compose up --build -d
	@echo "✅ Production environment started"
	@make health

# Build all services
build:
	@echo "📦 Building all services..."
	@docker-compose build
	@echo "✅ Build completed"

# Run tests
test:
	@echo "🧪 Running tests..."
	@cd frontend && cargo test
	@cd backend/vision-service && if [ -d venv ]; then source venv/bin/activate && python -m pytest; fi
	@cd backend/queue-worker && if [ -d venv ]; then source venv/bin/activate && python -m pytest; fi
	@echo "✅ Tests completed"

# Code quality checks
lint:
	@echo "🔍 Running code quality checks..."
	@pre-commit run --all-files
	@echo "✅ Lint checks completed"

# Format code
format:
	@echo "✨ Formatting code..."
	@cd frontend && cargo fmt
	@cd backend/vision-service && if [ -d venv ]; then source venv/bin/activate && black . && isort .; fi
	@cd backend/queue-worker && if [ -d venv ]; then source venv/bin/activate && black . && isort .; fi
	@echo "✅ Code formatted"

# Check service health
health:
	@echo "🏥 Checking service health..."
	@curl -f http://localhost:2001/health 2>/dev/null && echo "✅ Vision Service: healthy" || echo "❌ Vision Service: down"
	@curl -f http://localhost:8001/health 2>/dev/null && echo "✅ Queue Worker: healthy" || echo "❌ Queue Worker: down"
	@curl -f http://localhost:8080 2>/dev/null > /dev/null && echo "✅ Frontend: healthy" || echo "❌ Frontend: down"

# Stop all services
stop:
	@echo "🛑 Stopping all services..."
	@docker-compose down 2>/dev/null || true
	@pkill -f "uvicorn app:app" 2>/dev/null || true
	@pkill -f "trunk serve" 2>/dev/null || true
	@echo "✅ All services stopped"

# Clean up
clean: stop
	@echo "🧹 Cleaning up..."
	@docker system prune -f
	@rm -rf backend/*/venv
	@rm -rf frontend/target frontend/dist
	@echo "✅ Cleanup completed"
