#!/bin/bash
# Dynamic Path Detection and Docker Compose Generation
# Automatically detects correct paths and generates appropriate docker-compose file

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
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

# Function to detect current environment
detect_environment() {
    if [ -n "${CI_PROJECT_DIR:-}" ]; then
        echo "gitlab-ci"
    elif [ -n "${GITHUB_WORKSPACE:-}" ]; then
        echo "github-actions"
    elif [ -n "${JENKINS_HOME:-}" ]; then
        echo "jenkins"
    else
        echo "local"
    fi
}

# Function to detect project root
detect_project_root() {
    local current_dir="$(pwd)"

    # Look for project markers
    while [ "$current_dir" != "/" ]; do
        if [ -f "$current_dir/docker-compose.yml" ] || [ -f "$current_dir/.git/config" ] || [ -f "$current_dir/Cargo.toml" ]; then
            echo "$current_dir"
            return 0
        fi
        current_dir="$(dirname "$current_dir")"
    done

    # If not found, use current directory
    echo "$(pwd)"
}

# Function to detect service paths
detect_service_paths() {
    local project_root="$1"
    local service_name="$2"

    # Common paths to check
    local paths=(
        "$project_root/ai-services/$service_name"
        "$project_root/$service_name"
        "../$service_name"
        "./$service_name"
    )

    for path in "${paths[@]}"; do
        if [ -d "$path" ] && [ -f "$path/Dockerfile" ]; then
            echo "$path"
            return 0
        fi
    done

    print_error "Could not find $service_name directory"
    return 1
}

# Function to generate docker-compose file with correct paths
generate_compose_file() {
    local environment="$1"
    local project_root="$2"
    local output_file="$3"

    print_status "Generating docker-compose file for $environment environment"

    # Detect service paths
    local vision_path
    local queue_path

    vision_path=$(detect_service_paths "$project_root" "vision-service") || return 1
    queue_path=$(detect_service_paths "$project_root" "queue-worker") || return 1

    print_status "Vision service path: $vision_path"
    print_status "Queue worker path: $queue_path"

    # Generate compose file based on environment
    case "$environment" in
        "gitlab-ci")
            # Use CI_PROJECT_DIR for absolute paths
            vision_path="${CI_PROJECT_DIR}/ai-services/vision-service"
            queue_path="${CI_PROJECT_DIR}/ai-services/queue-worker"
            ;;
        "github-actions")
            # Use GITHUB_WORKSPACE for absolute paths
            vision_path="${GITHUB_WORKSPACE}/ai-services/vision-service"
            queue_path="${GITHUB_WORKSPACE}/ai-services/queue-worker"
            ;;
        "local")
            # Use relative paths for local development
            if [ ! -d "$vision_path" ]; then
                vision_path="../vision-service"
            fi
            if [ ! -d "$queue_path" ]; then
                queue_path="../queue-worker"
            fi
            ;;
    esac

    # Create docker-compose file
    cat > "$output_file" << EOF
# AI4Thai Crop Guardian - AI Services Deployment
# Generated automatically for $environment environment
# Generated on: $(date)

version: '3.8'

services:
  # Vision Service - Pest Detection + Disease Detection (H100 16GB Optimized)
  vision-service:
    build:
      context: $vision_path
      dockerfile: Dockerfile
      target: \${BUILD_TARGET:-h100-optimized}
      args:
        - BUILDKIT_INLINE_CACHE=1
    image: team10-vision-service:\${IMAGE_TAG:-latest}
    container_name: team10-vision-service
    ports:
      - "\${VISION_SERVICE_PORT:-2001}:2001"
    environment:
      # Model Configuration - H100 16GB Optimized
      - MODEL_CACHE_DIR=/app/models
      - HUGGINGFACE_HUB_CACHE=/app/models/hub
      - TRANSFORMERS_CACHE=/app/models/transformers

      # HuggingFace Configuration
      - HUGGINGFACE_HUB_TOKEN=\${HUGGINGFACE_HUB_TOKEN}
      - HF_HOME=/app/models

      # Service Configuration
      - SERVICE_HOST=0.0.0.0
      - SERVICE_PORT=2001
      - MAX_WORKERS=1
      - WORKER_TIMEOUT=300

      # Model Settings - Memory Optimized
      - PEST_DETECTION_MODEL=underdogquality/yolo11s-pest-detection
      - DISEASE_DETECTION_MODEL=YuchengShi/LLaVA-v1.5-7B-Plant-Leaf-Diseases-Detection
      - CONFIDENCE_THRESHOLD=0.01
      - MAX_IMAGE_SIZE=5242880  # Reduced to 5MB for memory efficiency
      - MAX_BATCH_SIZE=1

      # H100 16GB GPU Configuration
      - CUDA_VISIBLE_DEVICES=\${CUDA_VISIBLE_DEVICES:-0}
      - TORCH_CUDA_ARCH_LIST="9.0"
      - PYTORCH_CUDA_ALLOC_CONF=max_split_size_mb:2048,expandable_segments:True
      - CUDA_LAUNCH_BLOCKING=0
      - TORCH_CUDNN_V8_API_ENABLED=1

      # Memory Optimization for 16GB VRAM
      - MODEL_MAX_LENGTH=512
      - GRADIENT_CHECKPOINTING=true
      - USE_FLASH_ATTENTION=true
      - TORCH_COMPILE=false  # Disable for stability

      # Performance - Conservative for Memory
      - OMP_NUM_THREADS=2
      - MKL_NUM_THREADS=2

      # Logging
      - LOG_LEVEL=\${LOG_LEVEL:-INFO}
      - PYTHONUNBUFFERED=1

      # Redis for caching
      - REDIS_URL=\${REDIS_URL:-redis://redis:6379}

      # Monitoring
      - ENABLE_METRICS=true
      - METRICS_PORT=9001
    volumes:
      - team10-root:/root
      - team10-data:/app/models
    networks:
      - ai-services-network
    deploy:
      resources:
        limits:
          memory: \${VISION_MEMORY_LIMIT:-4G}
        reservations:
          memory: \${VISION_MEMORY_RESERVATION:-2G}
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:2001/health"]
      interval: 30s
      timeout: 15s
      retries: 2
      start_period: 60s
    restart: unless-stopped

  # Queue Worker Service - Vision Job Processing
  queue-worker:
    build:
      context: $queue_path
      dockerfile: Dockerfile
      target: \${BUILD_TARGET:-production}
      args:
        - BUILDKIT_INLINE_CACHE=1
    image: team10-queue-worker:\${IMAGE_TAG:-latest}
    container_name: team10-queue-worker
    ports:
      - "\${QUEUE_WORKER_PORT:-2003}:2003"
    environment:
      - REDIS_URL=redis://redis:6379/0
      - VISION_SERVICE_URL=http://vision-service:2001
      - MAX_IMAGE_SIZE=10485760  # 10MB
      - LOG_LEVEL=\${LOG_LEVEL:-INFO}
      - PYTHONUNBUFFERED=1
      - QUEUE_PORT=2003
    depends_on:
      - redis
      - vision-service
    volumes:
      - team10-root:/root
      - team10-data:/app/data
    networks:
      - ai-services-network
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:2003/health"]
      interval: 30s
      timeout: 15s
      retries: 3
      start_period: 30s
    restart: unless-stopped

  # Redis - Cache, Session Management, and Job Queue
  redis:
    image: redis:7-alpine
    container_name: team10-ai-redis
    command: redis-server --appendonly yes --maxmemory 4gb --maxmemory-policy allkeys-lru
    ports:
      - "\${REDIS_PORT:-6379}:6379"
    volumes:
      - team10-root:/root
      - team10-data:/data
    networks:
      - ai-services-network
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 30s
      timeout: 10s
      retries: 3
    restart: unless-stopped

networks:
  ai-services-network:
    driver: bridge
    ipam:
      config:
        - subnet: 172.20.0.0/16

volumes:
  team10-root:
    driver: local
  team10-data:
    driver: local
EOF

    print_status "Generated $output_file with correct paths for $environment"
}

# Function to validate generated compose file
validate_generated_file() {
    local compose_file="$1"

    print_status "Validating generated compose file..."

    if command -v docker-compose >/dev/null 2>&1; then
        if docker-compose -f "$compose_file" config >/dev/null 2>&1; then
            print_status "Generated compose file is valid"
            return 0
        else
            print_error "Generated compose file has syntax errors"
            return 1
        fi
    else
        print_warning "docker-compose not available for validation"
        return 0
    fi
}

# Main function
main() {
    local environment
    local project_root
    local output_file="${1:-docker-compose.generated.yml}"

    print_status "Starting dynamic path detection..."

    # Detect environment
    environment=$(detect_environment)
    print_status "Detected environment: $environment"

    # Detect project root
    project_root=$(detect_project_root)
    print_status "Detected project root: $project_root"

    # Generate compose file
    generate_compose_file "$environment" "$project_root" "$output_file"

    # Validate generated file
    validate_generated_file "$output_file"

    print_status "Path detection and compose file generation completed"
    print_status "Use: docker-compose -f $output_file up --build -d"
}

# Run main function if script is executed directly
if [ "${BASH_SOURCE[0]}" == "${0}" ]; then
    main "$@"
fi
