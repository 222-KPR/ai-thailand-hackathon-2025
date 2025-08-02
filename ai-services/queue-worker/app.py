"""
AI Services Queue Worker
Background job processing for Vision and LLM services
"""

from fastapi import FastAPI, BackgroundTasks
from celery import Celery
import redis
import structlog
from typing import Dict, Any
import os
from pydantic import BaseModel

# Configure structured logging
logger = structlog.get_logger(__name__)

# Initialize FastAPI app
app = FastAPI(
    title="AI4Thai Queue Worker",
    description="Background job processing for AI services",
    version="1.0.0"
)

# Initialize Celery
celery_app = Celery(
    "ai_services_queue",
    broker=os.getenv("REDIS_URL", "redis://localhost:6379/0"),
    backend=os.getenv("REDIS_URL", "redis://localhost:6379/0"),
    include=["app.tasks"]
)

# Initialize Redis client
redis_client = redis.from_url(
    os.getenv("REDIS_URL", "redis://localhost:6379/0"),
    decode_responses=True
)

class JobRequest(BaseModel):
    """Job request model"""
    job_type: str
    payload: Dict[str, Any]
    priority: int = 1
    retry_count: int = 3

class JobStatus(BaseModel):
    """Job status model"""
    job_id: str
    status: str
    result: Dict[str, Any] = None
    error: str = None

@app.get("/health")
async def health_check():
    """Health check endpoint"""
    try:
        # Check Redis connection
        redis_client.ping()
        return {"status": "healthy", "services": {"redis": "connected"}}
    except Exception as e:
        logger.error("Health check failed", error=str(e))
        return {"status": "unhealthy", "error": str(e)}

@app.post("/jobs", response_model=Dict[str, str])
async def create_job(job_request: JobRequest, background_tasks: BackgroundTasks):
    """Create a new background job"""
    try:
        # Dispatch job based on type
        if job_request.job_type == "vision_analysis":
            task = celery_app.send_task(
                "tasks.process_vision_analysis",
                args=[job_request.payload],
                kwargs={"retry_count": job_request.retry_count}
            )
        elif job_request.job_type == "llm_generation":
            task = celery_app.send_task(
                "tasks.process_llm_generation", 
                args=[job_request.payload],
                kwargs={"retry_count": job_request.retry_count}
            )
        else:
            return {"error": f"Unknown job type: {job_request.job_type}"}
        
        logger.info("Job created", job_id=task.id, job_type=job_request.job_type)
        return {"job_id": task.id, "status": "queued"}
        
    except Exception as e:
        logger.error("Failed to create job", error=str(e))
        return {"error": str(e)}

@app.get("/jobs/{job_id}", response_model=JobStatus)
async def get_job_status(job_id: str):
    """Get job status and result"""
    try:
        task = celery_app.AsyncResult(job_id)
        
        if task.state == "PENDING":
            return JobStatus(job_id=job_id, status="pending")
        elif task.state == "SUCCESS":
            return JobStatus(job_id=job_id, status="completed", result=task.result)
        elif task.state == "FAILURE":
            return JobStatus(job_id=job_id, status="failed", error=str(task.info))
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
        return {"error": str(e)}

@app.get("/stats")
async def get_queue_stats():
    """Get queue statistics"""
    try:
        inspect = celery_app.control.inspect()
        active = inspect.active()
        scheduled = inspect.scheduled()
        reserved = inspect.reserved()
        
        return {
            "active_jobs": len(active.get('celery@worker', [])) if active else 0,
            "scheduled_jobs": len(scheduled.get('celery@worker', [])) if scheduled else 0,
            "reserved_jobs": len(reserved.get('celery@worker', [])) if reserved else 0
        }
        
    except Exception as e:
        logger.error("Failed to get queue stats", error=str(e))
        return {"error": str(e)}

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(
        "app:app",
        host="0.0.0.0",
        port=int(os.getenv("QUEUE_PORT", "8003")),
        reload=True
    )