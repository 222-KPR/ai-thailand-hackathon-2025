# Multi-stage build for queue worker - optimized
FROM python:3.11-slim AS base

# Set environment variables for optimization
ENV PYTHONDONTWRITEBYTECODE=1
ENV PYTHONUNBUFFERED=1
ENV DEBIAN_FRONTEND=noninteractive
ENV PIP_NO_CACHE_DIR=1
ENV PIP_DISABLE_PIP_VERSION_CHECK=1
ENV PYTHONFAULTHANDLER=1
ENV PYTHONHASHSEED=random

# Install minimal system dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Set work directory
WORKDIR /app

# Copy requirements and install Python dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir --upgrade pip setuptools wheel && \
    pip install --no-cache-dir -r requirements.txt

# Copy only essential application files
COPY app.py tasks.py ./

# Development stage (minimal)
FROM base AS development
CMD ["uvicorn", "app:app", "--host", "0.0.0.0", "--port", "2003", "--reload"]

# Production stage (minimal)
FROM base AS production
# Create non-root user
RUN useradd --create-home --shell /bin/bash --uid 1000 worker && \
    chown -R worker:worker /app
USER worker

# Health check with shorter timeout
HEALTHCHECK --interval=30s --timeout=15s --start-period=5s --retries=2 \
    CMD curl -f http://localhost:2003/health || exit 1

# Optimized start command
CMD ["uvicorn", "app:app", "--host", "0.0.0.0", "--port", "2003", "--workers", "1"]
