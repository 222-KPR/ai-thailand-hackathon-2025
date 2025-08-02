#!/bin/bash
# AI4Thai - H100 16GB Optimization Build Script

set -e

echo "🚀 Building AI Services optimized for H100 16GB VRAM..."

# Enable Docker BuildKit for better optimization
export DOCKER_BUILDKIT=1
export BUILDKIT_PROGRESS=plain

cd "$(dirname "$0")"

# Build vision service with H100 optimization
echo "🔧 Building Vision Service (H100 optimized)..."
docker build \
    --target h100-optimized \
    --compress \
    --no-cache \
    --tag team10-vision-service:h100-latest \
    ./ai-services/vision-service/

# Build queue worker (production optimized)
echo "🔧 Building Queue Worker (optimized)..."
docker build \
    --target production \
    --compress \
    --no-cache \
    --tag team10-queue-worker:latest \
    ./ai-services/queue-worker/

# Clean up intermediate images
echo "🧹 Cleaning up intermediate images..."
docker image prune -f

# Show image sizes
echo "📊 Image sizes:"
docker images | grep team10

# Test memory usage
echo "🧠 Testing memory configuration..."
docker run --rm --gpus all \
    -e CUDA_VISIBLE_DEVICES=0 \
    team10-vision-service:h100-latest \
    python -c "
import torch
print(f'CUDA available: {torch.cuda.is_available()}')
if torch.cuda.is_available():
    print(f'GPU count: {torch.cuda.device_count()}')
    print(f'Current device: {torch.cuda.current_device()}')
    print(f'Device name: {torch.cuda.get_device_name()}')
    print(f'Memory allocated: {torch.cuda.memory_allocated() / 1024**3:.2f} GB')
    print(f'Memory reserved: {torch.cuda.memory_reserved() / 1024**3:.2f} GB')
"

echo "✅ Build completed successfully!"
echo "💡 To deploy: cd ai-services/deployment && docker-compose up -d"
