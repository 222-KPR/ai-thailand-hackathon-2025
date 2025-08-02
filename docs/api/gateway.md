# API Gateway Specification

## Overview

The API Gateway serves as the central entry point for all client requests, handling authentication, routing, and communication with backend services.

**Base URL**: `http://localhost:3000` (development) | `https://api.ai4thai.com` (production)

## Authentication

All endpoints except `/health` and `/auth/*` require authentication via JWT token.

```http
Authorization: Bearer <jwt_token>
```

## API Endpoints

### Authentication Endpoints

#### POST /auth/register
Register a new user account.

**Request Body:**
```json
{
  "email": "farmer@example.com",
  "password": "securePassword123",
  "full_name": "John Farmer",
  "phone": "+66812345678",
  "location": {
    "province": "Chiang Mai",
    "district": "Mueang",
    "coordinates": {
      "lat": 18.7883,
      "lng": 98.9853
    }
  },
  "farm_info": {
    "farm_size": 5.5,
    "primary_crops": ["rice", "cassava"],
    "farming_experience": 10
  }
}
```

**Response (201):**
```json
{
  "success": true,
  "data": {
    "user_id": "usr_123456789",
    "email": "farmer@example.com",
    "full_name": "John Farmer",
    "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9...",
    "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9...",
    "expires_in": 3600
  },
  "message": "User registered successfully",
  "timestamp": "2025-08-02T13:00:00Z"
}
```

#### POST /auth/login
Authenticate user and receive access tokens.

**Request Body:**
```json
{
  "email": "farmer@example.com",
  "password": "securePassword123"
}
```

**Response (200):**
```json
{
  "success": true,
  "data": {
    "user_id": "usr_123456789",
    "email": "farmer@example.com",
    "full_name": "John Farmer",
    "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9...",
    "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9...",
    "expires_in": 3600
  },
  "message": "Login successful",
  "timestamp": "2025-08-02T13:00:00Z"
}
```

#### POST /auth/refresh
Refresh access token using refresh token.

**Request Body:**
```json
{
  "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9..."
}
```

**Response (200):**
```json
{
  "success": true,
  "data": {
    "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9...",
    "expires_in": 3600
  },
  "message": "Token refreshed successfully",
  "timestamp": "2025-08-02T13:00:00Z"
}
```

### Disease Detection Endpoints

#### POST /api/diagnose
Analyze crop image for disease detection.

**Request (multipart/form-data):**
```
image: <file> (required) - Image file (JPG, PNG, WebP, max 10MB)
crop_type: string (required) - One of: rice, cassava, durian, mango, rubber
confidence_threshold: number (optional, default: 0.7) - Minimum confidence (0.0-1.0)
location: string (optional) - Farm location for context
```

**Response (200):**
```json
{
  "success": true,
  "data": {
    "diagnosis_id": "diag_123456789",
    "crop_type": "rice",
    "image_analysis": {
      "image_quality": "good",
      "crop_detected": true,
      "affected_area_percentage": 25.5
    },
    "diseases": [
      {
        "name": "Rice Blast",
        "scientific_name": "Pyricularia oryzae",
        "confidence": 0.95,
        "severity": "moderate",
        "affected_parts": ["leaves", "stems"],
        "description": "Fungal disease causing lesions on leaves and stems"
      }
    ],
    "treatment_recommendations": [
      {
        "type": "chemical",
        "treatment": "Apply fungicide containing tricyclazole",
        "dosage": "2ml per liter of water",
        "frequency": "Every 7-10 days",
        "duration": "2-3 applications"
      },
      {
        "type": "cultural",
        "treatment": "Improve field drainage",
        "description": "Reduce humidity to prevent fungal growth"
      }
    ],
    "prevention_tips": [
      "Use resistant rice varieties",
      "Maintain proper plant spacing",
      "Apply balanced fertilization"
    ],
    "confidence_score": 0.95,
    "processing_time_ms": 1850
  },
  "message": "Disease detection completed",
  "timestamp": "2025-08-02T13:00:00Z"
}
```

#### GET /api/diagnoses
Get user's diagnosis history.

**Query Parameters:**
- `page`: number (default: 1) - Page number
- `limit`: number (default: 20) - Items per page
- `crop_type`: string (optional) - Filter by crop type
- `date_from`: string (optional) - ISO date string
- `date_to`: string (optional) - ISO date string

**Response (200):**
```json
{
  "success": true,
  "data": {
    "diagnoses": [
      {
        "diagnosis_id": "diag_123456789",
        "crop_type": "rice",
        "diseases": ["Rice Blast"],
        "confidence_score": 0.95,
        "created_at": "2025-08-02T13:00:00Z",
        "image_url": "/uploads/images/diag_123456789.jpg"
      }
    ],
    "pagination": {
      "current_page": 1,
      "total_pages": 5,
      "total_items": 95,
      "items_per_page": 20
    }
  },
  "message": "Diagnoses retrieved successfully",
  "timestamp": "2025-08-02T13:00:00Z"
}
```

### Chat Endpoints

#### POST /api/chat/conversations
Create a new conversation.

**Request Body:**
```json
{
  "title": "Rice farming questions",
  "context": {
    "crop_type": "rice",
    "location": "Chiang Mai",
    "season": "wet"
  }
}
```

**Response (201):**
```json
{
  "success": true,
  "data": {
    "conversation_id": "conv_123456789",
    "title": "Rice farming questions",
    "created_at": "2025-08-02T13:00:00Z",
    "message_count": 0
  },
  "message": "Conversation created successfully",
  "timestamp": "2025-08-02T13:00:00Z"
}
```

#### POST /api/chat/conversations/{conversation_id}/messages
Send a message in a conversation.

**Request Body:**
```json
{
  "message": "How do I treat rice blast disease?",
  "message_type": "text",
  "language": "th",
  "attachments": [
    {
      "type": "image",
      "url": "/uploads/images/crop_image.jpg"
    }
  ]
}
```

**Response (200):**
```json
{
  "success": true,
  "data": {
    "message_id": "msg_123456789",
    "conversation_id": "conv_123456789",
    "user_message": {
      "content": "How do I treat rice blast disease?",
      "type": "text",
      "language": "th",
      "timestamp": "2025-08-02T13:00:00Z"
    },
    "ai_response": {
      "content": "การรักษาโรคไหม้ข้าวสามารถทำได้หลายวิธี...",
      "type": "text",
      "language": "th",
      "confidence": 0.92,
      "sources": ["agricultural_knowledge_base", "disease_database"],
      "timestamp": "2025-08-02T13:00:05Z"
    },
    "processing_time_ms": 2150
  },
  "message": "Message sent successfully",
  "timestamp": "2025-08-02T13:00:05Z"
}
```

#### GET /api/chat/conversations/{conversation_id}/messages
Get conversation message history.

**Query Parameters:**
- `page`: number (default: 1)
- `limit`: number (default: 50)

**Response (200):**
```json
{
  "success": true,
  "data": {
    "conversation_id": "conv_123456789",
    "messages": [
      {
        "message_id": "msg_123456789",
        "sender": "user",
        "content": "How do I treat rice blast disease?",
        "type": "text",
        "language": "th",
        "timestamp": "2025-08-02T13:00:00Z"
      },
      {
        "message_id": "msg_123456790",
        "sender": "ai",
        "content": "การรักษาโรคไหม้ข้าวสามารถทำได้หลายวิธี...",
        "type": "text",
        "language": "th",
        "confidence": 0.92,
        "timestamp": "2025-08-02T13:00:05Z"
      }
    ],
    "pagination": {
      "current_page": 1,
      "total_pages": 3,
      "total_items": 25,
      "items_per_page": 50
    }
  },
  "message": "Messages retrieved successfully",
  "timestamp": "2025-08-02T13:00:00Z"
}
```

### User Profile Endpoints

#### GET /api/profile
Get current user profile.

**Response (200):**
```json
{
  "success": true,
  "data": {
    "user_id": "usr_123456789",
    "email": "farmer@example.com",
    "full_name": "John Farmer",
    "phone": "+66812345678",
    "location": {
      "province": "Chiang Mai",
      "district": "Mueang",
      "coordinates": {
        "lat": 18.7883,
        "lng": 98.9853
      }
    },
    "farm_info": {
      "farm_size": 5.5,
      "primary_crops": ["rice", "cassava"],
      "farming_experience": 10
    },
    "preferences": {
      "language": "th",
      "notifications": {
        "email": true,
        "push": true,
        "sms": false
      }
    },
    "statistics": {
      "total_diagnoses": 45,
      "total_conversations": 12,
      "member_since": "2024-01-15T00:00:00Z"
    }
  },
  "message": "Profile retrieved successfully",
  "timestamp": "2025-08-02T13:00:00Z"
}
```

#### PUT /api/profile
Update user profile.

**Request Body:**
```json
{
  "full_name": "John Updated Farmer",
  "phone": "+66812345679",
  "location": {
    "province": "Chiang Mai",
    "district": "Mueang",
    "coordinates": {
      "lat": 18.7883,
      "lng": 98.9853
    }
  },
  "farm_info": {
    "farm_size": 6.0,
    "primary_crops": ["rice", "cassava", "corn"],
    "farming_experience": 11
  },
  "preferences": {
    "language": "th",
    "notifications": {
      "email": true,
      "push": true,
      "sms": true
    }
  }
}
```

### Weather Integration Endpoints

#### GET /api/weather/current
Get current weather for user's location.

**Response (200):**
```json
{
  "success": true,
  "data": {
    "location": {
      "province": "Chiang Mai",
      "district": "Mueang",
      "coordinates": {
        "lat": 18.7883,
        "lng": 98.9853
      }
    },
    "current": {
      "temperature": 28.5,
      "humidity": 75,
      "rainfall": 0,
      "wind_speed": 5.2,
      "weather_condition": "partly_cloudy",
      "description": "Partly cloudy with light winds"
    },
    "agricultural_advice": [
      "Good conditions for rice planting",
      "Monitor humidity levels for disease prevention"
    ],
    "last_updated": "2025-08-02T13:00:00Z"
  },
  "message": "Weather data retrieved successfully",
  "timestamp": "2025-08-02T13:00:00Z"
}
```

### System Endpoints

#### GET /health
System health check.

**Response (200):**
```json
{
  "status": "healthy",
  "timestamp": "2025-08-02T13:00:00Z",
  "version": "1.0.0",
  "services": {
    "database": "healthy",
    "redis": "healthy",
    "vision_service": "healthy",
    "llm_service": "healthy"
  },
  "uptime": 86400
}
```

#### GET /api/system/info
Get system information and supported features.

**Response (200):**
```json
{
  "success": true,
  "data": {
    "version": "1.0.0",
    "supported_crops": ["rice", "cassava", "durian", "mango", "rubber"],
    "supported_languages": ["th", "en"],
    "supported_image_formats": ["jpg", "jpeg", "png", "webp"],
    "max_image_size": 10485760,
    "features": {
      "disease_detection": true,
      "chat_interface": true,
      "weather_integration": true,
      "voice_input": true,
      "offline_mode": false
    },
    "ai_models": {
      "vision": {
        "crop_classifier": "google/vit-base-patch16-224",
        "disease_detector": "microsoft/resnet-50"
      },
      "llm": {
        "agricultural_advisor": "microsoft/DialoGPT-large",
        "thai_language": "airesearch/wangchanberta-base-att-spm-uncased"
      }
    }
  },
  "message": "System information retrieved successfully",
  "timestamp": "2025-08-02T13:00:00Z"
}
```

## Error Responses

### Standard Error Format

```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid input data",
    "details": {
      "field": "email",
      "reason": "Invalid email format"
    }
  },
  "timestamp": "2025-08-02T13:00:00Z"
}
```

### Common Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `VALIDATION_ERROR` | 400 | Request validation failed |
| `UNAUTHORIZED` | 401 | Authentication required |
| `FORBIDDEN` | 403 | Insufficient permissions |
| `NOT_FOUND` | 404 | Resource not found |
| `RATE_LIMIT_EXCEEDED` | 429 | Too many requests |
| `INTERNAL_ERROR` | 500 | Internal server error |
| `SERVICE_UNAVAILABLE` | 503 | External service unavailable |

## Rate Limits

| Endpoint Category | Limit | Window |
|-------------------|-------|--------|
| Authentication | 10 requests | 1 minute |
| Disease Detection | 50 requests | 1 hour |
| Chat Messages | 200 requests | 1 hour |
| Profile Updates | 20 requests | 1 hour |
| General API | 1000 requests | 1 hour |

## WebSocket API

### Connection
```
ws://localhost:3000/ws?token=<jwt_token>
```

### Message Format
```json
{
  "type": "chat_message",
  "conversation_id": "conv_123456789",
  "data": {
    "message": "Hello, I need help with my crops",
    "language": "th"
  }
}
```

### Response Format
```json
{
  "type": "chat_response",
  "conversation_id": "conv_123456789",
  "data": {
    "message": "สวัสดีครับ ผมพร้อมช่วยเหลือเรื่องการเกษตรครับ",
    "confidence": 0.95,
    "timestamp": "2025-08-02T13:00:00Z"
  }
}
```
