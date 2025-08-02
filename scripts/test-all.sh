#!/bin/bash

# AI4Thai Crop Guardian - Comprehensive Test Runner
# This script runs all tests across the entire project

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

# Function to run tests with error handling
run_test() {
    local test_name="$1"
    local test_command="$2"
    local directory="$3"
    
    print_status "Running $test_name..."
    
    if [ -n "$directory" ]; then
        cd "$directory"
    fi
    
    if eval "$test_command"; then
        print_success "$test_name passed"
        if [ -n "$directory" ]; then
            cd - > /dev/null
        fi
        return 0
    else
        print_error "$test_name failed"
        if [ -n "$directory" ]; then
            cd - > /dev/null
        fi
        return 1
    fi
}

# Initialize test results
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Function to update test results
update_results() {
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    if [ $1 -eq 0 ]; then
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
}

print_status "Starting comprehensive test suite for AI4Thai Crop Guardian..."

# Check prerequisites
print_status "Checking prerequisites..."

if ! command_exists cargo; then
    print_error "Cargo not found. Please install Rust."
    exit 1
fi

if ! command_exists python3; then
    print_error "Python3 not found. Please install Python 3.9+."
    exit 1
fi

if ! command_exists wasm-pack; then
    print_warning "wasm-pack not found. Installing..."
    cargo install wasm-pack
fi

if ! command_exists trunk; then
    print_warning "trunk not found. Installing..."
    cargo install trunk
fi

print_success "Prerequisites check completed"

# 1. Rust Backend Tests
print_status "=== RUST BACKEND TESTS ==="

# API Gateway Tests
if [ -d "api-gateway" ]; then
    run_test "API Gateway Unit Tests" "cargo test --lib" "api-gateway"
    update_results $?
    
    run_test "API Gateway Integration Tests" "cargo test --test '*'" "api-gateway"
    update_results $?
    
    run_test "API Gateway Clippy Lints" "cargo clippy -- -D warnings" "api-gateway"
    update_results $?
    
    run_test "API Gateway Format Check" "cargo fmt -- --check" "api-gateway"
    update_results $?
else
    print_warning "API Gateway directory not found, skipping tests"
fi

# Shared Library Tests
if [ -d "shared" ]; then
    run_test "Shared Library Tests" "cargo test" "shared"
    update_results $?
    
    run_test "Shared Library Clippy" "cargo clippy -- -D warnings" "shared"
    update_results $?
else
    print_warning "Shared directory not found, skipping tests"
fi

# Queue Worker Tests
if [ -d "queue-worker" ]; then
    run_test "Queue Worker Tests" "cargo test" "queue-worker"
    update_results $?
    
    run_test "Queue Worker Clippy" "cargo clippy -- -D warnings" "queue-worker"
    update_results $?
else
    print_warning "Queue Worker directory not found, skipping tests"
fi

# 2. Python AI Services Tests
print_status "=== PYTHON AI SERVICES TESTS ==="

if [ -d "ai-services" ]; then
    # Check if virtual environment exists
    if [ ! -d "ai-services/venv" ]; then
        print_warning "Python virtual environment not found. Creating..."
        cd ai-services
        python3 -m venv venv
        source venv/bin/activate
        pip install -r vision-service/requirements.txt
        pip install -r llm-service/requirements.txt
        pip install pytest pytest-cov black flake8
        cd ..
    fi
    
    # Activate virtual environment
    source ai-services/venv/bin/activate
    
    # Vision Service Tests
    if [ -d "ai-services/vision-service" ]; then
        run_test "Vision Service Unit Tests" "python -m pytest tests/ -v" "ai-services/vision-service"
        update_results $?
        
        run_test "Vision Service Code Coverage" "python -m pytest tests/ --cov=. --cov-report=term-missing --cov-fail-under=80" "ai-services/vision-service"
        update_results $?
        
        run_test "Vision Service Code Format" "python -m black --check ." "ai-services/vision-service"
        update_results $?
        
        run_test "Vision Service Linting" "python -m flake8 . --max-line-length=88 --extend-ignore=E203,W503" "ai-services/vision-service"
        update_results $?
    fi
    
    # LLM Service Tests
    if [ -d "ai-services/llm-service" ]; then
        run_test "LLM Service Unit Tests" "python -m pytest tests/ -v" "ai-services/llm-service"
        update_results $?
        
        run_test "LLM Service Code Coverage" "python -m pytest tests/ --cov=. --cov-report=term-missing --cov-fail-under=80" "ai-services/llm-service"
        update_results $?
        
        run_test "LLM Service Code Format" "python -m black --check ." "ai-services/llm-service"
        update_results $?
        
        run_test "LLM Service Linting" "python -m flake8 . --max-line-length=88 --extend-ignore=E203,W503" "ai-services/llm-service"
        update_results $?
    fi
    
    deactivate
else
    print_warning "AI Services directory not found, skipping Python tests"
fi

# 3. Frontend WebAssembly Tests
print_status "=== FRONTEND WEBASSEMBLY TESTS ==="

if [ -d "frontend" ]; then
    # Check if wasm32 target is installed
    if ! rustup target list --installed | grep -q wasm32-unknown-unknown; then
        print_warning "WebAssembly target not installed. Installing..."
        rustup target add wasm32-unknown-unknown
    fi
    
    run_test "Frontend Unit Tests" "wasm-pack test --headless --firefox" "frontend"
    update_results $?
    
    run_test "Frontend Component Tests" "wasm-pack test --headless --firefox -- --test component_tests" "frontend"
    update_results $?
    
    run_test "Frontend Design System Tests" "wasm-pack test --headless --firefox -- --test design_system_tests" "frontend"
    update_results $?
    
    run_test "Frontend Clippy Lints" "cargo clippy --target wasm32-unknown-unknown -- -D warnings" "frontend"
    update_results $?
    
    run_test "Frontend Format Check" "cargo fmt -- --check" "frontend"
    update_results $?
    
    # Build test to ensure everything compiles
    run_test "Frontend Build Test" "trunk build --release" "frontend"
    update_results $?
    
    # Clean up build artifacts
    if [ -d "frontend/dist" ]; then
        rm -rf frontend/dist
    fi
else
    print_warning "Frontend directory not found, skipping WebAssembly tests"
fi

# 4. Integration Tests
print_status "=== INTEGRATION TESTS ==="

if [ -d "tests" ]; then
    run_test "API Integration Tests" "cargo test --test api_integration" "tests"
    update_results $?
    
    run_test "Service Integration Tests" "cargo test --test service_integration" "tests"
    update_results $?
    
    run_test "Database Integration Tests" "cargo test --test database_integration" "tests"
    update_results $?
else
    print_warning "Integration tests directory not found, skipping integration tests"
fi

# 5. Documentation Tests
print_status "=== DOCUMENTATION TESTS ==="

# Test that all Rust documentation builds
run_test "Rust Documentation Build" "cargo doc --no-deps --document-private-items"
update_results $?

# Check for broken links in documentation (if mdbook is available)
if command_exists mdbook; then
    if [ -f "book.toml" ]; then
        run_test "Documentation Build" "mdbook build"
        update_results $?
    fi
fi

# 6. Security Tests
print_status "=== SECURITY TESTS ==="

if command_exists cargo-audit; then
    run_test "Rust Security Audit" "cargo audit"
    update_results $?
else
    print_warning "cargo-audit not found. Install with: cargo install cargo-audit"
fi

# Python security check
if [ -d "ai-services" ] && command_exists safety; then
    source ai-services/venv/bin/activate
    run_test "Python Security Check" "safety check" "ai-services"
    update_results $?
    deactivate
fi

# 7. Performance Tests
print_status "=== PERFORMANCE TESTS ==="

if [ -d "tests/performance" ]; then
    run_test "Performance Benchmarks" "cargo bench" "tests/performance"
    update_results $?
fi

# 8. End-to-End Tests (if available)
print_status "=== END-TO-END TESTS ==="

if [ -d "tests/e2e" ]; then
    # Check if Docker is available for E2E tests
    if command_exists docker; then
        run_test "End-to-End Tests" "./run-e2e-tests.sh" "tests/e2e"
        update_results $?
    else
        print_warning "Docker not available, skipping E2E tests"
    fi
fi

# Generate Test Report
print_status "=== TEST SUMMARY ==="

echo ""
echo "╔══════════════════════════════════════╗"
echo "║           TEST RESULTS               ║"
echo "╠══════════════════════════════════════╣"
printf "║ Total Tests:    %-20s ║\n" "$TOTAL_TESTS"
printf "║ Passed:         %-20s ║\n" "$PASSED_TESTS"
printf "║ Failed:         %-20s ║\n" "$FAILED_TESTS"
echo "╚══════════════════════════════════════╝"
echo ""

if [ $FAILED_TESTS -eq 0 ]; then
    print_success "All tests passed! 🎉"
    
    # Generate coverage report if tarpaulin is available
    if command_exists cargo-tarpaulin; then
        print_status "Generating coverage report..."
        cargo tarpaulin --out Html --output-dir coverage/ --skip-clean
        print_success "Coverage report generated in coverage/tarpaulin-report.html"
    fi
    
    exit 0
else
    print_error "$FAILED_TESTS test(s) failed"
    
    echo ""
    print_status "To run specific test categories:"
    echo "  Backend only:     ./scripts/test-backend.sh"
    echo "  Frontend only:    ./scripts/test-frontend.sh"
    echo "  Python only:      ./scripts/test-python.sh"
    echo "  Integration only: ./scripts/test-integration.sh"
    
    exit 1
fi
