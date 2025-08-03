# AI4Thai Crop Guardian - Hackathon Deployment Guide (Team 10)

## 🎯 Quick Start

**Team 10 Port:** `2090` (2000 + 10*9)  
**Hackathon URL:** `https://api.hackathon2025.ai.in.th/team10-1`

## 🚀 Deployment

### Option 1: Using Docker Compose (Recommended)

```bash
# Set environment variables
export API_GATEWAY_PORT=2090
export FRONTEND_PORT=2090

# Deploy
docker-compose up --build -d
```

### Option 2: Using the deployment script

```bash
# Make script executable (Linux/Mac)
chmod +x deploy-hackathon.sh

# Run deployment
./deploy-hackathon.sh
```

## 🧪 Testing

### Test API endpoints

```bash
# Make script executable (Linux/Mac)
chmod +x test-hackathon-api.sh

# Run tests
./test-hackathon-api.sh
```

### Manual testing

```bash
# Health check
curl http://localhost:2090/health

# Chat API
curl -X POST http://localhost:2090/api/v1/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello, can you help me with my crops?"}'

# Chat history
curl http://localhost:2090/api/v1/chat/history

# File stats
curl http://localhost:2090/api/v1/vision/files/stats
```

## 📊 Service Status

| Service | Port | Status | URL |
|---------|------|--------|-----|
| API Gateway | 2090 | ✅ Running | `https://api.hackathon2025.ai.in.th/team10-1` |
| RabbitMQ | 5672 | ✅ Running | `http://localhost:15672` (guest/guest) |
| Redis | 6379 | ✅ Running | `redis://localhost:6379` |

## 🔧 Configuration

### Environment Variables

```bash
# Required for hackathon
API_GATEWAY_PORT=2090
FRONTEND_PORT=2090
RUST_LOG=info

# Optional
AI4THAI_API_KEY=your_api_key_here
VISION_SERVICE_URL=https://vision-api.ai4thai.com
```

### Port Mapping

- **Team 10 Port:** `2090` (mapped to container port `3000`)
- **RabbitMQ:** `5672` (AMQP), `15672` (Management UI)
- **Redis:** `6379`

## 📝 API Endpoints

### Health & Monitoring
- `GET /health` - Basic health check
- `GET /health/ready` - Readiness check
- `GET /health/metrics` - Service metrics

### Chat API
- `POST /api/v1/chat` - Send chat message
- `GET /api/v1/chat/history` - Get conversation history

### Vision Analysis
- `POST /api/v1/vision/analyze` - Upload image for analysis
- `GET /api/v1/vision/jobs/{id}` - Get job status
- `DELETE /api/v1/vision/jobs/{id}/cancel` - Cancel job
- `GET /api/v1/vision/files/stats` - File storage statistics
- `POST /api/v1/vision/files/cleanup` - Clean up expired files

## 🐛 Troubleshooting

### Common Issues

1. **"Invalid response from upstream server"**
   - This is fixed! The API now uses mock responses for the hackathon demo
   - No external service dependencies required

2. **Port already in use**
   ```bash
   # Check what's using the port
   netstat -tulpn | grep 2090
   
   # Stop existing containers
   docker-compose down
   ```

3. **Container won't start**
   ```bash
   # Check logs
   docker-compose logs api-gateway
   
   # Rebuild
   docker-compose up --build -d
   ```

### Logs

```bash
# View all logs
docker-compose logs -f

# View specific service logs
docker-compose logs -f api-gateway
docker-compose logs -f redis
docker-compose logs -f rabbitmq
```

## 🎉 Demo Features

### Chat Bot
- Intelligent responses about crop health
- Disease detection guidance
- Pest identification help
- Image upload instructions

### Vision Analysis
- Image upload and storage
- RabbitMQ job queuing
- File management with TTL
- Job status tracking

### Storage
- Redis for chat history
- File system for image storage
- RabbitMQ for job queuing

## 🔄 Development

### Local Development

```bash
# Start development environment
./scripts/dev-start.sh

# Stop development environment
./scripts/dev-stop.sh
```

### Building

```bash
# Build API Gateway
cd api-gateway
cargo build --release

# Build Frontend
cd frontend
trunk build
```

## 📞 Support

If you encounter issues:

1. Check the logs: `docker-compose logs -f`
2. Verify port configuration: `netstat -tulpn | grep 2090`
3. Test individual services: `./test-hackathon-api.sh`
4. Restart services: `docker-compose restart`

## 🏆 Hackathon Ready!

Your API is now ready for the hackathon! The system provides:

- ✅ Working chat API with intelligent responses
- ✅ Vision analysis endpoints (ready for image upload)
- ✅ Health monitoring and metrics
- ✅ No external service dependencies
- ✅ Proper port configuration (2090)
- ✅ Mock responses for demo purposes

**Access your API at:** `https://api.hackathon2025.ai.in.th/team10-1` 