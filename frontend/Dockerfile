# Multi-stage Docker build for AI4Thai Crop Guardian Frontend
# Stage 1: Build environment
FROM rust:1.75-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install WebAssembly target
RUN rustup target add wasm32-unknown-unknown

# Install trunk and wasm-pack
RUN cargo install trunk wasm-pack

# Create app directory
WORKDIR /app

# Copy workspace files
COPY ../Cargo.toml /workspace/Cargo.toml
COPY ../shared /workspace/shared

# Copy frontend source
COPY Cargo.toml Trunk.toml ./
COPY src ./src
COPY index.html ./
COPY static ./static

# Set workspace root
ENV CARGO_MANIFEST_DIR=/workspace

# Build the frontend
RUN trunk build --release --public-url /

# Stage 2: Runtime environment with Nginx
FROM nginx:alpine

# Install curl for health checks
RUN apk add --no-cache curl

# Remove default nginx website
RUN rm -rf /usr/share/nginx/html/*

# Copy built frontend files
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy nginx configuration
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Copy PWA files to root if they exist
COPY static/manifest.json /usr/share/nginx/html/ || true
COPY static/sw.js /usr/share/nginx/html/ || true

# Create nginx cache directory
RUN mkdir -p /var/cache/nginx/client_temp

# Set proper permissions
RUN chown -R nginx:nginx /usr/share/nginx/html && \
    chmod -R 755 /usr/share/nginx/html

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost/ || exit 1

# Expose port
EXPOSE 80

# Start nginx
CMD ["nginx", "-g", "daemon off;"]