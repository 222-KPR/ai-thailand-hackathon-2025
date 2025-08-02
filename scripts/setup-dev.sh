#!/bin/bash

# AI4Thai Crop Guardian - Development Environment Setup
# This script sets up the complete development environment

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

# Function to check OS
get_os() {
    case "$(uname -s)" in
        Darwin*) echo "macos" ;;
        Linux*) echo "linux" ;;
        CYGWIN*|MINGW*|MSYS*) echo "windows" ;;
        *) echo "unknown" ;;
    esac
}

OS=$(get_os)

print_status "Setting up AI4Thai Crop Guardian development environment..."
print_status "Detected OS: $OS"

# 1. Check and install Rust
print_status "=== RUST SETUP ==="

if ! command_exists rustc; then
    print_warning "Rust not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
    print_success "Rust installed successfully"
else
    print_success "Rust already installed: $(rustc --version)"
fi

# Update Rust to latest stable
print_status "Updating Rust to latest stable..."
rustup update stable
rustup default stable

# Install WebAssembly target
print_status "Installing WebAssembly target..."
rustup target add wasm32-unknown-unknown

# Install Rust tools
print_status "Installing Rust development tools..."

# Essential tools
RUST_TOOLS=(
    "trunk"           # WebAssembly build tool
    "wasm-pack"       # WebAssembly package tool
    "cargo-watch"     # Auto-rebuild on changes
    "cargo-edit"      # Cargo subcommands for editing Cargo.toml
    "cargo-audit"     # Security audit
    "cargo-tarpaulin" # Code coverage
)

for tool in "${RUST_TOOLS[@]}"; do
    if ! command_exists "$tool"; then
        print_status "Installing $tool..."
        cargo install "$tool"
    else
        print_success "$tool already installed"
    fi
done

# 2. Check and install Python
print_status "=== PYTHON SETUP ==="

if ! command_exists python3; then
    print_error "Python 3 not found. Please install Python 3.9+ manually."
    case $OS in
        "macos")
            echo "Install with: brew install python@3.9"
            ;;
        "linux")
            echo "Install with: sudo apt-get install python3.9 python3.9-venv python3.9-pip"
            ;;
        "windows")
            echo "Download from: https://www.python.org/downloads/"
            ;;
    esac
    exit 1
else
    python_version=$(python3 --version | cut -d' ' -f2)
    print_success "Python already installed: $python_version"
fi

# Check Python version (require 3.9+)
python_major=$(python3 -c "import sys; print(sys.version_info.major)")
python_minor=$(python3 -c "import sys; print(sys.version_info.minor)")

if [ "$python_major" -lt 3 ] || ([ "$python_major" -eq 3 ] && [ "$python_minor" -lt 9 ]); then
    print_error "Python 3.9+ required. Current version: $python_major.$python_minor"
    exit 1
fi

# Set up Python virtual environment for AI services
if [ ! -d "ai-services" ]; then
    print_status "Creating ai-services directory..."
    mkdir -p ai-services
fi

cd ai-services

if [ ! -d "venv" ]; then
    print_status "Creating Python virtual environment..."
    python3 -m venv venv
    print_success "Virtual environment created"
else
    print_success "Virtual environment already exists"
fi

# Activate virtual environment and install dependencies
print_status "Installing Python dependencies..."
source venv/bin/activate

# Upgrade pip
pip install --upgrade pip

# Install common development dependencies
pip install \
    fastapi \
    uvicorn \
    pytest \
    pytest-cov \
    pytest-asyncio \
    black \
    flake8 \
    mypy \
    httpx \
    python-multipart \
    pillow \
    numpy \
    torch \
    torchvision \
    transformers \
    datasets \
    accelerate \
    safety

# Install vision service dependencies if requirements exist
if [ -f "vision-service/requirements.txt" ]; then
    print_status "Installing vision service dependencies..."
    pip install -r vision-service/requirements.txt
fi

# Install LLM service dependencies if requirements exist
if [ -f "llm-service/requirements.txt" ]; then
    print_status "Installing LLM service dependencies..."
    pip install -r llm-service/requirements.txt
fi

deactivate
cd ..

print_success "Python environment setup completed"

# 3. Check and install Node.js (for tooling)
print_status "=== NODE.JS SETUP ==="

if ! command_exists node; then
    print_warning "Node.js not found. Installing Node.js..."
    case $OS in
        "macos")
            if command_exists brew; then
                brew install node
            else
                print_error "Homebrew not found. Please install Node.js manually from https://nodejs.org/"
                exit 1
            fi
            ;;
        "linux")
            curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
            sudo apt-get install -y nodejs
            ;;
        "windows")
            print_error "Please install Node.js manually from https://nodejs.org/"
            exit 1
            ;;
    esac
else
    node_version=$(node --version)
    print_success "Node.js already installed: $node_version"
fi

# Install global Node.js tools
print_status "Installing Node.js development tools..."

NODE_TOOLS=(
    "lighthouse"      # PWA auditing
    "serve"          # Static file server
    "@trunk-rs/launcher" # Trunk launcher
)

for tool in "${NODE_TOOLS[@]}"; do
    if ! npm list -g "$tool" >/dev/null 2>&1; then
        print_status "Installing $tool..."
        npm install -g "$tool"
    else
        print_success "$tool already installed"
    fi
done

# 4. Check and install Docker
print_status "=== DOCKER SETUP ==="

if ! command_exists docker; then
    print_warning "Docker not found. Please install Docker manually."
    case $OS in
        "macos")
            echo "Download Docker Desktop from: https://www.docker.com/products/docker-desktop"
            ;;
        "linux")
            echo "Install with: curl -fsSL https://get.docker.com -o get-docker.sh && sh get-docker.sh"
            ;;
        "windows")
            echo "Download Docker Desktop from: https://www.docker.com/products/docker-desktop"
            ;;
    esac
else
    docker_version=$(docker --version)
    print_success "Docker already installed: $docker_version"
fi

if ! command_exists docker-compose; then
    print_warning "Docker Compose not found. Installing..."
    case $OS in
        "macos"|"linux")
            sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
            sudo chmod +x /usr/local/bin/docker-compose
            ;;
        "windows")
            print_warning "Docker Compose should be included with Docker Desktop"
            ;;
    esac
else
    compose_version=$(docker-compose --version)
    print_success "Docker Compose already installed: $compose_version"
fi

# 5. Set up environment configuration
print_status "=== ENVIRONMENT CONFIGURATION ==="

if [ ! -f ".env" ]; then
    if [ -f ".env.example" ]; then
        print_status "Creating .env file from .env.example..."
        cp .env.example .env
        print_warning "Please edit .env file with your configuration"
    else
        print_status "Creating basic .env file..."
        cat > .env << EOF
# AI4Thai Crop Guardian Environment Configuration

# Application Environment
APP_ENV=development
RUST_LOG=debug

# Database Configuration
DATABASE_URL=postgresql://postgres:password@localhost:5432/ai4thai
REDIS_URL=redis://localhost:6379

# API Configuration
API_GATEWAY_PORT=3000
FRONTEND_PORT=8080

# AI Services Configuration
VISION_SERVICE_URL=http://localhost:8001
LLM_SERVICE_URL=http://localhost:8002

# External API Keys (replace with actual keys)
AI4THAI_API_KEY=your_ai4thai_api_key_here
WEATHER_API_KEY=your_weather_api_key_here
OPENAI_API_KEY=your_openai_api_key_here

# JWT Configuration
JWT_SECRET=your-super-secret-jwt-key-change-this-in-production

# File Upload Configuration
MAX_FILE_SIZE=10485760
UPLOAD_DIR=./uploads

# Monitoring Configuration
ENABLE_METRICS=true
METRICS_PORT=9090
EOF
        print_success ".env file created"
    fi
else
    print_success ".env file already exists"
fi

# 6. Create necessary directories
print_status "=== DIRECTORY SETUP ==="

DIRECTORIES=(
    "uploads"
    "logs"
    "data"
    "coverage"
    "docs/generated"
)

for dir in "${DIRECTORIES[@]}"; do
    if [ ! -d "$dir" ]; then
        mkdir -p "$dir"
        print_status "Created directory: $dir"
    fi
done

# 7. Set up Git hooks (if .git exists)
if [ -d ".git" ]; then
    print_status "=== GIT HOOKS SETUP ==="

    if [ ! -d ".git/hooks" ]; then
        mkdir -p .git/hooks
    fi

    # Pre-commit hook
    cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
# Pre-commit hook for AI4Thai Crop Guardian

set -e

echo "Running pre-commit checks..."

# Check Rust formatting
if command -v cargo >/dev/null 2>&1; then
    echo "Checking Rust formatting..."
    cargo fmt --all -- --check
fi

# Check Python formatting (if virtual environment exists)
if [ -d "ai-services/venv" ]; then
    echo "Checking Python formatting..."
    source ai-services/venv/bin/activate

    if [ -d "ai-services/vision-service" ]; then
        cd ai-services/vision-service
        python -m black --check .
        cd ../..
    fi

    if [ -d "ai-services/llm-service" ]; then
        cd ai-services/llm-service
        python -m black --check .
        cd ../..
    fi

    deactivate
fi

echo "Pre-commit checks passed!"
EOF

    chmod +x .git/hooks/pre-commit
    print_success "Git pre-commit hook installed"
fi

# 8. Verify installation
print_status "=== INSTALLATION VERIFICATION ==="

print_status "Verifying installations..."

# Check Rust tools
rust_tools_ok=true
for tool in rustc cargo trunk wasm-pack; do
    if command_exists "$tool"; then
        print_success "✓ $tool: $(command -v $tool)"
    else
        print_error "✗ $tool: not found"
        rust_tools_ok=false
    fi
done

# Check Python environment
python_ok=true
if [ -d "ai-services/venv" ]; then
    source ai-services/venv/bin/activate
    if python -c "import fastapi, uvicorn, pytest" 2>/dev/null; then
        print_success "✓ Python environment: OK"
    else
        print_error "✗ Python environment: missing dependencies"
        python_ok=false
    fi
    deactivate
else
    print_error "✗ Python virtual environment: not found"
    python_ok=false
fi

# Check Node.js tools
node_ok=true
if command_exists node && command_exists npm; then
    print_success "✓ Node.js: $(node --version)"
else
    print_error "✗ Node.js: not found"
    node_ok=false
fi

# Check Docker
docker_ok=true
if command_exists docker; then
    if docker info >/dev/null 2>&1; then
        print_success "✓ Docker: running"
    else
        print_warning "⚠ Docker: installed but not running"
    fi
else
    print_error "✗ Docker: not found"
    docker_ok=false
fi

# 9. Final setup steps
print_status "=== FINAL SETUP ==="

# Make scripts executable
if [ -d "scripts" ]; then
    print_status "Making scripts executable..."
    chmod +x scripts/*.sh
    print_success "Scripts made executable"
fi

# Generate initial documentation
if command_exists cargo; then
    print_status "Generating Rust documentation..."
    cargo doc --no-deps --document-private-items --quiet
    print_success "Rust documentation generated"
fi

# Summary
print_status "=== SETUP SUMMARY ==="

echo ""
echo "╔══════════════════════════════════════╗"
echo "║        DEVELOPMENT SETUP             ║"
echo "╠══════════════════════════════════════╣"

if $rust_tools_ok; then
    echo "║ ✅ Rust & WebAssembly Tools         ║"
else
    echo "║ ❌ Rust & WebAssembly Tools         ║"
fi

if $python_ok; then
    echo "║ ✅ Python AI Services Environment   ║"
else
    echo "║ ❌ Python AI Services Environment   ║"
fi

if $node_ok; then
    echo "║ ✅ Node.js Development Tools        ║"
else
    echo "║ ❌ Node.js Development Tools        ║"
fi

if $docker_ok; then
    echo "║ ✅ Docker Container Platform        ║"
else
    echo "║ ❌ Docker Container Platform        ║"
fi

echo "║ ✅ Environment Configuration        ║"
echo "║ ✅ Directory Structure              ║"
echo "║ ✅ Git Hooks                        ║"
echo "╚══════════════════════════════════════╝"
echo ""

if $rust_tools_ok && $python_ok && $node_ok; then
    print_success "🎉 Development environment setup completed successfully!"

    echo ""
    print_status "Next steps:"
    echo "  1. Edit .env file with your API keys and configuration"
    echo "  2. Start development services: ./scripts/dev-start.sh"
    echo "  3. Run tests: ./scripts/test-all.sh"
    echo "  4. Start frontend development: cd frontend && trunk serve"
    echo ""
    print_status "Useful commands:"
    echo "  • Full test suite: ./scripts/test-all.sh"
    echo "  • Frontend tests: ./scripts/test-frontend.sh"
    echo "  • Start all services: ./scripts/dev-start.sh"
    echo "  • View documentation: cargo doc --open"

else
    print_warning "⚠️  Setup completed with some issues. Please resolve the failed components."
    echo ""
    print_status "Common solutions:"
    echo "  • Rust issues: Restart terminal and run 'source ~/.cargo/env'"
    echo "  • Python issues: Check Python version (3.9+ required)"
    echo "  • Node.js issues: Install from https://nodejs.org/"
    echo "  • Docker issues: Start Docker Desktop or Docker daemon"
fi

echo ""
print_status "For help and documentation, visit: docs/README.md"
