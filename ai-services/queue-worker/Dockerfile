# Multi-stage build for queue worker
FROM python:3.11-slim as base

# Set environment variables
ENV PYTHONDONTWRITEBYTECODE=1
ENV PYTHONUNBUFFERED=1
ENV DEBIAN_FRONTEND=noninteractive

# Install system dependencies
RUN apt-get update && apt-get install -y \
    gcc \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Set work directory
WORKDIR /app

# Copy requirements and install Python dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy application code
COPY app.py tasks.py ./

# Development stage
FROM base as development
RUN pip install --no-cache-dir pytest pytest-asyncio
CMD ["uvicorn", "app:app", "--host", "0.0.0.0", "--port", "2003", "--reload"]

# Production stage
FROM base as production
# Create non-root user
RUN useradd --create-home --shell /bin/bash worker
USER worker

# Health check
HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:2003/health || exit 1

# Start the application
CMD ["uvicorn", "app:app", "--host", "0.0.0.0", "--port", "2003"]
