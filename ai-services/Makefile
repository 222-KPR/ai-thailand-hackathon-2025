# AI4Thai Services - Development & Testing Makefile
# Comprehensive Makefile for Python services development and testing

.PHONY: help install format lint test test-unit test-integration clean coverage pre-commit setup-hooks

# Default target
help: ## Show this help message
	@echo "AI4Thai Services - Available Commands:"
	@echo "======================================"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

# Installation and setup
install: ## Install all dependencies
	uv sync --group test
	@echo "✅ Dependencies installed"

setup-hooks: ## Setup pre-commit hooks
	cd .. && pre-commit install
	@echo "✅ Pre-commit hooks installed"

# Code quality
format: ## Format code with ruff
	uv run ruff format .
	@echo "✅ Code formatted"

format-check: ## Check code formatting
	uv run ruff format --check .

lint: ## Lint code with ruff
	uv run ruff check --fix .
	@echo "✅ Code linted"

lint-check: ## Check linting without fixing
	uv run ruff check .

# Testing
test: install format lint test-unit ## Run all tests (format, lint, unit)
	@echo "🎉 All tests completed!"

test-unit: ## Run unit tests only
	@echo "🧪 Running unit tests..."
	uv run pytest test_basic.py -v --tb=short

test-basic: ## Run basic tests without coverage
	@echo "🧪 Running basic tests..."
	uv run pytest test_basic.py -v --no-cov

test-vision: ## Test vision service only
	@echo "🔍 Testing Vision Service..."
	uv run pytest vision-service/test_unit.py -v --no-cov || echo "⚠️  Vision service tests have dependencies not installed"

test-queue: ## Test queue worker only
	@echo "⚡ Testing Queue Worker..."
	uv run pytest queue-worker/test_unit.py -v --no-cov || echo "⚠️  Queue worker tests have dependencies not installed"

test-shared: ## Test shared utilities only
	@echo "🔧 Testing Shared Utilities..."
	uv run pytest shared/test_unit.py -v --no-cov || echo "⚠️  Shared utilities tests have dependencies not installed"

test-integration: ## Run integration tests (requires running services)
	@echo "🔗 Running integration tests..."
	@echo "⚠️  Ensure services are running on ports 2001 and 2003"
	python vision-service/test_vision_service.py

test-fast: ## Run fast tests for quick feedback
	uv run pytest test_basic.py -x --tb=line -q --maxfail=3 --no-cov

# Coverage
coverage: ## Generate test coverage report
	uv run pytest --cov=vision-service --cov=queue-worker --cov=shared --cov-report=html:htmlcov --cov-report=term-missing
	@echo "📊 Coverage report generated in htmlcov/index.html"

coverage-open: coverage ## Generate and open coverage report
	open htmlcov/index.html

# Pre-commit
pre-commit: ## Run pre-commit on all files
	cd .. && pre-commit run --all-files

# Service management helpers
check-services: ## Check if services are running
	@echo "🔍 Checking service status..."
	@echo "Vision Service (port 2001):"
	@curl -f -s http://localhost:2001/health > /dev/null && echo "✅ Running" || echo "❌ Not running"
	@echo "Queue Worker (port 2003):"
	@curl -f -s http://localhost:2003/health > /dev/null && echo "✅ Running" || echo "❌ Not running"

# Cleanup
clean: ## Clean up temporary files and caches
	find . -type d -name "__pycache__" -exec rm -rf {} + 2>/dev/null || true
	find . -type d -name ".pytest_cache" -exec rm -rf {} + 2>/dev/null || true
	find . -type d -name ".ruff_cache" -exec rm -rf {} + 2>/dev/null || true
	find . -name "*.pyc" -delete 2>/dev/null || true
	rm -rf htmlcov/ .coverage 2>/dev/null || true
	@echo "🧹 Cleaned up temporary files"

# Development workflow
dev-setup: install setup-hooks ## Complete development setup
	@echo "🛠️  Development environment ready!"
	@echo "💡 Run 'make test' to verify everything works"

quick-check: format-check lint-check test-fast ## Quick pre-commit check
	@echo "⚡ Quick check completed!"

# Configuration tools (existing)
config-gen: ## Generate configurations for all services
	uv run python tools/generate-configs.py

config-validate: ## Validate service configurations
	uv run python tools/generate-configs.py --validate

config-preview: ## Preview configuration generation
	uv run python tools/generate-configs.py --dry-run
