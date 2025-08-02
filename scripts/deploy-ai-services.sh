#!/bin/bash

# AI Services Deployment Script for CI/CD
# Deploys AI services with team10 configuration

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Configuration
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
AI_SERVICES_DIR="$PROJECT_ROOT/ai-services/deployment"

# Default environment variables
export VISION_SERVICE_PORT="${VISION_SERVICE_PORT:-2001}"
export QUEUE_WORKER_PORT="${QUEUE_WORKER_PORT:-2003}"
export VISION_LB_PORT="${VISION_LB_PORT:-2011}"
export REDIS_PORT="${REDIS_PORT:-6379}"
export PROMETHEUS_PORT="${PROMETHEUS_PORT:-9090}"
export GRAFANA_PORT="${GRAFANA_PORT:-3001}"
export BUILD_TARGET="${BUILD_TARGET:-production}"

# Functions
check_requirements() {
    log "Checking deployment requirements..."

    if ! command -v docker &> /dev/null; then
        error "Docker is not installed or not in PATH"
    fi

    if ! command -v docker-compose &> /dev/null; then
        error "Docker Compose is not installed or not in PATH"
    fi

    if ! docker info &> /dev/null; then
        error "Docker daemon is not running"
    fi

    success "All requirements satisfied"
}

build_services() {
    log "Building AI services..."

    cd "$AI_SERVICES_DIR"

    # Build with docker-compose
    docker-compose build --no-cache

    success "AI services built successfully"
}

deploy_services() {
    log "Deploying AI services..."

    cd "$AI_SERVICES_DIR"

    # Stop existing services
    log "Stopping existing services..."
    docker-compose down --remove-orphans || true

    # Start services
    log "Starting AI services..."
    docker-compose up -d

    success "AI services deployed successfully"
}

wait_for_services() {
    log "Waiting for services to be ready..."

    local max_attempts=30
    local attempt=1

    while [[ $attempt -le $max_attempts ]]; do
        log "Health check attempt $attempt/$max_attempts"

        # Check Vision Service
        if curl -f "http://localhost:$VISION_SERVICE_PORT/health" &> /dev/null; then
            success "Vision Service is healthy"
            break
        fi

        if [[ $attempt -eq $max_attempts ]]; then
            error "Vision Service failed to become healthy after $max_attempts attempts"
        fi

        sleep 10
        ((attempt++))
    done

    # Check Queue Worker
    attempt=1
    while [[ $attempt -le $max_attempts ]]; do
        log "Queue Worker health check attempt $attempt/$max_attempts"

        if curl -f "http://localhost:$QUEUE_WORKER_PORT/health" &> /dev/null; then
            success "Queue Worker is healthy"
            break
        fi

        if [[ $attempt -eq $max_attempts ]]; then
            error "Queue Worker failed to become healthy after $max_attempts attempts"
        fi

        sleep 10
        ((attempt++))
    done
}

show_status() {
    log "Deployment Status:"
    echo

    log "Running Containers:"
    docker ps --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}" | grep team10 || warn "No team10 containers found"
    echo

    log "Team10 Volumes:"
    docker volume ls | grep team10 || warn "No team10 volumes found"
    echo

    log "Service URLs:"
    echo "  • Vision Service:    http://localhost:$VISION_SERVICE_PORT"
    echo "  • Queue Worker:      http://localhost:$QUEUE_WORKER_PORT"
    echo "  • Vision LB:         http://localhost:$VISION_LB_PORT"
    echo "  • Prometheus:        http://localhost:$PROMETHEUS_PORT"
    echo "  • Grafana:           http://localhost:$GRAFANA_PORT"
    echo "  • Redis:             localhost:$REDIS_PORT"
}

cleanup_services() {
    log "Cleaning up AI services..."

    cd "$AI_SERVICES_DIR"

    # Stop and remove containers, networks, and volumes
    docker-compose down --volumes --remove-orphans

    # Clean up Docker system
    docker system prune -f

    success "Cleanup completed"
}

# Main deployment function
main() {
    local action="${1:-deploy}"

    case "$action" in
        "deploy")
            log "Starting AI services deployment..."
            check_requirements
            build_services
            deploy_services
            wait_for_services
            show_status
            success "AI services deployment completed successfully!"
            ;;
        "cleanup")
            cleanup_services
            ;;
        "status")
            show_status
            ;;
        "health")
            log "Performing health checks..."
            curl -f "http://localhost:$VISION_SERVICE_PORT/health" || error "Vision Service health check failed"
            curl -f "http://localhost:$QUEUE_WORKER_PORT/health" || error "Queue Worker health check failed"
            success "All services are healthy!"
            ;;
        *)
            echo "Usage: $0 {deploy|cleanup|status|health}"
            echo
            echo "Commands:"
            echo "  deploy  - Build and deploy AI services (default)"
            echo "  cleanup - Stop services and clean up resources"
            echo "  status  - Show deployment status"
            echo "  health  - Perform health checks"
            exit 1
            ;;
    esac
}

# Run main function with all arguments
main "$@"
