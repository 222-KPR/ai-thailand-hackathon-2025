#!/bin/bash
# AI4Thai Docker Validation Script
# Run before pushing to GitLab CI to catch build issues early

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print status
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to validate Dockerfile syntax
validate_dockerfile() {
    local dockerfile_path="$1"
    local service_name="$2"

    print_status "Validating $service_name Dockerfile syntax..."

    # Check if Dockerfile exists
    if [[ ! -f "$dockerfile_path" ]]; then
        print_error "Dockerfile not found: $dockerfile_path"
        return 1
    fi

    # Check for common Docker anti-patterns
    local issues=0

    # Check FROM casing consistency
    if grep -q "FROM.*as " "$dockerfile_path"; then
        print_warning "$service_name: Inconsistent FROM...AS casing found"
        ((issues++))
    fi

    # Check for pip cache purge with --no-cache-dir
    if grep -q "pip.*--no-cache-dir.*pip cache purge" "$dockerfile_path"; then
        print_error "$service_name: pip cache purge used with --no-cache-dir (will fail)"
        ((issues++))
    fi

    # Check for root user in production
    if grep -A5 -B5 "FROM.*production" "$dockerfile_path" | grep -q "USER root" && ! grep -A10 "USER root" "$dockerfile_path" | grep -q "USER [^r]"; then
        print_warning "$service_name: Running as root in production stage"
    fi

    # Validate with hadolint if available
    if command -v hadolint >/dev/null 2>&1; then
        print_status "Running hadolint on $service_name..."
        if ! hadolint "$dockerfile_path"; then
            print_warning "hadolint found issues in $service_name Dockerfile"
            ((issues++))
        fi
    else
        print_warning "hadolint not installed - skipping advanced validation"
    fi

    if [[ $issues -eq 0 ]]; then
        print_status "$service_name Dockerfile validation passed"
        return 0
    else
        print_error "$service_name Dockerfile validation failed with $issues issues"
        return 1
    fi
}

# Function to validate requirements.txt
validate_requirements() {
    local requirements_path="$1"
    local service_name="$2"

    print_status "Validating $service_name requirements.txt..."

    if [[ ! -f "$requirements_path" ]]; then
        print_error "Requirements file not found: $requirements_path"
        return 1
    fi

    # Check for conflicting versions
    local issues=0

    # Check for duplicate packages
    if sort "$requirements_path" | uniq -d | grep -v "^#" | grep -v "^$" | head -1 >/dev/null; then
        print_error "$service_name: Duplicate packages found in requirements.txt"
        sort "$requirements_path" | uniq -d
        ((issues++))
    fi

    # Check for very old versions that might have security issues
    if grep -E "(torch==1\.|transformers==3\.|fastapi==0\.[0-8])" "$requirements_path"; then
        print_warning "$service_name: Very old package versions detected - consider updating"
    fi

    # Validate package names (basic check)
    if grep -E "^[^a-zA-Z0-9_-]" "$requirements_path" | grep -v "^#" | grep -v "^$" | head -1 >/dev/null; then
        print_error "$service_name: Invalid package names found"
        ((issues++))
    fi

    if [[ $issues -eq 0 ]]; then
        print_status "$service_name requirements.txt validation passed"
        return 0
    else
        print_error "$service_name requirements.txt validation failed"
        return 1
    fi
}

# Function to test Docker build
test_docker_build() {
    local context_path="$1"
    local service_name="$2"
    local target="${3:-production}"

    print_status "Testing $service_name Docker build (target: $target)..."

    # Build with build kit for better error messages
    export DOCKER_BUILDKIT=1

    local image_name="ai4thai-$service_name:test"

    if docker build \
        --target "$target" \
        --tag "$image_name" \
        --file "$context_path/Dockerfile" \
        --progress=plain \
        "$context_path"; then
        print_status "$service_name build successful"

        # Clean up test image
        docker rmi "$image_name" >/dev/null 2>&1 || true
        return 0
    else
        print_error "$service_name build failed"
        return 1
    fi
}

# Function to validate docker-compose
validate_docker_compose() {
    local compose_path="$1"

    print_status "Validating docker-compose.yml..."

    if [[ ! -f "$compose_path" ]]; then
        print_error "docker-compose.yml not found: $compose_path"
        return 1
    fi

    # Validate compose file syntax
    if docker-compose -f "$compose_path" config >/dev/null 2>&1; then
        print_status "docker-compose.yml syntax validation passed"
        return 0
    else
        print_error "docker-compose.yml syntax validation failed"
        docker-compose -f "$compose_path" config
        return 1
    fi
}

# Main validation function
main() {
    print_status "Starting AI4Thai Docker validation..."

    local validation_errors=0
    local base_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

    # Validate Vision Service
    print_status "=== Validating Vision Service ==="
    validate_dockerfile "$base_dir/vision-service/Dockerfile" "vision-service" || ((validation_errors++))
    validate_requirements "$base_dir/vision-service/requirements.txt" "vision-service" || ((validation_errors++))

    # Validate Queue Worker
    print_status "=== Validating Queue Worker ==="
    validate_dockerfile "$base_dir/queue-worker/Dockerfile" "queue-worker" || ((validation_errors++))
    validate_requirements "$base_dir/queue-worker/requirements.txt" "queue-worker" || ((validation_errors++))

    # Validate docker-compose
    print_status "=== Validating Docker Compose ==="
    validate_docker_compose "$base_dir/deployment/docker-compose.yml" || ((validation_errors++))

    # Test builds if requested
    if [[ "${1:-}" == "--build" ]]; then
        print_status "=== Testing Docker Builds ==="
        test_docker_build "$base_dir/vision-service" "vision-service" "h100-optimized" || ((validation_errors++))
        test_docker_build "$base_dir/queue-worker" "queue-worker" "production" || ((validation_errors++))
    fi

    # Summary
    echo ""
    if [[ $validation_errors -eq 0 ]]; then
        print_status "✅ All Docker validations passed!"
        echo -e "${GREEN}Ready for GitLab CI deployment${NC}"
        exit 0
    else
        print_error "❌ Found $validation_errors validation errors"
        echo -e "${RED}Fix issues before pushing to GitLab CI${NC}"
        exit 1
    fi
}

# Help function
show_help() {
    cat << EOF
AI4Thai Docker Validation Script

Usage: $0 [OPTIONS]

OPTIONS:
    --build     Also test Docker builds (requires Docker)
    --help      Show this help message

Examples:
    $0                  # Validate syntax only
    $0 --build         # Validate and test builds

This script validates:
- Dockerfile syntax and best practices
- requirements.txt format and dependencies
- docker-compose.yml syntax
- Optional: actual Docker builds

EOF
}

# Parse arguments
case "${1:-}" in
    --help|-h)
        show_help
        exit 0
        ;;
    --build)
        main "$@"
        ;;
    "")
        main
        ;;
    *)
        print_error "Unknown option: $1"
        show_help
        exit 1
        ;;
esac
