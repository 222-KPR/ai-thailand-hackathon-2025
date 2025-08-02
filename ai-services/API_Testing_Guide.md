# AI4Thai Vision Service API Testing Guide

## Overview

This guide provides comprehensive testing instructions for the AI4Thai Vision Service APIs using the provided Postman collections.

## Quick Start

### 1. Import Postman Collections

Import both collections into Postman:
- `AI4Thai_Vision_Service.postman_collection.json` - Direct API testing
- `AI4Thai_Queue_Worker.postman_collection.json` - Async job processing

### 2. Environment Setup

Set the following variables in Postman:
- **Vision Service**: `base_url` = `http://localhost:2001`
- **Queue Worker**: `queue_base_url` = `http://localhost:2003`

For production testing, update URLs accordingly.

### 3. Test Images

Prepare test images for upload:
- **Pest Detection**: Agricultural images with visible pests/insects
- **Disease Detection**: Leaf images showing plant diseases or healthy leaves
- **Supported Formats**: JPEG, PNG, WEBP, BMP
- **Max Size**: 10MB

## API Testing Workflows

### Vision Service (Direct Processing)

#### Health Check Workflow
1. **Basic Health Check** → `/health`
   - Verify service is running
   - Check response time < 5 seconds

2. **Detailed Health Check** → `/health/detailed`
   - Verify model loading status
   - Check available pest classes count

3. **Service Info** → `/info`
   - Verify model information
   - Check endpoint documentation

#### Pest Detection Workflow
1. **Basic Detection** → `/detect/pests`
   - Upload test image with pests
   - Use default confidence (0.01)
   - Verify detected pests list

2. **Detailed Detection** → `/detect/pests`
   - Set `return_details=true`
   - Verify bounding box coordinates
   - Check confidence scores

3. **High Confidence** → `/detect/pests`
   - Set `confidence_threshold=0.5`
   - Compare results with basic detection

#### Disease Detection Workflow
1. **Default Analysis** → `/detect/disease`
   - Upload leaf image
   - Check disease identification
   - Verify Thai language summary

2. **Custom Prompt** → `/detect/disease`
   - Use specific analysis prompt
   - Verify enhanced response detail

#### Comprehensive Analysis Workflow
1. **Basic Comprehensive** → `/analyze/comprehensive`
   - Single image for both analyses
   - Check parallel processing results
   - Verify summary generation

2. **Detailed Comprehensive** → `/analyze/comprehensive`
   - Enable pest details
   - Add custom disease prompt
   - Verify complete analysis report

### Queue Worker (Async Processing)

#### Setup Workflow
1. **Health Check** → `/health`
   - Verify Redis connection
   - Check Celery worker count

2. **Queue Stats** → `/queue/stats`
   - Check initial queue state
   - Verify worker availability

#### Async Job Workflow
1. **Queue Pest Job** → `/analyze/pest`
   - Upload image, get job_id
   - Note job_id for status checking

2. **Check Job Status** → `/jobs/{job_id}`
   - Monitor job progression
   - Wait for completion (status: "completed")

3. **Get Results** → `/jobs/{job_id}`
   - Retrieve final analysis results
   - Compare with direct API results

#### Comprehensive Testing
1. **Queue Multiple Jobs**:
   - Pest detection job
   - Disease detection job
   - Comprehensive analysis job

2. **Monitor Progress**:
   - Check queue statistics
   - Monitor job status changes
   - Verify parallel processing

3. **Image Storage**:
   - Store images with metadata
   - Check storage statistics
   - Trigger cleanup

## Test Scenarios

### Performance Testing

#### Response Time Benchmarks
- **Health checks**: < 2 seconds
- **Pest detection**: < 10 seconds
- **Disease detection**: < 30 seconds (LLaVA model)
- **Comprehensive**: < 45 seconds
- **Queue operations**: < 5 seconds

#### Load Testing
1. **Sequential Requests**:
   - Submit 10 pest detection requests
   - Monitor response times
   - Check for degradation

2. **Concurrent Jobs**:
   - Queue 5 jobs simultaneously
   - Monitor queue statistics
   - Verify all jobs complete

### Error Handling Testing

#### Input Validation
1. **Invalid Image Format**:
   - Upload text file as image
   - Expect 400 error with clear message

2. **Missing Parameters**:
   - Submit without image
   - Expect parameter validation error

3. **Invalid Parameter Values**:
   - Use non-numeric confidence threshold
   - Expect type validation error

#### Service Errors
1. **Large Image Upload**:
   - Upload >10MB image
   - Expect size limit error

2. **Invalid Job ID**:
   - Request status for non-existent job
   - Expect job not found error

### Integration Testing

#### Vision Service → Queue Worker
1. **Direct vs Async Comparison**:
   - Process same image via both APIs
   - Compare result consistency
   - Verify processing time differences

2. **Job Lifecycle**:
   - Queue → Pending → Processing → Completed
   - Monitor each status transition
   - Verify result persistence

## Expected Results

### Successful Responses

#### Pest Detection Response
```json
{
  "success": true,
  "data": {
    "detected_pests": ["aphid", "spider_mite"],
    "pest_count": 2,
    "has_pests": true,
    "thai_summary": "แมลงศัตรูพืชที่ตรวจพบ: aphid, spider_mite",
    "detection_details": [...]  // if return_details=true
  },
  "processing_time_ms": 1250.5,
  "timestamp": 1704067200.0
}
```

#### Disease Detection Response
```json
{
  "success": true,
  "data": {
    "raw_response": "This leaf shows signs of...",
    "disease_analysis": {
      "disease_name": "Leaf Spot",
      "severity": "Medium",
      "thai_summary": "พบอาการของโรคพืช: Leaf Spot ระดับความรุนแรง: ปานกลาง",
      "recommendations": ["แยกต้นพืชที่เป็นโรค...", "ปรึกษาผู้เชี่ยวชาญ..."],
      "is_healthy": false
    }
  },
  "processing_time_ms": 15420.2,
  "timestamp": 1704067200.0
}
```

#### Queue Job Response
```json
{
  "job_id": "celery-task-uuid-12345",
  "status": "queued",
  "type": "pest_detection"
}
```

### Error Responses

#### Validation Error
```json
{
  "detail": "Invalid image file type"
}
```

#### Job Not Found
```json
{
  "job_id": "invalid-job-id",
  "status": "error",
  "error": "Job not found"
}
```

## Performance Benchmarks

### Acceptable Performance
- **Health Check**: < 2s
- **Pest Detection**: < 10s
- **Disease Detection**: < 30s
- **Comprehensive**: < 45s
- **Queue Job**: < 5s

### Memory Usage
- **Vision Service**: < 8GB RAM
- **Queue Worker**: < 2GB RAM
- **Redis**: < 1GB RAM

### Concurrent Capacity
- **Direct API**: 5 concurrent requests
- **Queue System**: 20+ queued jobs
- **Worker Throughput**: 1 job/minute (disease), 4 jobs/minute (pest)

## Troubleshooting

### Common Issues

#### Model Loading Errors
- **Symptom**: 503 Service Unavailable
- **Check**: `/health/detailed` for model status
- **Solution**: Wait for model initialization or restart service

#### Queue Worker Offline
- **Symptom**: Jobs stuck in "pending"
- **Check**: `/queue/stats` for worker count
- **Solution**: Start Celery workers

#### High Memory Usage
- **Symptom**: Slow responses, OOM errors
- **Check**: System memory usage
- **Solution**: Reduce concurrent requests, restart services

#### Redis Connection Issues
- **Symptom**: Queue operations fail
- **Check**: Redis connectivity
- **Solution**: Verify Redis server status

### Monitoring Commands

```bash
# Check service logs
docker logs team10-vision-service
docker logs team10-queue-worker

# Monitor resource usage
docker stats team10-vision-service team10-queue-worker

# Test service connectivity
curl -f http://localhost:2001/health
curl -f http://localhost:2003/health
```

## Test Automation

### Postman Test Scripts

The collections include automated test scripts that verify:
- Response time limits
- Response structure validation
- Error handling consistency
- Job lifecycle management

### CI/CD Integration

For automated testing in pipelines:

```bash
# Install Newman (Postman CLI)
npm install -g newman

# Run Vision Service tests
newman run AI4Thai_Vision_Service.postman_collection.json \
  --env-var base_url=http://localhost:2001

# Run Queue Worker tests
newman run AI4Thai_Queue_Worker.postman_collection.json \
  --env-var queue_base_url=http://localhost:2003
```

## Best Practices

### Testing Strategy
1. **Start with health checks** before functional testing
2. **Use small test images** for faster iteration
3. **Monitor queue statistics** during load testing
4. **Test error cases** to verify robustness
5. **Compare direct vs async results** for consistency

### Image Selection
- **Pest Testing**: Clear images with visible insects
- **Disease Testing**: High-quality leaf close-ups
- **Edge Cases**: Blurry, dark, or unclear images
- **Negative Cases**: Healthy plants, non-agricultural images

### Performance Testing
- **Baseline**: Test with single requests first
- **Load**: Gradually increase concurrent requests
- **Monitor**: Watch for memory leaks or degradation
- **Recovery**: Test service behavior after errors

---

## Contact & Support

For issues with API testing or service deployment:
- Check service logs for detailed error information
- Verify all dependencies (Redis, Celery workers) are running
- Ensure adequate system resources (RAM, GPU for models)
- Review this testing guide for troubleshooting steps
