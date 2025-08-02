#!/bin/bash

# AI4Thai Crop Guardian - Frontend Docker Management Script
# Manages Docker-based frontend development and production builds

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

usage() {
    echo "Usage: $0 {dev|prod|build|stop|logs|clean}"
    echo
    echo "Commands:"
    echo "  dev     - Start development frontend with hot reload"
    echo "  prod    - Start production frontend build"
    echo "  build   - Build frontend Docker images"
    echo "  stop    - Stop all frontend containers"
    echo "  logs    - Show frontend container logs"
    echo "  clean   - Clean up frontend containers and images"
    echo
    exit 1
}

ensure_network() {
    if ! docker network ls | grep -q "ai4thai-network"; then
        log "Creating Docker network..."
        docker network create ai4thai-network
    fi
}

dev() {
    log "Starting frontend development environment..."
    ensure_network

    # Ensure Redis is running
    if ! docker ps --format "table {{.Names}}" | grep -q "team10-redis"; then
        log "Starting Redis dependency..."
        docker-compose up -d redis
        sleep 3
    fi

    # Start frontend development
    docker-compose -f docker-compose.frontend.yml up -d frontend-dev

    # Wait for container to be ready
    log "Waiting for frontend to be ready..."
    sleep 10

    if curl -s http://localhost:8080 > /dev/null 2>&1; then
        success "Frontend development server is ready!"
        echo
        echo "🚀 Frontend Development Environment:"
        echo "  • URL:          http://localhost:8080"
        echo "  • Hot Reload:   Enabled"
        echo "  • Logs:         docker logs -f ai4thai-frontend-dev"
        echo "  • Stop:         ./frontend-docker.sh stop"
        echo
    else
        error "Frontend failed to start"
        docker logs ai4thai-frontend-dev
        exit 1
    fi
}

prod() {
    log "Starting frontend production environment..."
    ensure_network

    docker-compose -f docker-compose.frontend.yml --profile production up -d frontend-prod

    log "Waiting for production frontend to be ready..."
    sleep 15

    if curl -s http://localhost:8081 > /dev/null 2>&1; then
        success "Frontend production server is ready!"
        echo
        echo "🚀 Frontend Production Environment:"
        echo "  • URL:          http://localhost:8081"
        echo "  • Optimized:    Yes"
        echo "  • Logs:         docker logs -f ai4thai-frontend-prod"
        echo
    else
        error "Production frontend failed to start"
        docker logs ai4thai-frontend-prod
        exit 1
    fi
}

build() {
    log "Building frontend Docker images..."
    docker-compose -f docker-compose.frontend.yml build --no-cache
    success "Frontend images built successfully!"
}

stop() {
    log "Stopping frontend containers..."
    docker-compose -f docker-compose.frontend.yml down
    success "Frontend containers stopped!"
}

logs() {
    if docker ps --format "table {{.Names}}" | grep -q "ai4thai-frontend-dev"; then
        docker logs -f ai4thai-frontend-dev
    elif docker ps --format "table {{.Names}}" | grep -q "ai4thai-frontend-prod"; then
        docker logs -f ai4thai-frontend-prod
    else
        warn "No frontend containers are running"
    fi
}

clean() {
    log "Cleaning up frontend containers and images..."
    docker-compose -f docker-compose.frontend.yml down --rmi all --volumes --remove-orphans
    success "Frontend cleanup completed!"
}

# Main command handling
case "${1:-}" in
    dev)
        dev
        ;;
    prod)
        prod
        ;;
    build)
        build
        ;;
    stop)
        stop
        ;;
    logs)
        logs
        ;;
    clean)
        clean
        ;;
    *)
        usage
        ;;
esac
