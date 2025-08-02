# H100 16GB VRAM Optimization Guide

This guide contains optimizations specifically designed for NVIDIA H100 GPUs with 16GB VRAM to resolve GitLab CI artifact size issues and improve performance.

## 🚀 Key Optimizations Implemented

### 1. Docker Image Optimization
- **Multi-stage builds** with minimal production images
- **Optimized .dockerignore** files to reduce build context
- **BuildKit enabled** for better compression and caching
- **Compressed artifacts** with gzip -9 for maximum compression
- **Minimal base images** using python:3.9-slim-bullseye

### 2. H100 16GB VRAM Specific Optimizations
- **Target architecture**: `TORCH_CUDA_ARCH_LIST="9.0"` for H100
- **Memory allocation**: `PYTORCH_CUDA_ALLOC_CONF=max_split_size_mb:2048,expandable_segments:True`
- **Reduced batch size**: `MAX_BATCH_SIZE=1`
- **Gradient checkpointing** enabled for memory efficiency
- **Flash Attention** enabled for faster processing
- **Model max length**: Limited to 512 tokens

### 3. CI/CD Pipeline Optimizations
- **Shorter artifact expiry**: 30 minutes instead of 1 hour
- **Compressed builds** with `--compress` flag
- **Intermediate layer cleanup** with `docker image prune -f`
- **Better compression** using `gzip -9`
- **H100-specific build target**: `h100-optimized`

### 4. Memory Usage Optimization
- **Reduced memory limits**: 8G limit, 4G reservation (down from 12G/6G)
- **Smaller image size**: Maximum 5MB instead of 10MB
- **Conservative threading**: 2 threads instead of 4
- **Optimized worker configuration**: `--max-requests=50`

## 📋 Before Deployment

### Prerequisites
1. NVIDIA H100 GPU with 16GB VRAM
2. Docker with BuildKit support
3. NVIDIA Container Runtime
4. Sufficient disk space for model caching

### Environment Setup
```bash
# Copy H100 environment configuration
cp ai-services/deployment/.env.h100 ai-services/deployment/.env

# Edit .env file and add your HuggingFace token if needed
nano ai-services/deployment/.env
```

## 🔧 Local Testing

### Build H100 Optimized Images
```bash
# Run the optimization build script
./build-h100-optimized.sh
```

### Deploy Locally
```bash
cd ai-services/deployment
docker-compose up -d
```

### Monitor GPU Usage
```bash
# Check GPU memory usage
nvidia-smi

# Check container logs
docker logs team10-vision-service
docker logs team10-queue-worker
```

## 🚀 GitLab CI Deployment

### Tag for Deployment
```bash
git tag hackathon-siamai2-v1.0
git push origin hackathon-siamai2-v1.0
```

### Monitor Pipeline
1. Go to GitLab CI/CD Pipelines
2. Check artifact sizes in build stage
3. Manually trigger deployment stage
4. Monitor health checks

## 📊 Expected Improvements

### Artifact Size Reduction
- **Vision Service**: ~60-70% smaller artifacts
- **Queue Worker**: ~50-60% smaller artifacts
- **Total artifacts**: Expected under 2GB combined

### Memory Usage
- **GPU VRAM**: Optimized for 16GB H100
- **System RAM**: Reduced from 12G to 8G limit
- **Startup time**: Faster due to optimized health checks

### Performance
- **H100 optimized**: Native architecture support
- **Flash Attention**: Faster inference
- **Memory efficient**: Better memory management

## 🔍 Troubleshooting

### If Artifacts Still Too Large
1. Check .dockerignore files are in place
2. Verify BuildKit is enabled
3. Consider removing development dependencies
4. Use multi-stage builds effectively

### If GPU Memory Issues
1. Reduce MAX_BATCH_SIZE to 1
2. Enable gradient checkpointing
3. Monitor with nvidia-smi
4. Adjust PYTORCH_CUDA_ALLOC_CONF

### If Models Don't Load
1. Check HuggingFace token
2. Verify model paths
3. Check disk space for model cache
4. Monitor startup logs

## 📁 File Changes Summary

### New Files
- `ai-services/vision-service/.dockerignore`
- `ai-services/queue-worker/.dockerignore`
- `ai-services/deployment/.env.h100`
- `build-h100-optimized.sh`

### Modified Files
- `ai-services/vision-service/Dockerfile` (H100 optimization stage)
- `ai-services/vision-service/requirements.txt` (minimal dependencies)
- `ai-services/queue-worker/Dockerfile` (optimized stages)
- `ai-services/deployment/docker-compose.yml` (H100 configuration)
- `.gitlab-ci.yml` (artifact optimization)

## 💡 Additional Tips

1. **Monitor resource usage** during deployment
2. **Use volume mounts** for model caching between deployments
3. **Consider model quantization** for further memory savings
4. **Regular cleanup** of unused Docker images and volumes
5. **Profile memory usage** in production to fine-tune settings

## 🆘 Support

If you encounter issues:
1. Check the logs: `docker logs team10-vision-service`
2. Monitor GPU: `nvidia-smi -l 1`
3. Test locally before CI deployment
4. Verify environment variables are set correctly
