"""
AI4Thai Crop Guardian - Vision Service
FastAPI application for agricultural pest detection and disease identification
"""

import asyncio
import logging
import time
from contextlib import asynccontextmanager
from typing import Optional

import uvicorn
from fastapi import FastAPI, File, Form, HTTPException, UploadFile
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse

from services.pest_detection import get_pest_detection_service
from services.disease_detection import get_disease_detection_service
from memory_manager import get_memory_manager

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


@asynccontextmanager
async def lifespan(app: FastAPI):
    """Application lifespan manager for startup and shutdown events."""
    try:
        # Initialize services
        logger.info("Initializing AI4Thai Vision Service...")

        # Initialize pest detection service
        logger.info("Loading pest detection model...")
        pest_service = await get_pest_detection_service()
        await pest_service.initialize_model()

        # Initialize disease detection service
        logger.info("Loading disease detection model...")
        disease_service = await get_disease_detection_service()
        await disease_service.initialize_model()

        # Initialize memory manager and start cleanup
        memory_manager = get_memory_manager()
        memory_manager.optimize_for_inference()

        # Start periodic memory cleanup task
        cleanup_task = asyncio.create_task(memory_manager.periodic_cleanup())

        logger.info("Vision Service initialized successfully")

        yield

        # Cleanup on shutdown
        cleanup_task.cancel()
        memory_manager.full_cleanup()

    except Exception as e:
        logger.error(f"Failed to initialize Vision Service: {e}")
        raise
    finally:
        logger.info("Shutting down Vision Service...")


# Create FastAPI app
app = FastAPI(
    title="AI4Thai Vision Service",
    description="Agricultural pest detection and disease identification service",
    version="1.0.0",
    lifespan=lifespan,
)

# Add CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # Configure appropriately for production
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.get("/health")
async def health_check():
    """Basic health check endpoint."""
    return {
        "status": "healthy",
        "service": "AI4Thai Vision Service - Pest Detection",
        "timestamp": time.time(),
    }


@app.get("/health/detailed")
async def detailed_health_check():
    """Detailed health check with pest detection service status."""
    try:
        pest_service = await get_pest_detection_service()
        health_status = await pest_service.health_check()

        return {
            "status": "healthy",
            "service": "AI4Thai Vision Service - Pest Detection",
            "pest_detection": health_status,
            "timestamp": time.time(),
        }
    except Exception as e:
        logger.error(f"Health check failed: {e}")
        raise HTTPException(status_code=503, detail=f"Service unavailable: {str(e)}")


@app.post("/detect/pests")
async def detect_pests(
    image: UploadFile = File(...),
    confidence_threshold: Optional[float] = Form(0.01),
    return_details: Optional[bool] = Form(False),
):
    """
    Detect pests in agricultural images using YOLO11s model.

    Args:
        image: Uploaded image file
        confidence_threshold: Minimum confidence threshold for pest detection (default: 0.01)
        return_details: Whether to return detailed detection information including bounding boxes

    Returns:
        Pest detection results with identified pests and optional bounding boxes
    """
    start_time = time.time()

    try:
        # Validate image
        if not image.content_type or not image.content_type.startswith("image/"):
            raise HTTPException(status_code=400, detail="Invalid image file type")

        # Read image bytes
        image_bytes = await image.read()
        if len(image_bytes) == 0:
            raise HTTPException(status_code=400, detail="Empty image file")

        # Get pest detection service
        pest_service = await get_pest_detection_service()

        # Run pest detection
        results = await pest_service.detect_pests_from_bytes(
            image_bytes=image_bytes,
            conf_threshold=confidence_threshold,
            return_details=return_details,
        )

        processing_time = time.time() - start_time

        logger.info(
            f"Pest detection completed in {processing_time:.2f}s - Found {results['pest_count']} pests"
        )

        return JSONResponse(
            status_code=200,
            content={
                "success": True,
                "data": results,
                "processing_time_ms": round(processing_time * 1000, 2),
                "timestamp": time.time(),
            },
        )

    except HTTPException:
        raise
    except Exception as e:
        processing_time = time.time() - start_time
        logger.error(f"Pest detection failed: {e}")
        raise HTTPException(status_code=500, detail=f"Pest detection failed: {str(e)}")


@app.post("/analyze")
async def analyze_image(
    image: UploadFile = File(...),
    confidence_threshold: Optional[float] = Form(0.01),
    include_details: Optional[bool] = Form(True),
):
    """
    Analyze agricultural image for pests (alias for /detect/pests).

    Args:
        image: Uploaded image file
        confidence_threshold: Minimum confidence threshold for pest detection
        include_details: Whether to include detailed detection information

    Returns:
        Pest analysis results
    """
    return await detect_pests(image, confidence_threshold, include_details)


@app.post("/detect/disease")
async def detect_disease(
    image: UploadFile = File(...), custom_prompt: Optional[str] = Form(None)
):
    """
    Detect diseases in plant leaf images using LLaVA model.

    Args:
        image: Uploaded image file (preferably leaf images)
        custom_prompt: Custom prompt for analysis (optional)

    Returns:
        Disease detection results with analysis and recommendations
    """
    start_time = time.time()

    try:
        # Validate image
        if not image.content_type or not image.content_type.startswith("image/"):
            raise HTTPException(status_code=400, detail="Invalid image file type")

        # Read image bytes
        image_bytes = await image.read()
        if len(image_bytes) == 0:
            raise HTTPException(status_code=400, detail="Empty image file")

        # Get disease detection service
        disease_service = await get_disease_detection_service()

        # Run disease detection
        results = await disease_service.detect_disease(
            image_bytes=image_bytes, custom_prompt=custom_prompt
        )

        processing_time = time.time() - start_time

        logger.info(
            f"Disease detection completed in {processing_time:.2f}s - Disease: {results['disease_analysis']['disease_name']}"
        )

        return JSONResponse(
            status_code=200,
            content={
                "success": True,
                "data": results,
                "processing_time_ms": round(processing_time * 1000, 2),
                "timestamp": time.time(),
            },
        )

    except HTTPException:
        raise
    except Exception as e:
        processing_time = time.time() - start_time
        logger.error(f"Disease detection failed: {e}")
        raise HTTPException(
            status_code=500, detail=f"Disease detection failed: {str(e)}"
        )


@app.post("/analyze/comprehensive")
async def comprehensive_analysis(
    image: UploadFile = File(...),
    pest_confidence: Optional[float] = Form(0.01),
    pest_details: Optional[bool] = Form(False),
    disease_prompt: Optional[str] = Form(None),
):
    """
    Comprehensive analysis including both pest and disease detection.

    Args:
        image: Uploaded image file
        pest_confidence: Confidence threshold for pest detection
        pest_details: Include pest detection details
        disease_prompt: Custom prompt for disease analysis

    Returns:
        Combined results from pest and disease detection
    """
    start_time = time.time()

    try:
        # Validate image
        if not image.content_type or not image.content_type.startswith("image/"):
            raise HTTPException(status_code=400, detail="Invalid image file type")

        # Read image bytes once
        image_bytes = await image.read()
        if len(image_bytes) == 0:
            raise HTTPException(status_code=400, detail="Empty image file")

        # Get services
        pest_service = await get_pest_detection_service()
        disease_service = await get_disease_detection_service()

        # Run both analyses in parallel
        pest_task = asyncio.create_task(
            pest_service.detect_pests_from_bytes(
                image_bytes=image_bytes,
                conf_threshold=pest_confidence,
                return_details=pest_details,
            )
        )

        disease_task = asyncio.create_task(
            disease_service.detect_disease(
                image_bytes=image_bytes, custom_prompt=disease_prompt
            )
        )

        # Wait for both analyses
        pest_results, disease_results = await asyncio.gather(
            pest_task, disease_task, return_exceptions=True
        )

        # Process results
        response_data = {"pest_analysis": {}, "disease_analysis": {}, "summary": {}}

        # Handle pest results
        if isinstance(pest_results, Exception):
            response_data["pest_analysis"] = {
                "success": False,
                "error": str(pest_results),
            }
        else:
            response_data["pest_analysis"] = {"success": True, "results": pest_results}

        # Handle disease results
        if isinstance(disease_results, Exception):
            response_data["disease_analysis"] = {
                "success": False,
                "error": str(disease_results),
            }
        else:
            response_data["disease_analysis"] = {
                "success": True,
                "results": disease_results,
            }

        # Create comprehensive summary
        issues = []
        thai_summaries = []
        recommendations = []

        # Check pest results
        if response_data["pest_analysis"].get("success") and response_data[
            "pest_analysis"
        ]["results"].get("has_pests"):
            issues.append("pests")
            thai_summaries.append(
                response_data["pest_analysis"]["results"]["thai_summary"]
            )

        # Check disease results
        if response_data["disease_analysis"].get("success") and not response_data[
            "disease_analysis"
        ]["results"]["disease_analysis"].get("is_healthy", True):
            issues.append("disease")
            thai_summaries.append(
                response_data["disease_analysis"]["results"]["disease_analysis"][
                    "thai_summary"
                ]
            )
            recommendations.extend(
                response_data["disease_analysis"]["results"]["disease_analysis"][
                    "recommendations"
                ]
            )

        # Generate summary
        if not issues:
            response_data["summary"] = {
                "status": "healthy",
                "issues": [],
                "thai_summary": "พืชมีสุขภาพดี ไม่พบศัตรูพืชหรือโรคพืช",
                "recommendations": ["ดูแลรักษาตามปกติ", "ตรวจสอบเป็นประจำ"],
            }
        else:
            response_data["summary"] = {
                "status": "issues_detected",
                "issues": issues,
                "thai_summary": " และ ".join(thai_summaries),
                "recommendations": recommendations
                if recommendations
                else ["ปรึกษาผู้เชี่ยวชาญการเกษตร"],
            }

        processing_time = time.time() - start_time

        return JSONResponse(
            status_code=200,
            content={
                "success": True,
                "data": response_data,
                "processing_time_ms": round(processing_time * 1000, 2),
                "timestamp": time.time(),
            },
        )

    except HTTPException:
        raise
    except Exception as e:
        processing_time = time.time() - start_time
        logger.error(f"Comprehensive analysis failed: {e}")
        raise HTTPException(status_code=500, detail=f"Analysis failed: {str(e)}")


@app.get("/health/disease")
async def disease_detection_health():
    """Health check for disease detection service."""
    try:
        disease_service = await get_disease_detection_service()
        health_status = await disease_service.health_check()
        return health_status
    except Exception as e:
        logger.error(f"Disease detection health check failed: {e}")
        raise HTTPException(
            status_code=503, detail=f"Disease detection service unavailable: {str(e)}"
        )


@app.get("/health/memory")
async def memory_status():
    """Get current memory usage statistics."""
    try:
        memory_manager = get_memory_manager()
        stats = memory_manager.get_memory_stats()
        return {"status": "healthy", "memory_stats": stats, "timestamp": time.time()}
    except Exception as e:
        logger.error(f"Memory status check failed: {e}")
        raise HTTPException(
            status_code=503, detail=f"Memory status unavailable: {str(e)}"
        )


@app.post("/maintenance/cleanup")
async def manual_cleanup():
    """Manually trigger memory cleanup."""
    try:
        memory_manager = get_memory_manager()
        memory_manager.full_cleanup()
        stats = memory_manager.get_memory_stats()
        return {
            "status": "completed",
            "memory_stats_after": stats,
            "timestamp": time.time(),
        }
    except Exception as e:
        logger.error(f"Manual cleanup failed: {e}")
        raise HTTPException(status_code=500, detail=f"Cleanup failed: {str(e)}")


@app.get("/info")
async def get_service_info():
    """Get service information and capabilities."""
    try:
        # Get services info
        pest_service = await get_pest_detection_service()
        disease_service = await get_disease_detection_service()

        pest_health = await pest_service.health_check()
        disease_health = await disease_service.health_check()

        return {
            "service": "AI4Thai Vision Service",
            "version": "1.0.0",
            "description": "Agricultural pest detection and disease identification service",
            "models": [
                {
                    "name": "YOLO11s",
                    "source": "underdogquality/yolo11s-pest-detection",
                    "type": "Object Detection",
                    "framework": "Ultralytics",
                    "purpose": "Pest Detection",
                },
                {
                    "name": "LLaVA-v1.5-7B",
                    "source": "YuchengShi/LLaVA-v1.5-7B-Plant-Leaf-Diseases-Detection",
                    "type": "Vision-Language Model",
                    "framework": "Transformers",
                    "purpose": "Disease Detection",
                },
            ],
            "capabilities": [
                "Agricultural pest detection",
                "Plant disease identification",
                "Multi-pest identification",
                "Disease severity assessment",
                "Confidence scoring",
                "Bounding box detection",
                "Thai language summaries",
                "Treatment recommendations",
                "Comprehensive plant health analysis",
            ],
            "endpoints": {
                "pest_detection": "/detect/pests",
                "disease_detection": "/detect/disease",
                "comprehensive": "/analyze/comprehensive",
                "analyze_alias": "/analyze",
                "health": "/health",
                "detailed_health": "/health/detailed",
                "pest_health": "/health/pests",
                "disease_health": "/health/disease",
                "info": "/info",
            },
            "pest_detection": {
                "status": pest_health.get("status", "unknown"),
                "model_loaded": pest_health.get("model_loaded", False),
                "available_classes": pest_health.get("available_classes", 0),
            },
            "disease_detection": {
                "status": disease_health.get("status", "unknown"),
                "model_loaded": disease_health.get("model_loaded", False),
                "device": disease_health.get("device", "unknown"),
            },
            "supported_formats": ["image/jpeg", "image/png", "image/webp", "image/bmp"],
            "default_pest_confidence": 0.01,
            "max_file_size": "10MB",
        }
    except Exception as e:
        logger.error(f"Failed to get service info: {e}")
        return {
            "service": "AI4Thai Vision Service - Pest Detection",
            "version": "1.0.0",
            "status": "error",
            "error": str(e),
        }


@app.get("/")
async def root():
    """Root endpoint with service overview."""
    return {
        "message": "AI4Thai Vision Service",
        "description": "Comprehensive agricultural pest detection and disease identification service",
        "models": {
            "pest_detection": "YOLO11s from underdogquality/yolo11s-pest-detection",
            "disease_detection": "LLaVA-v1.5-7B from YuchengShi/LLaVA-v1.5-7B-Plant-Leaf-Diseases-Detection",
        },
        "endpoints": {
            "pest_detection": "/detect/pests",
            "disease_detection": "/detect/disease",
            "comprehensive_analysis": "/analyze/comprehensive",
            "analyze_alias": "/analyze",
            "health": "/health",
            "info": "/info",
        },
        "usage": {
            "pest_detection": "Upload an image to /detect/pests to identify agricultural pests",
            "disease_detection": "Upload a leaf image to /detect/disease to identify plant diseases",
            "comprehensive": "Upload an image to /analyze/comprehensive for both pest and disease analysis",
        },
        "features": [
            "Agricultural pest detection",
            "Plant disease identification",
            "Thai language summaries",
            "Treatment recommendations",
            "Parallel processing for comprehensive analysis",
        ],
    }


if __name__ == "__main__":
    uvicorn.run("app:app", host="0.0.0.0", port=2001, reload=True)
