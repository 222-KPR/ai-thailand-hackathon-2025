#!/bin/bash

# AI4Thai Crop Guardian - Simple Development Stop Script
# Stops the main Rust components (API Gateway + Redis)

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

log "Stopping AI4Thai Crop Guardian development environment..."

# Stop API Gateway
log "Stopping API Gateway..."
if pgrep -f "target.*api-gateway" > /dev/null; then
    pkill -f "target.*api-gateway"
    success "API Gateway stopped"
else
    warn "API Gateway was not running"
fi

# Stop Redis
log "Stopping Redis..."
if docker ps --format "table {{.Names}}" | grep -q "team10-redis"; then
    docker-compose stop redis
    success "Redis stopped"
else
    warn "Redis was not running"
fi

echo
success "🛑 Development environment stopped!"
echo
echo "To start again: ./start-dev.sh"
echo
