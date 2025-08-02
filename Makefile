# Makefile for AI4Thai Crop Guardian
.PHONY: fmt check clippy fix test pre-commit clean setup install-tools dylint help

# Default target
help:
	@echo "AI4Thai Crop Guardian - Development Commands"
	@echo ""
	@echo "Setup Commands:"
	@echo "  setup          - Setup development environment"
	@echo "  install-tools  - Install required Rust tools"
	@echo ""
	@echo "Code Quality Commands:"
	@echo "  fmt            - Format code with rustfmt"
	@echo "  check          - Check code without building"
	@echo "  clippy         - Run clippy linter"
	@echo "  fix            - Fix code automatically with cargo fix"
	@echo "  dylint         - Run dylint additional lints"
	@echo "  test           - Run all tests"
	@echo "  pre-commit     - Run all pre-commit checks"
	@echo "  full-check     - Run complete code quality check"
	@echo ""
	@echo "Utility Commands:"
	@echo "  clean          - Clean build artifacts"
	@echo "  wasm-check     - Check WASM compilation for frontend"
	@echo ""

# Setup development environment
setup: install-tools
	@echo "🔧 Setting up development environment..."
	@if command -v pre-commit >/dev/null 2>&1; then \
		echo "✅ pre-commit already installed"; \
	else \
		echo "📦 Installing pre-commit..."; \
		pip install pre-commit || pip3 install pre-commit; \
	fi
	@echo "🔗 Installing pre-commit hooks..."
	@pre-commit install
	@echo "✅ Development environment setup complete!"

# Install required Rust tools
install-tools:
	@echo "📦 Installing Rust tools..."
	@rustup component add rustfmt clippy
	@echo "📦 Installing dylint..."
	@cargo install cargo-dylint dylint-link || echo "⚠️  dylint installation failed (optional)"
	@echo "📦 Installing WASM target..."
	@rustup target add wasm32-unknown-unknown
	@echo "📦 Installing trunk for frontend..."
	@cargo install trunk || echo "⚠️  trunk installation failed"
	@echo "✅ Tools installation complete!"

# Format code
fmt:
	@echo "🎨 Formatting code..."
	@cargo fmt --all

# Check code without building
check:
	@echo "🔍 Checking code..."
	@cargo check --workspace --all-targets --all-features

# Run clippy
clippy:
	@echo "📎 Running clippy..."
	@cargo clippy --workspace --all-targets --all-features -- -D warnings

# Fix code automatically
fix:
	@echo "🔧 Running cargo fix..."
	@cargo fix --workspace --all-targets --all-features --allow-dirty --allow-staged

# Run tests
test:
	@echo "🧪 Running tests..."
	@cargo test --workspace

# Run dylint
dylint:
	@echo "🔍 Running dylint..."
	@if command -v cargo-dylint >/dev/null 2>&1; then \
		cargo dylint --workspace --all; \
	else \
		echo "⚠️  dylint not installed, skipping..."; \
		echo "   Install with: cargo install cargo-dylint dylint-link"; \
	fi

# Check WASM compilation for frontend
wasm-check:
	@echo "🌐 Checking WASM compilation..."
	@if [ -d "frontend" ]; then \
		cd frontend && cargo check --target wasm32-unknown-unknown; \
	else \
		echo "⚠️  Frontend directory not found"; \
	fi

# Run all pre-commit checks
pre-commit:
	@echo "🚀 Running pre-commit checks..."
	@./scripts/pre-commit-rust.sh

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	@cargo clean

# Full check (format, fix, clippy, test)
full-check: fix fmt clippy dylint test wasm-check
	@echo ""
	@echo "✅ All checks passed! Your code is ready."
	@echo ""

# Quick check for CI
ci-check: check clippy test
	@echo "✅ CI checks passed!"

# Development workflow - run before committing
dev-check: fix fmt clippy test
	@echo "✅ Development checks passed!"
