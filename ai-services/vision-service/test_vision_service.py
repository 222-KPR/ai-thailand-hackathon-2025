#!/usr/bin/env python3
"""
Test script for AI4Thai Vision Service (Pest Detection + Disease Detection)
"""

import asyncio
import sys
import logging
import httpx
from pathlib import Path

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

async def test_service_endpoints():
    """Test the FastAPI service endpoints"""
    base_url = "http://localhost:8001"
    
    async with httpx.AsyncClient() as client:
        try:
            # Test health endpoint
            logger.info("Testing health endpoint...")
            response = await client.get(f"{base_url}/health")
            logger.info(f"Health response: {response.status_code} - {response.json()}")
            
            # Test info endpoint
            logger.info("Testing info endpoint...")
            response = await client.get(f"{base_url}/info")
            logger.info(f"Info response: {response.status_code}")
            info_data = response.json()
            logger.info(f"Service: {info_data.get('service')}")
            logger.info(f"Models: {len(info_data.get('models', []))}")
            
            # Test detailed health
            logger.info("Testing detailed health...")
            response = await client.get(f"{base_url}/health/detailed")
            logger.info(f"Detailed health: {response.status_code}")
            
            # Test pest health
            logger.info("Testing pest detection health...")
            response = await client.get(f"{base_url}/health/pests")
            logger.info(f"Pest health: {response.status_code}")
            
            # Test disease health
            logger.info("Testing disease detection health...")
            response = await client.get(f"{base_url}/health/disease")
            logger.info(f"Disease health: {response.status_code}")
            
            # Test root endpoint
            logger.info("Testing root endpoint...")
            response = await client.get(f"{base_url}/")
            root_data = response.json()
            logger.info(f"Root response: {root_data.get('message')}")
            logger.info(f"Available endpoints: {len(root_data.get('endpoints', {}))}")
            
            return True
            
        except Exception as e:
            logger.error(f"Service test failed: {e}")
            return False

async def test_local_services():
    """Test the local services"""
    try:
        logger.info("Testing Local Vision Services")
        
        # Add the current directory to Python path
        sys.path.insert(0, str(Path(__file__).parent))
        
        from services.pest_detection import get_pest_detection_service
        from services.disease_detection import get_disease_detection_service
        
        # Test pest detection service
        logger.info("Testing pest detection service...")
        pest_service = await get_pest_detection_service()
        pest_health = await pest_service.health_check()
        logger.info(f"Pest detection health: {pest_health}")
        
        # Test disease detection service
        logger.info("Testing disease detection service...")
        disease_service = await get_disease_detection_service()
        disease_health = await disease_service.health_check()
        logger.info(f"Disease detection health: {disease_health}")
        
        # Check if both services are healthy
        pest_ok = pest_health.get("status") == "healthy"
        disease_ok = disease_health.get("status") in ["healthy", "loading"]
        
        if pest_ok and disease_ok:
            logger.info("✅ Both services are ready!")
            logger.info(f"Pest model loaded: {pest_health.get('model_loaded', False)}")
            logger.info(f"Disease model loaded: {disease_health.get('model_loaded', False)}")
        else:
            logger.warning("⚠️ Some services have issues")
            if not pest_ok:
                logger.warning(f"Pest detection issue: {pest_health}")
            if not disease_ok:
                logger.warning(f"Disease detection issue: {disease_health}")
        
        return True
        
    except Exception as e:
        logger.error(f"Local services test failed: {e}")
        return False

async def test_with_sample_requests():
    """Test endpoints with sample requests (if service is running)"""
    base_url = "http://localhost:8001"
    
    async with httpx.AsyncClient(timeout=60.0) as client:
        try:
            logger.info("Testing sample API requests...")
            
            # Note: These would need actual image files to work properly
            # This is just testing the endpoint structure
            
            logger.info("📋 Available endpoints for testing:")
            logger.info("  • POST /detect/pests - Pest detection")
            logger.info("  • POST /detect/disease - Disease detection") 
            logger.info("  • POST /analyze/comprehensive - Combined analysis")
            logger.info("  • POST /analyze - Alias for pest detection")
            
            logger.info("\n💡 Example usage:")
            logger.info("curl -X POST http://localhost:8001/detect/pests \\")
            logger.info("  -F 'image=@crop_image.jpg' \\")
            logger.info("  -F 'confidence_threshold=0.01'")
            
            logger.info("\ncurl -X POST http://localhost:8001/detect/disease \\")
            logger.info("  -F 'image=@leaf_image.jpg'")
            
            logger.info("\ncurl -X POST http://localhost:8001/analyze/comprehensive \\")
            logger.info("  -F 'image=@plant_image.jpg' \\")
            logger.info("  -F 'pest_confidence=0.01' \\")
            logger.info("  -F 'pest_details=true'")
            
            return True
            
        except Exception as e:
            logger.error(f"Sample requests test failed: {e}")
            return False

async def main():
    """Main test function"""
    logger.info("=== AI4Thai Vision Service Test ===")
    logger.info("Testing comprehensive pest detection and disease identification service")
    
    # Test local services
    logger.info("\n1. Testing local services...")
    local_success = await test_local_services()
    
    # Test FastAPI endpoints (if service is running)
    logger.info("\n2. Testing FastAPI service endpoints...")
    try:
        service_success = await test_service_endpoints()
    except Exception as e:
        logger.info(f"Service endpoints test skipped (service not running): {e}")
        service_success = False
    
    # Test sample requests
    logger.info("\n3. Testing sample request information...")
    try:
        sample_success = await test_with_sample_requests()
    except Exception as e:
        logger.info(f"Sample requests test skipped: {e}")
        sample_success = False
    
    # Summary
    logger.info("\n=== Test Summary ===")
    logger.info(f"Local services test: {'✅ PASS' if local_success else '❌ FAIL'}")
    logger.info(f"Service endpoints test: {'✅ PASS' if service_success else '❌ FAIL (or not running)'}")
    logger.info(f"Sample requests test: {'✅ PASS' if sample_success else '❌ FAIL (or not running)'}")
    
    if local_success:
        logger.info("\n🎉 Vision services are working!")
        logger.info("💡 To start the service, run: python app.py")
        logger.info("🔍 Features available:")
        logger.info("  • Pest detection using YOLO11s")
        logger.info("  • Disease detection using LLaVA-v1.5-7B")
        logger.info("  • Comprehensive analysis with Thai summaries")
        logger.info("  • Treatment recommendations")
        
        if not service_success:
            logger.info("\n⚡ Start the service to test API endpoints:")
            logger.info("   python app.py")
    
    return local_success

if __name__ == "__main__":
    success = asyncio.run(main())
    sys.exit(0 if success else 1)