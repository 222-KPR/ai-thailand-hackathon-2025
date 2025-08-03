"""
Celery tasks for Vision Service queue processing and image data management
"""

from celery import Celery
import httpx
import structlog
import os
import base64
import uuid
import tempfile
from typing import Dict, Any, Optional
from PIL import Image
import io
import hashlib
import json

# Configure structured logging
logger = structlog.get_logger(__name__)

# Initialize Celery
app = Celery(
    "vision_queue",
    broker=os.getenv("REDIS_URL", "redis://localhost:6379/0"),
    backend=os.getenv("REDIS_URL", "redis://localhost:6379/0"),
)

# Service URLs
VISION_SERVICE_URL = os.getenv("VISION_SERVICE_URL", "http://localhost:2001")

# Image storage configuration
MAX_IMAGE_SIZE = int(os.getenv("MAX_IMAGE_SIZE", "10485760"))  # 10MB
SUPPORTED_FORMATS = ["JPEG", "PNG", "WEBP", "BMP"]


class ImageProcessor:
    """Helper class for image processing and validation"""

    @staticmethod
    def validate_image(image_data: bytes) -> Dict[str, Any]:
        """Validate image data and extract metadata"""
        try:
            # Check size
            if len(image_data) > MAX_IMAGE_SIZE:
                raise ValueError(
                    f"Image size {len(image_data)} exceeds maximum {MAX_IMAGE_SIZE} bytes"
                )

            # Open image to validate format
            image = Image.open(io.BytesIO(image_data))

            if image.format not in SUPPORTED_FORMATS:
                raise ValueError(f"Unsupported image format: {image.format}")

            # Extract metadata
            metadata = {
                "format": image.format,
                "size": image.size,
                "mode": image.mode,
                "file_size": len(image_data),
                "hash": hashlib.md5(image_data).hexdigest(),
            }

            return metadata

        except Exception as e:
            raise ValueError(f"Invalid image data: {str(e)}")

    @staticmethod
    def process_image_for_service(image_data: bytes, target_service: str) -> bytes:
        """Process image for specific service requirements"""
        try:
            image = Image.open(io.BytesIO(image_data))

            # Convert to RGB if needed
            if image.mode != "RGB":
                image = image.convert("RGB")

            # Resize if too large (optional optimization)
            max_dimension = 2048
            if max(image.size) > max_dimension:
                ratio = max_dimension / max(image.size)
                new_size = tuple(int(dim * ratio) for dim in image.size)
                image = image.resize(new_size, Image.Resampling.LANCZOS)

            # Save processed image
            output = io.BytesIO()
            image.save(output, format="JPEG", quality=85, optimize=True)
            return output.getvalue()

        except Exception as e:
            logger.warning(f"Image processing failed, using original: {str(e)}")
            return image_data


@app.task(
    bind=True,
    autoretry_for=(Exception,),
    retry_kwargs={"max_retries": 3, "countdown": 60},
)
def process_pest_detection(
    self,
    image_data: str,
    confidence_threshold: float = 0.01,
    return_details: bool = False,
):
    """Process pest detection job"""
    try:
        task_id = self.request.id
        logger.info(
            "Processing pest detection",
            task_id=task_id,
            confidence=confidence_threshold,
        )

        # Decode base64 image data
        image_bytes = base64.b64decode(image_data)

        # Validate image
        metadata = ImageProcessor.validate_image(image_bytes)
        logger.info("Image validated", task_id=task_id, metadata=metadata)

        # Process image for pest detection
        processed_image = ImageProcessor.process_image_for_service(
            image_bytes, "pest_detection"
        )

        # Prepare multipart data
        files = {"image": ("image.jpg", processed_image, "image/jpeg")}
        data = {
            "confidence_threshold": confidence_threshold,
            "return_details": return_details,
        }

        # Call vision service
        with httpx.Client(timeout=120.0) as client:
            response = client.post(
                f"{VISION_SERVICE_URL}/detect/pests", files=files, data=data
            )

        if response.status_code == 200:
            result = response.json()
            result["metadata"] = metadata
            result["task_id"] = task_id
            logger.info(
                "Pest detection completed",
                task_id=task_id,
                status="success",
                pests_found=result.get("data", {}).get("pest_count", 0),
            )
            return result
        else:
            error_msg = (
                f"Vision service error: {response.status_code} - {response.text}"
            )
            logger.error("Pest detection failed", task_id=task_id, error=error_msg)
            raise Exception(error_msg)

    except Exception as e:
        logger.error(
            "Pest detection task failed", task_id=self.request.id, error=str(e)
        )
        raise self.retry(exc=e)


@app.task(
    bind=True,
    autoretry_for=(Exception,),
    retry_kwargs={"max_retries": 3, "countdown": 60},
)
def process_disease_detection(
    self, image_data: str, custom_prompt: Optional[str] = None
):
    """Process disease detection job"""
    try:
        task_id = self.request.id
        logger.info(
            "Processing disease detection",
            task_id=task_id,
            has_prompt=bool(custom_prompt),
        )

        # Decode base64 image data
        image_bytes = base64.b64decode(image_data)

        # Validate image
        metadata = ImageProcessor.validate_image(image_bytes)
        logger.info("Image validated", task_id=task_id, metadata=metadata)

        # Process image for disease detection
        processed_image = ImageProcessor.process_image_for_service(
            image_bytes, "disease_detection"
        )

        # Prepare multipart data
        files = {"image": ("image.jpg", processed_image, "image/jpeg")}
        data = {}
        if custom_prompt:
            data["custom_prompt"] = custom_prompt

        # Call vision service
        with httpx.Client(timeout=180.0) as client:  # Longer timeout for LLaVA
            response = client.post(
                f"{VISION_SERVICE_URL}/detect/disease", files=files, data=data
            )

        if response.status_code == 200:
            result = response.json()
            result["metadata"] = metadata
            result["task_id"] = task_id
            disease_name = (
                result.get("data", {})
                .get("disease_analysis", {})
                .get("disease_name", "Unknown")
            )
            logger.info(
                "Disease detection completed",
                task_id=task_id,
                status="success",
                disease=disease_name,
            )
            return result
        else:
            error_msg = (
                f"Vision service error: {response.status_code} - {response.text}"
            )
            logger.error("Disease detection failed", task_id=task_id, error=error_msg)
            raise Exception(error_msg)

    except Exception as e:
        logger.error(
            "Disease detection task failed", task_id=self.request.id, error=str(e)
        )
        raise self.retry(exc=e)


@app.task(
    bind=True,
    autoretry_for=(Exception,),
    retry_kwargs={"max_retries": 3, "countdown": 60},
)
def process_comprehensive_analysis(
    self,
    image_data: str,
    pest_confidence: float = 0.01,
    pest_details: bool = False,
    disease_prompt: Optional[str] = None,
):
    """Process comprehensive analysis (pest + disease detection)"""
    try:
        task_id = self.request.id
        logger.info(
            "Processing comprehensive analysis",
            task_id=task_id,
            pest_confidence=pest_confidence,
            has_disease_prompt=bool(disease_prompt),
        )

        # Decode base64 image data
        image_bytes = base64.b64decode(image_data)

        # Validate image
        metadata = ImageProcessor.validate_image(image_bytes)
        logger.info("Image validated", task_id=task_id, metadata=metadata)

        # Process image
        processed_image = ImageProcessor.process_image_for_service(
            image_bytes, "comprehensive"
        )

        # Prepare multipart data
        files = {"image": ("image.jpg", processed_image, "image/jpeg")}
        data = {"pest_confidence": pest_confidence, "pest_details": pest_details}
        if disease_prompt:
            data["disease_prompt"] = disease_prompt

        # Call vision service
        with httpx.Client(timeout=300.0) as client:  # Extended timeout for both models
            response = client.post(
                f"{VISION_SERVICE_URL}/analyze/comprehensive", files=files, data=data
            )

        if response.status_code == 200:
            result = response.json()
            result["metadata"] = metadata
            result["task_id"] = task_id

            # Extract summary info for logging
            analysis_data = result.get("data", {})
            pest_count = (
                analysis_data.get("pest_analysis", {})
                .get("results", {})
                .get("pest_count", 0)
            )
            disease_name = (
                analysis_data.get("disease_analysis", {})
                .get("results", {})
                .get("disease_analysis", {})
                .get("disease_name", "Unknown")
            )

            logger.info(
                "Comprehensive analysis completed",
                task_id=task_id,
                status="success",
                pest_count=pest_count,
                disease=disease_name,
            )
            return result
        else:
            error_msg = (
                f"Vision service error: {response.status_code} - {response.text}"
            )
            logger.error(
                "Comprehensive analysis failed", task_id=task_id, error=error_msg
            )
            raise Exception(error_msg)

    except Exception as e:
        logger.error(
            "Comprehensive analysis task failed", task_id=self.request.id, error=str(e)
        )
        raise self.retry(exc=e)


@app.task(bind=True)
def store_image_data(self, image_data: str, metadata: Dict[str, Any]) -> str:
    """Store image data with metadata and return storage key"""
    try:
        task_id = self.request.id

        # Generate storage key
        storage_key = f"image:{uuid.uuid4().hex}"

        # Store in Redis with TTL (24 hours)
        import redis

        redis_client = redis.from_url(
            os.getenv("REDIS_URL", "redis://localhost:6379/0")
        )

        # Store image data and metadata
        redis_client.setex(
            f"{storage_key}:data",
            86400,  # 24 hours TTL
            image_data,
        )
        redis_client.setex(f"{storage_key}:metadata", 86400, json.dumps(metadata))

        logger.info(
            "Image stored", task_id=task_id, storage_key=storage_key, metadata=metadata
        )
        return storage_key

    except Exception as e:
        logger.error("Image storage failed", task_id=self.request.id, error=str(e))
        raise e


@app.task
def cleanup_old_images():
    """Cleanup old images and job results from Redis"""
    try:
        logger.info("Cleaning up old images and job results")

        import redis

        redis_client = redis.from_url(
            os.getenv("REDIS_URL", "redis://localhost:6379/0")
        )

        # Clean up expired keys (Redis handles this automatically with TTL)
        # This task can be extended to clean up specific patterns or handle manual cleanup

        # Count current stored images
        image_keys = redis_client.keys("image:*:data")
        metadata_keys = redis_client.keys("image:*:metadata")

        logger.info(
            "Cleanup completed",
            stored_images=len(image_keys),
            metadata_entries=len(metadata_keys),
        )

        return {
            "status": "completed",
            "stored_images": len(image_keys),
            "metadata_entries": len(metadata_keys),
        }

    except Exception as e:
        logger.error("Cleanup task failed", error=str(e))
        raise e


@app.task
def health_check():
    """Health check task for queue worker"""
    try:
        # Check Redis connection
        import redis

        redis_client = redis.from_url(
            os.getenv("REDIS_URL", "redis://localhost:6379/0")
        )
        redis_client.ping()

        # Check vision service availability
        with httpx.Client(timeout=10.0) as client:
            response = client.get(f"{VISION_SERVICE_URL}/health")
            vision_healthy = response.status_code == 200

        result = {
            "status": "healthy",
            "redis_connected": True,
            "vision_service_available": vision_healthy,
            "timestamp": str(task_id := uuid.uuid4()),
        }

        logger.info("Health check completed", **result)
        return result

    except Exception as e:
        logger.error("Health check failed", error=str(e))
        return {"status": "unhealthy", "error": str(e), "timestamp": str(uuid.uuid4())}


# Celery beat schedule for periodic tasks
app.conf.beat_schedule = {
    "cleanup-old-images": {
        "task": "tasks.cleanup_old_images",
        "schedule": 3600.0,  # Run every hour
    },
    "health-check": {
        "task": "tasks.health_check",
        "schedule": 300.0,  # Run every 5 minutes
    },
}

app.conf.timezone = "UTC"

# Additional Celery configuration
app.conf.update(
    task_serializer="json",
    accept_content=["json"],
    result_serializer="json",
    result_expires=86400,  # Results expire after 24 hours
    worker_prefetch_multiplier=1,  # Important for CPU-intensive tasks
    task_acks_late=True,
    worker_disable_rate_limits=False,
)
