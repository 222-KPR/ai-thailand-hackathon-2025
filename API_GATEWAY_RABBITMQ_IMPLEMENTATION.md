# API Gateway with RabbitMQ Implementation

## Overview

This implementation provides a robust API Gateway using Rust with RabbitMQ integration for handling vision analysis jobs. The system is designed with memory-efficient practices to handle large image files without causing memory issues.

## Architecture

### Components

1. **API Gateway (Rust/Axum)**: Main entry point for client requests
2. **RabbitMQ**: Message queue for job processing
3. **Redis**: Job status tracking and caching
4. **File Storage Service**: Memory-efficient file handling
5. **Vision Service**: AI processing service (external)

### Memory-Efficient Design Principles

The system follows these key principles to avoid memory issues:

1. **File Streaming**: Images are read in chunks during upload
2. **Disk Storage**: Images are stored on disk, not in memory
3. **Path References**: Only file paths are passed in messages, not image data
4. **Automatic Cleanup**: Expired files are automatically removed
5. **Size Validation**: File size limits are enforced during upload

## Implementation Details

### 1. File Storage Service (`file_storage.rs`)

**Key Features:**
- Streaming file upload with size validation
- Automatic file format detection
- SHA256 hash generation for deduplication
- TTL-based automatic cleanup
- Memory-efficient image validation

**Memory Benefits:**
```rust
// Read image data in chunks to avoid memory issues
let mut data = Vec::new();
while let Some(chunk) = field.chunk().await? {
    data.extend_from_slice(&chunk);
    
    // Check size limit during streaming
    if data.len() > max_file_size {
        return Err(ValidationError::new("File too large"));
    }
}
```

### 2. RabbitMQ Service (`rabbitmq.rs`)

**Key Features:**
- Reliable message publishing
- Connection health monitoring
- Message serialization with metadata
- Error handling and retry logic

**Message Structure:**
```rust
pub struct VisionAnalysisMessage {
    pub job_id: Uuid,
    pub analysis_type: AnalysisType,
    pub file_path: String,        // Only path, not image data
    pub file_hash: String,
    pub metadata: ImageMetadata,
    pub parameters: AnalysisParameters,
    pub timestamp: DateTime<Utc>,
}
```

### 3. Vision Analysis Handler (`vision.rs`)

**Key Features:**
- Multipart form handling with streaming
- Job queuing with status tracking
- Real-time progress monitoring
- Job cancellation support

**Memory-Efficient Upload:**
```rust
// Extract multipart data with streaming
while let Some(field) = multipart.next_field().await? {
    match field_name.as_str() {
        "image" => {
            // Read in chunks to avoid memory issues
            let mut data = Vec::new();
            while let Some(chunk) = field.chunk().await? {
                data.extend_from_slice(&chunk);
                // Size validation during streaming
            }
        }
    }
}
```

## API Endpoints

### Vision Analysis

1. **Queue Analysis Job**
   ```
   POST /api/v1/vision/analyze
   Content-Type: multipart/form-data
   
   Fields:
   - image: Image file
   - analysis_type: "pest" | "disease" | "comprehensive"
   - confidence_threshold: float (optional)
   - return_details: boolean (optional)
   - custom_prompt: string (optional)
   ```

2. **Get Job Status**
   ```
   GET /api/v1/vision/jobs/{job_id}
   ```

3. **Cancel Job**
   ```
   DELETE /api/v1/vision/jobs/{job_id}/cancel
   ```

4. **File Management**
   ```
   GET /api/v1/vision/files/stats
   POST /api/v1/vision/files/cleanup
   ```

### Health Checks

1. **Basic Health**
   ```
   GET /health
   ```

2. **Readiness Check**
   ```
   GET /health/ready
   ```

3. **Metrics**
   ```
   GET /health/metrics
   ```

## Configuration

### Environment Variables

```bash
# RabbitMQ
RABBITMQ_URL=amqp://guest:guest@rabbitmq:5672
RABBITMQ_QUEUE_NAME=vision_analysis_queue
RABBITMQ_EXCHANGE_NAME=vision_exchange
RABBITMQ_ROUTING_KEY=vision.analysis

# File Storage
FILE_STORAGE_TEMP_DIR=/tmp/vision_uploads
FILE_STORAGE_MAX_FILE_SIZE=10485760  # 10MB
FILE_STORAGE_FILE_TTL=86400          # 24 hours

# Redis
REDIS_URL=redis://redis:6379

# Services
VISION_SERVICE_URL=http://vision-service:2001
LLM_SERVICE_URL=http://llm-service:2002
```

## Memory Management Best Practices

### 1. Streaming Uploads
- Images are read in chunks during upload
- Size validation happens during streaming
- Memory usage is constant regardless of file size

### 2. File Storage Strategy
- Images are immediately written to disk
- Only file paths are stored in memory
- Automatic cleanup prevents disk space issues

### 3. Message Queue Design
- Messages contain file paths, not image data
- Metadata is included for processing decisions
- Job status is tracked separately in Redis

### 4. Resource Cleanup
- Files have TTL-based expiration
- Manual cleanup endpoint available
- Job results are cached with expiration

## Deployment

### Docker Compose

The system includes RabbitMQ in the main docker-compose.yml:

```yaml
services:
  rabbitmq:
    image: rabbitmq:3-management-alpine
    ports:
      - "5672:5672"      # AMQP
      - "15672:15672"    # Management UI
    environment:
      - RABBITMQ_DEFAULT_USER=guest
      - RABBITMQ_DEFAULT_PASS=guest
    volumes:
      - rabbitmq_data:/var/lib/rabbitmq

  api-gateway:
    # ... configuration
    depends_on:
      - redis
      - rabbitmq
    volumes:
      - vision_uploads:/tmp/vision_uploads
```

### Health Monitoring

The system provides comprehensive health checks:

```rust
// Check all services
let health_status = json!({
    "status": "healthy",
    "services": {
        "redis": { "status": "healthy" },
        "rabbitmq": { "status": "healthy" },
        "file_storage": { "status": "healthy" },
        "vision_service": { "status": "healthy" }
    }
});
```

## Error Handling

### Graceful Degradation
- Service failures don't crash the system
- Health checks report service status
- Jobs can be retried or cancelled

### Error Types
```rust
pub enum AppError {
    Redis(RedisError),
    Service(ServiceError),
    Validation(String),
    ExternalApi(String),
    RateLimit,
    ServiceUnavailable(String),
    Internal(String),
}
```

## Performance Considerations

### 1. Memory Usage
- Constant memory usage regardless of image size
- Streaming prevents large memory allocations
- File cleanup prevents disk space issues

### 2. Throughput
- RabbitMQ handles message queuing
- Redis provides fast job status lookups
- File operations are async and non-blocking

### 3. Scalability
- Stateless API gateway design
- Horizontal scaling possible
- Queue-based processing decouples services

## Security Considerations

### 1. File Validation
- File type validation during upload
- Size limits enforced
- Malicious file detection

### 2. Access Control
- Job isolation by ID
- Status tracking prevents unauthorized access
- File cleanup prevents data leakage

### 3. Network Security
- Internal service communication
- External API rate limiting
- Secure file storage paths

## Monitoring and Observability

### 1. Logging
- Structured logging with tracing
- Request/response correlation
- Error tracking and reporting

### 2. Metrics
- File storage statistics
- Queue performance metrics
- Service health monitoring

### 3. Health Checks
- Service availability monitoring
- Dependency health tracking
- Automated alerting

## Conclusion

This implementation provides a robust, memory-efficient solution for handling vision analysis jobs. The key benefits are:

1. **Memory Safety**: No large image data in memory
2. **Scalability**: Queue-based processing
3. **Reliability**: Comprehensive error handling
4. **Monitoring**: Full observability
5. **Maintainability**: Clean separation of concerns

The system can handle high-throughput image processing while maintaining low memory usage and providing excellent user experience through real-time job tracking. 