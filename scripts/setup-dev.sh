#!/bin/bash

# AI4Thai Crop Guardian - Development Setup Script
# This script sets up the development environment

set -e

echo "🌾 AI4Thai Crop Guardian - Development Setup"
echo "============================================="

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

# Check if running on macOS or Linux
OS="$(uname -s)"
case "${OS}" in
    Linux*)     MACHINE=Linux;;
    Darwin*)    MACHINE=Mac;;
    *)          MACHINE="UNKNOWN:${OS}"
esac

print_status "Detected OS: $MACHINE"

# Check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    # Check Rust
    if ! command -v rustc &> /dev/null; then
        print_error "Rust is not installed. Please install from https://rustup.rs/"
        exit 1
    fi
    print_success "Rust found: $(rustc --version)"
    
    # Check Python
    if ! command -v python3 &> /dev/null; then
        print_error "Python 3 is not installed. Please install Python 3.9+"
        exit 1
    fi
    print_success "Python found: $(python3 --version)"
    
    # Check Node.js
    if ! command -v node &> /dev/null; then
        print_error "Node.js is not installed. Please install Node.js 18+"
        exit 1
    fi
    print_success "Node.js found: $(node --version)"
    
    # Check Docker
    if ! command -v docker &> /dev/null; then
        print_error "Docker is not installed. Please install Docker"
        exit 1
    fi
    print_success "Docker found: $(docker --version)"
    
    # Check Docker Compose
    if ! command -v docker-compose &> /dev/null; then
        print_error "Docker Compose is not installed. Please install Docker Compose"
        exit 1
    fi
    print_success "Docker Compose found: $(docker-compose --version)"
}

# Install Rust tools
install_rust_tools() {
    print_status "Installing Rust tools..."
    
    # Add WebAssembly target
    rustup target add wasm32-unknown-unknown
    
    # Install trunk for frontend development
    if ! command -v trunk &> /dev/null; then
        cargo install trunk
    fi
    
    # Install wasm-pack
    if ! command -v wasm-pack &> /dev/null; then
        cargo install wasm-pack
    fi
    
    # Install cargo tools
    cargo install cargo-watch
    cargo install cargo-tarpaulin
    
    print_success "Rust tools installed"
}

# Setup Python environment
setup_python_env() {
    print_status "Setting up Python environment..."
    
    # Create virtual environment for AI services
    if [ ! -d "ai-services/venv" ]; then
        cd ai-services
        python3 -m venv venv
        source venv/bin/activate
        pip install --upgrade pip
        
        # Install vision service dependencies
        if [ -f "vision-service/requirements.txt" ]; then
            pip install -r vision-service/requirements.txt
        fi
        
        # Install LLM service dependencies
        if [ -f "llm-service/requirements.txt" ]; then
            pip install -r llm-service/requirements.txt
        fi
        
        cd ..
        print_success "Python virtual environment created"
    else
        print_success "Python virtual environment already exists"
    fi
}

# Setup environment files
setup_env_files() {
    print_status "Setting up environment files..."
    
    if [ ! -f ".env" ]; then
        if [ -f ".env.example" ]; then
            cp .env.example .env
            print_success "Environment file created from template"
            print_warning "Please edit .env file with your configuration"
        else
            print_warning "No .env.example found, creating basic .env file"
            cat > .env << EOF
# Database Configuration
DATABASE_URL=postgresql://postgres:password@localhost:5432/ai4thai
REDIS_URL=redis://localhost:6379

# API Configuration
API_PORT=3000
VISION_SERVICE_URL=http://localhost:8001
LLM_SERVICE_URL=http://localhost:8002

# External APIs
OPENAI_API_KEY=your_openai_key_here
WEATHER_API_KEY=your_weather_key_here

# Security
JWT_SECRET=your_jwt_secret_here
CORS_ORIGINS=http://localhost:8080

# Development
RUST_LOG=debug
PYTHON_LOG_LEVEL=DEBUG
EOF
        fi
    else
        print_success "Environment file already exists"
    fi
}

# Setup database
setup_database() {
    print_status "Setting up database..."
    
    # Start PostgreSQL and Redis with Docker
    docker-compose up -d postgres redis
    
    # Wait for database to be ready
    print_status "Waiting for database to be ready..."
    sleep 5
    
    # Run migrations if they exist
    if [ -d "api-gateway/migrations" ]; then
        cd api-gateway
        if command -v diesel &> /dev/null; then
            diesel migration run
            print_success "Database migrations completed"
        else
            print_warning "Diesel CLI not found, skipping migrations"
        fi
        cd ..
    fi
}

# Build all services
build_services() {
    print_status "Building services..."
    
    # Build API Gateway
    cd api-gateway
    cargo build
    cd ..
    
    # Build frontend
    cd frontend
    trunk build
    cd ..
    
    # Build queue worker
    cd queue-worker
    cargo build
    cd ..
    
    print_success "All services built successfully"
}

# Create development scripts
create_dev_scripts() {
    print_status "Creating development scripts..."
    
    # Create start script
    cat > scripts/dev-start.sh << 'EOF'
#!/bin/bash
echo "🚀 Starting AI4Thai Crop Guardian development environment..."

# Start infrastructure services
docker-compose up -d postgres redis

# Start backend services in background
cd api-gateway && cargo run &
API_PID=$!

cd ../ai-services
source venv/bin/activate
cd vision-service && python -m uvicorn app:app --reload --port 8001 &
VISION_PID=$!

cd ../llm-service && python -m uvicorn app:app --reload --port 8002 &
LLM_PID=$!

cd ../../queue-worker && cargo run &
QUEUE_PID=$!

# Start frontend
cd ../frontend && trunk serve --port 8080 &
FRONTEND_PID=$!

echo "✅ All services started!"
echo "Frontend: http://localhost:8080"
echo "API Gateway: http://localhost:3000"
echo "Vision Service: http://localhost:8001"
echo "LLM Service: http://localhost:8002"

# Wait for Ctrl+C
trap "kill $API_PID $VISION_PID $LLM_PID $QUEUE_PID $FRONTEND_PID; docker-compose down" EXIT
wait
EOF

    chmod +x scripts/dev-start.sh
    
    # Create test script
    cat > scripts/test-all.sh << 'EOF'
#!/bin/bash
echo "🧪 Running all tests..."

# Run Rust tests
echo "Running Rust tests..."
cd api-gateway && cargo test
cd ../queue-worker && cargo test
cd ../shared && cargo test

# Run Python tests
echo "Running Python tests..."
cd ../ai-services
source venv/bin/activate
cd vision-service && pytest
cd ../llm-service && pytest

# Run frontend tests
echo "Running frontend tests..."
cd ../../frontend && wasm-pack test --headless --firefox

echo "✅ All tests completed!"
EOF

    chmod +x scripts/test-all.sh
    
    print_success "Development scripts created"
}

# Main setup function
main() {
    print_status "Starting development environment setup..."
    
    check_prerequisites
    install_rust_tools
    setup_python_env
    setup_env_files
    setup_database
    build_services
    create_dev_scripts
    
    print_success "Development environment setup completed!"
    echo ""
    echo "🎉 Setup Complete!"
    echo "=================="
    echo "Next steps:"
    echo "1. Edit .env file with your configuration"
    echo "2. Run './scripts/dev-start.sh' to start all services"
    echo "3. Open http://localhost:8080 in your browser"
    echo ""
    echo "For more information, see docs/development/README.md"
}

# Run main function
main "$@"
