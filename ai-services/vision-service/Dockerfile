# AI4Thai Vision Service - Docker Configuration
# Multi-stage build for optimized production image

# Base image with CUDA support
FROM nvidia/cuda:11.8-devel-ubuntu22.04 as base

# Set environment variables
ENV PYTHONUNBUFFERED=1
ENV PYTHONDONTWRITEBYTECODE=1
ENV DEBIAN_FRONTEND=noninteractive

# Install system dependencies
RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    python3-dev \
    build-essential \
    curl \
    wget \
    git \
    libglib2.0-0 \
    libsm6 \
    libxext6 \
    libxrender-dev \
    libgomp1 \
    libgl1-mesa-glx \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd --create-home --shell /bin/bash app

# Set working directory
WORKDIR /app

# Copy requirements first for better caching
COPY requirements.txt .

# Install Python dependencies
RUN pip3 install --no-cache-dir --upgrade pip && \
    pip3 install --no-cache-dir -r requirements.txt

# Development stage
FROM base as development

# Install development dependencies
RUN pip3 install --no-cache-dir \
    jupyter \
    ipython \
    notebook

# Copy source code
COPY . .

# Change ownership to app user
RUN chown -R app:app /app

# Switch to non-root user
USER app

# Expose port
EXPOSE 8001

# Development command
CMD ["uvicorn", "app:app", "--host", "0.0.0.0", "--port", "8001", "--reload"]

# Production stage
FROM base as production

# Copy source code
COPY . .

# Create necessary directories
RUN mkdir -p /app/models /app/logs /app/temp && \
    chown -R app:app /app

# Switch to non-root user
USER app

# Download and cache models (optional - can be done at runtime)
# RUN python3 -c "from models.model_registry import ModelRegistry; ModelRegistry.download_models()"

# Expose port
EXPOSE 8001

# Health check
HEALTHCHECK --interval=30s --timeout=30s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:8001/health || exit 1

# Production command
CMD ["gunicorn", "app:app", "-w", "1", "-k", "uvicorn.workers.UvicornWorker", "--bind", "0.0.0.0:8001", "--timeout", "120"]

# GPU-optimized production stage
FROM production as gpu-production

# Install additional GPU optimizations
USER root
RUN pip3 install --no-cache-dir \
    nvidia-ml-py3 \
    pynvml

USER app

# GPU-specific environment variables
ENV CUDA_VISIBLE_DEVICES=0
ENV TORCH_CUDA_ARCH_LIST="6.0;6.1;7.0;7.5;8.0;8.6"

# GPU production command with optimizations
CMD ["gunicorn", "app:app", "-w", "1", "-k", "uvicorn.workers.UvicornWorker", "--bind", "0.0.0.0:8001", "--timeout", "120", "--preload"]
