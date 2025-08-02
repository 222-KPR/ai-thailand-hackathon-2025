# Vision Service API Specification

## Overview

The Vision Service provides computer vision capabilities for crop disease detection and plant classification using HuggingFace transformer models.

**Base URL**: `http://localhost:8001` (development) | `https://vision.ai4thai.com` (production)

## Models

### Crop Classification Model
- **Model**: `google/vit-base-patch16-224`
- **Purpose**: Identify crop types from images
- **Input**: 224x224 RGB images
- **Supported Crops**: rice, cassava, durian, mango, rubber

### Disease Detection Model
- **Model**: `microsoft/resnet-50` (fine-tuned)
- **Purpose**: Detect diseases in crop images
- **Input**: 256x256 RGB images
- **Confidence Threshold**: 0.7 (configurable)

## API Endpoints

### Health Check Endpoints

#### GET /health
Basic health check.

**Response (200):**
```json
{
  "status": "healthy",
  "timestamp": 1691000000.0
}
```

#### GET /health/detailed
Detailed health check with service status.

**Response (200):**
```json
{
  "status": "healthy",
  "timestamp": 1691000000.0,
  "services": {
    "model_registry": "healthy",
    "image_processor": "healthy",
    "inference_engine": "healthy",
    "gpu": "available"
  },
  "system": {
    "cpu_usage": 45.2,
    "memory_usage": 68.5,
    "gpu_usage": 32.1,
    "gpu_memory": 4096
  }
}
```

#### GET /health/models
Check model loading status.

**Response (200):**
```json
{
  "models": {
    "crop_classifier": {
      "status": "loaded",
      "model_name": "google/vit-base-patch16-224",
      "load_time": 15.2,
      "memory_usage": 1024
    },
    "disease_detector": {
      "status": "loaded",
      "model_name": "microsoft/resnet-50",
      "load_time": 12.8,
      "memory_usage": 2048
    }
  }
}
```

### Model Information

#### GET /models
List available models and their configurations.

**Response (200):**
```json
{
  "models": {
    "crop_classifier": {
      "model_name": "google/vit-base-patch16-224",
      "model_type": "vision_transformer",
      "input_size": [224, 224],
      "supported_crops": ["rice", "cassava", "durian", "mango", "rubber"],
      "confidence_threshold": 0.8,
      "batch_size": 8
    },
    "disease_detector": {
      "model_name": "microsoft/resnet-50",
      "model_type": "convolutional_neural_network",
      "input_size": [256, 256],
      "supported_diseases": {
        "rice": ["blast", "brown_spot", "bacterial_blight"],
        "cassava": ["mosaic_virus", "root_rot"],
        "durian": ["fusarium_wilt", "anthracnose"],
        "mango": ["anthracnose", "powdery_mildew"],
        "rubber": ["leaf_blight", "tapping_panel_dryness"]
      },
      "confidence_threshold": 0.7,
      "batch_size": 4
    }
  }
}
```

### Disease Detection

#### POST /detect/disease
Detect diseases in crop images.

**Request (multipart/form-data):**
```
image: <file> (required) - Image file (JPG, PNG, WebP, max 10MB)
crop_type: string (required) - One of: rice, cassava, durian, mango, rubber
confidence_threshold: number (optional, default: 0.7) - Minimum confidence (0.0-1.0)
```

**Response (200):**
```json
{
  "success": true,
  "data": {
    "crop_type": "rice",
    "image_analysis": {
      "image_quality": "good",
      "resolution": [1024, 768],
      "crop_detected": true,
      "crop_confidence": 0.98,
      "affected_area_percentage": 25.5,
      "image_preprocessing": {
        "resized_to": [256, 256],
        "normalized": true,
        "augmented": false
      }
    },
    "diseases": [
      {
        "disease_id": "rice_blast",
        "name": "Rice Blast",
        "scientific_name": "Pyricularia oryzae",
        "confidence": 0.95,
        "severity": "moderate",
        "affected_parts": ["leaves", "stems"],
        "bounding_boxes": [
          {
            "x": 120,
            "y": 80,
            "width": 200,
            "height": 150,
            "confidence": 0.95
          }
        ],
        "symptoms": [
          "Diamond-shaped lesions on leaves",
          "Gray centers with brown borders",
          "Lesions on stems and panicles"
        ],
        "description": "Fungal disease causing lesions on leaves and stems"
      }
    ],
    "model_info": {
      "model_name": "microsoft/resnet-50",
      "model_version": "1.0.0",
      "inference_time_ms": 850,
      "preprocessing_time_ms": 120,
      "postprocessing_time_ms": 80
    },
    "confidence_score": 0.95
  },
  "processing_time_ms": 1850,
  "timestamp": 1691000000.0
}
```

### Crop Classification

#### POST /classify/crop
Classify crop type from images.

**Request (multipart/form-data):**
```
image: <file> (required) - Image file (JPG, PNG, WebP, max 10MB)
confidence_threshold: number (optional, default: 0.8) - Minimum confidence (0.0-1.0)
```

**Response (200):**
```json
{
  "success": true,
  "data": {
    "predictions": [
      {
        "crop_type": "rice",
        "confidence": 0.92,
        "scientific_name": "Oryza sativa",
        "common_names": {
          "th": "ข้าว",
          "en": "Rice"
        },
        "growth_stage": "vegetative",
        "growth_stage_confidence": 0.85
      },
      {
        "crop_type": "cassava",
        "confidence": 0.06,
        "scientific_name": "Manihot esculenta",
        "common_names": {
          "th": "มันสำปะหลัง",
          "en": "Cassava"
        }
      }
    ],
    "top_prediction": {
      "crop_type": "rice",
      "confidence": 0.92,
      "scientific_name": "Oryza sativa",
      "common_names": {
        "th": "ข้าว",
        "en": "Rice"
      }
    },
    "image_analysis": {
      "image_quality": "excellent",
      "resolution": [1024, 768],
      "plant_coverage": 0.78,
      "lighting_conditions": "good"
    },
    "model_info": {
      "model_name": "google/vit-base-patch16-224",
      "model_version": "1.0.0",
      "inference_time_ms": 650
    }
  },
  "processing_time_ms": 1200,
  "timestamp": 1691000000.0
}
```

### Batch Processing

#### POST /analyze/batch
Batch analysis of multiple images.

**Request (multipart/form-data):**
```
images: <file[]> (required) - Array of image files (max 10 images)
crop_types: string[] (required) - Array of crop types corresponding to images
confidence_threshold: number (optional, default: 0.7) - Minimum confidence
```

**Response (200):**
```json
{
  "success": true,
  "data": {
    "batch_id": "batch_123456789",
    "batch_size": 3,
    "results": [
      {
        "image_index": 0,
        "filename": "rice_field_1.jpg",
        "crop_type": "rice",
        "status": "success",
        "result": {
          "diseases": [
            {
              "name": "Rice Blast",
              "confidence": 0.95,
              "severity": "moderate"
            }
          ],
          "confidence_score": 0.95
        },
        "processing_time_ms": 1850
      },
      {
        "image_index": 1,
        "filename": "cassava_plant.jpg",
        "crop_type": "cassava",
        "status": "success",
        "result": {
          "diseases": [
            {
              "name": "Cassava Mosaic Virus",
              "confidence": 0.88,
              "severity": "severe"
            }
          ],
          "confidence_score": 0.88
        },
        "processing_time_ms": 1920
      },
      {
        "image_index": 2,
        "filename": "blurry_image.jpg",
        "crop_type": "rice",
        "status": "error",
        "error": "Image quality too low for analysis",
        "processing_time_ms": 200
      }
    ],
    "summary": {
      "total_images": 3,
      "successful_analyses": 2,
      "failed_analyses": 1,
      "average_processing_time_ms": 1323,
      "total_processing_time_ms": 3970
    }
  },
  "processing_time_ms": 3970,
  "timestamp": 1691000000.0
}
```

### Service Information

#### GET /info
Get service information and capabilities.

**Response (200):**
```json
{
  "service": "AI4Thai Vision Service",
  "version": "1.0.0",
  "models": [
    {
      "name": "crop_classifier",
      "model": "google/vit-base-patch16-224",
      "status": "loaded"
    },
    {
      "name": "disease_detector",
      "model": "microsoft/resnet-50",
      "status": "loaded"
    }
  ],
  "supported_crops": ["rice", "cassava", "durian", "mango", "rubber"],
  "supported_formats": ["jpg", "jpeg", "png", "webp"],
  "max_image_size": 10485760,
  "max_batch_size": 10,
  "capabilities": {
    "disease_detection": true,
    "crop_classification": true,
    "batch_processing": true,
    "severity_assessment": true,
    "bounding_box_detection": true
  },
  "hardware": {
    "gpu_available": true,
    "gpu_count": 1,
    "gpu_memory": 8192,
    "cuda_version": "11.8"
  },
  "performance": {
    "average_inference_time_ms": 850,
    "max_concurrent_requests": 4,
    "requests_processed": 15420,
    "uptime_seconds": 86400
  }
}
```

### Metrics

#### GET /metrics
Get service metrics (Prometheus format).

**Response (200):**
```
# HELP vision_service_requests_total Total number of requests processed
# TYPE vision_service_requests_total counter
vision_service_requests_total{endpoint="detect_disease",status="success"} 1234
vision_service_requests_total{endpoint="detect_disease",status="error"} 56
vision_service_requests_total{endpoint="classify_crop",status="success"} 789

# HELP vision_service_processing_time_seconds Processing time in seconds
# TYPE vision_service_processing_time_seconds histogram
vision_service_processing_time_seconds_bucket{endpoint="detect_disease",le="1.0"} 500
vision_service_processing_time_seconds_bucket{endpoint="detect_disease",le="2.0"} 1200
vision_service_processing_time_seconds_bucket{endpoint="detect_disease",le="5.0"} 1290
vision_service_processing_time_seconds_bucket{endpoint="detect_disease",le="+Inf"} 1290

# HELP vision_service_model_inference_time_seconds Model inference time
# TYPE vision_service_model_inference_time_seconds gauge
vision_service_model_inference_time_seconds{model="crop_classifier"} 0.65
vision_service_model_inference_time_seconds{model="disease_detector"} 0.85

# HELP vision_service_gpu_utilization GPU utilization percentage
# TYPE vision_service_gpu_utilization gauge
vision_service_gpu_utilization 32.5

# HELP vision_service_memory_usage_bytes Memory usage in bytes
# TYPE vision_service_memory_usage_bytes gauge
vision_service_memory_usage_bytes{type="system"} 2147483648
vision_service_memory_usage_bytes{type="gpu"} 3221225472
```

## Error Responses

### Standard Error Format

```json
{
  "success": false,
  "error": {
    "code": "INVALID_IMAGE_FORMAT",
    "message": "Unsupported image format",
    "details": {
      "supported_formats": ["jpg", "jpeg", "png", "webp"],
      "received_format": "gif"
    }
  },
  "timestamp": 1691000000.0
}
```

### Vision Service Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `INVALID_IMAGE_FORMAT` | 400 | Unsupported image format |
| `IMAGE_TOO_LARGE` | 400 | Image exceeds size limit |
| `IMAGE_QUALITY_LOW` | 400 | Image quality insufficient for analysis |
| `INVALID_CROP_TYPE` | 400 | Unsupported crop type |
| `MODEL_NOT_LOADED` | 503 | Required model not loaded |
| `GPU_OUT_OF_MEMORY` | 503 | Insufficient GPU memory |
| `INFERENCE_TIMEOUT` | 504 | Model inference timeout |
| `BATCH_SIZE_EXCEEDED` | 400 | Too many images in batch |

## Image Requirements

### Supported Formats
- JPEG (.jpg, .jpeg)
- PNG (.png)
- WebP (.webp)

### Size Limits
- **Maximum file size**: 10MB
- **Minimum resolution**: 224x224 pixels
- **Maximum resolution**: 4096x4096 pixels
- **Aspect ratio**: Any (will be resized maintaining aspect ratio)

### Quality Guidelines
- **Good lighting**: Avoid shadows and overexposure
- **Clear focus**: Ensure the plant/disease area is in focus
- **Crop coverage**: Plant should cover at least 50% of the image
- **Background**: Minimize background clutter
- **Distance**: Capture from appropriate distance to show details

## Rate Limits

| Endpoint | Limit | Window |
|----------|-------|--------|
| `/detect/disease` | 50 requests | 1 hour |
| `/classify/crop` | 100 requests | 1 hour |
| `/analyze/batch` | 10 requests | 1 hour |
| Health checks | 1000 requests | 1 hour |

## Performance Characteristics

### Response Times (95th percentile)
- **Disease Detection**: < 2 seconds
- **Crop Classification**: < 1.5 seconds
- **Batch Processing**: < 10 seconds (for 5 images)

### Throughput
- **Concurrent Requests**: Up to 4 (GPU limited)
- **Queue Capacity**: 20 requests
- **Processing Rate**: ~40 images per minute

### Resource Usage
- **CPU**: 4-8 cores recommended
- **RAM**: 8-16GB recommended
- **GPU**: 8GB VRAM minimum, 16GB recommended
- **Storage**: 50GB for models and cache
