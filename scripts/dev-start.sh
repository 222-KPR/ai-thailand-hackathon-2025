#!/bin/bash

# AI4Thai Crop Guardian - Development Start Script
# Starts main application services (API Gateway + Frontend)
# AI services are deployed separately or use external APIs

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
ENV_FILE="$PROJECT_ROOT/.env"

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

# Check if .env file exists
check_env_file() {
    if [[ ! -f "$ENV_FILE" ]]; then
        error ".env file not found. Please copy .env.example to .env and configure it."
        exit 1
    fi
    success "Environment file found"
}

# Check if required tools are installed
check_dependencies() {
    log "Checking dependencies..."
    
    local missing_deps=()
    
    # Check Rust and Cargo
    if ! command -v cargo &> /dev/null; then
        missing_deps+=("cargo (Rust)")
    fi
    
    # Check trunk for frontend
    if ! command -v trunk &> /dev/null; then
        missing_deps+=("trunk (install with: cargo install trunk)")
    fi
    
    # Check Docker for Redis
    if ! command -v docker &> /dev/null; then
        missing_deps+=("docker")
    fi
    
    if [[ ${#missing_deps[@]} -gt 0 ]]; then
        error "Missing dependencies: ${missing_deps[*]}"
        error "Please install missing dependencies or run ./scripts/setup-dev.sh"
        exit 1
    fi
    
    success "All dependencies found"
}

# Start Redis for chat storage
start_redis() {
    log "Starting Redis for chat storage..."
    
    if docker ps --format "table {{.Names}}" | grep -q "team10-redis"; then
        warn "Redis container already running"
        return 0
    fi
    
    # Start Redis using docker-compose
    cd "$PROJECT_ROOT"
    docker-compose up -d redis
    
    # Wait for Redis to be ready
    local max_attempts=30
    local attempt=1
    
    while [[ $attempt -le $max_attempts ]]; do
        if docker exec team10-redis redis-cli ping &> /dev/null; then
            success "Redis is ready"
            return 0
        fi
        
        log "Waiting for Redis... (attempt $attempt/$max_attempts)"
        sleep 2
        ((attempt++))
    done
    
    error "Redis failed to start within timeout"
    exit 1
}

# Build API Gateway
build_api_gateway() {
    log "Building API Gateway..."
    cd "$PROJECT_ROOT/api-gateway"
    
    if cargo build; then
        success "API Gateway built successfully"
    else
        error "Failed to build API Gateway"
        exit 1
    fi
}

# Build Frontend
build_frontend() {
    log "Building Frontend..."
    cd "$PROJECT_ROOT/frontend"
    
    # Check if wasm32-unknown-unknown target is installed
    if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
        log "Installing wasm32-unknown-unknown target..."
        rustup target add wasm32-unknown-unknown
    fi
    
    if trunk build; then
        success "Frontend built successfully"
    else
        error "Failed to build Frontend"
        exit 1
    fi
}

# Start API Gateway in background
start_api_gateway() {
    log "Starting API Gateway..."
    cd "$PROJECT_ROOT/api-gateway"
    
    # Kill existing process if running
    if pgrep -f "target.*api-gateway" > /dev/null; then
        warn "Stopping existing API Gateway process..."
        pkill -f "target.*api-gateway" || true
        sleep 2
    fi
    
    # Start API Gateway
    RUST_LOG=info cargo run > "$PROJECT_ROOT/logs/api-gateway.log" 2>&1 &
    local api_pid=$!
    
    # Wait for API Gateway to be ready
    local max_attempts=30
    local attempt=1
    
    while [[ $attempt -le $max_attempts ]]; do
        if curl -s http://localhost:3000/health > /dev/null 2>&1; then
            success "API Gateway is ready (PID: $api_pid)"
            echo "$api_pid" > "$PROJECT_ROOT/logs/api-gateway.pid"
            return 0
        fi
        
        log "Waiting for API Gateway... (attempt $attempt/$max_attempts)"
        sleep 2
        ((attempt++))
    done
    
    error "API Gateway failed to start within timeout"
    exit 1
}

# Start Frontend development server
start_frontend() {
    log "Starting Frontend development server..."
    cd "$PROJECT_ROOT/frontend"
    
    # Kill existing trunk serve process if running
    if pgrep -f "trunk serve" > /dev/null; then
        warn "Stopping existing Frontend development server..."
        pkill -f "trunk serve" || true
        sleep 2
    fi
    
    # Start Frontend development server
    trunk serve --port 8080 --open > "$PROJECT_ROOT/logs/frontend.log" 2>&1 &
    local frontend_pid=$!
    
    success "Frontend development server started (PID: $frontend_pid)"
    echo "$frontend_pid" > "$PROJECT_ROOT/logs/frontend.pid"
    
    # Wait a bit for the server to start
    sleep 3
    
    if curl -s http://localhost:8080 > /dev/null 2>&1; then
        success "Frontend is ready at http://localhost:8080"
    else
        warn "Frontend may still be starting up..."
    fi
}

# Create logs directory
setup_logging() {
    mkdir -p "$PROJECT_ROOT/logs"
    log "Logging directory created"
}

# Check AI services configuration
check_ai_services() {
    log "Checking AI services configuration..."
    
    # Source environment variables
    if [[ -f "$ENV_FILE" ]]; then
        set -a
        source "$ENV_FILE"
        set +a
    fi
    
    if [[ -n "${AI4THAI_API_KEY:-}" ]]; then
        success "Using external AI4Thai API services"
        log "Vision Service: ${VISION_SERVICE_URL:-https://vision-api.ai4thai.com}"
    elif [[ -n "${VISION_SERVICE_URL:-}" ]]; then
        success "Using self-hosted AI services"
        log "Vision Service: $VISION_SERVICE_URL"
        log "Queue Worker: ${QUEUE_WORKER_URL:-http://localhost:2003}"
        warn "Make sure AI services are running separately!"
        warn "Start them with: cd ai-services/deployment && docker-compose up -d"
    else
        warn "No AI services configured. Please set either:"
        warn "  - AI4THAI_API_KEY for external services, or"
        warn "  - VISION_SERVICE_URL for self-hosted services"
    fi
}

# Display running services
show_services() {
    echo
    success "Main application started successfully!"
    echo
    echo "Services running:"
    echo "  • Frontend:     http://localhost:8080"
    echo "  • API Gateway:  http://localhost:3000"
    echo "  • Redis:        localhost:6379"
    echo
    echo "AI Services (external):"
    if [[ -n "${AI4THAI_API_KEY:-}" ]]; then
        echo "  • Using AI4Thai external API services"
    elif [[ -n "${VISION_SERVICE_URL:-}" ]]; then
        echo "  • Vision Service: ${VISION_SERVICE_URL:-Not configured}"
        echo "  • Queue Worker:   ${QUEUE_WORKER_URL:-Not configured}"
        echo "  • Note: Start AI services separately if self-hosting"
    else
        echo "  • Not configured - please check .env file"
    fi
    echo
    echo "Logs:"
    echo "  • API Gateway:  $PROJECT_ROOT/logs/api-gateway.log"
    echo "  • Frontend:     $PROJECT_ROOT/logs/frontend.log"
    echo
    echo "Commands:"
    echo "  • Stop services:     ./scripts/dev-stop.sh"
    echo "  • View API logs:     tail -f logs/api-gateway.log"
    echo "  • View frontend logs: tail -f logs/frontend.log"
    echo "  • Start AI services: cd ai-services/deployment && docker-compose up -d"
    echo
}

# Cleanup function
cleanup() {
    log "Cleaning up..."
    
    # Kill background processes
    if [[ -f "$PROJECT_ROOT/logs/api-gateway.pid" ]]; then
        local api_pid=$(cat "$PROJECT_ROOT/logs/api-gateway.pid")
        if kill -0 "$api_pid" 2>/dev/null; then
            kill "$api_pid" || true
        fi
        rm -f "$PROJECT_ROOT/logs/api-gateway.pid"
    fi
    
    if [[ -f "$PROJECT_ROOT/logs/frontend.pid" ]]; then
        local frontend_pid=$(cat "$PROJECT_ROOT/logs/frontend.pid")
        if kill -0 "$frontend_pid" 2>/dev/null; then
            kill "$frontend_pid" || true
        fi
        rm -f "$PROJECT_ROOT/logs/frontend.pid"
    fi
}

# Set up signal handlers
trap cleanup EXIT INT TERM

# Main execution
main() {
    log "Starting AI4Thai Crop Guardian main application..."
    
    check_env_file
    check_dependencies
    setup_logging
    check_ai_services
    start_redis
    build_api_gateway
    build_frontend
    start_api_gateway
    start_frontend
    show_services
    
    # Keep script running
    log "Main application is running. Press Ctrl+C to stop."
    while true; do
        sleep 10
        
        # Check if services are still running
        if ! curl -s http://localhost:3000/health > /dev/null 2>&1; then
            error "API Gateway appears to be down"
            exit 1
        fi
        
        if ! curl -s http://localhost:8080 > /dev/null 2>&1; then
            warn "Frontend appears to be down"
        fi
    done
}

# Run main function
main "$@"
