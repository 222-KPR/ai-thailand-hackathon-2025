#!/bin/bash

# AI4Thai Crop Guardian - AI Services Deployment Script
# Deploy Vision and LLM services separately using HuggingFace models

set -e

echo "🤖 AI4Thai AI Services Deployment"
echo "=================================="

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

# Configuration
AI_SERVICES_DIR="ai-services/deployment"
DEPLOYMENT_MODE=${1:-"basic"}  # basic, full, gpu, monitoring

# Check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    # Check Docker
    if ! command -v docker &> /dev/null; then
        print_error "Docker is not installed. Please install Docker first."
        exit 1
    fi
    print_success "Docker found: $(docker --version)"
    
    # Check Docker Compose
    if ! command -v docker-compose &> /dev/null; then
        print_error "Docker Compose is not installed. Please install Docker Compose first."
        exit 1
    fi
    print_success "Docker Compose found: $(docker-compose --version)"
    
    # Check if Docker daemon is running
    if ! docker info &> /dev/null; then
        print_error "Docker daemon is not running. Please start Docker first."
        exit 1
    fi
    print_success "Docker daemon is running"
    
    # Check for NVIDIA Docker (if GPU mode)
    if [[ "$DEPLOYMENT_MODE" == "gpu" ]]; then
        if ! command -v nvidia-docker &> /dev/null && ! docker run --rm --gpus all nvidia/cuda:11.8-base-ubuntu22.04 nvidia-smi &> /dev/null; then
            print_warning "NVIDIA Docker not found. GPU acceleration may not work."
        else
            print_success "NVIDIA Docker support detected"
        fi
    fi
}

# Setup environment
setup_environment() {
    print_status "Setting up environment..."
    
    cd "$AI_SERVICES_DIR"
    
    if [ ! -f ".env" ]; then
        if [ -f ".env.example" ]; then
            cp .env.example .env
            print_success "Environment file created from template"
            print_warning "Please edit .env file with your configuration:"
            print_warning "- HUGGINGFACE_HUB_TOKEN (required for model downloads)"
            print_warning "- OPENAI_API_KEY (optional fallback)"
            print_warning "- CUDA_VISIBLE_DEVICES (for GPU selection)"
        else
            print_error "No .env.example found"
            exit 1
        fi
    else
        print_success "Environment file already exists"
    fi
    
    # Check if HuggingFace token is set
    if grep -q "HUGGINGFACE_HUB_TOKEN=your_huggingface_token_here" .env; then
        print_warning "Please update HUGGINGFACE_HUB_TOKEN in .env file"
        print_warning "Get your token from: https://huggingface.co/settings/tokens"
    fi
}

# Download models
download_models() {
    print_status "Downloading AI models..."
    
    # Run model downloader
    docker-compose --profile init up model-downloader
    
    if [ $? -eq 0 ]; then
        print_success "Models downloaded successfully"
    else
        print_warning "Model download failed or skipped"
        print_status "Models will be downloaded on first service startup"
    fi
}

# Deploy services based on mode
deploy_services() {
    print_status "Deploying AI services in $DEPLOYMENT_MODE mode..."
    
    case $DEPLOYMENT_MODE in
        "basic")
            print_status "Starting basic AI services..."
            docker-compose up -d vision-service llm-service redis
            ;;
        "full")
            print_status "Starting full AI services with load balancers..."
            docker-compose --profile load-balancer up -d
            ;;
        "gpu")
            print_status "Starting GPU-optimized AI services..."
            export BUILD_TARGET=gpu-production
            docker-compose up -d vision-service llm-service redis
            ;;
        "monitoring")
            print_status "Starting AI services with monitoring..."
            docker-compose --profile monitoring up -d
            ;;
        "all")
            print_status "Starting all AI services with full features..."
            docker-compose --profile load-balancer --profile monitoring up -d
            ;;
        *)
            print_error "Unknown deployment mode: $DEPLOYMENT_MODE"
            print_status "Available modes: basic, full, gpu, monitoring, all"
            exit 1
            ;;
    esac
    
    # Wait for services to be ready
    print_status "Waiting for services to be ready..."
    sleep 15
}

# Check service health
check_service_health() {
    print_status "Checking service health..."
    
    # Check if containers are running
    if docker-compose ps | grep -q "Up"; then
        print_success "Services are running"
    else
        print_error "Some services failed to start"
        docker-compose ps
        exit 1
    fi
    
    # Check Vision Service health
    print_status "Checking Vision Service health..."
    for i in {1..30}; do
        if curl -f http://localhost:8001/health &> /dev/null; then
            print_success "Vision Service is healthy"
            break
        fi
        if [ $i -eq 30 ]; then
            print_error "Vision Service health check failed"
            print_status "Check logs with: docker-compose logs vision-service"
        fi
        sleep 2
    done
    
    # Check LLM Service health
    print_status "Checking LLM Service health..."
    for i in {1..30}; do
        if curl -f http://localhost:8002/health &> /dev/null; then
            print_success "LLM Service is healthy"
            break
        fi
        if [ $i -eq 30 ]; then
            print_error "LLM Service health check failed"
            print_status "Check logs with: docker-compose logs llm-service"
        fi
        sleep 2
    done
}

# Display service information
show_service_info() {
    echo ""
    print_success "🎉 AI4Thai AI Services deployed successfully!"
    echo "=============================================="
    echo ""
    echo "🤖 Vision Service:"
    echo "   URL: http://localhost:8001"
    echo "   Health: http://localhost:8001/health"
    echo "   Models: http://localhost:8001/models"
    echo "   Docs: http://localhost:8001/docs"
    echo ""
    echo "🧠 LLM Service:"
    echo "   URL: http://localhost:8002"
    echo "   Health: http://localhost:8002/health"
    echo "   Models: http://localhost:8002/models"
    echo "   Docs: http://localhost:8002/docs"
    echo ""
    echo "🔄 Redis Cache:"
    echo "   Host: localhost:6379"
    echo ""
    
    if [[ "$DEPLOYMENT_MODE" == "full" || "$DEPLOYMENT_MODE" == "all" ]]; then
        echo "⚖️  Load Balancers:"
        echo "   Vision LB: http://localhost:8011"
        echo "   LLM LB: http://localhost:8012"
        echo ""
    fi
    
    if [[ "$DEPLOYMENT_MODE" == "monitoring" || "$DEPLOYMENT_MODE" == "all" ]]; then
        echo "📊 Monitoring:"
        echo "   Prometheus: http://localhost:9090"
        echo "   Grafana: http://localhost:3001 (admin/admin)"
        echo ""
    fi
    
    echo "🔧 Management Commands:"
    echo "   View logs: docker-compose logs -f"
    echo "   Stop services: docker-compose down"
    echo "   Restart: docker-compose restart"
    echo "   Scale vision: docker-compose up -d --scale vision-service=3"
    echo ""
    echo "📊 Service Status:"
    docker-compose ps
    echo ""
    print_warning "Note: First startup may take longer due to model downloads"
}

# Test services
test_services() {
    echo ""
    read -p "Would you like to run service tests? (y/n): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        print_status "Testing AI services..."
        
        # Test Vision Service
        print_status "Testing Vision Service..."
        if curl -f http://localhost:8001/info &> /dev/null; then
            print_success "Vision Service API is responding"
        else
            print_error "Vision Service API test failed"
        fi
        
        # Test LLM Service
        print_status "Testing LLM Service..."
        if curl -f http://localhost:8002/info &> /dev/null; then
            print_success "LLM Service API is responding"
        else
            print_error "LLM Service API test failed"
        fi
        
        print_status "For detailed testing, use the API documentation at /docs endpoints"
    fi
}

# Show logs option
show_logs() {
    echo ""
    read -p "Would you like to view the logs? (y/n): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        print_status "Showing logs (Press Ctrl+C to exit)..."
        docker-compose logs -f
    fi
}

# Cleanup function
cleanup() {
    print_status "Cleaning up..."
    cd "$AI_SERVICES_DIR"
    docker-compose down
    print_success "AI services stopped"
}

# Show usage
show_usage() {
    echo "Usage: $0 [MODE]"
    echo ""
    echo "Deployment Modes:"
    echo "  basic      - Vision Service, LLM Service, Redis (default)"
    echo "  full       - Basic + Load Balancers"
    echo "  gpu        - GPU-optimized deployment"
    echo "  monitoring - Basic + Prometheus + Grafana"
    echo "  all        - Full deployment with all features"
    echo ""
    echo "Examples:"
    echo "  $0 basic      # Basic deployment"
    echo "  $0 gpu        # GPU-optimized deployment"
    echo "  $0 all        # Full deployment with monitoring"
    echo ""
}

# Main execution
main() {
    if [[ "$1" == "--help" || "$1" == "-h" ]]; then
        show_usage
        exit 0
    fi
    
    print_status "Starting AI4Thai AI Services deployment in $DEPLOYMENT_MODE mode..."
    
    check_prerequisites
    setup_environment
    download_models
    deploy_services
    check_service_health
    show_service_info
    test_services
    show_logs
}

# Handle Ctrl+C
trap cleanup EXIT

# Run main function
main "$@"
