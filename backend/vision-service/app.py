"""
AI4Thai Vision Service - Computer Vision API

This service provides AI-powered crop disease detection capabilities.
Implements mock disease detection suitable for hackathon demonstration.

For production: Integrate with actual ML models (TensorFlow/PyTorch).
"""
import base64
import io
import logging
from datetime import datetime, timezone
from typing import List

import uvicorn
from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from PIL import Image
from pydantic import BaseModel, Field, validator

# Configure logging
logging.basicConfig(
    level=logging.INFO, format="%(asctime)s - %(name)s - %(levelname)s - %(message)s"
)
logger = logging.getLogger(__name__)

# Application configuration
APP_CONFIG = {
    "title": "AI4Thai Vision Service",
    "version": "1.0.0",
    "description": "Computer vision service for crop disease detection",
}

app = FastAPI(**APP_CONFIG)

# CORS configuration
ALLOWED_ORIGINS = ["http://localhost:8080", "http://localhost:3000"]
app.add_middleware(
    CORSMiddleware,
    allow_origins=ALLOWED_ORIGINS,
    allow_credentials=True,
    allow_methods=["GET", "POST"],
    allow_headers=["*"],
)

# Constants
SUPPORTED_CROPS = ["rice", "cassava", "durian", "mango", "rubber"]
MIN_CONFIDENCE = 0.1
MAX_CONFIDENCE = 1.0


# Pydantic models with proper validation
class VisionRequest(BaseModel):
    image_data: str = Field(..., description="Base64 encoded image data")
    crop_type: str = Field(..., description="Type of crop to analyze")
    confidence_threshold: float = Field(
        default=0.5, ge=MIN_CONFIDENCE, le=MAX_CONFIDENCE
    )

    @validator("crop_type")
    def validate_crop_type(cls, v):  # noqa: N805
        if v.lower() not in SUPPORTED_CROPS:
            raise ValueError(
                f"Unsupported crop type. Supported: {', '.join(SUPPORTED_CROPS)}"
            )
        return v.lower()

    @validator("image_data")
    def validate_image_data(cls, v):  # noqa: N805
        try:
            # Validate base64 format
            base64.b64decode(v)
            return v
        except Exception:
            raise ValueError("Invalid base64 image data")


class TreatmentStep(BaseModel):
    step: int
    description: str
    timing: str


class VisionResponse(BaseModel):
    model_config = {"protected_namespaces": ()}

    disease: str
    confidence: float = Field(..., ge=0.0, le=1.0)
    severity: str = Field(..., pattern="^(low|moderate|high|critical)$")
    treatment_steps: List[TreatmentStep]
    model_version: str
    analysis_timestamp: datetime = Field(
        default_factory=lambda: datetime.now(timezone.utc)
    )


class HealthResponse(BaseModel):
    status: str
    service: str
    version: str
    timestamp: datetime = Field(default_factory=lambda: datetime.now(timezone.utc))


@app.get("/health")
async def health_check():
    """Health check endpoint"""
    return {
        "status": "healthy",
        "service": "AI4Thai Vision Service",
    }


@app.post("/analyze", response_model=VisionResponse)
async def analyze_image(request: VisionRequest):
    """Analyze crop image for disease detection"""
    try:
        # Decode base64 image and validate format
        image_data = base64.b64decode(request.image_data)
        _ = Image.open(io.BytesIO(image_data)).convert("RGB")  # Validate image format

        # Mock response for demo - replace with real model
        disease = get_mock_disease(request.crop_type)
        confidence = 0.85
        severity = determine_severity(confidence)
        treatment_steps = get_mock_treatment_steps(disease)

        return VisionResponse(
            disease=disease,
            confidence=confidence,
            severity=severity,
            treatment_steps=treatment_steps,
            model_version="mock-v1",
        )

    except Exception as e:
        logger.error(f"Analysis failed: {e}")
        raise HTTPException(status_code=500, detail=f"Analysis failed: {e!s}")


def get_mock_disease(crop_type: str) -> str:
    """Get mock disease for demo purposes"""
    mock_diseases = {
        "rice": "bacterial_leaf_blight",
        "cassava": "cassava_mosaic_disease",
        "durian": "anthracnose",
        "mango": "anthracnose",
        "rubber": "leaf_fall_disease",
    }
    return mock_diseases.get(crop_type.lower(), "unknown_disease")


def determine_severity(confidence: float) -> str:
    """Determine disease severity based on confidence"""
    if confidence > 0.9:
        return "high"
    elif confidence > 0.7:
        return "moderate"
    elif confidence > 0.5:
        return "low"
    else:
        return "critical"


def get_mock_treatment_steps(disease: str) -> List[TreatmentStep]:
    """Get mock treatment steps for demo purposes"""
    return [
        TreatmentStep(
            step=1,
            description="Remove affected plant parts",
            timing="Immediate",
        ),
        TreatmentStep(
            step=2,
            description="Apply fungicide treatment",
            timing="Within 24 hours",
        ),
        TreatmentStep(
            step=3,
            description="Monitor plant recovery",
            timing="Weekly for 4 weeks",
        ),
    ]


if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=2001)
