#!/bin/bash
# AI Services Deployment Path Validation Script
# Ensures all paths are correct before deployment

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
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

# Track validation results
VALIDATION_ERRORS=0
VALIDATION_WARNINGS=0

# Function to validate file/directory exists
validate_path() {
    local path="$1"
    local description="$2"
    local required="$3"  # true/false

    if [ -e "$path" ]; then
        print_status "$description: $path"
        return 0
    else
        if [ "$required" = "true" ]; then
            print_error "$description missing: $path"
            ((VALIDATION_ERRORS++))
            return 1
        else
            print_warning "$description missing (optional): $path"
            ((VALIDATION_WARNINGS++))
            return 1
        fi
    fi
}

# Function to validate Dockerfile and context
validate_docker_context() {
    local service_name="$1"
    local context_path="$2"
    local dockerfile_path="$3"

    print_header "Validating Docker Context: $service_name"

    # Validate context directory
    if ! validate_path "$context_path" "Context directory" "true"; then
        return 1
    fi

    # Validate Dockerfile
    local full_dockerfile_path="$context_path/$dockerfile_path"
    if ! validate_path "$full_dockerfile_path" "Dockerfile" "true"; then
        return 1
    fi

    # Check if Dockerfile has required stages
    print_status "Checking Dockerfile stages in $full_dockerfile_path..."

    local stages=($(grep -i "^FROM .* AS " "$full_dockerfile_path" | awk '{print $4}' | tr '\n' ' '))
    if [ ${#stages[@]} -eq 0 ]; then
        print_warning "No multi-stage build found in Dockerfile"
    else
        print_status "Found stages: ${stages[*]}"

        # Check for required stages
        local has_production=false
        local has_h100_optimized=false

        for stage in "${stages[@]}"; do
            if [ "$stage" = "production" ]; then
                has_production=true
            elif [ "$stage" = "h100-optimized" ]; then
                has_h100_optimized=true
            fi
        done

        if [ "$has_production" = "true" ]; then
            print_status "Production stage found"
        else
            print_error "Production stage missing"
            ((VALIDATION_ERRORS++))
        fi

        if [ "$has_h100_optimized" = "true" ]; then
            print_status "H100-optimized stage found"
        else
            print_warning "H100-optimized stage missing"
            ((VALIDATION_WARNINGS++))
        fi
    fi

    # Validate requirements.txt exists
    validate_path "$context_path/requirements.txt" "Requirements file" "true"

    return 0
}

# Function to validate docker-compose configuration
validate_compose_config() {
    print_header "Validating Docker Compose Configuration"

    local compose_file="$1"

    # Check if compose file exists
    if ! validate_path "$compose_file" "Docker Compose file" "true"; then
        return 1
    fi

    # Check compose file syntax
    print_status "Validating compose file syntax..."
    if docker-compose -f "$compose_file" config >/dev/null 2>&1; then
        print_status "Compose file syntax is valid"
    else
        print_error "Compose file syntax is invalid"
        ((VALIDATION_ERRORS++))
        echo "Running syntax check..."
        docker-compose -f "$compose_file" config
        return 1
    fi

    # Extract and validate build contexts from compose file
    print_status "Validating build contexts from compose file..."

    # Use python to parse YAML and extract build contexts
    python3 << EOF
import yaml
import sys
import os

try:
    with open('$compose_file', 'r') as f:
        compose_data = yaml.safe_load(f)

    services = compose_data.get('services', {})
    compose_dir = os.path.dirname('$compose_file')

    for service_name, service_config in services.items():
        build_config = service_config.get('build', {})
        if isinstance(build_config, str):
            context_path = build_config
            dockerfile_path = 'Dockerfile'
        elif isinstance(build_config, dict):
            context_path = build_config.get('context', '.')
            dockerfile_path = build_config.get('dockerfile', 'Dockerfile')
        else:
            continue

        # Convert relative paths to absolute
        if not os.path.isabs(context_path):
            context_path = os.path.join(compose_dir, context_path)

        print(f"{service_name}|{context_path}|{dockerfile_path}")

except Exception as e:
    print(f"ERROR: Failed to parse compose file: {e}", file=sys.stderr)
    sys.exit(1)
EOF

    return 0
}

# Function to validate environment variables
validate_environment() {
    print_header "Validating Environment Variables"

    # Check for required environment variables
    local required_vars=(
        "BUILD_TARGET"
        "IMAGE_TAG"
    )

    local optional_vars=(
        "VISION_SERVICE_PORT"
        "QUEUE_WORKER_PORT"
        "REDIS_PORT"
        "HUGGINGFACE_HUB_TOKEN"
        "VISION_MEMORY_LIMIT"
        "VISION_MEMORY_RESERVATION"
    )

    # Check required variables
    for var in "${required_vars[@]}"; do
        if [ -n "${!var:-}" ]; then
            print_status "Required environment variable: $var=${!var}"
        else
            print_warning "Required environment variable not set: $var (using default)"
            ((VALIDATION_WARNINGS++))
        fi
    done

    # Check optional variables
    for var in "${optional_vars[@]}"; do
        if [ -n "${!var:-}" ]; then
            print_status "Optional environment variable: $var=${!var}"
        else
            print_status "Optional environment variable not set: $var (using default)"
        fi
    done
}

# Function to validate Docker daemon
validate_docker() {
    print_header "Validating Docker Environment"

    # Check if Docker is running
    if docker info >/dev/null 2>&1; then
        print_status "Docker daemon is running"
    else
        print_error "Docker daemon is not running"
        ((VALIDATION_ERRORS++))
        return 1
    fi

    # Check Docker Compose
    if command -v docker-compose >/dev/null 2>&1; then
        local compose_version=$(docker-compose --version)
        print_status "Docker Compose available: $compose_version"
    else
        print_error "Docker Compose not found"
        ((VALIDATION_ERRORS++))
        return 1
    fi

    # Check available disk space
    local available_space=$(df -BG . | awk 'NR==2{print $4}' | sed 's/G//')
    if [ "$available_space" -gt 10 ]; then
        print_status "Sufficient disk space: ${available_space}GB"
    else
        print_warning "Low disk space: ${available_space}GB (recommended: >10GB)"
        ((VALIDATION_WARNINGS++))
    fi

    return 0
}

# Function to test build contexts
test_build_contexts() {
    print_header "Testing Build Contexts"

    local compose_file="$1"

    # Get build contexts from validation
    validate_compose_config "$compose_file" | while IFS='|' read -r service_name context_path dockerfile_path; do
        if [ -n "$service_name" ] && [ "$service_name" != "ERROR" ]; then
            validate_docker_context "$service_name" "$context_path" "$dockerfile_path"
        fi
    done
}

# Function to create deployment validation report
create_validation_report() {
    print_header "Deployment Validation Report"

    echo "Validation Summary:"
    echo "  Errors: $VALIDATION_ERRORS"
    echo "  Warnings: $VALIDATION_WARNINGS"
    echo ""

    if [ $VALIDATION_ERRORS -eq 0 ]; then
        print_status "✅ Deployment validation PASSED"
        echo ""
        echo "Your deployment should succeed without path errors."
        echo "You can proceed with:"
        echo "  docker-compose up --build -d"
        return 0
    else
        print_error "❌ Deployment validation FAILED"
        echo ""
        echo "Please fix the errors above before deploying."
        return 1
    fi
}

# Main validation function
main() {
    local compose_file="${1:-docker-compose.yml}"

    print_header "AI Services Deployment Path Validation"
    echo "Compose file: $compose_file"
    echo ""

    # Change to the directory containing the compose file
    local compose_dir=$(dirname "$compose_file")
    if [ "$compose_dir" != "." ]; then
        cd "$compose_dir"
        compose_file=$(basename "$compose_file")
    fi

    # Set default environment variables for validation
    export BUILD_TARGET=${BUILD_TARGET:-h100-optimized}
    export IMAGE_TAG=${IMAGE_TAG:-latest}

    # Run all validations
    validate_docker || true
    validate_environment || true
    validate_compose_config "$compose_file" || true
    test_build_contexts "$compose_file" || true

    # Generate report
    create_validation_report
}

# Run main function if script is executed directly
if [ "${BASH_SOURCE[0]}" == "${0}" ]; then
    main "$@"
fi
