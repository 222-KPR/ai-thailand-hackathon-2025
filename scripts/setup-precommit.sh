#!/bin/bash
# Setup pre-commit tools and run basic checks

set -e

echo "🔧 Setting up pre-commit tools for AI4Thai Crop Guardian..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "No Cargo.toml found. Are you in the project root?"
    exit 1
fi

print_step "1/6 Installing Rust components..."
rustup component add rustfmt clippy 2>/dev/null || print_warning "rustfmt/clippy already installed"

print_step "2/6 Installing dylint..."
if ! command -v cargo-dylint &> /dev/null; then
    cargo install cargo-dylint dylint-link || print_warning "dylint installation failed (optional)"
else
    print_status "dylint already installed"
fi

print_step "3/6 Installing WASM target..."
rustup target add wasm32-unknown-unknown 2>/dev/null || print_status "WASM target already installed"

print_step "4/6 Installing trunk..."
if ! command -v trunk &> /dev/null; then
    cargo install trunk || print_warning "trunk installation failed"
else
    print_status "trunk already installed"
fi

print_step "5/6 Installing pre-commit..."
if command -v pre-commit &> /dev/null; then
    print_status "pre-commit already installed"
elif command -v pip3 &> /dev/null; then
    pip3 install pre-commit
elif command -v pip &> /dev/null; then
    pip install pre-commit
else
    print_warning "Could not install pre-commit - pip not found"
fi

print_step "6/6 Installing pre-commit hooks..."
if command -v pre-commit &> /dev/null; then
    pre-commit install || print_warning "pre-commit hook installation failed"
else
    print_warning "pre-commit not available, skipping hook installation"
fi

echo ""
print_status "🎉 Pre-commit setup complete!"
echo ""

print_step "Running basic checks on compilable code..."

# Check what can be formatted
print_status "Running rustfmt on shared library..."
cd shared && cargo fmt --check || (cargo fmt && print_status "Formatted shared library")
cd ..

print_status "Running rustfmt on API gateway..."
cd api-gateway && cargo fmt --check || (cargo fmt && print_status "Formatted API gateway")
cd ..

# Try to run clippy on individual components
print_status "Running clippy on shared library..."
cd shared && cargo clippy -- -D warnings || print_warning "Clippy found issues in shared library"
cd ..

print_status "Running clippy on API gateway..."
cd api-gateway && cargo clippy -- -D warnings || print_warning "Clippy found issues in API gateway"
cd ..

echo ""
print_status "✅ Pre-commit tools are now set up!"
echo ""
print_warning "Note: Frontend has compilation errors that need to be fixed before running full pre-commit checks."
echo "      Use 'make pre-commit' to run all checks once compilation issues are resolved."
echo ""
