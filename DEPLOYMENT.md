# AI Services Deployment Guide

This document describes the GitLab CI/CD pipeline for deploying AI4Thai Crop Guardian AI services.

## 🚀 Quick Deployment

### Prerequisites

1. **GitLab Runner** with tag `hackathon-siamai2`
2. **Docker** and **Docker Compose** installed on runner
3. **Sufficient resources**: 8GB+ RAM, 4+ CPU cores
4. **Network access** to ports 2001, 2003, 2011, 6379, 9090, 3001

### Deployment Process

1. **Tag your commit** with any tag name:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. **Pipeline will automatically trigger** for tagged commits on runners with `hackathon-siamai2` tag

3. **Manual deployment** - Click "Deploy" button in GitLab CI/CD pipeline

## 🏗️ Pipeline Architecture

### Stages

```
┌─────────────┐    ┌──────────────┐
│    BUILD    │ -> │    DEPLOY    │
│             │    │              │
│ • Vision    │    │ • Deploy     │
│ • Queue     │    │ • Health     │
│ • Images    │    │ • Cleanup    │
└─────────────┘    └──────────────┘
```

### Jobs

#### 1. **build-ai-services**
- Builds Docker images for Vision Service and Queue Worker
- Tags images with commit SHA and latest
- Saves images as artifacts for deployment stage
- **Trigger**: Automatic on tags with `hackathon-siamai2` runner

#### 2. **deploy-ai-services** 
- Loads built Docker images
- Deploys AI services using docker-compose
- Performs health checks
- **Trigger**: Manual after build completes
- **Environment**: Production

#### 3. **cleanup-ai-services**
- Stops all AI services
- Removes containers, volumes, and networks
- Cleans up Docker system
- **Trigger**: Manual only

#### 4. **health-check**
- Independent health check job
- Tests all service endpoints
- **Trigger**: Manual only

## 🔧 Configuration

### Environment Variables

Set these in **GitLab Project Settings > CI/CD > Variables**:

| Variable | Default | Description |
|----------|---------|-------------|
| `VISION_SERVICE_PORT` | `2001` | Vision service port |
| `QUEUE_WORKER_PORT` | `2003` | Queue worker port |
| `VISION_LB_PORT` | `2011` | Load balancer port |
| `REDIS_PORT` | `6379` | Redis port |
| `PROMETHEUS_PORT` | `9090` | Prometheus port |
| `GRAFANA_PORT` | `3001` | Grafana port |
| `BUILD_TARGET` | `production` | Docker build target |

### GitLab Runner Requirements

```yaml
# .gitlab-ci.yml runner requirements
tags:
  - hackathon-siamai2

# Runner configuration needed:
executor = "docker"
[runners.docker]
  image = "docker:24.0.5"
  privileged = true
  volumes = ["/var/run/docker.sock:/var/run/docker.sock", "/cache"]
```

## 📦 Deployed Services

After successful deployment, the following services will be running:

| Service | Container Name | Port | Purpose |
|---------|----------------|------|---------|
| **Vision Service** | `team10-vision-service` | 2001 | Computer vision API |
| **Queue Worker** | `team10-queue-worker` | 2003 | Background job processing |
| **Celery Worker** | `team10-celery-worker` | - | Task processor |
| **Celery Beat** | `team10-celery-beat` | - | Task scheduler |
| **Redis** | `team10-ai-redis` | 6379 | Cache and job queue |
| **Load Balancer** | `team10-vision-lb` | 2011 | Vision service LB |
| **Prometheus** | `team10-ai-prometheus` | 9090 | Metrics collection |
| **Grafana** | `team10-ai-grafana` | 3001 | Monitoring dashboards |

### Service URLs

- **Vision API**: `http://localhost:2001`
- **Queue Worker API**: `http://localhost:2003`
- **Load Balancer**: `http://localhost:2011`
- **Prometheus**: `http://localhost:9090`
- **Grafana**: `http://localhost:3001` (admin/admin)

## 🗄️ Volumes

All services use standardized team10 volumes:

- **`team10-root`**: Root access for all services
- **`team10-data`**: Data storage (models, configs, databases)

```bash
# View volumes
docker volume ls | grep team10

# Inspect volume usage
docker volume inspect team10-data team10-root
```

## 🔍 Monitoring & Health Checks

### Automatic Health Checks

The pipeline performs automatic health checks:

```bash
# Vision Service
curl -f http://localhost:2001/health

# Queue Worker  
curl -f http://localhost:2003/health
```

### Manual Monitoring

```bash
# Check running containers
docker ps --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}" | grep team10

# View logs
docker logs team10-vision-service
docker logs team10-queue-worker

# Monitor resources
docker stats $(docker ps --format "{{.Names}}" | grep team10)
```

## 🛠️ Local Development

For local testing of the deployment:

```bash
# Use the deployment script
./scripts/deploy-ai-services.sh deploy

# Check status
./scripts/deploy-ai-services.sh status

# Health check
./scripts/deploy-ai-services.sh health

# Cleanup
./scripts/deploy-ai-services.sh cleanup
```

## 🚨 Troubleshooting

### Common Issues

#### 1. **Build Failures**
```bash
# Check Docker daemon
docker info

# Check available space
df -h

# Clean up Docker
docker system prune -f
```

#### 2. **Deployment Failures**
```bash
# Check container logs
docker logs team10-vision-service
docker logs team10-queue-worker

# Check port conflicts
netstat -tulpn | grep -E ':(2001|2003|2011|6379|9090|3001)'
```

#### 3. **Health Check Failures**
```bash
# Manual health checks
curl -v http://localhost:2001/health
curl -v http://localhost:2003/health

# Check service status
docker ps | grep team10
```

### Pipeline Debugging

1. **Check GitLab Runner logs**
2. **Verify runner has `hackathon-siamai2` tag**
3. **Ensure sufficient resources on runner**
4. **Check environment variables in GitLab**

## 📋 Deployment Checklist

Before deploying:

- [ ] GitLab runner with `hackathon-siamai2` tag is available
- [ ] Runner has Docker and Docker Compose installed
- [ ] Runner has sufficient resources (8GB+ RAM)
- [ ] Required ports are available (2001, 2003, 2011, 6379, 9090, 3001)
- [ ] Environment variables are set in GitLab
- [ ] Code is tagged and pushed to GitLab

After deployment:

- [ ] All containers are running (`docker ps | grep team10`)
- [ ] Health checks pass for Vision Service and Queue Worker
- [ ] Services are accessible on configured ports
- [ ] Volumes are created (`docker volume ls | grep team10`)
- [ ] Logs show no critical errors

## 🔄 Rollback Procedure

If deployment fails:

1. **Run cleanup job** in GitLab pipeline
2. **Or manually cleanup**:
   ```bash
   cd ai-services/deployment
   docker-compose down --volumes --remove-orphans
   docker system prune -f
   ```
3. **Deploy previous working tag**

---

For support, check the main [README.md](README.md) or contact the development team.
