#!/bin/bash

# AI4Thai Crop Guardian - Simple Development Start Script
# Starts the main Rust components (API Gateway + Redis)

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1"
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

# Create logs directory
mkdir -p logs

log "Starting AI4Thai Crop Guardian development environment..."

# Start Redis
log "Starting Redis..."
if docker ps --format "table {{.Names}}" | grep -q "team10-redis"; then
    warn "Redis container already running"
else
    docker-compose up -d redis
    sleep 3
fi

# Check Redis health
if docker exec team10-redis redis-cli ping &> /dev/null; then
    success "Redis is ready"
else
    error "Redis failed to start"
    exit 1
fi

# Start API Gateway
log "Starting API Gateway..."
if pgrep -f "target.*api-gateway" > /dev/null; then
    warn "API Gateway already running"
else
    cd api-gateway
    APP_ENV=development RUST_LOG=info cargo run > ../logs/api-gateway.log 2>&1 &
    cd ..
    sleep 5
fi

# Check API Gateway health
if curl -s http://localhost:3000/health > /dev/null 2>&1; then
    success "API Gateway is ready"
else
    error "API Gateway failed to start"
    exit 1
fi

echo
success "🚀 Development environment is ready!"
echo
echo "Services running:"
echo "  • Redis:        localhost:6379"
echo "  • API Gateway:  http://localhost:3000"
echo
echo "Health checks:"
echo "  • Health:       curl http://localhost:3000/health"
echo "  • Ready:        curl http://localhost:3000/health/ready"
echo "  • Metrics:      curl http://localhost:3000/health/metrics"
echo
echo "Chat API:"
echo "  • Endpoint:     POST http://localhost:3000/api/v1/chat"
echo "  • Test:         curl -X POST http://localhost:3000/api/v1/chat -H 'Content-Type: application/json' -d '{\"message\": \"Hello\"}'"
echo
echo "Logs:"
echo "  • API Gateway:  tail -f logs/api-gateway.log"
echo
echo "To stop: ./stop-dev.sh"
echo
