#!/bin/bash

# AI4Thai Crop Guardian - Frontend Test Runner
# This script runs all frontend-specific tests

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

print_status "Starting Frontend Test Suite for AI4Thai Crop Guardian..."

# Check prerequisites
print_status "Checking prerequisites..."

if ! command_exists cargo; then
    print_error "Cargo not found. Please install Rust."
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

# Check if wasm32 target is installed
if ! rustup target list --installed | grep -q wasm32-unknown-unknown; then
    print_warning "WebAssembly target not installed. Installing..."
    rustup target add wasm32-unknown-unknown
fi

print_success "Prerequisites check completed"

# Navigate to frontend directory
if [ ! -d "frontend" ]; then
    print_error "Frontend directory not found!"
    exit 1
fi

cd frontend

# Initialize test results
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Function to run tests with error handling
run_test() {
    local test_name="$1"
    local test_command="$2"

    print_status "Running $test_name..."
    TOTAL_TESTS=$((TOTAL_TESTS + 1))

    if eval "$test_command"; then
        print_success "$test_name passed"
        PASSED_TESTS=$((PASSED_TESTS + 1))
        return 0
    else
        print_error "$test_name failed"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        return 1
    fi
}

print_status "=== FRONTEND WEBASSEMBLY TESTS ==="

# 1. Unit Tests
run_test "Component Unit Tests" "wasm-pack test --headless --firefox"

# 2. Design System Tests
run_test "Design System Tests" "wasm-pack test --headless --firefox -- --test design_system_tests"

# 3. Component Architecture Tests
run_test "Component Architecture Tests" "wasm-pack test --headless --firefox -- --test component_tests"

# 4. Integration Tests
run_test "Frontend Integration Tests" "wasm-pack test --headless --firefox -- --test integration_tests"

# 5. Accessibility Tests
run_test "Accessibility Tests" "wasm-pack test --headless --firefox -- --test accessibility_tests"

# 6. Performance Tests
run_test "Performance Tests" "wasm-pack test --headless --firefox -- --test performance_tests"

# 7. PWA Tests
run_test "PWA Functionality Tests" "wasm-pack test --headless --firefox -- --test pwa_tests"

# 8. Code Quality Tests
run_test "Clippy Lints" "cargo clippy --target wasm32-unknown-unknown -- -D warnings"

run_test "Format Check" "cargo fmt -- --check"

# 9. Build Tests
run_test "Development Build" "trunk build"

run_test "Production Build" "trunk build --release"

# 10. Bundle Size Check
if [ -f "dist/index.html" ]; then
    print_status "Checking bundle sizes..."

    # Check WASM bundle size
    wasm_size=$(find dist -name "*.wasm" -exec wc -c {} + | tail -1 | awk '{print $1}')
    wasm_size_kb=$((wasm_size / 1024))

    print_status "WASM bundle size: ${wasm_size_kb}KB"

    if [ $wasm_size_kb -gt 500 ]; then
        print_warning "WASM bundle size (${wasm_size_kb}KB) exceeds recommended 500KB"
    else
        print_success "WASM bundle size within limits"
    fi

    # Check total asset size
    total_size=$(find dist -type f -exec wc -c {} + | tail -1 | awk '{print $1}')
    total_size_mb=$((total_size / 1024 / 1024))

    print_status "Total asset size: ${total_size_mb}MB"

    if [ $total_size_mb -gt 2 ]; then
        print_warning "Total asset size (${total_size_mb}MB) exceeds recommended 2MB"
    else
        print_success "Total asset size within limits"
    fi
fi

# 11. Lighthouse Audit (if available)
if command_exists lighthouse; then
    print_status "Running Lighthouse audit..."

    # Start development server in background
    trunk serve --port 8080 &
    SERVER_PID=$!

    # Wait for server to start
    sleep 5

    # Run Lighthouse audit
    if lighthouse http://localhost:8080 --only-categories=pwa,performance,accessibility --chrome-flags="--headless" --output=json --output-path=lighthouse-report.json; then
        print_success "Lighthouse audit completed"

        # Parse and display key metrics
        if command_exists jq; then
            pwa_score=$(jq '.categories.pwa.score * 100' lighthouse-report.json)
            performance_score=$(jq '.categories.performance.score * 100' lighthouse-report.json)
            accessibility_score=$(jq '.categories.accessibility.score * 100' lighthouse-report.json)

            print_status "Lighthouse Scores:"
            echo "  PWA: ${pwa_score}%"
            echo "  Performance: ${performance_score}%"
            echo "  Accessibility: ${accessibility_score}%"
        fi

        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        print_error "Lighthouse audit failed"
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi

    TOTAL_TESTS=$((TOTAL_TESTS + 1))

    # Stop development server
    kill $SERVER_PID 2>/dev/null || true
else
    print_warning "Lighthouse not found. Install with: npm install -g lighthouse"
fi

# Clean up build artifacts
print_status "Cleaning up build artifacts..."
if [ -d "dist" ]; then
    rm -rf dist
fi

# Generate Test Report
cd ..
print_status "=== FRONTEND TEST SUMMARY ==="

echo ""
echo "╔══════════════════════════════════════╗"
echo "║        FRONTEND TEST RESULTS         ║"
echo "╠══════════════════════════════════════╣"
printf "║ Total Tests:    %-20s ║\n" "$TOTAL_TESTS"
printf "║ Passed:         %-20s ║\n" "$PASSED_TESTS"
printf "║ Failed:         %-20s ║\n" "$FAILED_TESTS"
echo "╚══════════════════════════════════════╝"
echo ""

if [ $FAILED_TESTS -eq 0 ]; then
    print_success "All frontend tests passed! 🎉"

    echo ""
    print_status "Frontend Test Coverage Summary:"
    echo "  ✅ Component Unit Tests"
    echo "  ✅ Design System Tests"
    echo "  ✅ Integration Tests"
    echo "  ✅ Accessibility Tests"
    echo "  ✅ Performance Tests"
    echo "  ✅ PWA Functionality"
    echo "  ✅ Code Quality"
    echo "  ✅ Build Validation"

    exit 0
else
    print_error "$FAILED_TESTS frontend test(s) failed"

    echo ""
    print_status "Common issues and solutions:"
    echo "  • WASM compilation errors: Check Rust version and target installation"
    echo "  • Test failures: Review test output and fix failing assertions"
    echo "  • Bundle size issues: Optimize imports and enable tree shaking"
    echo "  • Lighthouse failures: Check PWA manifest and service worker"

    exit 1
fi
