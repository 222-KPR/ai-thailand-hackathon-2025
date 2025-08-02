#!/bin/bash

# AI4Thai Crop Guardian - Development Stop Script
# Stops main application services (API Gateway + Frontend + Redis)

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Logging function
log() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Stop API Gateway
stop_api_gateway() {
    log "Stopping API Gateway..."

    # Stop by PID file if exists
    if [[ -f "$PROJECT_ROOT/logs/api-gateway.pid" ]]; then
        local api_pid=$(cat "$PROJECT_ROOT/logs/api-gateway.pid")
        if kill -0 "$api_pid" 2>/dev/null; then
            kill "$api_pid"
            success "API Gateway stopped (PID: $api_pid)"
        else
            warn "API Gateway process not found (PID: $api_pid)"
        fi
        rm -f "$PROJECT_ROOT/logs/api-gateway.pid"
    fi

    # Kill any remaining processes
    if pgrep -f "target.*api-gateway" > /dev/null; then
        pkill -f "target.*api-gateway"
        success "Stopped remaining API Gateway processes"
    fi
}

# Stop Frontend
stop_frontend() {
    log "Stopping Frontend development server..."

    # Stop by PID file if exists
    if [[ -f "$PROJECT_ROOT/logs/frontend.pid" ]]; then
        local frontend_pid=$(cat "$PROJECT_ROOT/logs/frontend.pid")
        if kill -0 "$frontend_pid" 2>/dev/null; then
            kill "$frontend_pid"
            success "Frontend development server stopped (PID: $frontend_pid)"
        else
            warn "Frontend process not found (PID: $frontend_pid)"
        fi
        rm -f "$PROJECT_ROOT/logs/frontend.pid"
    fi

    # Kill any remaining trunk serve processes
    if pgrep -f "trunk serve" > /dev/null; then
        pkill -f "trunk serve"
        success "Stopped remaining Frontend processes"
    fi
}

# Stop Redis
stop_redis() {
    log "Stopping Redis..."

    cd "$PROJECT_ROOT"

    if docker ps --format "table {{.Names}}" | grep -q "team10-redis"; then
        docker-compose stop redis
        success "Redis container stopped"
    else
        warn "Redis container not running"
    fi
}

# Clean up logs and temporary files
cleanup_files() {
    log "Cleaning up temporary files..."

    # Remove PID files
    rm -f "$PROJECT_ROOT/logs/api-gateway.pid"
    rm -f "$PROJECT_ROOT/logs/frontend.pid"

    # Optionally clean up log files (commented out to preserve logs)
    # rm -f "$PROJECT_ROOT/logs/api-gateway.log"
    # rm -f "$PROJECT_ROOT/logs/frontend.log"

    success "Cleanup completed"
}

# Display status
show_status() {
    echo
    success "Development services stopped!"
    echo
    echo "Stopped services:"
    echo "  • API Gateway"
    echo "  • Frontend development server"
    echo "  • Redis container"
    echo
    echo "Log files preserved:"
    echo "  • API Gateway:  $PROJECT_ROOT/logs/api-gateway.log"
    echo "  • Frontend:     $PROJECT_ROOT/logs/frontend.log"
    echo
    echo "To restart services, run: ./scripts/dev-start.sh"
    echo
}

# Main execution
main() {
    log "Stopping AI4Thai Crop Guardian development services..."

    stop_api_gateway
    stop_frontend
    stop_redis
    cleanup_files
    show_status

    success "All development services stopped successfully"
}

# Run main function
main "$@"
