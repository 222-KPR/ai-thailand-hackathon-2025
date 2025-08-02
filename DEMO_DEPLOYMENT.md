# 🌾 AI4Thai Crop Guardian - Demo Deployment

> Simplified deployment for demonstration with external AI services provided by AI4Thai

## 🎯 Quick Demo Setup

### Prerequisites
- Docker and Docker Compose installed
- AI4Thai API key (contact AI4Thai team)
- Internet connection

### 1-Minute Setup

```bash
# Clone and setup
git clone <repository-url>
cd ai4thai-crop-guardian

# Configure API key
cp .env.example .env
# Edit .env and set: AI4THAI_API_KEY=your_actual_api_key

# Start demo
./scripts/demo-start.sh
```

### Access Demo
- **Frontend**: http://localhost:8080
- **API**: http://localhost:3000

## 🏗️ Architecture

```
┌─────────────────────────────────────┐
│           Local Services            │
│  ┌─────────┐  ┌─────────┐  ┌──────┐ │
│  │Frontend │  │   API   │  │Queue │ │
│  │  :8080  │  │Gateway  │  │Worker│ │
│  │         │  │  :3000  │  │      │ │
│  └─────────┘  └─────────┘  └──────┘ │
│  ┌─────────┐  ┌─────────┐           │
│  │PostgreSQL│ │  Redis  │           │
│  │  :5432  │  │  :6379  │           │
│  └─────────┘  └─────────┘           │
└─────────────────────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────┐
│        AI4Thai Services             │
│  ┌─────────────┐ ┌─────────────┐    │
│  │Vision Service│ │ LLM Service │    │
│  │   (External)│ │  (External) │    │
│  └─────────────┘ └─────────────┘    │
└─────────────────────────────────────┘
```

## 🚀 Services

### Local Services (Docker Compose)
- **Frontend**: Yew WebAssembly PWA
- **API Gateway**: Rust/Axum backend
- **Queue Worker**: Background job processor
- **PostgreSQL**: Database
- **Redis**: Cache and job queue

### External Services (AI4Thai Managed)
- **Vision Service**: Crop disease detection AI
- **LLM Service**: Agricultural advisory AI

## 🔧 Configuration

### Required Environment Variables

```bash
# AI4Thai API Key (Required)
AI4THAI_API_KEY=your_api_key_here

# Service URLs (Default to AI4Thai production)
VISION_SERVICE_URL=https://vision-api.ai4thai.com
LLM_SERVICE_URL=https://llm-api.ai4thai.com

# Optional: Weather integration
WEATHER_API_KEY=your_weather_key
```

### Optional Services

```bash
# Enable file storage
docker-compose --profile storage up -d

# Enable monitoring
docker-compose --profile monitoring up -d
```

## 🎮 Demo Features

### Core Functionality
- ✅ Crop disease detection via image upload
- ✅ AI-powered agricultural advice chat
- ✅ Thai language support
- ✅ User registration and profiles
- ✅ Real-time WebSocket communication

### Demo Data
- Sample crop types and diseases
- Demo user: `demo@ai4thai.com` / `demo123`
- Pre-loaded agricultural knowledge base

## 🔍 Monitoring

### Health Checks
```bash
# Check all services
docker-compose ps

# API health
curl http://localhost:3000/health

# View logs
docker-compose logs -f
```

### Service Status
```bash
# Individual service logs
docker-compose logs api-gateway
docker-compose logs frontend
docker-compose logs queue-worker
```

## 🛠️ Management

### Start/Stop
```bash
# Start demo
./scripts/demo-start.sh

# Stop demo
docker-compose down

# Restart specific service
docker-compose restart api-gateway
```

### Database Management
```bash
# Access database
docker-compose exec postgres psql -U postgres -d ai4thai

# Reset database
docker-compose down -v
docker-compose up -d
```

## 🚨 Troubleshooting

### Common Issues

#### Port Conflicts
```bash
# Check port usage
netstat -tulpn | grep :3000
netstat -tulpn | grep :8080

# Change ports in .env if needed
API_GATEWAY_PORT=3001
FRONTEND_PORT=8081
```

#### AI Service Connection
```bash
# Test AI4Thai connectivity
curl -H "Authorization: Bearer $AI4THAI_API_KEY" \
     https://vision-api.ai4thai.com/health
```

#### Service Won't Start
```bash
# Check Docker daemon
docker info

# Rebuild services
docker-compose build --no-cache
docker-compose up -d
```

## 📞 Support

### Getting Help
1. Check service logs: `docker-compose logs -f`
2. Verify API key configuration
3. Test external service connectivity
4. Contact AI4Thai support team

### Demo Limitations
- External AI services required
- Internet connection needed
- Not suitable for production use
- Limited to demonstration features

---

**Note**: This is a simplified deployment for demonstration purposes. The AI services are provided and managed by AI4Thai infrastructure, making the local setup much simpler while still showcasing the full functionality of the system.
