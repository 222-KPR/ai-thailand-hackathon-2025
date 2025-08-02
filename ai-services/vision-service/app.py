"""
AI4Thai Crop Guardian - Vision Service
FastAPI application for crop disease detection using HuggingFace models
"""

import asyncio
import logging
import time
from contextlib import asynccontextmanager
from typing import Dict, List, Optional

import torch
import uvicorn
from fastapi import FastAPI, File, Form, HTTPException, UploadFile, Depends
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse
from PIL import Image
import io

from models.model_registry import ModelRegistry
from services.image_processor import ImageProcessor
from services.inference_engine import InferenceEngine
from services.result_formatter import ResultFormatter
from utils.validation import validate_image, validate_crop_type
from config.settings import get_settings
from shared.monitoring.health_check import HealthChecker
from shared.monitoring.metrics import MetricsCollector

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

# Global variables for models and services
model_registry: Optional[ModelRegistry] = None
image_processor: Optional[ImageProcessor] = None
inference_engine: Optional[InferenceEngine] = None
result_formatter: Optional[ResultFormatter] = None
health_checker: Optional[HealthChecker] = None
metrics_collector: Optional[MetricsCollector] = None

@asynccontextmanager
async def lifespan(app: FastAPI):
    """Application lifespan manager for startup and shutdown events."""
    global model_registry, image_processor, inference_engine, result_formatter
    global health_checker, metrics_collector
    
    settings = get_settings()
    
    try:
        # Initialize services
        logger.info("Initializing Vision Service...")
        
        # Initialize model registry and load models
        model_registry = ModelRegistry(settings.model_config)
        await model_registry.load_models()
        
        # Initialize processors
        image_processor = ImageProcessor(settings.image_config)
        inference_engine = InferenceEngine(model_registry, settings.inference_config)
        result_formatter = ResultFormatter(settings.output_config)
        
        # Initialize monitoring
        health_checker = HealthChecker()
        metrics_collector = MetricsCollector("vision_service")
        
        logger.info("Vision Service initialized successfully")
        
        yield
        
    except Exception as e:
        logger.error(f"Failed to initialize Vision Service: {e}")
        raise
    finally:
        # Cleanup
        logger.info("Shutting down Vision Service...")
        if model_registry:
            await model_registry.cleanup()

# Create FastAPI app
app = FastAPI(
    title="AI4Thai Vision Service",
    description="Crop disease detection and plant classification service using HuggingFace models",
    version="1.0.0",
    lifespan=lifespan
)

# Add CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # Configure appropriately for production
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Dependency to get services
def get_services():
    """Dependency to get initialized services."""
    if not all([model_registry, image_processor, inference_engine, result_formatter]):
        raise HTTPException(status_code=503, detail="Service not ready")
    return {
        "model_registry": model_registry,
        "image_processor": image_processor,
        "inference_engine": inference_engine,
        "result_formatter": result_formatter,
        "metrics_collector": metrics_collector
    }

@app.get("/health")
async def health_check():
    """Basic health check endpoint."""
    return {"status": "healthy", "timestamp": time.time()}

@app.get("/health/detailed")
async def detailed_health_check():
    """Detailed health check with service status."""
    if not health_checker:
        raise HTTPException(status_code=503, detail="Health checker not initialized")
    
    health_status = await health_checker.check_all()
    return health_status

@app.get("/health/models")
async def models_health_check():
    """Check model loading status."""
    if not model_registry:
        raise HTTPException(status_code=503, detail="Model registry not initialized")
    
    model_status = await model_registry.get_model_status()
    return {"models": model_status}

@app.get("/models")
async def list_models():
    """List available models and their configurations."""
    if not model_registry:
        raise HTTPException(status_code=503, detail="Model registry not initialized")
    
    models = await model_registry.list_models()
    return {"models": models}

@app.post("/detect/disease")
async def detect_disease(
    image: UploadFile = File(...),
    crop_type: str = Form(...),
    confidence_threshold: Optional[float] = Form(0.7),
    services: Dict = Depends(get_services)
):
    """
    Detect diseases in crop images.
    
    Args:
        image: Uploaded image file
        crop_type: Type of crop (rice, cassava, durian, mango, rubber)
        confidence_threshold: Minimum confidence threshold for predictions
        
    Returns:
        Disease detection results with confidence scores and treatment recommendations
    """
    start_time = time.time()
    
    try:
        # Validate inputs
        validate_crop_type(crop_type)
        image_data = await validate_image(image)
        
        # Process image
        processed_image = await services["image_processor"].process_image(
            image_data, crop_type
        )
        
        # Run inference
        predictions = await services["inference_engine"].predict_disease(
            processed_image, crop_type, confidence_threshold
        )
        
        # Format results
        results = await services["result_formatter"].format_disease_results(
            predictions, crop_type, confidence_threshold
        )
        
        # Record metrics
        processing_time = time.time() - start_time
        await services["metrics_collector"].record_request(
            endpoint="detect_disease",
            processing_time=processing_time,
            success=True
        )
        
        return JSONResponse(
            status_code=200,
            content={
                "success": True,
                "data": results,
                "processing_time_ms": round(processing_time * 1000, 2),
                "timestamp": time.time()
            }
        )
        
    except Exception as e:
        processing_time = time.time() - start_time
        await services["metrics_collector"].record_request(
            endpoint="detect_disease",
            processing_time=processing_time,
            success=False,
            error=str(e)
        )
        
        logger.error(f"Disease detection failed: {e}")
        raise HTTPException(status_code=500, detail=f"Disease detection failed: {str(e)}")

@app.post("/classify/crop")
async def classify_crop(
    image: UploadFile = File(...),
    confidence_threshold: Optional[float] = Form(0.8),
    services: Dict = Depends(get_services)
):
    """
    Classify crop type from images.
    
    Args:
        image: Uploaded image file
        confidence_threshold: Minimum confidence threshold for predictions
        
    Returns:
        Crop classification results with confidence scores
    """
    start_time = time.time()
    
    try:
        # Validate inputs
        image_data = await validate_image(image)
        
        # Process image
        processed_image = await services["image_processor"].process_image(
            image_data, "unknown"
        )
        
        # Run inference
        predictions = await services["inference_engine"].predict_crop(
            processed_image, confidence_threshold
        )
        
        # Format results
        results = await services["result_formatter"].format_crop_results(
            predictions, confidence_threshold
        )
        
        # Record metrics
        processing_time = time.time() - start_time
        await services["metrics_collector"].record_request(
            endpoint="classify_crop",
            processing_time=processing_time,
            success=True
        )
        
        return JSONResponse(
            status_code=200,
            content={
                "success": True,
                "data": results,
                "processing_time_ms": round(processing_time * 1000, 2),
                "timestamp": time.time()
            }
        )
        
    except Exception as e:
        processing_time = time.time() - start_time
        await services["metrics_collector"].record_request(
            endpoint="classify_crop",
            processing_time=processing_time,
            success=False,
            error=str(e)
        )
        
        logger.error(f"Crop classification failed: {e}")
        raise HTTPException(status_code=500, detail=f"Crop classification failed: {str(e)}")

@app.post("/analyze/batch")
async def analyze_batch(
    images: List[UploadFile] = File(...),
    crop_types: List[str] = Form(...),
    confidence_threshold: Optional[float] = Form(0.7),
    services: Dict = Depends(get_services)
):
    """
    Batch analysis of multiple images.
    
    Args:
        images: List of uploaded image files
        crop_types: List of crop types corresponding to images
        confidence_threshold: Minimum confidence threshold for predictions
        
    Returns:
        Batch analysis results
    """
    start_time = time.time()
    
    try:
        if len(images) != len(crop_types):
            raise HTTPException(
                status_code=400, 
                detail="Number of images must match number of crop types"
            )
        
        if len(images) > 10:  # Limit batch size
            raise HTTPException(
                status_code=400,
                detail="Batch size limited to 10 images"
            )
        
        results = []
        
        for i, (image, crop_type) in enumerate(zip(images, crop_types)):
            try:
                # Validate inputs
                validate_crop_type(crop_type)
                image_data = await validate_image(image)
                
                # Process image
                processed_image = await services["image_processor"].process_image(
                    image_data, crop_type
                )
                
                # Run inference
                predictions = await services["inference_engine"].predict_disease(
                    processed_image, crop_type, confidence_threshold
                )
                
                # Format results
                result = await services["result_formatter"].format_disease_results(
                    predictions, crop_type, confidence_threshold
                )
                
                results.append({
                    "image_index": i,
                    "filename": image.filename,
                    "crop_type": crop_type,
                    "result": result
                })
                
            except Exception as e:
                results.append({
                    "image_index": i,
                    "filename": image.filename,
                    "crop_type": crop_type,
                    "error": str(e)
                })
        
        # Record metrics
        processing_time = time.time() - start_time
        await services["metrics_collector"].record_request(
            endpoint="analyze_batch",
            processing_time=processing_time,
            success=True,
            batch_size=len(images)
        )
        
        return JSONResponse(
            status_code=200,
            content={
                "success": True,
                "data": {
                    "batch_size": len(images),
                    "results": results
                },
                "processing_time_ms": round(processing_time * 1000, 2),
                "timestamp": time.time()
            }
        )
        
    except Exception as e:
        processing_time = time.time() - start_time
        await services["metrics_collector"].record_request(
            endpoint="analyze_batch",
            processing_time=processing_time,
            success=False,
            error=str(e)
        )
        
        logger.error(f"Batch analysis failed: {e}")
        raise HTTPException(status_code=500, detail=f"Batch analysis failed: {str(e)}")

@app.get("/metrics")
async def get_metrics():
    """Get service metrics."""
    if not metrics_collector:
        raise HTTPException(status_code=503, detail="Metrics collector not initialized")
    
    metrics = await metrics_collector.get_metrics()
    return {"metrics": metrics}

@app.get("/info")
async def get_service_info():
    """Get service information."""
    settings = get_settings()
    
    return {
        "service": "AI4Thai Vision Service",
        "version": "1.0.0",
        "models": await model_registry.list_models() if model_registry else [],
        "supported_crops": settings.supported_crops,
        "supported_formats": settings.supported_image_formats,
        "max_image_size": settings.max_image_size,
        "gpu_available": torch.cuda.is_available(),
        "gpu_count": torch.cuda.device_count() if torch.cuda.is_available() else 0
    }

if __name__ == "__main__":
    settings = get_settings()
    uvicorn.run(
        "app:app",
        host=settings.host,
        port=settings.port,
        reload=settings.debug,
        workers=1  # Use 1 worker for GPU models
    )
