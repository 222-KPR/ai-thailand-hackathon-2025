# AI4Thai Vision Service

Comprehensive agricultural pest detection and disease identification service for Thai farmers.

## Features

- **Pest Detection**: Identifies agricultural pests using YOLO11s model
- **Disease Detection**: Identifies plant diseases using LLaVA vision-language model
- **Comprehensive Analysis**: Combined pest and disease detection in single call
- **Thai Language**: Results in Thai for local farmers
- **AI-Powered**: State-of-the-art models for accurate identification
- **Treatment Recommendations**: Actionable advice for farmers
- **REST API**: Simple HTTP endpoints for integration
- **Async Processing**: Non-blocking parallel analysis

## Quick Start

### 1. Install Dependencies
```bash
pip install -r requirements.txt
```

### 2. Run Service
```bash
python app.py
```

### 3. Test Service
```bash
python test_pest_detection.py
```

## API Endpoints

### POST /detect/pests
Detect pests in agricultural images.

**Parameters:**
- `image`: Image file (JPEG, PNG, WebP, BMP)
- `confidence_threshold`: Detection confidence (default: 0.01)
- `return_details`: Include bounding boxes (default: false)

**Example:**
```bash
curl -X POST http://localhost:8001/detect/pests \
  -F "image=@crop_image.jpg" \
  -F "confidence_threshold=0.01" \
  -F "return_details=true"
```

### POST /detect/disease
Detect diseases in plant leaf images.

**Parameters:**
- `image`: Leaf image file (JPEG, PNG, WebP, BMP)
- `custom_prompt`: Custom analysis prompt (optional)

**Example:**
```bash
curl -X POST http://localhost:8001/detect/disease \
  -F "image=@leaf_image.jpg" \
  -F "custom_prompt=Analyze this leaf for diseases and provide treatment advice"
```

### POST /analyze/comprehensive
Combined pest and disease analysis.

**Parameters:**
- `image`: Image file
- `pest_confidence`: Pest detection confidence (default: 0.01)
- `pest_details`: Include pest bounding boxes (default: false)
- `disease_prompt`: Custom disease analysis prompt (optional)

**Example:**
```bash
curl -X POST http://localhost:8001/analyze/comprehensive \
  -F "image=@plant_image.jpg" \
  -F "pest_confidence=0.01" \
  -F "pest_details=true"
```

**Response Example:**
```json
{
  "success": true,
  "data": {
    "pest_analysis": {
      "success": true,
      "results": {
        "detected_pests": ["aphid"],
        "pest_count": 1,
        "has_pests": true,
        "thai_summary": "แมลงศัตรูพืชที่ตรวจพบ: aphid"
      }
    },
    "disease_analysis": {
      "success": true,
      "results": {
        "disease_analysis": {
          "disease_name": "Leaf Spot",
          "severity": "Medium",
          "thai_summary": "พบอาการของโรคพืช: Leaf Spot ระดับความรุนแรง: ปานกลาง",
          "recommendations": ["แยกต้นพืชที่เป็นโรคออกจากต้นอื่น", "ปรึกษาผู้เชี่ยวชาญด้านการเกษตร"]
        }
      }
    },
    "summary": {
      "status": "issues_detected",
      "issues": ["pests", "disease"],
      "thai_summary": "แมลงศัตรูพืชที่ตรวจพบ: aphid และ พบอาการของโรคพืช: Leaf Spot ระดับความรุนแรง: ปานกลาง"
    }
  },
  "processing_time_ms": 2340.15
}
```

### GET /health
Service health check.

### GET /info
Service information and capabilities.

## Models

### Pest Detection
- **Model**: YOLO11s
- **Source**: underdogquality/yolo11s-pest-detection
- **Framework**: Ultralytics
- **Type**: Object Detection

### Disease Detection
- **Model**: LLaVA-v1.5-7B
- **Source**: YuchengShi/LLaVA-v1.5-7B-Plant-Leaf-Diseases-Detection
- **Framework**: Transformers
- **Type**: Vision-Language Model

## Docker

```bash
docker build -t ai4thai-vision .
docker run -p 8001:8001 ai4thai-vision
```

## Development

### Project Structure
```
vision-service/
├── app.py                 # FastAPI application
├── requirements.txt       # Dependencies
├── services/
│   └── pest_detection.py  # YOLO pest detection service
└── test_pest_detection.py # Test suite
```

### Testing
```bash
# Test locally
python test_pest_detection.py

# Test with actual image
curl -X POST http://localhost:8001/detect/pests \
  -F "image=@test_image.jpg"
```

## License

Part of AI4Thai Crop Guardian project.