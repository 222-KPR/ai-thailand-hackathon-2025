# API Documentation

This section contains comprehensive API documentation for all services in the AI4Thai Crop Guardian system.

## 📋 API Services

### 🌐 [API Gateway](gateway.md)
Main API gateway endpoints for client applications.

### 👁️ [Vision Service](vision-service.md)
Computer vision and image processing endpoints.

### 🤖 [LLM Service](llm-service.md)
Language model and advisory service endpoints.

### 👤 [User Service](user-service.md)
User management and authentication endpoints.

### 💬 [Chat Service](chat-service.md)
Real-time messaging and WebSocket endpoints.

## 🔗 Base URLs

### Development
- **API Gateway**: `http://localhost:3000`
- **Vision Service**: `http://localhost:8001`
- **LLM Service**: `http://localhost:8002`

### Production
- **API Gateway**: `https://api.ai4thai.com`
- **Vision Service**: `https://vision.ai4thai.com`
- **LLM Service**: `https://llm.ai4thai.com`

## 🔐 Authentication

All API endpoints require authentication unless otherwise specified. The system uses JWT (JSON Web Tokens) for authentication.

### Authentication Flow

1. **Login**: POST `/auth/login` with credentials
2. **Receive Token**: Get JWT access token and refresh token
3. **Use Token**: Include in `Authorization: Bearer <token>` header
4. **Refresh Token**: Use refresh token to get new access token

### Example Authentication

```bash
# Login
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "farmer@example.com", "password": "password123"}'

# Response
{
  "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9...",
  "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9...",
  "expires_in": 3600,
  "token_type": "Bearer"
}

# Use token in subsequent requests
curl -X GET http://localhost:3000/api/profile \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9..."
```

## 📊 Response Format

All API responses follow a consistent format:

### Success Response
```json
{
  "success": true,
  "data": {
    // Response data
  },
  "message": "Operation completed successfully",
  "timestamp": "2025-08-02T11:00:00Z"
}
```

### Error Response
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
  "timestamp": "2025-08-02T11:00:00Z"
}
```

## 🔢 HTTP Status Codes

| Code | Meaning | Description |
|------|---------|-------------|
| 200 | OK | Request successful |
| 201 | Created | Resource created successfully |
| 400 | Bad Request | Invalid request data |
| 401 | Unauthorized | Authentication required |
| 403 | Forbidden | Insufficient permissions |
| 404 | Not Found | Resource not found |
| 422 | Unprocessable Entity | Validation error |
| 429 | Too Many Requests | Rate limit exceeded |
| 500 | Internal Server Error | Server error |
| 503 | Service Unavailable | Service temporarily unavailable |

## 🚦 Rate Limiting

API endpoints are rate-limited to ensure fair usage and system stability.

### Rate Limits
- **Authenticated Users**: 1000 requests per hour
- **Anonymous Users**: 100 requests per hour
- **Image Upload**: 50 requests per hour
- **Chat Messages**: 200 requests per hour

### Rate Limit Headers
```
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1659441600
```

## 📝 Request/Response Examples

### Disease Detection
```bash
# Upload image for disease detection
curl -X POST http://localhost:3000/api/diagnose \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: multipart/form-data" \
  -F "image=@crop_image.jpg" \
  -F "crop_type=rice"

# Response
{
  "success": true,
  "data": {
    "diagnosis_id": "diag_123456",
    "crop_type": "rice",
    "diseases": [
      {
        "name": "Rice Blast",
        "confidence": 0.95,
        "severity": "moderate",
        "affected_area": 0.3
      }
    ],
    "treatment_advice": "Apply fungicide containing tricyclazole...",
    "confidence_score": 0.95,
    "processing_time_ms": 1850
  },
  "message": "Disease detection completed",
  "timestamp": "2025-08-02T11:00:00Z"
}
```

### Chat Message
```bash
# Send chat message
curl -X POST http://localhost:3000/api/chat/messages \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "conversation_id": "conv_123",
    "message": "How do I treat rice blast disease?",
    "message_type": "text"
  }'

# Response
{
  "success": true,
  "data": {
    "message_id": "msg_456",
    "conversation_id": "conv_123",
    "response": "Rice blast disease can be treated with...",
    "response_type": "text",
    "created_at": "2025-08-02T11:00:00Z"
  },
  "message": "Message sent successfully",
  "timestamp": "2025-08-02T11:00:00Z"
}
```

## 🔧 Development Tools

### OpenAPI/Swagger
Interactive API documentation is available at:
- Development: `http://localhost:3000/docs`
- Production: `https://api.ai4thai.com/docs`

### Postman Collection
Download the Postman collection for easy API testing:
- [AI4Thai API Collection](../tools/postman/ai4thai-api.json)

### cURL Examples
Complete cURL examples for all endpoints:
- [cURL Examples](examples/curl-examples.md)

## 🐛 Error Handling

### Common Error Codes

| Code | Description | Solution |
|------|-------------|----------|
| `INVALID_TOKEN` | JWT token is invalid or expired | Refresh token or re-authenticate |
| `VALIDATION_ERROR` | Request data validation failed | Check request format and required fields |
| `RATE_LIMIT_EXCEEDED` | Too many requests | Wait before making more requests |
| `SERVICE_UNAVAILABLE` | Backend service is down | Try again later or check service status |
| `INSUFFICIENT_PERMISSIONS` | User lacks required permissions | Contact administrator |

### Debugging Tips

1. **Check Authentication**: Ensure valid JWT token in Authorization header
2. **Validate Request Data**: Use OpenAPI schema to validate request format
3. **Monitor Rate Limits**: Check rate limit headers in responses
4. **Check Service Status**: Verify all services are running
5. **Review Logs**: Check application logs for detailed error information

## 📚 Additional Resources

- [Authentication Guide](../development/authentication.md)
- [Error Handling Guide](../development/error-handling.md)
- [Rate Limiting Guide](../development/rate-limiting.md)
- [WebSocket Guide](websocket.md)
- [File Upload Guide](file-upload.md)

---

For API questions or issues, please create an issue in the repository or contact the development team.
