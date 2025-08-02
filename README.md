# AI4Thai Crop Guardian 🌾

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54)](https://www.python.org/)

> AI-powered crop disease detection and advisory system for Thai farmers

## 🎯 Overview

AI4Thai Crop Guardian democratizes AI-powered agricultural expertise for Thai farmers through intelligent crop disease detection, multimodal chat interface, and personalized treatment recommendations.

### Key Features

- 🔍 **Smart Disease Detection**: High accuracy crop disease identification using computer vision
- 💬 **Multimodal Chat**: Text, voice, and image-based farmer interaction
- 🌐 **Thai Language Support**: Native Thai language processing and responses
- 📱 **PWA Experience**: Offline-capable progressive web application
- 🚀 **High Performance**: Sub-3 second response times with Rust backend

### Supported Crops

- 🌾 Rice (ข้าว) - Blast, Brown spot, Bacterial blight
- 🥔 Cassava (มันสำปะหลัง) - Mosaic virus, Root rot
- 🥭 Durian (ทุเรียน) - Fusarium wilt, Anthracnose
- 🥭 Mango (มะม่วง) - Anthracnose, Powdery mildew
- 🌳 Rubber (ยางพารา) - Leaf blight, Tapping panel dryness

## 🏗️ Architecture

The system supports **two deployment modes**:

### 1. Standalone AI Services (Self-hosted)
```
Frontend PWA         API Gateway         AI Services Cluster
(Yew WebAssembly) → (Rust/Axum)    → [Vision + Queue Worker]
                         ↓                    ↓
                 PostgreSQL          Redis + Celery
```

### 2. External AI Services (Demo/Production)
```
Frontend PWA         API Gateway         External APIs
(Yew WebAssembly) → (Rust/Axum)    → (AI4Thai Services)
                         ↓
                 PostgreSQL + Redis
```

## 🚀 Quick Start

### Option 1: Demo with External AI Services (Recommended)
```bash
git clone <repository-url>
cd ai4thai-crop-guardian

# Configure environment
cp .env.example .env
# Edit .env: Set AI4THAI_API_KEY=your_api_key

# Start demo (main application only)
./scripts/demo-start.sh
```
**Access**: Frontend at http://localhost:8080, API at http://localhost:3000

### Option 2: Full Self-hosted AI Services
```bash
# Deploy standalone AI services first
cd ai-services/deployment
docker-compose up -d

# Then start main application
cd ../../
./scripts/setup-dev.sh
./scripts/dev-start.sh
```
**Access**: AI Services at http://localhost:8001-8003, Main app at http://localhost:8080

## 🛠️ Development

### Build Commands
```bash
# Main application (API Gateway + Frontend)
cargo build --workspace
cd frontend && trunk build

# AI Services (if self-hosting)
cd ai-services/deployment && docker-compose build
```

### Testing
```bash
./scripts/test-all.sh            # All tests
cd api-gateway && cargo test     # Backend tests
cd frontend && wasm-pack test --headless --firefox  # Frontend tests
```

## 📁 Project Structure

```
ai4thai-crop-guardian/
├── api-gateway/          # Rust API Gateway (main application)
├── frontend/             # Yew WebAssembly PWA (main application)  
├── shared/               # Common Rust types (main application)
├── ai-services/          # Standalone AI Services Deployment
│   ├── vision-service/   # Computer vision service
│   ├── queue-worker/     # Background job processing
│   └── deployment/       # Docker compose for AI services
└── scripts/              # Development scripts
```

**Deployment Modes**:
- **External AI**: Deploy only `api-gateway` + `frontend` (connects to AI4Thai APIs)
- **Self-hosted AI**: Deploy `ai-services/` cluster + `api-gateway` + `frontend`

## 🔧 Configuration

### Required Environment Variables
```bash
# For demo deployment (external AI services)
AI4THAI_API_KEY=your_api_key_here

# For self-hosted deployment
VISION_SERVICE_URL=http://localhost:2001
QUEUE_WORKER_URL=http://localhost:2003

# Chat storage (both modes)
REDIS_URL=redis://localhost:6379
```

### Service Ports
**Main Application:**
- Frontend: 8080
- API Gateway: 3000

**AI Services (when self-hosted):**
- Vision Service: 2001
- Queue Worker: 2003

## 📊 API Reference

### POST `/v1/chat`
Unified endpoint for crop analysis and advisory.

**Request**: `multipart/form-data`
- `image`: Image file
- `crop_type`: Crop type (e.g., "rice", "cassava")
- `query`: User query (e.g., "Check for diseases")

**Response**: `{"answer": "Analysis result..."}`

## 🤝 Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open Pull Request

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

## 📞 Support

- 📧 Email: rpithaksiripan@gmail.com

---

Made with ❤️ for Thai farmers by KPR team for AI Thailand Hackathon 2025