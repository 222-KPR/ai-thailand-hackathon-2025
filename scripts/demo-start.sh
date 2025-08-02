#!/bin/bash

# AI4Thai Crop Guardian - Demo Startup Script
# Simplified deployment for demonstration with external AI services

set -e

echo "🌾 AI4Thai Crop Guardian - Demo Deployment"
echo "=========================================="

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
}

# Setup environment
setup_environment() {
    print_status "Setting up environment..."

    if [ ! -f ".env" ]; then
        if [ -f ".env.example" ]; then
            cp .env.example .env
            print_success "Environment file created from template"
            print_warning "Please edit .env file with your AI4THAI_API_KEY"
        else
            print_error "No .env.example found"
            exit 1
        fi
    else
        print_success "Environment file already exists"
    fi

    # Check if AI4THAI_API_KEY is set
    if grep -q "AI4THAI_API_KEY=your_ai4thai_api_key_here" .env; then
        print_warning "Please update AI4THAI_API_KEY in .env file with your actual API key"
        print_warning "Contact AI4Thai team to get your API key"
    fi
}

# Check port availability
check_ports() {
    print_status "Checking port availability..."

    PORTS=(3000 8080 5432 6379)
    for port in "${PORTS[@]}"; do
        if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1; then
            print_warning "Port $port is already in use"
            print_status "You may need to stop other services or change port configuration"
        else
            print_success "Port $port is available"
        fi
    done
}

# Start services
start_services() {
    print_status "Starting AI4Thai Crop Guardian services..."

    # Pull latest images
    print_status "Pulling Docker images..."
    docker-compose pull

    # Build local services
    print_status "Building local services..."
    docker-compose build

    # Start services
    print_status "Starting services..."
    docker-compose up -d

    # Wait for services to be ready
    print_status "Waiting for services to be ready..."
    sleep 10

    # Check service health
    check_service_health
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

    # Check API Gateway health
    print_status "Checking API Gateway health..."
    for i in {1..30}; do
        if curl -f http://localhost:3000/health &> /dev/null; then
            print_success "API Gateway is healthy"
            break
        fi
        if [ $i -eq 30 ]; then
            print_error "API Gateway health check failed"
            print_status "Check logs with: docker-compose logs api-gateway"
        fi
        sleep 2
    done

    # Check Frontend availability
    print_status "Checking Frontend availability..."
    for i in {1..15}; do
        if curl -f http://localhost:8080 &> /dev/null; then
            print_success "Frontend is available"
            break
        fi
        if [ $i -eq 15 ]; then
            print_warning "Frontend may still be starting up"
        fi
        sleep 2
    done
}

# Display access information
show_access_info() {
    echo ""
    print_success "🎉 AI4Thai Crop Guardian Demo is ready!"
    echo "================================================"
    echo ""
    echo "📱 Frontend Application:"
    echo "   URL: http://localhost:8080"
    echo ""
    echo "🔌 API Gateway:"
    echo "   URL: http://localhost:3000"
    echo "   Health: http://localhost:3000/health"
    echo ""
    echo "🗄️  Database (PostgreSQL):"
    echo "   Host: localhost:5432"
    echo "   Database: ai4thai"
    echo "   User: postgres"
    echo ""
    echo "🔄 Redis Cache:"
    echo "   Host: localhost:6379"
    echo ""
    echo "🤖 External AI Services:"
    echo "   Vision: https://vision-api.ai4thai.com"
    echo "   LLM: https://llm-api.ai4thai.com"
    echo ""
    echo "📊 Demo Features:"
    echo "   • Crop disease detection"
    echo "   • AI-powered agricultural advice"
    echo "   • Thai language support"
    echo "   • Real-time chat interface"
    echo ""
    echo "🔧 Management Commands:"
    echo "   View logs: docker-compose logs -f"
    echo "   Stop demo: docker-compose down"
    echo "   Restart: docker-compose restart"
    echo ""
    print_warning "Note: This is a demonstration setup with external AI services"
    print_warning "Make sure your AI4THAI_API_KEY is configured in .env file"
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
    docker-compose down
    print_success "Demo stopped"
}

# Main execution
main() {
    print_status "Starting AI4Thai Crop Guardian demo deployment..."

    check_prerequisites
    setup_environment
    check_ports
    start_services
    show_access_info
    show_logs
}

# Handle Ctrl+C
trap cleanup EXIT

# Run main function
main "$@"
