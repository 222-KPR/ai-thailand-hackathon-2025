"""
AI4Thai Crop Guardian - LLM Service
FastAPI application for agricultural advisory using HuggingFace language models
"""

import asyncio
import logging
import time
from contextlib import asynccontextmanager
from typing import Dict, List, Optional

import torch
import uvicorn
from fastapi import FastAPI, HTTPException, Depends
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse
from pydantic import BaseModel

from models.model_registry import ModelRegistry
from services.chat_engine import ChatEngine
from services.context_manager import ContextManager
from services.response_generator import ResponseGenerator
from utils.validation import validate_message, validate_language
from config.settings import get_settings
from shared.monitoring.health_check import HealthChecker
from shared.monitoring.metrics import MetricsCollector

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

# Request/Response Models
class ChatRequest(BaseModel):
    message: str
    language: str = "th"  # Default to Thai
    context: Optional[Dict] = None
    conversation_id: Optional[str] = None
    user_id: Optional[str] = None

class ChatResponse(BaseModel):
    response: str
    language: str
    confidence: float
    conversation_id: str
    context: Dict
    processing_time_ms: float

class AdviceRequest(BaseModel):
    crop_type: str
    disease: Optional[str] = None
    symptoms: Optional[List[str]] = None
    location: Optional[str] = None
    season: Optional[str] = None
    language: str = "th"

class AdviceResponse(BaseModel):
    advice: str
    treatment_steps: List[str]
    prevention_tips: List[str]
    confidence: float
    language: str

# Global variables for models and services
model_registry: Optional[ModelRegistry] = None
chat_engine: Optional[ChatEngine] = None
context_manager: Optional[ContextManager] = None
response_generator: Optional[ResponseGenerator] = None
health_checker: Optional[HealthChecker] = None
metrics_collector: Optional[MetricsCollector] = None

@asynccontextmanager
async def lifespan(app: FastAPI):
    """Application lifespan manager for startup and shutdown events."""
    global model_registry, chat_engine, context_manager, response_generator
    global health_checker, metrics_collector
    
    settings = get_settings()
    
    try:
        # Initialize services
        logger.info("Initializing LLM Service...")
        
        # Initialize model registry and load models
        model_registry = ModelRegistry(settings.model_config)
        await model_registry.load_models()
        
        # Initialize processors
        context_manager = ContextManager(settings.context_config)
        response_generator = ResponseGenerator(model_registry, settings.generation_config)
        chat_engine = ChatEngine(
            model_registry, 
            context_manager, 
            response_generator,
            settings.chat_config
        )
        
        # Initialize monitoring
        health_checker = HealthChecker()
        metrics_collector = MetricsCollector("llm_service")
        
        logger.info("LLM Service initialized successfully")
        
        yield
        
    except Exception as e:
        logger.error(f"Failed to initialize LLM Service: {e}")
        raise
    finally:
        # Cleanup
        logger.info("Shutting down LLM Service...")
        if model_registry:
            await model_registry.cleanup()

# Create FastAPI app
app = FastAPI(
    title="AI4Thai LLM Service",
    description="Agricultural advisory and chat service using HuggingFace language models",
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
    if not all([model_registry, chat_engine, context_manager, response_generator]):
        raise HTTPException(status_code=503, detail="Service not ready")
    return {
        "model_registry": model_registry,
        "chat_engine": chat_engine,
        "context_manager": context_manager,
        "response_generator": response_generator,
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

@app.post("/chat", response_model=ChatResponse)
async def chat(
    request: ChatRequest,
    services: Dict = Depends(get_services)
):
    """
    General chat endpoint for agricultural questions.
    
    Args:
        request: Chat request with message and context
        
    Returns:
        Chat response with generated answer
    """
    start_time = time.time()
    
    try:
        # Validate inputs
        validate_message(request.message)
        validate_language(request.language)
        
        # Process chat request
        response = await services["chat_engine"].process_chat(
            message=request.message,
            language=request.language,
            context=request.context,
            conversation_id=request.conversation_id,
            user_id=request.user_id
        )
        
        # Record metrics
        processing_time = time.time() - start_time
        await services["metrics_collector"].record_request(
            endpoint="chat",
            processing_time=processing_time,
            success=True,
            language=request.language
        )
        
        return ChatResponse(
            response=response["response"],
            language=response["language"],
            confidence=response["confidence"],
            conversation_id=response["conversation_id"],
            context=response["context"],
            processing_time_ms=round(processing_time * 1000, 2)
        )
        
    except Exception as e:
        processing_time = time.time() - start_time
        await services["metrics_collector"].record_request(
            endpoint="chat",
            processing_time=processing_time,
            success=False,
            error=str(e)
        )
        
        logger.error(f"Chat processing failed: {e}")
        raise HTTPException(status_code=500, detail=f"Chat processing failed: {str(e)}")

@app.post("/advice", response_model=AdviceResponse)
async def get_agricultural_advice(
    request: AdviceRequest,
    services: Dict = Depends(get_services)
):
    """
    Get specific agricultural advice for crop diseases and treatments.
    
    Args:
        request: Advice request with crop and disease information
        
    Returns:
        Structured agricultural advice with treatment steps
    """
    start_time = time.time()
    
    try:
        # Validate inputs
        validate_language(request.language)
        
        # Generate agricultural advice
        advice_response = await services["response_generator"].generate_advice(
            crop_type=request.crop_type,
            disease=request.disease,
            symptoms=request.symptoms,
            location=request.location,
            season=request.season,
            language=request.language
        )
        
        # Record metrics
        processing_time = time.time() - start_time
        await services["metrics_collector"].record_request(
            endpoint="advice",
            processing_time=processing_time,
            success=True,
            crop_type=request.crop_type,
            language=request.language
        )
        
        return AdviceResponse(
            advice=advice_response["advice"],
            treatment_steps=advice_response["treatment_steps"],
            prevention_tips=advice_response["prevention_tips"],
            confidence=advice_response["confidence"],
            language=advice_response["language"]
        )
        
    except Exception as e:
        processing_time = time.time() - start_time
        await services["metrics_collector"].record_request(
            endpoint="advice",
            processing_time=processing_time,
            success=False,
            error=str(e)
        )
        
        logger.error(f"Advice generation failed: {e}")
        raise HTTPException(status_code=500, detail=f"Advice generation failed: {str(e)}")

@app.post("/translate")
async def translate_text(
    text: str,
    source_language: str,
    target_language: str,
    services: Dict = Depends(get_services)
):
    """
    Translate text between Thai and English.
    
    Args:
        text: Text to translate
        source_language: Source language code (th/en)
        target_language: Target language code (th/en)
        
    Returns:
        Translated text
    """
    start_time = time.time()
    
    try:
        # Validate inputs
        validate_language(source_language)
        validate_language(target_language)
        
        # Perform translation
        translation = await services["response_generator"].translate(
            text=text,
            source_language=source_language,
            target_language=target_language
        )
        
        # Record metrics
        processing_time = time.time() - start_time
        await services["metrics_collector"].record_request(
            endpoint="translate",
            processing_time=processing_time,
            success=True
        )
        
        return JSONResponse(
            status_code=200,
            content={
                "success": True,
                "data": {
                    "original_text": text,
                    "translated_text": translation["text"],
                    "source_language": source_language,
                    "target_language": target_language,
                    "confidence": translation["confidence"]
                },
                "processing_time_ms": round(processing_time * 1000, 2),
                "timestamp": time.time()
            }
        )
        
    except Exception as e:
        processing_time = time.time() - start_time
        await services["metrics_collector"].record_request(
            endpoint="translate",
            processing_time=processing_time,
            success=False,
            error=str(e)
        )
        
        logger.error(f"Translation failed: {e}")
        raise HTTPException(status_code=500, detail=f"Translation failed: {str(e)}")

@app.post("/summarize")
async def summarize_text(
    text: str,
    language: str = "th",
    max_length: Optional[int] = 150,
    services: Dict = Depends(get_services)
):
    """
    Summarize agricultural text content.
    
    Args:
        text: Text to summarize
        language: Language of the text
        max_length: Maximum length of summary
        
    Returns:
        Summarized text
    """
    start_time = time.time()
    
    try:
        # Validate inputs
        validate_language(language)
        
        # Generate summary
        summary = await services["response_generator"].summarize(
            text=text,
            language=language,
            max_length=max_length
        )
        
        # Record metrics
        processing_time = time.time() - start_time
        await services["metrics_collector"].record_request(
            endpoint="summarize",
            processing_time=processing_time,
            success=True
        )
        
        return JSONResponse(
            status_code=200,
            content={
                "success": True,
                "data": {
                    "original_text": text,
                    "summary": summary["text"],
                    "language": language,
                    "compression_ratio": summary["compression_ratio"]
                },
                "processing_time_ms": round(processing_time * 1000, 2),
                "timestamp": time.time()
            }
        )
        
    except Exception as e:
        processing_time = time.time() - start_time
        await services["metrics_collector"].record_request(
            endpoint="summarize",
            processing_time=processing_time,
            success=False,
            error=str(e)
        )
        
        logger.error(f"Summarization failed: {e}")
        raise HTTPException(status_code=500, detail=f"Summarization failed: {str(e)}")

@app.get("/conversation/{conversation_id}")
async def get_conversation_history(
    conversation_id: str,
    services: Dict = Depends(get_services)
):
    """Get conversation history."""
    try:
        history = await services["context_manager"].get_conversation_history(conversation_id)
        return {"conversation_id": conversation_id, "history": history}
        
    except Exception as e:
        logger.error(f"Failed to get conversation history: {e}")
        raise HTTPException(status_code=500, detail=f"Failed to get conversation history: {str(e)}")

@app.delete("/conversation/{conversation_id}")
async def clear_conversation(
    conversation_id: str,
    services: Dict = Depends(get_services)
):
    """Clear conversation history."""
    try:
        await services["context_manager"].clear_conversation(conversation_id)
        return {"message": f"Conversation {conversation_id} cleared successfully"}
        
    except Exception as e:
        logger.error(f"Failed to clear conversation: {e}")
        raise HTTPException(status_code=500, detail=f"Failed to clear conversation: {str(e)}")

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
        "service": "AI4Thai LLM Service",
        "version": "1.0.0",
        "models": await model_registry.list_models() if model_registry else [],
        "supported_languages": settings.supported_languages,
        "supported_crops": settings.supported_crops,
        "max_context_length": settings.max_context_length,
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
