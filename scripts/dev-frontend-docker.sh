#!/bin/bash

# AI4Thai Crop Guardian - Frontend Docker Development Script
# This script manages Docker-based frontend development

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    if ! command_exists docker; then
        print_error "Docker not found. Please install Docker."
        exit 1
    fi
    
    if ! command_exists docker-compose; then
        print_error "Docker Compose not found. Please install Docker Compose."
        exit 1
    fi
    
    # Check if Docker daemon is running
    if ! docker info >/dev/null 2>&1; then
        print_error "Docker daemon is not running. Please start Docker."
        exit 1
    fi
    
    print_success "Prerequisites check passed"
}

# Function to show usage
show_usage() {
    echo "Usage: $0 [COMMAND] [OPTIONS]"
    echo ""
    echo "Commands:"
    echo "  start           Start frontend development server"
    echo "  start-fullstack Start frontend with API Gateway and database"
    echo "  stop            Stop all services"
    echo "  restart         Restart frontend service"
    echo "  logs            Show frontend logs"
    echo "  shell           Open shell in frontend container"
    echo "  build           Build frontend Docker image"
    echo "  test            Run frontend tests in container"
    echo "  clean           Clean up containers and volumes"
    echo "  status          Show service status"
    echo ""
    echo "Options:"
    echo "  -d, --detach    Run in detached mode"
    echo "  -f, --follow    Follow logs output"
    echo "  --no-cache      Build without cache"
    echo ""
    echo "Examples:"
    echo "  $0 start                    # Start frontend only"
    echo "  $0 start-fullstack         # Start with backend services"
    echo "  $0 logs -f                 # Follow logs"
    echo "  $0 build --no-cache        # Rebuild without cache"
}

# Navigate to frontend directory
cd "$(dirname "$0")/../frontend" || {
    print_error "Frontend directory not found"
    exit 1
}

# Parse command line arguments
COMMAND=""
DETACH=""
FOLLOW=""
NO_CACHE=""

while [[ $# -gt 0 ]]; do
    case $1 in
        start|start-fullstack|stop|restart|logs|shell|build|test|clean|status)
            COMMAND="$1"
            shift
            ;;
        -d|--detach)
            DETACH="-d"
            shift
            ;;
        -f|--follow)
            FOLLOW="-f"
            shift
            ;;
        --no-cache)
            NO_CACHE="--no-cache"
            shift
            ;;
        -h|--help)
            show_usage
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Check if command is provided
if [ -z "$COMMAND" ]; then
    print_error "No command provided"
    show_usage
    exit 1
fi

# Check prerequisites
check_prerequisites

# Execute commands
case $COMMAND in
    start)
        print_status "Starting frontend development server..."
        docker-compose -f docker-compose.dev.yml up $DETACH frontend-dev
        if [ -z "$DETACH" ]; then
            print_success "Frontend development server started"
            print_status "Access the application at: http://localhost:8080"
        fi
        ;;
        
    start-fullstack)
        print_status "Starting full-stack development environment..."
        docker-compose -f docker-compose.dev.yml --profile fullstack up $DETACH
        if [ -z "$DETACH" ]; then
            print_success "Full-stack development environment started"
            print_status "Frontend: http://localhost:8080"
            print_status "API Gateway: http://localhost:3000"
            print_status "Database: localhost:5432"
            print_status "Redis: localhost:6379"
        fi
        ;;
        
    stop)
        print_status "Stopping all services..."
        docker-compose -f docker-compose.dev.yml --profile fullstack down
        print_success "All services stopped"
        ;;
        
    restart)
        print_status "Restarting frontend service..."
        docker-compose -f docker-compose.dev.yml restart frontend-dev
        print_success "Frontend service restarted"
        ;;
        
    logs)
        print_status "Showing frontend logs..."
        docker-compose -f docker-compose.dev.yml logs $FOLLOW frontend-dev
        ;;
        
    shell)
        print_status "Opening shell in frontend container..."
        docker-compose -f docker-compose.dev.yml exec frontend-dev /bin/bash
        ;;
        
    build)
        print_status "Building frontend Docker image..."
        docker-compose -f docker-compose.dev.yml build $NO_CACHE frontend-dev
        print_success "Frontend Docker image built"
        ;;
        
    test)
        print_status "Running frontend tests in container..."
        docker-compose -f docker-compose.dev.yml exec frontend-dev wasm-pack test --headless --firefox
        ;;
        
    clean)
        print_status "Cleaning up containers and volumes..."
        docker-compose -f docker-compose.dev.yml --profile fullstack down -v --remove-orphans
        docker system prune -f
        print_success "Cleanup completed"
        ;;
        
    status)
        print_status "Service status:"
        docker-compose -f docker-compose.dev.yml ps
        echo ""
        print_status "Container resource usage:"
        docker stats --no-stream --format "table {{.Container}}\t{{.CPUPerc}}\t{{.MemUsage}}\t{{.NetIO}}"
        ;;
        
    *)
        print_error "Unknown command: $COMMAND"
        show_usage
        exit 1
        ;;
esac
