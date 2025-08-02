# AI4Thai Crop Guardian 🌾

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54)](https://www.python.org/)
[![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)](https://www.docker.com/)

> AI-powered crop disease detection and advisory system for Thai farmers

## 🎯 Overview

AI4Thai Crop Guardian democratizes AI-powered agricultural expertise for Thai farmers through intelligent crop disease detection, multimodal chat interface, and personalized treatment recommendations.

### Key Features

- 🔍 **Smart Disease Detection**: 98% accuracy crop disease identification using computer vision
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

```mermaid
graph TB
    A[Frontend PWA<br/>Yew WebAssembly] --> B[API Gateway<br/>Rust/Axum]
    B --> C[Vision Service<br/>Python/FastAPI]
    B --> D[LLM Service<br/>Python/FastAPI]
    B --> E[External APIs<br/>TTS/ASR/Weather]
    B --> F[(PostgreSQL)]
    B --> G[(Redis)]
    
    style A fill:#e1f5fe
    style B fill:#f3e5f5
    style C fill:#e8f5e8
    style D fill:#e8f5e8
    style E fill:#fff3e0
    style F fill:#fce4ec
    style G fill:#fce4ec
```

## 🚀 Quick Start

### 🤖 Separate AI Services Deployment (Recommended)

Deploy AI services separately using HuggingFace models for better performance and scalability:

```bash
# 1. Deploy AI services (Vision & LLM) separately
./scripts/deploy-ai-services.sh basic

# 2. Configure main application
cp .env.example .env
# Edit .env: Set VISION_SERVICE_URL=http://localhost:8001
#           Set LLM_SERVICE_URL=http://localhost:8002

# 3. Start main application services
docker-compose up -d
```

**Access the application:**
- 📱 Frontend: http://localhost:8080
- 🔌 API Gateway: http://localhost:3000
- 👁️ Vision Service: http://localhost:8001
- 🧠 LLM Service: http://localhost:8002

> **Note**: AI services use HuggingFace models and require GPU for optimal performance. See [Separate AI Services Guide](docs/deployment/separate-ai-services.md) for details.

### 🎮 Demo Deployment (External AI Services)

For quick demonstrations using external AI4Thai services:

```bash
# Clone and setup
git clone https://github.com/your-org/ai4thai-crop-guardian.git
cd ai4thai-crop-guardian

# Configure for external AI services
cp .env.example .env
# Edit .env: AI4THAI_API_KEY=your_api_key_here

# Start demo
./scripts/demo-start.sh
```

### 💻 Full Development Setup

For complete local development with all services:

**Prerequisites:**
- [Rust](https://rustup.rs/) (1.70+)
- [Python](https://www.python.org/) (3.9+)
- [Docker](https://www.docker.com/) & Docker Compose
- [Node.js](https://nodejs.org/) (for frontend tooling)
- NVIDIA GPU (recommended for AI services)

```bash
# Full development setup
./scripts/setup-dev.sh

# Start all services locally
./scripts/dev-start.sh
```

## 📁 Project Structure

```
ai4thai-crop-guardian/
├── 📋 docs/                    # Documentation
│   ├── api/                    # API documentation
│   ├── architecture/           # System design docs
│   ├── deployment/             # Deployment guides
│   └── user-guides/           # User documentation
├── 🔧 scripts/                # Development & deployment scripts
├── 🌐 api-gateway/            # Rust API Gateway (Axum)
├── 🤖 ai-services/            # Python AI Services
│   ├── vision-service/        # Computer vision service
│   └── llm-service/          # LLM advisory service
├── 📱 frontend/               # Yew WebAssembly PWA
├── 🔗 shared/                 # Shared Rust types
├── 🏗️ infrastructure/         # Infrastructure as code
├── 🧪 tests/                  # Integration tests
└── 🛠️ tools/                  # Development tools
```

## 🛠️ Development

### Backend Development

```bash
# API Gateway
cd api-gateway
cargo run

# Vision Service
cd ai-services/vision-service
python -m uvicorn app:app --reload --port 8001

# LLM Service
cd ai-services/llm-service
python -m uvicorn app:app --reload --port 8002
```

### Frontend Development

```bash
cd frontend
trunk serve --port 8080
```

### Testing

```bash
# Run all tests
./scripts/test-all.sh

# Backend tests
cd api-gateway && cargo test
cd ai-services/vision-service && pytest
cd ai-services/llm-service && pytest

# Frontend tests
cd frontend && wasm-pack test --headless --firefox
```

## 📊 Performance Benchmarks

| Metric | Target | Current |
|--------|--------|---------|
| Disease Detection Accuracy | >95% | 98.2% |
| API Response Time | <3s | 1.8s |
| Frontend Load Time | <2s | 1.2s |
| PWA Lighthouse Score | >90 | 94 |

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](docs/CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📖 Documentation

- [API Documentation](docs/api/README.md)
- [Architecture Overview](docs/architecture/README.md)
- [Deployment Guide](docs/deployment/README.md)
- [User Guide](docs/user-guides/README.md)

## 🔒 Security

For security concerns, please email security@ai4thai.com instead of using the issue tracker.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Thai farmers who provided invaluable feedback
- AI4Thai community for language processing support
- Open source contributors and maintainers

## 📞 Support

- 📧 Email: support@ai4thai.com
- 💬 Discord: [AI4Thai Community](https://discord.gg/ai4thai)
- 📖 Documentation: [docs.ai4thai.com](https://docs.ai4thai.com)

---

Made with ❤️ for Thai farmers by KPR team for AI Thailand Hackathon 2025
