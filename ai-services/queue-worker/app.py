"""
AI4Thai Vision Queue Worker
Background job processing and image data management for Vision Service
"""

from fastapi import FastAPI, HTTPException, UploadFile, File, Form
from fastapi.middleware.cors import CORSMiddleware
from celery import Celery
import redis
import structlog
from typing import Dict, Any, Optional
import os
import base64
import uuid
from pydantic import BaseModel

# Configure structured logging
logger = structlog.get_logger(__name__)

# Initialize FastAPI app
app = FastAPI(
    title="AI4Thai Vision Queue Worker",
    description="Background job processing and image data management for Vision Service",
    version="1.0.0",
)

# Add CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # Configure appropriately for production
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Initialize Celery
celery_app = Celery(
    "vision_queue",
    broker=os.getenv("REDIS_URL", "redis://localhost:6379/0"),
    backend=os.getenv("REDIS_URL", "redis://localhost:6379/0"),
    include=["tasks"],
)

# Initialize Redis client
redis_client = redis.from_url(
    os.getenv("REDIS_URL", "redis://localhost:6379/0"), decode_responses=True
)


# Pydantic models
class JobRequest(BaseModel):
    """Job request model for vision analysis"""

    job_type: str  # 'pest_detection', 'disease_detection', 'comprehensive_analysis'
    image_data: str  # base64 encoded image
    parameters: Dict[str, Any] = {}


class JobStatus(BaseModel):
    """Job status model"""

    job_id: str
    status: str
    result: Optional[Dict[str, Any]] = None
    error: Optional[str] = None
    progress: Optional[Dict[str, Any]] = None


class ImageUploadRequest(BaseModel):
    """Image upload request"""

    confidence_threshold: Optional[float] = 0.01
    return_details: Optional[bool] = False
    custom_prompt: Optional[str] = None


@app.get("/health")
async def health_check():
    """Health check endpoint"""
    try:
        # Check Redis connection
        redis_client.ping()

        # Check Celery worker status
        inspect = celery_app.control.inspect()
        active_workers = inspect.active()
        worker_count = len(active_workers) if active_workers else 0

        return {
            "status": "healthy",
            "services": {"redis": "connected", "celery_workers": worker_count},
            "timestamp": str(uuid.uuid4()),
        }
    except Exception as e:
        logger.error("Health check failed", error=str(e))
        return {"status": "unhealthy", "error": str(e)}


@app.post("/analyze/pest", response_model=Dict[str, str])
async def queue_pest_detection(
    image: UploadFile = File(...),
    confidence_threshold: float = Form(0.01),
    return_details: bool = Form(False),
):
    """Queue pest detection job"""
    try:
        # Read and encode image
        image_data = await image.read()
        image_b64 = base64.b64encode(image_data).decode("utf-8")

        # Create Celery task
        task = celery_app.send_task(
            "tasks.process_pest_detection",
            args=[image_b64, confidence_threshold, return_details],
        )

        logger.info(
            "Pest detection job queued", job_id=task.id, image_size=len(image_data)
        )
        return {"job_id": task.id, "status": "queued", "type": "pest_detection"}

    except Exception as e:
        logger.error("Failed to queue pest detection", error=str(e))
        raise HTTPException(status_code=500, detail=str(e))


@app.post("/analyze/disease", response_model=Dict[str, str])
async def queue_disease_detection(
    image: UploadFile = File(...), custom_prompt: Optional[str] = Form(None)
):
    """Queue disease detection job"""
    try:
        # Read and encode image
        image_data = await image.read()
        image_b64 = base64.b64encode(image_data).decode("utf-8")

        # Create Celery task
        task = celery_app.send_task(
            "tasks.process_disease_detection", args=[image_b64, custom_prompt]
        )

        logger.info(
            "Disease detection job queued", job_id=task.id, image_size=len(image_data)
        )
        return {"job_id": task.id, "status": "queued", "type": "disease_detection"}

    except Exception as e:
        logger.error("Failed to queue disease detection", error=str(e))
        raise HTTPException(status_code=500, detail=str(e))


@app.post("/analyze/comprehensive", response_model=Dict[str, str])
async def queue_comprehensive_analysis(
    image: UploadFile = File(...),
    pest_confidence: float = Form(0.01),
    pest_details: bool = Form(False),
    disease_prompt: Optional[str] = Form(None),
):
    """Queue comprehensive analysis (pest + disease detection)"""
    try:
        # Read and encode image
        image_data = await image.read()
        image_b64 = base64.b64encode(image_data).decode("utf-8")

        # Create Celery task
        task = celery_app.send_task(
            "tasks.process_comprehensive_analysis",
            args=[image_b64, pest_confidence, pest_details, disease_prompt],
        )

        logger.info(
            "Comprehensive analysis job queued",
            job_id=task.id,
            image_size=len(image_data),
        )
        return {"job_id": task.id, "status": "queued", "type": "comprehensive_analysis"}

    except Exception as e:
        logger.error("Failed to queue comprehensive analysis", error=str(e))
        raise HTTPException(status_code=500, detail=str(e))


@app.post("/images/store", response_model=Dict[str, str])
async def store_image(
    image: UploadFile = File(...), metadata: Optional[str] = Form(None)
):
    """Store image data for later processing"""
    try:
        # Read and encode image
        image_data = await image.read()
        image_b64 = base64.b64encode(image_data).decode("utf-8")

        # Parse metadata if provided
        import json

        meta_dict = json.loads(metadata) if metadata else {}

        # Store image
        task = celery_app.send_task(
            "tasks.store_image_data", args=[image_b64, meta_dict]
        )

        # Get storage key
        storage_key = task.get()

        logger.info("Image stored", storage_key=storage_key, image_size=len(image_data))
        return {"storage_key": storage_key, "status": "stored"}

    except Exception as e:
        logger.error("Failed to store image", error=str(e))
        raise HTTPException(status_code=500, detail=str(e))


@app.get("/jobs/{job_id}", response_model=JobStatus)
async def get_job_status(job_id: str):
    """Get job status and result"""
    try:
        task = celery_app.AsyncResult(job_id)

        # Prepare response based on task state
        if task.state == "PENDING":
            return JobStatus(job_id=job_id, status="pending")
        elif task.state == "SUCCESS":
            return JobStatus(job_id=job_id, status="completed", result=task.result)
        elif task.state == "FAILURE":
            error_info = str(task.info) if task.info else "Unknown error"
            return JobStatus(job_id=job_id, status="failed", error=error_info)
        elif task.state == "RETRY":
            return JobStatus(
                job_id=job_id,
                status="retrying",
                progress={
                    "retry_count": getattr(task.info, "retries", 0) if task.info else 0
                },
            )
        else:
            return JobStatus(job_id=job_id, status=task.state.lower())

    except Exception as e:
        logger.error("Failed to get job status", job_id=job_id, error=str(e))
        return JobStatus(job_id=job_id, status="error", error=str(e))


@app.delete("/jobs/{job_id}")
async def cancel_job(job_id: str):
    """Cancel a pending job"""
    try:
        celery_app.control.revoke(job_id, terminate=True)
        logger.info("Job cancelled", job_id=job_id)
        return {"job_id": job_id, "status": "cancelled"}

    except Exception as e:
        logger.error("Failed to cancel job", job_id=job_id, error=str(e))
        raise HTTPException(status_code=500, detail=str(e))


@app.get("/queue/stats")
async def get_queue_stats():
    """Get queue statistics"""
    try:
        inspect = celery_app.control.inspect()
        active = inspect.active()
        scheduled = inspect.scheduled()
        reserved = inspect.reserved()

        # Count jobs by worker
        active_count = sum(len(jobs) for jobs in active.values()) if active else 0
        scheduled_count = (
            sum(len(jobs) for jobs in scheduled.values()) if scheduled else 0
        )
        reserved_count = sum(len(jobs) for jobs in reserved.values()) if reserved else 0

        # Get Redis stats
        redis_info = redis_client.info()

        return {
            "queue_stats": {
                "active_jobs": active_count,
                "scheduled_jobs": scheduled_count,
                "reserved_jobs": reserved_count,
                "total_pending": active_count + scheduled_count + reserved_count,
            },
            "redis_stats": {
                "connected_clients": redis_info.get("connected_clients", 0),
                "used_memory_human": redis_info.get("used_memory_human", "0B"),
                "total_commands_processed": redis_info.get(
                    "total_commands_processed", 0
                ),
            },
            "workers": list(active.keys()) if active else [],
        }

    except Exception as e:
        logger.error("Failed to get queue stats", error=str(e))
        raise HTTPException(status_code=500, detail=str(e))


@app.get("/images/stats")
async def get_image_stats():
    """Get image storage statistics"""
    try:
        # Count stored images
        image_keys = redis_client.keys("image:*:data")
        metadata_keys = redis_client.keys("image:*:metadata")

        # Get memory usage for images
        total_memory = 0
        for key in image_keys[:100]:  # Sample first 100 to avoid performance issues
            try:
                memory = redis_client.memory_usage(key)
                if memory:
                    total_memory += memory
            except:
                continue

        return {
            "stored_images": len(image_keys),
            "metadata_entries": len(metadata_keys),
            "estimated_memory_bytes": total_memory,
            "estimated_memory_mb": round(total_memory / (1024 * 1024), 2),
        }

    except Exception as e:
        logger.error("Failed to get image stats", error=str(e))
        raise HTTPException(status_code=500, detail=str(e))


@app.post("/maintenance/cleanup")
async def trigger_cleanup():
    """Trigger cleanup of old images and job results"""
    try:
        task = celery_app.send_task("tasks.cleanup_old_images")
        result = task.get(timeout=30)

        logger.info("Cleanup triggered", result=result)
        return {"status": "completed", "result": result}

    except Exception as e:
        logger.error("Failed to trigger cleanup", error=str(e))
        raise HTTPException(status_code=500, detail=str(e))


@app.get("/info")
async def get_service_info():
    """Get service information"""
    return {
        "service": "AI4Thai Vision Queue Worker",
        "version": "1.0.0",
        "description": "Background job processing and image data management for Vision Service",
        "capabilities": [
            "Async pest detection processing",
            "Async disease detection processing",
            "Comprehensive analysis queuing",
            "Image data storage and management",
            "Job status tracking",
            "Queue statistics",
            "Automatic cleanup",
        ],
        "endpoints": {
            "queue_pest": "/analyze/pest",
            "queue_disease": "/analyze/disease",
            "queue_comprehensive": "/analyze/comprehensive",
            "store_image": "/images/store",
            "job_status": "/jobs/{job_id}",
            "queue_stats": "/queue/stats",
            "image_stats": "/images/stats",
        },
        "supported_job_types": [
            "pest_detection",
            "disease_detection",
            "comprehensive_analysis",
        ],
        "redis_url": os.getenv("REDIS_URL", "redis://localhost:6379/0").split("@")[
            -1
        ],  # Hide credentials
        "vision_service_url": os.getenv("VISION_SERVICE_URL", "http://localhost:8001"),
    }


@app.get("/")
async def root():
    """Root endpoint with service overview"""
    return {
        "message": "AI4Thai Vision Queue Worker",
        "description": "Background processing for agricultural vision analysis",
        "features": [
            "Asynchronous image processing",
            "Vision service job queuing",
            "Image data management",
            "Job tracking and monitoring",
        ],
        "usage": {
            "queue_job": "POST /analyze/{type} with image file",
            "check_status": "GET /jobs/{job_id}",
            "get_stats": "GET /queue/stats",
        },
    }


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(
        "app:app",
        host="0.0.0.0",
        port=int(os.getenv("QUEUE_PORT", "2003")),
        reload=True,
    )
