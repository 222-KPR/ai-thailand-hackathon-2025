#!/bin/bash
# Local Path Testing and Deployment Script
# Test deployment paths before CI/CD

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() {
    echo -e "${GREEN}[✓]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[⚠]${NC} $1"
}

print_error() {
    echo -e "${RED}[✗]${NC} $1"
}

print_header() {
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE} $1${NC}"
    echo -e "${BLUE}========================================${NC}"
}

# Function to test local deployment
test_local_deployment() {
    print_header "Testing Local Deployment"

    # Validate paths
    print_status "Running path validation..."
    ./validate-deployment.sh docker-compose.yml || {
        print_error "Path validation failed"
        return 1
    }

    # Generate dynamic compose file
    print_status "Generating dynamic compose file..."
    ./generate-compose.sh docker-compose.test.yml || {
        print_error "Failed to generate compose file"
        return 1
    }

    # Test build without actually building
    print_status "Testing build contexts..."
    export BUILD_TARGET=production
    export IMAGE_TAG=test

    if docker-compose -f docker-compose.test.yml config >/dev/null 2>&1; then
        print_status "Compose configuration is valid"
    else
        print_error "Compose configuration is invalid"
        docker-compose -f docker-compose.test.yml config
        return 1
    fi

    # Optional: Quick build test (can be skipped to save time)
    read -p "Do you want to test actual builds? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        print_status "Testing builds..."
        docker-compose -f docker-compose.test.yml build --no-cache || {
            print_error "Build test failed"
            return 1
        }
        print_status "Build test passed"
    else
        print_status "Skipping build test"
    fi

    print_status "Local deployment test completed successfully"
}

# Function to simulate CI environment
test_ci_simulation() {
    print_header "Simulating CI Environment"

    # Set CI-like environment variables
    export CI_PROJECT_DIR="$(pwd)/../../"
    export BUILD_TARGET=h100-optimized
    export IMAGE_TAG=ci-test

    # Generate CI compose file
    print_status "Generating CI compose file..."
    ./generate-compose.sh docker-compose.ci-test.yml || {
        print_error "Failed to generate CI compose file"
        return 1
    }

    # Validate CI compose file
    print_status "Validating CI compose file..."
    ./validate-deployment.sh docker-compose.ci-test.yml || {
        print_error "CI compose validation failed"
        return 1
    }

    print_status "CI simulation test passed"
}

# Function to clean up test files
cleanup() {
    print_status "Cleaning up test files..."
    rm -f docker-compose.test.yml docker-compose.ci-test.yml docker-compose.ci-generated.yml
    print_status "Cleanup completed"
}

# Main function
main() {
    cd "$(dirname "$0")"

    print_header "AI Services Path Testing Suite"

    # Make scripts executable
    chmod +x validate-deployment.sh generate-compose.sh

    # Run tests
    test_local_deployment || exit 1
    test_ci_simulation || exit 1

    print_header "All Tests Passed!"
    print_status "Your deployment should work without path errors"
    print_status "Ready for CI/CD deployment"

    # Optional cleanup
    read -p "Clean up test files? (Y/n): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Nn]$ ]]; then
        cleanup
    fi
}

# Trap to ensure cleanup on exit
trap cleanup EXIT

# Run main function
main "$@"
