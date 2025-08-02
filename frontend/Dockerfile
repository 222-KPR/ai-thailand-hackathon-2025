# Production Frontend Dockerfile for AI4Thai Crop Guardian
# Multi-stage build for optimized WebAssembly bundle

# Build stage
FROM rust:1.75-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    wget \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js for build tools
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
    && apt-get install -y nodejs

# Install WebAssembly target
RUN rustup target add wasm32-unknown-unknown

# Install Trunk and wasm-opt for optimization
RUN cargo install trunk wasm-pack
RUN npm install -g wasm-opt

# Set working directory
WORKDIR /app

# Copy dependency files
COPY Cargo.toml Cargo.lock ./
COPY Trunk.toml ./

# Copy source code
COPY src/ ./src/
COPY static/ ./static/
COPY styles/ ./styles/
COPY index.html ./

# Build the application with optimizations
RUN trunk build --release

# Production stage
FROM nginx:alpine

# Copy built assets from builder stage
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy custom nginx configuration
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Add health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost/ || exit 1

# Expose port 80
EXPOSE 80

# Start nginx
CMD ["nginx", "-g", "daemon off;"]
