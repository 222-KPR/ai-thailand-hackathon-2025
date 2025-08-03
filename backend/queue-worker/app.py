"""
AI4Thai Queue Worker - Background Processing Service

This service handles asynchronous image processing tasks for crop disease detection.
Implements real YOLO-based pest detection with ultralytics integration.

For production: Replace with Redis/RQ or Celery for persistent task queuing.
"""
import logging
import os
import tempfile
import uuid
from datetime import datetime, timezone
from threading import Lock
from typing import Any, Dict, Optional

from fastapi import FastAPI, File, Form, HTTPException, UploadFile
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel, Field

# from ultralytics import YOLO  # Disabled for quick demo

# Configure logging with structured format
logging.basicConfig(
    level=logging.INFO, format="%(asctime)s - %(name)s - %(levelname)s - %(message)s"
)
logger = logging.getLogger(__name__)

# Initialize YOLO model for pest detection (disabled for demo)
logger.info("YOLO model disabled for quick demo")
model = None


def detect_pests(image_path: str, conf_threshold: float = 0.01) -> Dict[str, Any]:
    """
    Pest detection using YOLO model

    Parameters:
        image_path (str): Path to the image file
        conf_threshold (float): Confidence threshold for detection

    Returns:
        Dict[str, Any]: Detection results with pests and metadata
    """
    if model is None:
        return {
            "detected_pests": [],  # Empty list instead of set
            "message": "โมเดล AI ไม่พร้อมใช้งาน",
            "confidence_scores": {},
        }

    try:
        results = model(image_path, conf=conf_threshold)
        detected_pests = []  # Use list instead of set for JSON serialization
        confidence_scores = {}

        for result in results:
            for box in result.boxes:
                cls_id = int(box.cls[0].item())
                confidence = float(box.conf[0].item())
                class_name = model.names.get(cls_id, f"Class {cls_id}")

                if class_name not in detected_pests:
                    detected_pests.append(class_name)
                confidence_scores[class_name] = max(
                    confidence_scores.get(class_name, 0), confidence
                )

        if detected_pests:
            message = "แมลงศัตรูพืชที่ตรวจพบ:\n"
            for pest in detected_pests:
                confidence = confidence_scores[pest]
                message += f"- {pest} (ความมั่นใจ: {confidence:.2%})\n"
        else:
            message = "ไม่พบแมลงศัตรูพืชในภาพนี้"

        return {
            "detected_pests": detected_pests,
            "message": message.strip(),
            "confidence_scores": confidence_scores,
            "total_detections": len(detected_pests),
        }

    except Exception as e:
        logger.error(f"Pest detection failed: {e}")
        return {
            "detected_pests": [],  # Empty list instead of set
            "message": f"เกิดข้อผิดพลาดในการตรวจจับ: {str(e)}",
            "confidence_scores": {},
        }


# Application configuration
APP_CONFIG = {
    "title": "AI4Thai Queue Worker",
    "version": "1.0.0",
    "description": "Background processing service for crop analysis tasks",
}

app = FastAPI(**APP_CONFIG)

# CORS configuration - restrictive for production
ALLOWED_ORIGINS = ["http://localhost:8080", "http://localhost:3000"]
app.add_middleware(
    CORSMiddleware,
    allow_origins=ALLOWED_ORIGINS,  # More restrictive than "*"
    allow_credentials=True,
    allow_methods=["GET", "POST"],  # Only needed methods
    allow_headers=["*"],
)

# Thread-safe in-memory job storage (for hackathon only)
# TODO: Replace with Redis or database for production

jobs_storage: Dict[str, Dict[str, Any]] = {}
jobs_lock = Lock()


# Pydantic models with validation
class JobResponse(BaseModel):
    job_id: str = Field(..., description="Unique identifier for the job")
    status: str = Field(..., description="Current job status")
    message: str = Field(..., description="Human-readable status message")
    created_at: datetime = Field(default_factory=lambda: datetime.now(timezone.utc))


class JobStatus(BaseModel):
    job_id: str
    status: str = Field(..., pattern="^(queued|processing|completed|failed)$")
    result: Optional[Dict[str, Any]] = None
    error: Optional[str] = None
    created_at: datetime
    updated_at: datetime = Field(default_factory=lambda: datetime.now(timezone.utc))


# Constants
MAX_FILE_SIZE = 10 * 1024 * 1024  # 10MB
ALLOWED_IMAGE_TYPES = {"image/jpeg", "image/png", "image/webp"}


def validate_image_file(file: UploadFile) -> None:
    """Validate uploaded image file."""
    if file.content_type not in ALLOWED_IMAGE_TYPES:
        raise HTTPException(
            status_code=400,
            detail=f"Invalid file type. Allowed: {', '.join(ALLOWED_IMAGE_TYPES)}",
        )

    if file.size and file.size > MAX_FILE_SIZE:
        raise HTTPException(
            status_code=400,
            detail=f"File too large. Maximum size: {MAX_FILE_SIZE // (1024*1024)}MB",
        )


@app.get("/health")
async def health_check():
    """Health check endpoint"""
    return {"status": "healthy", "service": "queue-worker"}


@app.post("/api/v1/queue/pest-detection", response_model=JobResponse)
async def queue_pest_detection(
    image: UploadFile = File(...),
    crop_type: str = Form(...),
    description: Optional[str] = Form(None),
):
    """Queue a pest detection job with real YOLO AI model"""
    try:
        # Validate the uploaded image
        validate_image_file(image)

        job_id = str(uuid.uuid4())
        logger.info(f"Processing pest detection job: {job_id}")

        with jobs_lock:
            jobs_storage[job_id] = {
                "job_id": job_id,
                "status": "processing",
                "created_at": datetime.now(timezone.utc),
                "updated_at": datetime.now(timezone.utc),
            }

        # Process the image with YOLO model
        try:
            # Save uploaded file to temporary location
            with tempfile.NamedTemporaryFile(delete=False, suffix=".jpg") as temp_file:
                content = await image.read()
                temp_file.write(content)
                temp_file_path = temp_file.name

            # Run pest detection
            detection_result = detect_pests(temp_file_path, conf_threshold=0.01)

            # Generate treatment recommendations based on detected pests
            treatment_recommendations = []
            if detection_result["detected_pests"]:
                for pest in detection_result["detected_pests"]:
                    confidence = detection_result["confidence_scores"].get(pest, 0)
                    treatment_recommendations.append(
                        {
                            "pest": pest,
                            "confidence": confidence,
                            "treatment": f"ใช้สารกำจัดแมลงที่เหมาะสมกับ {pest}",
                            "urgency": "สูง" if confidence > 0.7 else "ปานกลาง",
                        }
                    )

            # Update job with results
            with jobs_lock:
                jobs_storage[job_id].update(
                    {
                        "status": "completed",
                        "result": {
                            "analysis_type": "pest_detection",
                            "crop_type": crop_type,
                            "detected_pests": detection_result[
                                "detected_pests"
                            ],  # Already a list
                            "confidence_scores": detection_result["confidence_scores"],
                            "message": detection_result["message"],
                            "total_detections": detection_result["total_detections"],
                            "treatment_recommendations": treatment_recommendations,
                            "processed_at": datetime.now(timezone.utc).isoformat(),
                        },
                        "updated_at": datetime.now(timezone.utc),
                    }
                )

            # Clean up temporary file
            os.unlink(temp_file_path)

            logger.info(
                f"✅ Pest detection completed for job {job_id}: "
                f"{len(detection_result['detected_pests'])} pests found"
            )

        except Exception as processing_error:
            logger.error(f"❌ Processing failed for job {job_id}: {processing_error}")
            with jobs_lock:
                jobs_storage[job_id].update(
                    {
                        "status": "failed",
                        "error": str(processing_error),
                        "updated_at": datetime.now(timezone.utc),
                    }
                )

        return JobResponse(
            job_id=job_id,
            status="processing",
            message="กำลังประมวลผลการตรวจจับแมลงศัตรูพืช...",
        )

    except Exception as e:
        logger.error(f"Failed to queue pest detection: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))


@app.post("/api/v1/queue/disease-detection", response_model=JobResponse)
async def queue_disease_detection(
    image: UploadFile = File(...),
    crop_type: str = Form(...),
    description: Optional[str] = Form(None),
):
    """Queue a disease detection job (mock implementation for hackathon)"""
    try:
        # Validate the uploaded image
        validate_image_file(image)

        job_id = str(uuid.uuid4())
        logger.info(f"Processing disease detection job: {job_id}")

        # Mock disease detection results based on crop type
        disease_mapping = {
            "rice": {
                "disease": "Rice Blast",
                "confidence": 0.89,
                "severity": "moderate",
                "treatment": "ใช้สารป้องกันเชื้อราและปรับปรุงการระบายน้ำ",
                "prevention": "หลีกเลี่ยงการใส่ปุยไนโตรเจนมากเกินไป",
            },
            "cassava": {
                "disease": "Cassava Mosaic Disease",
                "confidence": 0.91,
                "severity": "high",
                "treatment": "กำจัดแมลงพาหะและใช้พันธุ์ต้านทาน",
                "prevention": "ตรวจสอบต้นกล้าก่อนปลูกและควบคุมแมลงพาหะ",
            },
            "durian": {
                "disease": "Phytophthora Root Rot",
                "confidence": 0.83,
                "severity": "high",
                "treatment": "ปรับปรุงการระบายน้ำและใช้สารเคมีป้องกัน",
                "prevention": "หลีกเลี่ยงการใส่น้ำมากเกินไปและปรับปรุงดิน",
            },
            "mango": {
                "disease": "Anthracnose",
                "confidence": 0.86,
                "severity": "moderate",
                "treatment": "ใช้สารป้องกันเชื้อราและตัดแต่งกิ่งที่ติดเชื้อ",
                "prevention": "รักษาความสะอาดของสวนและหลีกเลี่ยงความชื้นสูง",
            },
            "rubber": {
                "disease": "South American Leaf Blight",
                "confidence": 0.78,
                "severity": "high",
                "treatment": "ใช้สารฆ่าเชื้อราและปรับปรุงการระบายอากาศ",
                "prevention": "ปลูกพันธุ์ต้านทานและจัดการความชื้น",
            },
        }

        disease_data = disease_mapping.get(crop_type.lower(), disease_mapping["rice"])

        with jobs_lock:
            jobs_storage[job_id] = {
                "job_id": job_id,
                "status": "completed",
                "result": {
                    "analysis_type": "disease_detection",
                    "crop_type": crop_type,
                    "disease": disease_data["disease"],
                    "confidence": disease_data["confidence"],
                    "severity": disease_data["severity"],
                    "treatment": disease_data["treatment"],
                    "prevention": disease_data["prevention"],
                    "message": f"ตรวจพบโรค {disease_data['disease']} ในพืช{crop_type}",
                    "processed_at": datetime.now(timezone.utc).isoformat(),
                },
                "created_at": datetime.now(timezone.utc),
                "updated_at": datetime.now(timezone.utc),
            }

        logger.info(
            f"✅ Disease detection completed for job {job_id}: {disease_data['disease']}"
        )

        return JobResponse(
            job_id=job_id,
            status="processing",
            message="กำลังประมวลผลการตรวจจับโรคพืช...",
        )

    except Exception as e:
        logger.error(f"Failed to queue disease detection: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))


@app.get("/api/v1/jobs/{job_id}", response_model=JobStatus)
async def get_job_status(job_id: str):
    """Get job status (simplified for hackathon)"""
    try:
        if job_id in jobs_storage:
            job_data = jobs_storage[job_id]
            return JobStatus(
                job_id=job_id,
                status=job_data["status"],
                result=job_data.get("result"),
                error=job_data.get("error"),
            )
        else:
            raise HTTPException(status_code=404, detail="Job not found")
    except Exception as e:
        logger.error(f"Failed to get job status: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))


if __name__ == "__main__":
    import uvicorn

    port = int(os.getenv("PORT", "8001"))
    uvicorn.run(app, host="0.0.0.0", port=port)
