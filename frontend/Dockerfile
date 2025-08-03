# Multi-stage Dockerfile for Yew WebAssembly Frontend

# Builder stage - use latest Rust for compatibility
FROM rust:1.82-slim AS builder

# Set environment variables
ENV DEBIAN_FRONTEND=noninteractive

# Install minimal system dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Install Rust WebAssembly target and tools with compatible versions
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk@0.21.14 wasm-bindgen-cli@0.2.100

# Set working directory
WORKDIR /app

# Copy Cargo files for dependency caching
COPY Cargo.toml ./
COPY Cargo.lock* ./

# Build dependencies first for caching
RUN cargo fetch

# Copy source code
COPY src/ ./src/
COPY static/ ./static/
COPY styles/ ./styles/
COPY index.html ./
COPY Trunk.toml ./

# Development stage (for live reload)
FROM builder AS development
EXPOSE 8080
CMD ["trunk", "serve", "--address", "0.0.0.0", "--port", "8080"]

# Production build stage
FROM builder AS production-build
RUN trunk build --release

# Production stage with nginx
FROM nginx:alpine AS production
COPY nginx.conf /etc/nginx/nginx.conf
COPY --from=production-build /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
