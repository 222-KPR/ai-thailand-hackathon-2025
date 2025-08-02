# Frontend Docker Development Guide

This guide explains how to run the AI4Thai Crop Guardian frontend using Docker for development and production.

## 🚀 Quick Start

### Prerequisites
- Docker 20.10+
- Docker Compose 2.0+
- 4GB+ RAM available for containers

### Development Mode (Recommended)

```bash
# Start frontend development server with hot reload
./scripts/dev-frontend-docker.sh start

# Or start with full backend stack
./scripts/dev-frontend-docker.sh start-fullstack
```

Access the application at: **http://localhost:8080**

## 📋 Available Commands

### Basic Commands
```bash
# Start frontend only
./scripts/dev-frontend-docker.sh start

# Start with backend services (API Gateway, Database, Redis)
./scripts/dev-frontend-docker.sh start-fullstack

# Stop all services
./scripts/dev-frontend-docker.sh stop

# Restart frontend service
./scripts/dev-frontend-docker.sh restart

# View logs
./scripts/dev-frontend-docker.sh logs

# Follow logs in real-time
./scripts/dev-frontend-docker.sh logs -f
```

### Development Commands
```bash
# Open shell in frontend container
./scripts/dev-frontend-docker.sh shell

# Run tests in container
./scripts/dev-frontend-docker.sh test

# Build Docker image
./scripts/dev-frontend-docker.sh build

# Rebuild without cache
./scripts/dev-frontend-docker.sh build --no-cache

# Check service status
./scripts/dev-frontend-docker.sh status

# Clean up containers and volumes
./scripts/dev-frontend-docker.sh clean
```

## 🏗️ Docker Architecture

### Development Setup
- **Hot Reload**: Source code changes trigger automatic rebuilds
- **Volume Mounting**: Local files mounted for instant updates
- **Cargo Caching**: Rust dependencies cached for faster builds
- **Port Mapping**: Frontend accessible on localhost:8080

### Services Available

#### Frontend Only Mode
- **frontend-dev**: Yew WebAssembly development server (port 8080)

#### Full-Stack Mode
- **frontend-dev**: Frontend development server (port 8080)
- **api-gateway**: Rust API Gateway (port 3000)
- **postgres**: PostgreSQL database (port 5432)
- **redis**: Redis cache (port 6379)

## 🔧 Configuration

### Environment Variables
Create a `.env` file in the frontend directory:

```bash
# Development configuration
RUST_LOG=debug
TRUNK_SERVE_HOST=0.0.0.0
TRUNK_SERVE_PORT=8080

# API Gateway URL (for full-stack mode)
API_GATEWAY_URL=http://api-gateway:3000

# External API keys
AI4THAI_API_KEY=your_api_key_here
WEATHER_API_KEY=your_weather_key_here
```

### Docker Compose Profiles

#### Default Profile (Frontend Only)
```bash
docker-compose -f docker-compose.dev.yml up
```

#### Full-Stack Profile
```bash
docker-compose -f docker-compose.dev.yml --profile fullstack up
```

## 📊 Performance Optimization

### Build Optimization
- **Multi-stage builds**: Separate build and runtime environments
- **Layer caching**: Optimized Dockerfile layer ordering
- **Dependency caching**: Cargo registry and target directory cached
- **Asset optimization**: WebAssembly bundle optimization with wasm-opt

### Development Optimization
- **Volume mounting**: Source code changes without container rebuilds
- **Incremental compilation**: Rust incremental compilation enabled
- **Hot reload**: Trunk serves with automatic reload on changes

## 🧪 Testing in Docker

### Run Tests
```bash
# Run all frontend tests
./scripts/dev-frontend-docker.sh test

# Run specific test suite
docker-compose -f docker-compose.dev.yml exec frontend-dev \
  wasm-pack test --headless --firefox -- --test component_tests

# Run with coverage
docker-compose -f docker-compose.dev.yml exec frontend-dev \
  cargo tarpaulin --out Html --output-dir coverage/
```

### Test Environment
- **Browser Testing**: Firefox headless for WebAssembly tests
- **WASM Testing**: wasm-bindgen-test for component testing
- **Coverage**: Tarpaulin for code coverage reporting

## 🔍 Debugging

### View Logs
```bash
# Frontend logs
./scripts/dev-frontend-docker.sh logs -f

# All services logs
docker-compose -f docker-compose.dev.yml --profile fullstack logs -f

# Specific service logs
docker-compose -f docker-compose.dev.yml logs -f api-gateway
```

### Debug in Container
```bash
# Open shell in frontend container
./scripts/dev-frontend-docker.sh shell

# Check Rust installation
docker-compose -f docker-compose.dev.yml exec frontend-dev rustc --version

# Check WebAssembly target
docker-compose -f docker-compose.dev.yml exec frontend-dev \
  rustup target list --installed

# Check Trunk installation
docker-compose -f docker-compose.dev.yml exec frontend-dev trunk --version
```

### Common Issues

#### Port Already in Use
```bash
# Check what's using port 8080
lsof -i :8080

# Kill process using port
kill -9 $(lsof -t -i:8080)

# Or use different port
FRONTEND_PORT=8081 ./scripts/dev-frontend-docker.sh start
```

#### Container Build Failures
```bash
# Clean build without cache
./scripts/dev-frontend-docker.sh build --no-cache

# Clean up Docker system
docker system prune -a

# Check Docker resources
docker system df
```

#### WebAssembly Compilation Issues
```bash
# Check Rust toolchain in container
./scripts/dev-frontend-docker.sh shell
rustup show

# Reinstall WebAssembly target
rustup target add wasm32-unknown-unknown

# Check Trunk configuration
cat Trunk.toml
```

## 🚀 Production Deployment

### Build Production Image
```bash
# Build optimized production image
docker build -f Dockerfile -t ai4thai-frontend:latest .

# Run production container
docker run -p 80:80 ai4thai-frontend:latest
```

### Production Features
- **Nginx**: High-performance web server
- **Gzip Compression**: Reduced bundle sizes
- **Security Headers**: XSS protection, CSRF protection
- **Caching**: Optimized caching for static assets
- **Health Checks**: Container health monitoring

## 📈 Monitoring

### Container Metrics
```bash
# Check resource usage
./scripts/dev-frontend-docker.sh status

# Detailed container stats
docker stats

# Container health
docker-compose -f docker-compose.dev.yml ps
```

### Application Metrics
- **Bundle Size**: Monitor WebAssembly bundle size
- **Build Time**: Track compilation performance
- **Memory Usage**: Monitor container memory usage
- **CPU Usage**: Track CPU utilization

## 🔧 Customization

### Custom Dockerfile
Create `Dockerfile.custom` for specific requirements:

```dockerfile
FROM ai4thai-frontend-dev:latest

# Add custom tools
RUN cargo install custom-tool

# Custom configuration
COPY custom-config.toml /app/

# Custom startup script
COPY custom-start.sh /app/
RUN chmod +x /app/custom-start.sh

CMD ["/app/custom-start.sh"]
```

### Custom Docker Compose
Create `docker-compose.custom.yml`:

```yaml
version: '3.8'

services:
  frontend-dev:
    extends:
      file: docker-compose.dev.yml
      service: frontend-dev
    environment:
      - CUSTOM_ENV_VAR=value
    volumes:
      - ./custom-config:/app/config
```

## 📚 Additional Resources

- [Trunk Documentation](https://trunkrs.dev/)
- [Yew Framework Guide](https://yew.rs/)
- [WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [Docker Best Practices](https://docs.docker.com/develop/dev-best-practices/)

---

For issues or questions, check the main project documentation or create an issue in the repository.
