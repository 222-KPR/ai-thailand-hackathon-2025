# Development Documentation

This section contains all development-related documentation for the AI4Thai Crop Guardian project.

## 📋 Development Guides

### 🚀 [Setup Guide](setup.md)
Complete development environment setup instructions.

### 🤝 [Contributing Guide](contributing.md)
How to contribute to the project, coding standards, and pull request process.

### 📝 [Coding Standards](coding-standards.md)
Code style guidelines, best practices, and conventions.

### 🧪 [Testing Guide](testing.md)
Testing strategies, frameworks, and best practices.

### 🔧 [Development Workflow](workflow.md)
Git workflow, branching strategy, and release process.

## 📋 Project Planning

### 🎯 [MVP Requirements](mvp-requirements.md)
Minimum viable product requirements and specifications.

### 🏆 [Hackathon MVP](hackathon-mvp.md)
Hackathon-specific MVP scope and deliverables.

### 📅 [Hackathon Plan](hackathon-plan.md)
Detailed execution plan for the hackathon.

## 🛠️ Development Environment

### Prerequisites
- **Rust**: 1.70+ with WebAssembly target
- **Python**: 3.9+ with pip and virtual environments
- **Node.js**: 18+ for frontend tooling
- **Docker**: Latest version with Docker Compose
- **Git**: Version control

### Quick Setup
```bash
# Clone repository
git clone <repository-url>
cd ai4thai-crop-guardian

# Setup environment
cp .env.example .env
./scripts/setup-dev.sh

# Start development environment
./scripts/dev-start.sh
```

## 🏗️ Architecture Overview

### Service Structure
```
├── api-gateway/          # Rust API Gateway (Axum)
├── ai-services/          # Python AI Services
│   ├── vision-service/   # Computer vision
│   └── llm-service/     # Language model
├── frontend/            # Yew WebAssembly PWA
├── shared/              # Shared Rust types
└── queue-worker/        # Background job processor
```

### Technology Stack
- **Backend**: Rust (Axum), Python (FastAPI)
- **Frontend**: Rust (Yew WebAssembly)
- **Database**: PostgreSQL, Redis
- **Infrastructure**: Docker, Kubernetes

## 🔄 Development Workflow

### 1. Feature Development
```bash
# Create feature branch
git checkout -b feature/crop-detection

# Make changes and test
cargo test
pytest

# Commit and push
git commit -m "feat: add crop detection endpoint"
git push origin feature/crop-detection
```

### 2. Code Review Process
1. Create pull request
2. Automated tests run
3. Code review by team members
4. Address feedback
5. Merge to main branch

### 3. Deployment Pipeline
1. **Development**: Automatic deployment on feature branches
2. **Staging**: Deployment on main branch merge
3. **Production**: Manual deployment with approval

## 🧪 Testing Strategy

### Unit Tests
- **Rust**: `cargo test`
- **Python**: `pytest`
- **Frontend**: `wasm-pack test`

### Integration Tests
- API endpoint testing
- Service-to-service communication
- Database integration

### End-to-End Tests
- User workflow testing
- Cross-browser compatibility
- Performance testing

## 📊 Code Quality

### Static Analysis
- **Rust**: Clippy linting
- **Python**: Black formatting, Flake8 linting
- **TypeScript**: ESLint, Prettier

### Code Coverage
- Target: 80% minimum coverage
- Tools: `cargo tarpaulin`, `pytest-cov`
- Reports: Generated on CI/CD pipeline

### Performance Monitoring
- Response time tracking
- Memory usage monitoring
- Database query optimization

## 🔧 Development Tools

### Recommended IDE Setup
- **VS Code** with extensions:
  - Rust Analyzer
  - Python
  - Docker
  - GitLens

### Debugging
- **Rust**: `rust-gdb`, VS Code debugger
- **Python**: `pdb`, VS Code debugger
- **Frontend**: Browser developer tools

### Profiling
- **Rust**: `cargo flamegraph`
- **Python**: `py-spy`, `cProfile`
- **Database**: `pg_stat_statements`

## 📝 Documentation Standards

### Code Documentation
- **Rust**: Doc comments with examples
- **Python**: Docstrings following Google style
- **API**: OpenAPI/Swagger specifications

### Architecture Documentation
- System design documents
- API documentation
- Database schema documentation

### User Documentation
- Setup guides
- API usage examples
- Troubleshooting guides

## 🚨 Troubleshooting

### Common Issues
1. **Build Failures**: Check Rust/Python versions
2. **Database Connection**: Verify PostgreSQL is running
3. **Port Conflicts**: Check for running services
4. **CORS Issues**: Verify frontend/backend configuration

### Getting Help
- Check existing issues in the repository
- Ask in the team chat channel
- Create a new issue with detailed information

---

For questions about development processes, please contact the development team or create an issue.
