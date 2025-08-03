# AI4Thai Vision Service - Development Makefile
.PHONY: help install dev-install lint format test run clean check

# Default target
help:
	@echo "AI4Thai Vision Service - Available commands:"
	@echo ""
	@echo "Environment Setup:"
	@echo "  install      Install production dependencies using uv"
	@echo "  dev-install  Install development dependencies using uv"
	@echo ""
	@echo "Code Quality:"
	@echo "  lint         Run ruff linter"
	@echo "  format       Format code with ruff"
	@echo "  check        Run both linting and formatting checks"
	@echo ""
	@echo "Testing:"
	@echo "  test         Run pytest with coverage"
	@echo "  test-fast    Run pytest without coverage"
	@echo ""
	@echo "Development:"
	@echo "  run          Start the vision service"
	@echo "  dev          Start service in development mode"
	@echo ""
	@echo "Maintenance:"
	@echo "  clean        Clean up cache and temporary files"

# Environment setup
install:
	@echo "Installing production dependencies with uv..."
	uv sync --frozen

dev-install:
	@echo "Installing development dependencies with uv..."
	uv sync --all-extras --dev

# Code quality
lint:
	@echo "Running ruff linter..."
	uv run ruff check .

format:
	@echo "Formatting code with ruff..."
	uv run ruff format .

check: lint
	@echo "Running all code quality checks..."
	uv run ruff check --diff .
	uv run ruff format --check .

# Testing
test:
	@echo "Running tests with coverage..."
	uv run pytest

test-fast:
	@echo "Running tests without coverage..."
	uv run pytest --no-cov

# Development
run:
	@echo "Starting AI4Thai Vision Service..."
	uv run python app.py

dev:
	@echo "Starting Vision Service in development mode..."
	uv run uvicorn app:app --host 0.0.0.0 --port 2001 --reload

# Maintenance
clean:
	@echo "Cleaning up cache and temporary files..."
	find . -type d -name "__pycache__" -exec rm -rf {} + 2>/dev/null || true
	find . -type d -name ".pytest_cache" -exec rm -rf {} + 2>/dev/null || true
	find . -type d -name ".ruff_cache" -exec rm -rf {} + 2>/dev/null || true
	find . -type d -name "htmlcov" -exec rm -rf {} + 2>/dev/null || true
	find . -name "*.pyc" -delete 2>/dev/null || true
	find . -name ".coverage" -delete 2>/dev/null || true

# Quick development setup
setup: dev-install
	@echo "Development environment setup complete!"
	@echo "Run 'make dev' to start the service in development mode"

# CI/CD targets
ci-install:
	uv sync --frozen

ci-check: check test

# Docker-related (for reference)
docker-build:
	docker build -t ai4thai-vision-service .

docker-run:
	docker run -p 2001:2001 ai4thai-vision-service
