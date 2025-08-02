#!/bin/bash
# scripts/pre-commit-rust.sh

set -e

echo "🔧 Running Rust pre-commit checks..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_step() {
    echo -e "${BLUE}[STEP]${NC} $1"
}

# Check if we're in a Rust project
if [ ! -f "Cargo.toml" ]; then
    print_error "No Cargo.toml found. Are you in a Rust project?"
    exit 1
fi

print_step "Starting pre-commit checks for AI4Thai Crop Guardian..."

# 1. Run cargo fix first to automatically fix issues
print_step "1/7 Running cargo fix..."
if ! cargo fix --workspace --all-targets --all-features --allow-dirty --allow-staged; then
    print_error "cargo fix failed"
    exit 1
fi
print_status "cargo fix completed successfully"

# 2. Run rustfmt to format code
print_step "2/7 Running rustfmt..."
if ! cargo fmt --all -- --check; then
    print_warning "Code formatting issues found. Running rustfmt..."
    cargo fmt --all
    print_status "Code formatted successfully"
else
    print_status "Code is already properly formatted"
fi

# 3. Run clippy for linting
print_step "3/7 Running clippy..."
if ! cargo clippy --workspace --all-targets --all-features -- -D warnings; then
    print_error "Clippy found issues that need to be fixed"
    print_error "Please fix the clippy warnings and try again"
    exit 1
fi
print_status "Clippy checks passed"

# 4. Run dylint if available
print_step "4/7 Running dylint..."
if command -v cargo-dylint &> /dev/null; then
    if ! cargo dylint --workspace --all; then
        print_warning "Dylint found issues (non-blocking)"
    else
        print_status "Dylint checks passed"
    fi
else
    print_warning "dylint not installed, skipping..."
    print_warning "Install with: cargo install cargo-dylint dylint-link"
fi

# 5. Check compilation for all workspace members
print_step "5/7 Checking compilation..."
if ! cargo check --workspace --all-targets --all-features; then
    print_error "Compilation check failed"
    exit 1
fi
print_status "Compilation check passed"

# 6. Check frontend WASM compilation if frontend exists
print_step "6/7 Running project-specific checks..."
if [ -d "frontend" ]; then
    print_status "Checking frontend WASM compilation..."
    cd frontend
    if ! cargo check --target wasm32-unknown-unknown; then
        print_error "Frontend WASM compilation failed"
        print_error "Make sure you have the WASM target installed: rustup target add wasm32-unknown-unknown"
        exit 1
    fi
    print_status "Frontend WASM compilation passed"
    cd ..
fi

# Check API Gateway compilation
if [ -d "api-gateway" ]; then
    print_status "Checking API Gateway compilation..."
    cd api-gateway
    if ! cargo check; then
        print_error "API Gateway compilation failed"
        exit 1
    fi
    print_status "API Gateway compilation passed"
    cd ..
fi

# Check shared library compilation
if [ -d "shared" ]; then
    print_status "Checking shared library compilation..."
    cd shared
    if ! cargo check; then
        print_error "Shared library compilation failed"
        exit 1
    fi
    print_status "Shared library compilation passed"
    cd ..
fi

# 7. Run tests
print_step "7/7 Running tests..."
if ! cargo test --workspace; then
    print_error "Tests failed"
    print_error "Please fix failing tests before committing"
    exit 1
fi
print_status "All tests passed"

echo ""
print_status "🎉 All pre-commit checks passed! Your code is ready to commit."
echo ""
