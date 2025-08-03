#!/bin/bash

# AI4Thai AI Services - Lint All Services
# Run linting and formatting for all Python services

set -e

echo "🔍 Running linting and formatting for all AI services..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to run linting for a service
lint_service() {
    local service_name=$1
    local service_dir=$2

    echo ""
    echo -e "${YELLOW}📦 Linting $service_name...${NC}"

    if [ ! -d "$service_dir" ]; then
        echo -e "${RED}❌ $service_dir not found${NC}"
        return 1
    fi

    cd "$service_dir"

    # Check if pyproject.toml exists
    if [ ! -f "pyproject.toml" ]; then
        echo -e "${RED}❌ pyproject.toml not found in $service_dir${NC}"
        cd - > /dev/null
        return 1
    fi

    # Run ruff check
    echo "   Running ruff check..."
    if uv run ruff check .; then
        echo -e "${GREEN}   ✅ Ruff check passed${NC}"
    else
        echo -e "${RED}   ❌ Ruff check failed${NC}"
        cd - > /dev/null
        return 1
    fi

    # Run ruff format check
    echo "   Checking code formatting..."
    if uv run ruff format --check .; then
        echo -e "${GREEN}   ✅ Format check passed${NC}"
    else
        echo -e "${RED}   ❌ Format check failed${NC}"
        echo "   Run 'make format' to fix formatting issues"
        cd - > /dev/null
        return 1
    fi

    echo -e "${GREEN}   ✅ $service_name linting complete${NC}"
    cd - > /dev/null
}

# Function to auto-fix issues
fix_service() {
    local service_name=$1
    local service_dir=$2

    echo ""
    echo -e "${YELLOW}🔧 Auto-fixing $service_name...${NC}"

    if [ ! -d "$service_dir" ]; then
        echo -e "${RED}❌ $service_dir not found${NC}"
        return 1
    fi

    cd "$service_dir"

    # Auto-fix with ruff
    echo "   Running ruff --fix..."
    uv run ruff check --fix .

    # Auto-format with ruff
    echo "   Running ruff format..."
    uv run ruff format .

    echo -e "${GREEN}   ✅ $service_name auto-fix complete${NC}"
    cd - > /dev/null
}

# Parse command line arguments
case "${1:-check}" in
    "check")
        echo "Running lint checks..."

        # Lint Vision Service
        lint_service "Vision Service" "vision-service"
        vision_status=$?

        # Lint Queue Worker
        lint_service "Queue Worker" "queue-worker"
        worker_status=$?

        echo ""
        if [ $vision_status -eq 0 ] && [ $worker_status -eq 0 ]; then
            echo -e "${GREEN}🎉 All services passed linting!${NC}"
            exit 0
        else
            echo -e "${RED}❌ Some services failed linting. Run '$0 fix' to auto-fix issues.${NC}"
            exit 1
        fi
        ;;

    "fix")
        echo "Auto-fixing lint issues..."

        # Fix Vision Service
        fix_service "Vision Service" "vision-service"

        # Fix Queue Worker
        fix_service "Queue Worker" "queue-worker"

        echo ""
        echo -e "${GREEN}🎉 Auto-fix complete! Run '$0 check' to verify.${NC}"
        ;;

    "help"|"-h"|"--help")
        echo "Usage: $0 [command]"
        echo ""
        echo "Commands:"
        echo "  check    Run linting checks (default)"
        echo "  fix      Auto-fix linting issues"
        echo "  help     Show this help message"
        ;;

    *)
        echo -e "${RED}❌ Unknown command: $1${NC}"
        echo "Run '$0 help' for usage information."
        exit 1
        ;;
esac
