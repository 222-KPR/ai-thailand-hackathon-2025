#!/bin/bash
# Docker Image Size Optimization Script
# Reduces Docker image sizes for GitLab CI artifact limits

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to get image size in MB
get_image_size_mb() {
    local image_name="$1"
    docker images --format "table {{.Repository}}:{{.Tag}}\t{{.Size}}" | grep "$image_name" | awk '{print $2}' | sed 's/GB/*1024/g; s/MB//g; s/KB\/1024/g' | bc 2>/dev/null || echo "0"
}

# Function to optimize single image
optimize_image() {
    local image_name="$1"
    local temp_name="${image_name}-temp"

    print_status "Optimizing image: $image_name"

    # Get original size
    local original_size=$(get_image_size_mb "$image_name")

    # Create optimized container
    print_status "Creating optimized container..."
    docker run --name temp-optimize "$image_name" /bin/sh -c "
        # Remove package caches and temporary files
        rm -rf /tmp/* /var/tmp/* || true
        rm -rf /root/.cache/* || true
        rm -rf /var/lib/apt/lists/* || true

        # Remove Python cache files
        find /usr/local -name '*.pyc' -delete || true
        find /usr/local -name '__pycache__' -type d -exec rm -rf {} + || true
        find /usr/local -name '*.pyo' -delete || true

        # Remove development and test files
        find /usr/local -name 'test' -type d -exec rm -rf {} + || true
        find /usr/local -name 'tests' -type d -exec rm -rf {} + || true
        find /usr/local -name '*.dev*' -delete || true

        # Remove documentation and examples
        find /usr/local -name 'doc' -type d -exec rm -rf {} + || true
        find /usr/local -name 'docs' -type d -exec rm -rf {} + || true
        find /usr/local -name 'examples' -type d -exec rm -rf {} + || true
        find /usr/local -name '*.md' -delete || true
        find /usr/local -name 'README*' -delete || true

        # Remove locale files (keep English only)
        find /usr/share/locale -mindepth 1 -maxdepth 1 ! -name 'en*' -exec rm -rf {} + || true

        echo 'Container optimization completed'
    "

    # Commit optimized container
    print_status "Committing optimized image..."
    docker commit temp-optimize "$temp_name"

    # Remove temp container
    docker rm temp-optimize || true

    # Replace original with optimized
    docker tag "$temp_name" "$image_name"
    docker rmi "$temp_name" || true

    # Get new size
    local new_size=$(get_image_size_mb "$image_name")
    local saved_mb=$(echo "$original_size - $new_size" | bc 2>/dev/null || echo "0")

    print_status "Image $image_name optimized: ${original_size}MB -> ${new_size}MB (saved ${saved_mb}MB)"
}

# Function to check if image size is within limits
check_size_limits() {
    local image_name="$1"
    local max_size_mb="$2"

    local size_mb=$(get_image_size_mb "$image_name")

    if (( $(echo "$size_mb > $max_size_mb" | bc -l) )); then
        print_error "Image $image_name is ${size_mb}MB, exceeds limit of ${max_size_mb}MB"
        return 1
    else
        print_status "Image $image_name is ${size_mb}MB, within limit of ${max_size_mb}MB"
        return 0
    fi
}

# Main optimization function
main() {
    print_status "Starting Docker image optimization for GitLab CI..."

    # Check if images exist
    local vision_image="team10-vision-service:latest"
    local queue_image="team10-queue-worker:latest"

    if ! docker images | grep -q "team10-vision-service"; then
        print_error "Vision service image not found"
        exit 1
    fi

    if ! docker images | grep -q "team10-queue-worker"; then
        print_error "Queue worker image not found"
        exit 1
    fi

    # Show original sizes
    print_status "Original image sizes:"
    docker images | grep team10

    # Optimize images
    optimize_image "$queue_image"
    optimize_image "$vision_image"

    # Final size check
    print_status "Optimized image sizes:"
    docker images | grep team10

    # Check against GitLab artifact limits (typically 1GB per artifact)
    local max_size_mb=800  # Conservative limit for compressed artifacts

    print_status "Checking size limits..."
    check_size_limits "$queue_image" "$max_size_mb"
    check_size_limits "$vision_image" "$max_size_mb"

    print_status "Image optimization completed successfully!"
}

# Run optimization
main "$@"
