# AI4Thai Crop Guardian - Project Structure

This document provides a comprehensive overview of the project structure and organization.

## 📁 Directory Structure

```
ai4thai-crop-guardian/
├── 📋 README.md                    # Main project documentation
├── 📄 LICENSE                      # MIT license
├── 🔧 .gitignore                   # Git ignore rules
├── ⚙️ .env.example                 # Environment configuration template
├── 🐳 docker-compose.yml           # Docker services configuration
├── 📊 Cargo.toml                   # Rust workspace configuration
│
├── 📚 docs/                        # Documentation
│   ├── 📋 README.md                # Documentation index
│   ├── 📝 CONTRIBUTING.md          # Contributing guidelines
│   ├── 📄 PRODUCT_SPEC.md          # Product specification
│   │
│   ├── 🏗️ architecture/            # System architecture docs
│   │   ├── README.md               # Architecture overview
│   │   ├── system-design.md        # High-level system design
│   │   ├── microservices.md        # Microservices architecture
│   │   ├── project-structure.md    # Code organization
│   │   ├── data-flow.md            # Data flow diagrams
│   │   ├── security.md             # Security architecture
│   │   └── performance.md          # Performance considerations
│   │
│   ├── 🔌 api/                     # API documentation
│   │   ├── README.md               # API overview
│   │   ├── gateway.md              # API Gateway endpoints
│   │   ├── vision-service.md       # Vision service API
│   │   ├── llm-service.md          # LLM service API
│   │   ├── user-service.md         # User service API
│   │   ├── chat-service.md         # Chat service API
│   │   ├── websocket.md            # WebSocket documentation
│   │   └── examples/               # API usage examples
│   │       └── curl-examples.md    # cURL examples
│   │
│   ├── 🚀 deployment/              # Deployment documentation
│   │   ├── README.md               # Deployment overview
│   │   ├── local.md                # Local development setup
│   │   ├── staging.md              # Staging deployment
│   │   ├── production.md           # Production deployment
│   │   ├── docker.md               # Docker deployment
│   │   ├── kubernetes.md           # Kubernetes deployment
│   │   ├── monitoring.md           # Monitoring setup
│   │   ├── backup-recovery.md      # Backup and recovery
│   │   └── security.md             # Security hardening
│   │
│   ├── 💻 development/             # Development documentation
│   │   ├── README.md               # Development overview
│   │   ├── setup.md                # Development setup
│   │   ├── contributing.md         # Contribution guidelines
│   │   ├── coding-standards.md     # Code style guidelines
│   │   ├── testing.md              # Testing guidelines
│   │   ├── workflow.md             # Git workflow
│   │   ├── mvp-requirements.md     # MVP requirements
│   │   ├── hackathon-mvp.md        # Hackathon MVP scope
│   │   └── hackathon-plan.md       # Hackathon execution plan
│   │
│   └── 👥 user-guides/             # User documentation
│       ├── README.md               # User guide overview
│       ├── getting-started.md      # Getting started guide
│       ├── disease-detection.md    # Disease detection guide
│       ├── chat-interface.md       # Chat interface guide
│       ├── mobile-app.md           # Mobile app guide
│       └── troubleshooting.md      # Troubleshooting guide
│
├── 🔧 scripts/                     # Development and deployment scripts
│   ├── setup-dev.sh               # Development environment setup
│   ├── dev-start.sh               # Start development services
│   ├── test-all.sh                # Run all tests
│   ├── build-all.sh               # Build all services
│   ├── deploy-staging.sh          # Deploy to staging
│   ├── deploy-production.sh       # Deploy to production
│   └── backup-db.sh               # Database backup script
│
├── 🌐 api-gateway/                 # Rust API Gateway (Axum)
│   ├── Cargo.toml                 # Rust dependencies
│   ├── src/                       # Source code
│   │   ├── main.rs                # Application entry point
│   │   ├── lib.rs                 # Library root
│   │   ├── config.rs              # Configuration management
│   │   ├── routes/                # HTTP route handlers
│   │   ├── middleware/            # Custom middleware
│   │   ├── services/              # Business logic services
│   │   ├── models/                # Data models
│   │   ├── utils/                 # Utility functions
│   │   └── error.rs               # Error handling
│   ├── migrations/                # Database migrations
│   ├── tests/                     # Integration tests
│   └── README.md                  # Service documentation
│
├── 🤖 ai-services/                 # Python AI Services
│   ├── vision-service/            # Computer vision service
│   │   ├── app.py                 # FastAPI application
│   │   ├── requirements.txt       # Python dependencies
│   │   ├── models/                # ML model files
│   │   ├── services/              # Business logic
│   │   ├── utils/                 # Utility functions
│   │   ├── tests/                 # Unit tests
│   │   ├── Dockerfile             # Docker configuration
│   │   └── README.md              # Service documentation
│   │
│   ├── llm-service/               # Language model service
│   │   ├── app.py                 # FastAPI application
│   │   ├── requirements.txt       # Python dependencies
│   │   ├── services/              # Business logic
│   │   ├── prompts/               # LLM prompts
│   │   ├── utils/                 # Utility functions
│   │   ├── tests/                 # Unit tests
│   │   ├── Dockerfile             # Docker configuration
│   │   └── README.md              # Service documentation
│   │
│   └── venv/                      # Python virtual environment
│
├── 📱 frontend/                    # Yew WebAssembly PWA
│   ├── Cargo.toml                 # Rust dependencies
│   ├── Trunk.toml                 # Trunk configuration
│   ├── index.html                 # HTML template
│   ├── src/                       # Source code
│   │   ├── main.rs                # Application entry point
│   │   ├── app.rs                 # Main app component
│   │   ├── components/            # Reusable components
│   │   ├── pages/                 # Page components
│   │   ├── services/              # API services
│   │   ├── utils/                 # Utility functions
│   │   └── styles/                # CSS styles
│   ├── static/                    # Static assets
│   │   ├── manifest.json          # PWA manifest
│   │   ├── sw.js                  # Service worker
│   │   └── icons/                 # App icons
│   ├── tests/                     # Frontend tests
│   └── README.md                  # Frontend documentation
│
├── 🔗 shared/                      # Shared Rust types and utilities
│   ├── Cargo.toml                 # Rust dependencies
│   ├── src/                       # Source code
│   │   ├── lib.rs                 # Library root
│   │   ├── types/                 # Shared data types
│   │   ├── utils/                 # Shared utilities
│   │   └── constants.rs           # Application constants
│   └── README.md                  # Shared library documentation
│
├── 🔄 queue-worker/                # Background job processor
│   ├── Cargo.toml                 # Rust dependencies
│   ├── src/                       # Source code
│   │   ├── main.rs                # Worker entry point
│   │   ├── jobs/                  # Job handlers
│   │   ├── processors/            # Job processors
│   │   └── utils/                 # Utility functions
│   ├── tests/                     # Worker tests
│   └── README.md                  # Worker documentation
│
├── 🏗️ infrastructure/              # Infrastructure as Code
│   ├── docker/                    # Docker configurations
│   │   ├── Dockerfile.api         # API Gateway Dockerfile
│   │   ├── Dockerfile.frontend    # Frontend Dockerfile
│   │   └── docker-compose.prod.yml # Production compose
│   ├── kubernetes/                # Kubernetes manifests
│   │   ├── namespace.yaml         # Namespace definition
│   │   ├── deployments/           # Deployment manifests
│   │   ├── services/              # Service manifests
│   │   ├── ingress/               # Ingress configurations
│   │   └── configmaps/            # Configuration maps
│   ├── terraform/                 # Terraform configurations
│   │   ├── main.tf                # Main configuration
│   │   ├── variables.tf           # Variable definitions
│   │   ├── outputs.tf             # Output definitions
│   │   └── modules/               # Terraform modules
│   └── helm/                      # Helm charts
│       └── crop-guardian/         # Main Helm chart
│           ├── Chart.yaml         # Chart metadata
│           ├── values.yaml        # Default values
│           └── templates/         # Template files
│
├── 🧪 tests/                       # Integration and E2E tests
│   ├── integration/               # Integration tests
│   │   ├── api_tests.rs           # API integration tests
│   │   ├── database_tests.rs      # Database tests
│   │   └── service_tests.rs       # Service integration tests
│   ├── e2e/                       # End-to-end tests
│   │   ├── user_flows.rs          # User workflow tests
│   │   └── performance_tests.rs   # Performance tests
│   ├── fixtures/                  # Test data and fixtures
│   │   ├── images/                # Test images
│   │   └── data/                  # Test data files
│   └── README.md                  # Testing documentation
│
└── 🛠️ tools/                       # Development tools and utilities
    ├── postman/                   # Postman collections
    │   └── ai4thai-api.json       # API collection
    ├── scripts/                   # Utility scripts
    │   ├── generate-docs.sh       # Documentation generation
    │   ├── lint-all.sh            # Code linting
    │   └── format-all.sh          # Code formatting
    ├── monitoring/                # Monitoring configurations
    │   ├── prometheus.yml         # Prometheus config
    │   ├── grafana/               # Grafana dashboards
    │   └── alerts/                # Alert rules
    └── README.md                  # Tools documentation
```

## 🏗️ Architecture Overview

### Service Architecture
The project follows a microservices architecture with the following components:

1. **API Gateway** (Rust/Axum): Central entry point, authentication, routing
2. **Vision Service** (Python/FastAPI): Computer vision and image processing
3. **LLM Service** (Python/FastAPI): Language model integration and advisory
4. **Frontend** (Yew/WebAssembly): Progressive web application
5. **Queue Worker** (Rust): Background job processing
6. **Shared Library** (Rust): Common types and utilities

### Data Flow
```
Frontend → API Gateway → Services → Database
                    ↓
              Queue Worker → Background Jobs
```

### Technology Stack
- **Backend**: Rust (Axum), Python (FastAPI)
- **Frontend**: Rust (Yew WebAssembly)
- **Database**: PostgreSQL, Redis
- **Infrastructure**: Docker, Kubernetes
- **Monitoring**: Prometheus, Grafana

## 📋 Key Files and Their Purpose

### Configuration Files
- **Cargo.toml**: Rust workspace configuration
- **docker-compose.yml**: Local development services
- **.env.example**: Environment variable template
- **Trunk.toml**: Frontend build configuration

### Documentation Files
- **README.md**: Main project documentation
- **LICENSE**: MIT license terms
- **CONTRIBUTING.md**: Contribution guidelines
- **PROJECT_STRUCTURE.md**: This file

### Development Files
- **.gitignore**: Git ignore rules
- **scripts/**: Development and deployment scripts
- **tests/**: Integration and end-to-end tests

## 🔄 Development Workflow

### 1. Setup
```bash
git clone <repository>
cd ai4thai-crop-guardian
./scripts/setup-dev.sh
```

### 2. Development
```bash
./scripts/dev-start.sh  # Start all services
./scripts/test-all.sh   # Run tests
```

### 3. Deployment
```bash
./scripts/deploy-staging.sh     # Deploy to staging
./scripts/deploy-production.sh  # Deploy to production
```

## 📊 Code Organization Principles

### 1. Separation of Concerns
- Each service has a single responsibility
- Clear boundaries between layers
- Shared code in dedicated libraries

### 2. Consistency
- Consistent naming conventions
- Standardized project structure across services
- Common error handling patterns

### 3. Maintainability
- Comprehensive documentation
- Automated testing
- Clear dependency management

### 4. Scalability
- Stateless service design
- Horizontal scaling capabilities
- Efficient resource utilization

## 🔧 Build and Deployment

### Local Development
- Docker Compose for service orchestration
- Hot reload for development
- Integrated testing environment

### Production Deployment
- Kubernetes for container orchestration
- Helm charts for deployment management
- Infrastructure as Code with Terraform

## 📚 Documentation Standards

### Code Documentation
- Inline comments for complex logic
- API documentation with examples
- README files for each service

### Architecture Documentation
- System design documents
- Data flow diagrams
- Deployment guides

### User Documentation
- Getting started guides
- Feature documentation
- Troubleshooting guides

## 🔍 Quality Assurance

### Testing Strategy
- Unit tests for individual components
- Integration tests for service interactions
- End-to-end tests for user workflows

### Code Quality
- Automated linting and formatting
- Code coverage requirements
- Security vulnerability scanning

### Performance Monitoring
- Response time tracking
- Resource utilization monitoring
- Error rate tracking

---

This project structure follows industry best practices for microservices architecture, ensuring maintainability, scalability, and developer productivity.
