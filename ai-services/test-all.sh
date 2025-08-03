#!/bin/bash
"""
AI Services Test Runner
Comprehensive testing script for all AI services
"""

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
AI_SERVICES_DIR="$SCRIPT_DIR"

echo -e "${BLUE}🧪 AI4Thai Services Test Suite${NC}"
echo "========================================"

# Function to print section headers
print_section() {
    echo -e "\n${YELLOW}📋 $1${NC}"
    echo "----------------------------------------"
}

# Function to check command exists
check_command() {
    if ! command -v $1 &> /dev/null; then
        echo -e "${RED}❌ $1 is not installed${NC}"
        exit 1
    fi
}

# Check prerequisites
print_section "Checking Prerequisites"
check_command uv
check_command python3

# Navigate to AI services directory
cd "$AI_SERVICES_DIR"

# Install test dependencies
print_section "Installing Test Dependencies"
echo "Installing Python dependencies with uv..."
if uv sync --group test; then
    echo -e "${GREEN}✅ Dependencies installed successfully${NC}"
else
    echo -e "${RED}❌ Failed to install dependencies${NC}"
    exit 1
fi

# Run ruff checks
print_section "Code Quality Checks"
echo "Running ruff format check..."
if uv run ruff format --check .; then
    echo -e "${GREEN}✅ Code formatting is correct${NC}"
else
    echo -e "${YELLOW}⚠️  Code formatting issues found. Running formatter...${NC}"
    uv run ruff format .
    echo -e "${GREEN}✅ Code formatted${NC}"
fi

echo "Running ruff linter..."
if uv run ruff check --fix .; then
    echo -e "${GREEN}✅ No linting issues found${NC}"
else
    echo -e "${YELLOW}⚠️  Linting issues found and fixed${NC}"
fi

# Run unit tests
print_section "Running Unit Tests"

# Test Vision Service
echo "Testing Vision Service..."
cd vision-service
if uv run pytest test_unit.py -v --tb=short; then
    echo -e "${GREEN}✅ Vision Service tests passed${NC}"
    VISION_TESTS_PASSED=true
else
    echo -e "${RED}❌ Vision Service tests failed${NC}"
    VISION_TESTS_PASSED=false
fi

# Test Queue Worker
echo -e "\nTesting Queue Worker..."
cd ../queue-worker
if uv run pytest test_unit.py -v --tb=short; then
    echo -e "${GREEN}✅ Queue Worker tests passed${NC}"
    QUEUE_TESTS_PASSED=true
else
    echo -e "${RED}❌ Queue Worker tests failed${NC}"
    QUEUE_TESTS_PASSED=false
fi

# Return to root
cd ..

# Run integration tests if unit tests pass
if $VISION_TESTS_PASSED && $QUEUE_TESTS_PASSED; then
    print_section "Running Integration Tests"

    # Check if services are running
    echo "Checking if services are available for integration tests..."

    # Test vision service health
    if curl -f -s http://localhost:2001/health > /dev/null 2>&1; then
        echo -e "${GREEN}✅ Vision Service is running${NC}"
        VISION_RUNNING=true
    else
        echo -e "${YELLOW}⚠️  Vision Service not running - skipping integration tests${NC}"
        VISION_RUNNING=false
    fi

    # Test queue worker health
    if curl -f -s http://localhost:2003/health > /dev/null 2>&1; then
        echo -e "${GREEN}✅ Queue Worker is running${NC}"
        QUEUE_RUNNING=true
    else
        echo -e "${YELLOW}⚠️  Queue Worker not running - skipping integration tests${NC}"
        QUEUE_RUNNING=false
    fi

    # Run integration tests if services are running
    if $VISION_RUNNING && $QUEUE_RUNNING; then
        cd vision-service
        if python test_vision_service.py; then
            echo -e "${GREEN}✅ Integration tests passed${NC}"
            INTEGRATION_PASSED=true
        else
            echo -e "${RED}❌ Integration tests failed${NC}"
            INTEGRATION_PASSED=false
        fi
        cd ..
    else
        echo -e "${YELLOW}⚠️  Skipping integration tests - services not running${NC}"
        INTEGRATION_PASSED="skipped"
    fi
else
    echo -e "${YELLOW}⚠️  Skipping integration tests - unit tests failed${NC}"
    INTEGRATION_PASSED="skipped"
fi

# Generate coverage report
print_section "Generating Coverage Report"
echo "Generating test coverage report..."
if uv run pytest --cov=vision-service --cov=queue-worker --cov=shared --cov-report=html:htmlcov --cov-report=term-missing; then
    echo -e "${GREEN}✅ Coverage report generated in htmlcov/index.html${NC}"
else
    echo -e "${YELLOW}⚠️  Coverage report generation had issues${NC}"
fi

# Summary
print_section "Test Summary"
echo "Results:"
echo "----------------------------------------"

if $VISION_TESTS_PASSED; then
    echo -e "Vision Service Tests:     ${GREEN}✅ PASSED${NC}"
else
    echo -e "Vision Service Tests:     ${RED}❌ FAILED${NC}"
fi

if $QUEUE_TESTS_PASSED; then
    echo -e "Queue Worker Tests:       ${GREEN}✅ PASSED${NC}"
else
    echo -e "Queue Worker Tests:       ${RED}❌ FAILED${NC}"
fi

case $INTEGRATION_PASSED in
    true)
        echo -e "Integration Tests:        ${GREEN}✅ PASSED${NC}"
        ;;
    false)
        echo -e "Integration Tests:        ${RED}❌ FAILED${NC}"
        ;;
    "skipped")
        echo -e "Integration Tests:        ${YELLOW}⚠️  SKIPPED${NC}"
        ;;
esac

echo "----------------------------------------"

# Exit with appropriate code
if $VISION_TESTS_PASSED && $QUEUE_TESTS_PASSED; then
    echo -e "${GREEN}🎉 All critical tests passed!${NC}"
    echo "View coverage report: open htmlcov/index.html"
    exit 0
else
    echo -e "${RED}💥 Some tests failed!${NC}"
    exit 1
fi
