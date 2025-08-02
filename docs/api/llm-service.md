# LLM Service API Specification

## Overview

The LLM Service provides natural language processing capabilities for agricultural advisory, chat interface, and Thai language support using HuggingFace transformer models.

**Base URL**: `http://localhost:8002` (development) | `https://llm.ai4thai.com` (production)

## Models

### Agricultural Advisor Model
- **Model**: `microsoft/DialoGPT-large` (fine-tuned)
- **Purpose**: Provide agricultural advice and chat responses
- **Context Length**: 512 tokens
- **Languages**: Thai, English

### Thai Language Model
- **Model**: `airesearch/wangchanberta-base-att-spm-uncased`
- **Purpose**: Thai language understanding and processing
- **Context Length**: 256 tokens
- **Specialization**: Thai agricultural terminology

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
    "chat_engine": "healthy",
    "context_manager": "healthy",
    "response_generator": "healthy",
    "gpu": "available"
  },
  "system": {
    "cpu_usage": 55.8,
    "memory_usage": 72.3,
    "gpu_usage": 45.2,
    "gpu_memory": 8192
  }
}
```

#### GET /health/models
Check model loading status.

**Response (200):**
```json
{
  "models": {
    "agricultural_advisor": {
      "status": "loaded",
      "model_name": "microsoft/DialoGPT-large",
      "load_time": 25.4,
      "memory_usage": 4096,
      "context_length": 512
    },
    "thai_language": {
      "status": "loaded",
      "model_name": "airesearch/wangchanberta-base-att-spm-uncased",
      "load_time": 18.7,
      "memory_usage": 2048,
      "context_length": 256
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
    "agricultural_advisor": {
      "model_name": "microsoft/DialoGPT-large",
      "model_type": "causal_language_model",
      "context_length": 512,
      "temperature": 0.7,
      "top_p": 0.9,
      "top_k": 50,
      "supported_languages": ["th", "en"],
      "specialization": "agricultural_advice"
    },
    "thai_language": {
      "model_name": "airesearch/wangchanberta-base-att-spm-uncased",
      "model_type": "masked_language_model",
      "context_length": 256,
      "supported_languages": ["th"],
      "specialization": "thai_language_understanding"
    }
  }
}
```

### Chat Interface

#### POST /chat
General chat endpoint for agricultural questions.

**Request Body:**
```json
{
  "message": "How do I treat rice blast disease?",
  "language": "th",
  "context": {
    "crop_type": "rice",
    "location": "Chiang Mai",
    "season": "wet",
    "previous_diseases": ["brown_spot"]
  },
  "conversation_id": "conv_123456789",
  "user_id": "usr_123456789"
}
```

**Response (200):**
```json
{
  "response": "การรักษาโรคไหม้ข้าวสามารถทำได้หลายวิธี:\n\n1. วิธีเคมี: ใช้สารเคมีป้องกันกำจัดเชื้อรา เช่น ไตรไซคลาโซล 2 มล.ต่อน้ำ 1 ลิตร พ่นทุก 7-10 วัน\n\n2. วิธีชีวภาพ: ใช้เชื้อราตัวดี เช่น ไตรโคเดอร์มา\n\n3. การจัดการแปลง: ปรับปรุงการระบายน้ำ ลดความชื้น\n\n4. การป้องกัน: ใช้พันธุ์ต้านทาน รักษาระยะห่างที่เหมาะสม",
  "language": "th",
  "confidence": 0.92,
  "conversation_id": "conv_123456789",
  "context": {
    "crop_type": "rice",
    "disease": "rice_blast",
    "treatment_methods": ["chemical", "biological", "cultural"],
    "location": "Chiang Mai",
    "season": "wet"
  },
  "processing_time_ms": 2150.5,
  "model_info": {
    "model_used": "agricultural_advisor",
    "tokens_generated": 156,
    "context_tokens": 89
  },
  "sources": [
    "agricultural_knowledge_base",
    "disease_treatment_database",
    "thai_farming_practices"
  ]
}
```

### Agricultural Advice

#### POST /advice
Get specific agricultural advice for crop diseases and treatments.

**Request Body:**
```json
{
  "crop_type": "rice",
  "disease": "blast",
  "symptoms": [
    "diamond-shaped lesions on leaves",
    "gray centers with brown borders",
    "lesions spreading to stems"
  ],
  "location": "Chiang Mai",
  "season": "wet",
  "language": "th"
}
```

**Response (200):**
```json
{
  "advice": "โรคไหม้ข้าวเป็นโรคที่พบได้บ่อยในฤดูฝน เกิดจากเชื้อรา Pyricularia oryzae การรักษาต้องทำอย่างรวดเร็วเพื่อป้องกันการแพร่กระจาย",
  "treatment_steps": [
    {
      "step": 1,
      "action": "ตรวจสอบความรุนแรงของโรค",
      "description": "สำรวจพื้นที่ที่เป็นโรคและประเมินความเสียหาย",
      "timeframe": "ทันที"
    },
    {
      "step": 2,
      "action": "พ่นสารเคมีป้องกันกำจัดเชื้อรา",
      "description": "ใช้ไตรไซคลาโซล 75% WP อัตรา 20 กรัมต่อน้ำ 20 ลิตร",
      "timeframe": "ภายใน 24 ชั่วโมง",
      "frequency": "ทุก 7-10 วัน",
      "applications": 2-3
    },
    {
      "step": 3,
      "action": "ปรับปรุงการจัดการน้ำ",
      "description": "ระบายน้ำออกจากแปลงเพื่อลดความชื้น",
      "timeframe": "ทันที"
    },
    {
      "step": 4,
      "action": "เพิ่มการระบายอากาศ",
      "description": "ตัดใบที่เป็นโรคออกและเผาทำลาย",
      "timeframe": "หลังพ่นยา"
    }
  ],
  "prevention_tips": [
    "ใช้พันธุ์ข้าวที่ต้านทานโรค เช่น ข้าวหอมมะลิ 105, กข 6",
    "รักษาระยะห่างระหว่างต้นให้เหมาะสม (20x20 ซม.)",
    "หลีกเลี่ยงการใส่ปุ๋ยไนโตรเจนมากเกินไป",
    "จัดการน้ำให้เหมาะสม ไม่ให้ขังน้ำนาน",
    "ทำลายตอซังข้าวหลังเก็บเกี่ยว"
  ],
  "confidence": 0.94,
  "language": "th",
  "estimated_cost": {
    "chemical_treatment": {
      "amount": "500-800 บาทต่อไร่",
      "currency": "THB"
    },
    "labor": {
      "amount": "200-300 บาทต่อไร่",
      "currency": "THB"
    }
  },
  "timeline": {
    "immediate_action": "0-24 ชั่วโมง",
    "treatment_period": "2-3 สัปดาห์",
    "recovery_time": "4-6 สัปดาห์"
  }
}
```

### Translation

#### POST /translate
Translate text between Thai and English.

**Request Body:**
```json
{
  "text": "How do I treat rice blast disease?",
  "source_language": "en",
  "target_language": "th"
}
```

**Response (200):**
```json
{
  "success": true,
  "data": {
    "original_text": "How do I treat rice blast disease?",
    "translated_text": "ฉันจะรักษาโรคไหม้ข้าวได้อย่างไร?",
    "source_language": "en",
    "target_language": "th",
    "confidence": 0.96,
    "translation_method": "neural_machine_translation",
    "model_used": "thai_language"
  },
  "processing_time_ms": 450.2,
  "timestamp": 1691000000.0
}
```

### Text Summarization

#### POST /summarize
Summarize agricultural text content.

**Request Body:**
```json
{
  "text": "โรคไหม้ข้าวเป็นโรคที่เกิดจากเชื้อรา Pyricularia oryzae ซึ่งสามารถทำลายข้าวได้อย่างรุนแรง โดยเฉพาะในช่วงที่มีความชื้นสูง อาการของโรคนี้จะปรากฏเป็นจุดสีน้ำตาลบนใบข้าว และจะขยายตัวเป็นรูปเพชรที่มีขอบสีน้ำตาลและตรงกลางสีเทา การรักษาสามารถทำได้โดยการพ่นสารเคมีป้องกันกำจัดเชื้อรา การปรับปรุงการระบายน้ำ และการใช้พันธุ์ข้าวที่ต้านทานโรค",
  "language": "th",
  "max_length": 100
}
```

**Response (200):**
```json
{
  "success": true,
  "data": {
    "original_text": "โรคไหม้ข้าวเป็นโรคที่เกิดจากเชื้อรา Pyricularia oryzae...",
    "summary": "โรคไหม้ข้าวเกิดจากเชื้อรา Pyricularia oryzae ทำลายข้าวในสภาพความชื้นสูง อาการเป็นจุดสีน้ำตาลรูปเพชรบนใบ รักษาด้วยสารเคมี ปรับปรุงระบายน้ำ และใช้พันธุ์ต้านทาน",
    "language": "th",
    "compression_ratio": 0.45,
    "original_length": 245,
    "summary_length": 110
  },
  "processing_time_ms": 680.3,
  "timestamp": 1691000000.0
}
```

### Conversation Management

#### GET /conversation/{conversation_id}
Get conversation history.

**Response (200):**
```json
{
  "conversation_id": "conv_123456789",
  "history": [
    {
      "message_id": "msg_001",
      "sender": "user",
      "content": "สวัสดีครับ ผมมีปัญหาเรื่องข้าว",
      "timestamp": "2025-08-02T13:00:00Z",
      "language": "th"
    },
    {
      "message_id": "msg_002",
      "sender": "ai",
      "content": "สวัสดีครับ ยินดีที่ได้ช่วยเหลือ คุณมีปัญหาอะไรเกี่ยวกับข้าวครับ?",
      "timestamp": "2025-08-02T13:00:02Z",
      "language": "th",
      "confidence": 0.95
    }
  ],
  "context": {
    "crop_type": "rice",
    "user_location": "Chiang Mai",
    "conversation_topic": "rice_farming"
  },
  "message_count": 2,
  "created_at": "2025-08-02T13:00:00Z",
  "last_updated": "2025-08-02T13:00:02Z"
}
```

#### DELETE /conversation/{conversation_id}
Clear conversation history.

**Response (200):**
```json
{
  "message": "Conversation conv_123456789 cleared successfully",
  "conversation_id": "conv_123456789",
  "messages_deleted": 15,
  "timestamp": 1691000000.0
}
```

### Service Information

#### GET /info
Get service information and capabilities.

**Response (200):**
```json
{
  "service": "AI4Thai LLM Service",
  "version": "1.0.0",
  "models": [
    {
      "name": "agricultural_advisor",
      "model": "microsoft/DialoGPT-large",
      "status": "loaded"
    },
    {
      "name": "thai_language",
      "model": "airesearch/wangchanberta-base-att-spm-uncased",
      "status": "loaded"
    }
  ],
  "supported_languages": ["th", "en"],
  "supported_crops": ["rice", "cassava", "durian", "mango", "rubber"],
  "max_context_length": 512,
  "capabilities": {
    "chat_interface": true,
    "agricultural_advice": true,
    "translation": true,
    "summarization": true,
    "conversation_management": true,
    "thai_language_processing": true
  },
  "hardware": {
    "gpu_available": true,
    "gpu_count": 1,
    "gpu_memory": 16384,
    "cuda_version": "11.8"
  },
  "performance": {
    "average_response_time_ms": 2150,
    "max_concurrent_conversations": 10,
    "conversations_processed": 8750,
    "uptime_seconds": 86400
  }
}
```

### Metrics

#### GET /metrics
Get service metrics (Prometheus format).

**Response (200):**
```
# HELP llm_service_requests_total Total number of requests processed
# TYPE llm_service_requests_total counter
llm_service_requests_total{endpoint="chat",status="success"} 2345
llm_service_requests_total{endpoint="chat",status="error"} 23
llm_service_requests_total{endpoint="advice",status="success"} 1567

# HELP llm_service_response_time_seconds Response generation time
# TYPE llm_service_response_time_seconds histogram
llm_service_response_time_seconds_bucket{endpoint="chat",le="1.0"} 234
llm_service_response_time_seconds_bucket{endpoint="chat",le="2.0"} 1890
llm_service_response_time_seconds_bucket{endpoint="chat",le="5.0"} 2340
llm_service_response_time_seconds_bucket{endpoint="chat",le="+Inf"} 2368

# HELP llm_service_tokens_generated_total Total tokens generated
# TYPE llm_service_tokens_generated_total counter
llm_service_tokens_generated_total{model="agricultural_advisor"} 1234567
llm_service_tokens_generated_total{model="thai_language"} 567890

# HELP llm_service_active_conversations Current active conversations
# TYPE llm_service_active_conversations gauge
llm_service_active_conversations 45

# HELP llm_service_gpu_utilization GPU utilization percentage
# TYPE llm_service_gpu_utilization gauge
llm_service_gpu_utilization 45.2

# HELP llm_service_memory_usage_bytes Memory usage in bytes
# TYPE llm_service_memory_usage_bytes gauge
llm_service_memory_usage_bytes{type="system"} 4294967296
llm_service_memory_usage_bytes{type="gpu"} 6442450944
```

## Error Responses

### Standard Error Format

```json
{
  "success": false,
  "error": {
    "code": "INVALID_LANGUAGE",
    "message": "Unsupported language",
    "details": {
      "supported_languages": ["th", "en"],
      "received_language": "fr"
    }
  },
  "timestamp": 1691000000.0
}
```

### LLM Service Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `INVALID_LANGUAGE` | 400 | Unsupported language code |
| `MESSAGE_TOO_LONG` | 400 | Message exceeds context length |
| `INVALID_CROP_TYPE` | 400 | Unsupported crop type |
| `CONVERSATION_NOT_FOUND` | 404 | Conversation ID not found |
| `MODEL_NOT_LOADED` | 503 | Required model not loaded |
| `GENERATION_TIMEOUT` | 504 | Response generation timeout |
| `CONTEXT_OVERFLOW` | 400 | Context length exceeded |
| `RATE_LIMIT_EXCEEDED` | 429 | Too many requests |

## Input Constraints

### Message Limits
- **Maximum message length**: 1000 characters
- **Maximum context length**: 512 tokens
- **Maximum conversation history**: 50 messages
- **Maximum batch size**: 10 messages

### Language Support
- **Primary**: Thai (th)
- **Secondary**: English (en)
- **Translation**: Bidirectional th ↔ en

### Content Filtering
- **Inappropriate content**: Filtered out
- **Spam detection**: Automatic detection
- **Agricultural focus**: Non-agricultural queries redirected

## Rate Limits

| Endpoint | Limit | Window |
|----------|-------|--------|
| `/chat` | 200 requests | 1 hour |
| `/advice` | 100 requests | 1 hour |
| `/translate` | 500 requests | 1 hour |
| `/summarize` | 50 requests | 1 hour |
| Health checks | 1000 requests | 1 hour |

## Performance Characteristics

### Response Times (95th percentile)
- **Chat Response**: < 3 seconds
- **Agricultural Advice**: < 4 seconds
- **Translation**: < 1 second
- **Summarization**: < 2 seconds

### Throughput
- **Concurrent Conversations**: Up to 10
- **Queue Capacity**: 50 requests
- **Processing Rate**: ~20 responses per minute

### Resource Usage
- **CPU**: 8-16 cores recommended
- **RAM**: 16-32GB recommended
- **GPU**: 16GB VRAM minimum, 24GB recommended
- **Storage**: 100GB for models and conversation history
