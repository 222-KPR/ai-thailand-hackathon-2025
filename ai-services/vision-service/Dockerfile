# AI4Thai Vision Service - Docker Configuration
# Multi-stage build optimized for H100 16GB VRAM

# Minimal base image with better optimization
FROM python:3.9-slim-bullseye AS base

# Set environment variables for optimization
ENV PYTHONUNBUFFERED=1
ENV PYTHONDONTWRITEBYTECODE=1
ENV DEBIAN_FRONTEND=noninteractive
ENV PIP_NO_CACHE_DIR=1
ENV PIP_DISABLE_PIP_VERSION_CHECK=1
ENV PYTHONFAULTHANDLER=1
ENV PYTHONHASHSEED=random

# Install minimal system dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    libglib2.0-0 \
    libgomp1 \
    libgl1-mesa-glx \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Create non-root user
RUN useradd --create-home --shell /bin/bash --uid 1000 app

# Set working directory
WORKDIR /app

# Copy requirements first for better caching
COPY requirements.txt .

# Install Python dependencies with optimizations and error handling
RUN pip install --no-cache-dir --upgrade pip setuptools wheel && \
    pip install --no-cache-dir -r requirements.txt && \
    python -c "import torch; print(f'PyTorch version: {torch.__version__}'); print(f'CUDA available: {torch.cuda.is_available()}')" && \
    python -c "import transformers; print(f'Transformers version: {transformers.__version__}')" && \
    python -c "import ultralytics; print(f'Ultralytics version: {ultralytics.__version__}')"

# Development stage (minimal)
FROM base AS development

# Copy only necessary source code
COPY app.py .
COPY services/ ./services/

# Change ownership
RUN chown -R app:app /app

# Switch to non-root user
USER app

# Expose port
EXPOSE 8001

# Development command
CMD ["uvicorn", "app:app", "--host", "0.0.0.0", "--port", "8001", "--reload"]

# Production stage (minimal)
FROM base AS production

# Copy only essential files
COPY app.py .
COPY services/ ./services/

# Create minimal directories
RUN mkdir -p /app/models /app/logs && \
    chown -R app:app /app

# Switch to non-root user
USER app

# Expose port
EXPOSE 2001

# Health check with shorter timeout
HEALTHCHECK --interval=30s --timeout=15s --start-period=30s --retries=2 \
    CMD curl -f http://localhost:2001/health || exit 1

# Optimized production command for H100
CMD ["gunicorn", "app:app", "-w", "1", "-k", "uvicorn.workers.UvicornWorker", "--bind", "0.0.0.0:2001", "--timeout", "300", "--max-requests", "100", "--preload"]

# H100-optimized stage for 16GB VRAM
FROM production AS h100-optimized

USER root

# Install H100-specific optimizations with error handling
RUN pip install --no-cache-dir nvidia-ml-py3>=12.535.108 && \
    python -c "import pynvml; pynvml.nvmlInit(); print(f'NVML initialized successfully')" || \
    echo "Warning: NVML not available - running in CPU mode"

USER app

# H100 16GB VRAM optimized environment
ENV CUDA_VISIBLE_DEVICES=0
ENV TORCH_CUDA_ARCH_LIST="9.0"
ENV PYTORCH_CUDA_ALLOC_CONF=max_split_size_mb:2048
ENV CUDA_LAUNCH_BLOCKING=0
ENV TORCH_CUDNN_V8_API_ENABLED=1

# Memory optimization for 16GB VRAM
ENV MODEL_MAX_LENGTH=512
ENV MAX_BATCH_SIZE=1
ENV GRADIENT_CHECKPOINTING=true
ENV USE_FLASH_ATTENTION=true

# H100 production command with memory optimizations
CMD ["gunicorn", "app:app", "-w", "1", "-k", "uvicorn.workers.UvicornWorker", "--bind", "0.0.0.0:2001", "--timeout", "300", "--max-requests", "50", "--preload", "--worker-tmp-dir", "/dev/shm"]
