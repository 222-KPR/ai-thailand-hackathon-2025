#!/bin/bash

# AI4Thai AI Services - Development Environment Setup
# This script sets up the development environment using uv

set -e  # Exit on any error

echo "🚀 Setting up AI4Thai AI Services development environment..."

# Check if uv is installed
if ! command -v uv &> /dev/null; then
    echo "❌ uv is not installed. Installing uv..."
    curl -LsSf https://astral.sh/uv/install.sh | sh
    source "$HOME/.cargo/env"
else
    echo "✅ uv is already installed"
fi

# Function to setup a service
setup_service() {
    local service_name=$1
    local service_dir=$2

    echo ""
    echo "📦 Setting up $service_name..."
    cd "$service_dir"

    # Check if pyproject.toml exists
    if [ ! -f "pyproject.toml" ]; then
        echo "❌ pyproject.toml not found in $service_dir"
        return 1
    fi

    # Install development dependencies
    echo "   Installing dependencies with uv..."
    uv sync --all-extras --dev

    echo "   ✅ $service_name setup complete"
    cd - > /dev/null
}

# Setup Vision Service
if [ -d "vision-service" ]; then
    setup_service "Vision Service" "vision-service"
else
    echo "❌ vision-service directory not found"
fi

# Setup Queue Worker
if [ -d "queue-worker" ]; then
    setup_service "Queue Worker" "queue-worker"
else
    echo "❌ queue-worker directory not found"
fi

echo ""
echo "🎉 Development environment setup complete!"
echo ""
echo "📝 Available commands:"
echo ""
echo "Vision Service:"
echo "  cd vision-service"
echo "  make help          # Show available commands"
echo "  make dev           # Start development server"
echo "  make lint          # Run linting"
echo "  make test          # Run tests"
echo ""
echo "Queue Worker:"
echo "  cd queue-worker"
echo "  make help          # Show available commands"
echo "  make dev           # Start development server"
echo "  make worker        # Start Celery worker"
echo "  make lint          # Run linting"
echo ""
echo "🔧 Development workflow:"
echo "1. Start Redis: docker run -d -p 6379:6379 redis:alpine"
echo "2. Start Vision Service: cd vision-service && make dev"
echo "3. Start Queue Worker: cd queue-worker && make dev"
echo "4. Start Celery Worker: cd queue-worker && make worker"
echo ""
echo "🧹 Code quality:"
echo "  make check         # Run linting and formatting checks"
echo "  make format        # Auto-format code with ruff"
echo ""
