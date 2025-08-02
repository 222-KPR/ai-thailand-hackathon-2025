"""
Celery tasks for AI services queue processing
"""

from celery import Celery
import requests
import structlog
import os
from typing import Dict, Any

# Configure structured logging
logger = structlog.get_logger(__name__)

# Initialize Celery
app = Celery(
    "ai_services_queue",
    broker=os.getenv("REDIS_URL", "redis://localhost:6379/0"),
    backend=os.getenv("REDIS_URL", "redis://localhost:6379/0")
)

# Service URLs
VISION_SERVICE_URL = os.getenv("VISION_SERVICE_URL", "http://localhost:8001")
LLM_SERVICE_URL = os.getenv("LLM_SERVICE_URL", "http://localhost:8002")

@app.task(bind=True, autoretry_for=(Exception,), retry_kwargs={'max_retries': 3, 'countdown': 60})
def process_vision_analysis(self, payload: Dict[str, Any], retry_count: int = 3):
    """Process vision analysis job"""
    try:
        logger.info("Processing vision analysis", task_id=self.request.id, payload_keys=list(payload.keys()))
        
        # Call vision service
        response = requests.post(
            f"{VISION_SERVICE_URL}/analyze",
            json=payload,
            timeout=120
        )
        
        if response.status_code == 200:
            result = response.json()
            logger.info("Vision analysis completed", task_id=self.request.id, status="success")
            return result
        else:
            error_msg = f"Vision service error: {response.status_code} - {response.text}"
            logger.error("Vision analysis failed", task_id=self.request.id, error=error_msg)
            raise Exception(error_msg)
            
    except Exception as e:
        logger.error("Vision analysis task failed", task_id=self.request.id, error=str(e))
        raise self.retry(exc=e)

@app.task(bind=True, autoretry_for=(Exception,), retry_kwargs={'max_retries': 3, 'countdown': 60})
def process_llm_generation(self, payload: Dict[str, Any], retry_count: int = 3):
    """Process LLM generation job"""
    try:
        logger.info("Processing LLM generation", task_id=self.request.id, payload_keys=list(payload.keys()))
        
        # Call LLM service
        response = requests.post(
            f"{LLM_SERVICE_URL}/generate",
            json=payload,
            timeout=180
        )
        
        if response.status_code == 200:
            result = response.json()
            logger.info("LLM generation completed", task_id=self.request.id, status="success")
            return result
        else:
            error_msg = f"LLM service error: {response.status_code} - {response.text}"
            logger.error("LLM generation failed", task_id=self.request.id, error=error_msg)
            raise Exception(error_msg)
            
    except Exception as e:
        logger.error("LLM generation task failed", task_id=self.request.id, error=str(e))
        raise self.retry(exc=e)

@app.task
def cleanup_old_results():
    """Cleanup old job results from Redis"""
    try:
        # This task can be run periodically to clean up old results
        logger.info("Cleaning up old job results")
        # Implementation would depend on your Redis key structure
        # For now, this is a placeholder
        return {"status": "completed", "cleaned_items": 0}
        
    except Exception as e:
        logger.error("Cleanup task failed", error=str(e))
        raise e

# Celery beat schedule for periodic tasks
app.conf.beat_schedule = {
    'cleanup-old-results': {
        'task': 'tasks.cleanup_old_results',
        'schedule': 3600.0,  # Run every hour
    },
}

app.conf.timezone = 'UTC'