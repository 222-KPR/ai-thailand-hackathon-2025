# AI4Thai CI/CD Deployment Guide

## Critical CI Pipeline Requirements

### Python Version Compatibility
- **Vision Service**: Python 3.11 (required for modern ML dependencies)
- **Queue Worker**: Python 3.11 (standardized across services)
- **Reason**: nvidia-ml-py3 >=12.x requires Python >=3.10, many transformers deps require >=3.11

### Docker Build Pipeline

#### Vision Service Dependencies
```dockerfile
FROM python:3.11-slim-bullseye AS base
```

**Key Dependencies (CI Compatible):**
- `torch==2.2.2` - H100 compatible, Python 3.11 support
- `transformers==4.39.3` - Latest stable with bitsandbytes compatibility
- `ultralytics==8.1.47` - YOLO11s support with updated dependencies
- `nvidia-ml-py3==7.352.0` - **Only available version compatible with current pip**
- `bitsandbytes==0.43.1` - Required for quantized LLaVA models

#### Queue Worker Dependencies
```dockerfile
FROM python:3.11-slim AS base
```

**Lightweight Dependencies:**
- No GPU libraries (offloads to Vision Service)
- Basic FastAPI, Redis, Celery stack
- Removed incompatible transformers/bitsandbytes deps

### Common CI Failures & Solutions

#### 1. nvidia-ml-py3 Version Issues
**Error**: `ERROR: Could not find a version that satisfies the requirement nvidia-ml-py3>=12.535.108`
**Solution**: Use `nvidia-ml-py3==7.352.0` (only version available on PyPI)

#### 2. Python Version Incompatibility
**Error**: `Requires-Python >=3.10` or `>=3.11`
**Solution**: Upgrade base image to `python:3.11-slim-bullseye`

#### 3. Dependency Conflicts
**Error**: Multiple packages requiring different versions
**Solution**: Pin exact versions in requirements.txt, separate GPU/CPU services

#### 4. pip cache purge with --no-cache-dir
**Error**: `pip cache commands can not function since cache is disabled`
**Solution**: Remove `pip cache purge` when using `--no-cache-dir`

### GitLab CI Pipeline Configuration

#### Runner Requirements
- **Tag**: `hackathon-siamai2`
- **Resources**: 8GB+ RAM, 4+ CPU cores
- **GPU**: H100 16GB (for Vision Service)
- **Docker**: BuildKit enabled for better error messages

#### Environment Variables
```bash
# Build Configuration
BUILD_TARGET=h100-optimized
DOCKER_BUILDKIT=1

# GPU Configuration
CUDA_VISIBLE_DEVICES=0
PYTORCH_CUDA_ALLOC_CONF=max_split_size_mb:2048

# Service Ports
VISION_SERVICE_PORT=2001
QUEUE_WORKER_PORT=2003
REDIS_PORT=6379
```

### Build Optimization

#### Multi-Stage Dockerfile Benefits
1. **Base Stage**: Common dependencies
2. **Development Stage**: Dev tools, debugging
3. **Production Stage**: Minimal runtime
4. **H100-Optimized Stage**: GPU-specific optimizations

#### .dockerignore Optimization
- Reduces build context by ~60%
- Excludes dev files, docs, test data
- Faster uploads to GitLab CI

### Pre-Deployment Validation

Run before pushing to CI:
```bash
# Syntax and dependency validation
./validate-docker.sh

# Full build test (optional)
./validate-docker.sh --build
```

### Monitoring CI Pipeline

#### Expected Build Times
- **Queue Worker**: 2-3 minutes
- **Vision Service**: 8-12 minutes (model downloads)
- **Total Pipeline**: 15-20 minutes

#### Success Indicators
```bash
# Vision Service Health
curl http://localhost:2001/health/detailed

# Queue Worker Health
curl http://localhost:2003/health

# Container Status
docker ps | grep team10
```

### Deployment Verification

#### Service Startup Order
1. **Redis** (cache/queue)
2. **Vision Service** (model loading ~5-10 min)
3. **Queue Worker** (API endpoints)
4. **Celery Workers** (background processing)

#### Health Check Sequence
```bash
# 1. Basic connectivity
curl -f http://localhost:2001/health
curl -f http://localhost:2003/health

# 2. Model loading status
curl http://localhost:2001/info

# 3. Queue processing
curl http://localhost:2003/queue/stats
```

### Troubleshooting Common Issues

#### Build Failures
```bash
# Check Docker logs
docker logs <container_name>

# Validate requirements
pip-compile --resolver=backtracking requirements.in

# Test local build
docker build --target h100-optimized -t test .
```

#### Runtime Issues
```bash
# GPU detection
nvidia-smi
docker exec <container> python -c "import torch; print(torch.cuda.is_available())"

# Memory usage
docker stats
free -h

# Service dependencies
docker-compose ps
docker network ls
```

### Version Upgrade Strategy

#### Safe Upgrade Process
1. **Test locally** with new versions
2. **Update requirements.txt** with exact pins
3. **Run validation script**
4. **Deploy to staging** environment
5. **Monitor resource usage**
6. **Gradual production rollout**

#### Dependency Compatibility Matrix
| Component | Python | PyTorch | Transformers | CUDA |
|-----------|--------|---------|--------------|------|
| Vision Service | 3.11 | 2.2.2 | 4.39.3 | 12.1+ |
| Queue Worker | 3.11 | N/A | N/A | N/A |
| H100 GPU | N/A | 2.2+ | 4.35+ | 12.1+ |

### Security Considerations

#### Build Security
- Pin exact dependency versions
- No secrets in Dockerfiles
- Use non-root users in production
- Minimal base images (slim variants)

#### Runtime Security
- Container resource limits
- Network isolation
- Read-only file systems where possible
- Health check timeouts

---

## Emergency Procedures

### Pipeline Failure Recovery
1. **Check GitLab CI logs** for specific error
2. **Run local validation** with `./validate-docker.sh --build`
3. **Revert to last known good commit** if needed
4. **Apply fixes incrementally** with validation

### Service Recovery
1. **Check container health** and restart if needed
2. **Verify GPU availability** with nvidia-smi
3. **Monitor memory usage** and adjust limits
4. **Review application logs** for errors

### Contact Information
- **CI/CD Issues**: Check GitLab runner status and resource availability
- **Docker Issues**: Review build logs and validation script output
- **Model Issues**: Verify HuggingFace access and model availability

---

*Last Updated: 2025-08-03*
*Pipeline Version: hackathon-siamai2*
