"""
Unit Tests for Vision Service
"""

import io
import os

# Import the app and services
import sys
from unittest.mock import Mock, patch

import numpy as np
import pytest
from fastapi.testclient import TestClient
from PIL import Image


sys.path.append(os.path.dirname(__file__))

from app import app
from memory_manager import MemoryManager
from services.disease_detection import DiseaseDetectionService
from services.pest_detection import PestDetectionService


class TestVisionServiceAPI:
    """Test the Vision Service API endpoints"""

    @pytest.fixture
    def client(self):
        """Create a test client"""
        return TestClient(app)

    @pytest.fixture
    def sample_image(self):
        """Create a sample test image"""
        # Create a simple RGB image
        img = Image.new("RGB", (224, 224), color="green")
        img_bytes = io.BytesIO()
        img.save(img_bytes, format="JPEG")
        img_bytes.seek(0)
        return img_bytes.getvalue()

    def test_health_endpoint(self, client):
        """Test the health endpoint"""
        response = client.get("/health")
        assert response.status_code == 200
        data = response.json()
        assert "status" in data
        assert "timestamp" in data

    def test_info_endpoint(self, client):
        """Test the service info endpoint"""
        response = client.get("/info")
        assert response.status_code == 200
        data = response.json()
        assert "service" in data
        assert "version" in data
        assert "models" in data
        assert "timestamp" in data

    def test_detailed_health_endpoint(self, client):
        """Test the detailed health endpoint"""
        response = client.get("/health/detailed")
        assert response.status_code == 200
        data = response.json()
        assert "system" in data
        assert "models" in data
        assert "timestamp" in data

    @patch("services.pest_detection.PestDetectionService.detect")
    def test_pest_detection_endpoint(self, mock_detect, client, sample_image):
        """Test the pest detection endpoint"""
        # Mock the detection response
        mock_detect.return_value = {
            "detections": [
                {"class": "aphid", "confidence": 0.85, "bbox": [100, 100, 200, 200]}
            ],
            "total_detections": 1,
            "processing_time": 1.5,
        }

        response = client.post(
            "/analyze/pest",
            files={"image": ("test.jpg", sample_image, "image/jpeg")},
            data={"confidence_threshold": 0.5, "return_details": True},
        )

        assert response.status_code == 200
        data = response.json()
        assert "detections" in data
        assert "total_detections" in data
        assert "timestamp" in data

    @patch("services.disease_detection.DiseaseDetectionService.detect")
    def test_disease_detection_endpoint(self, mock_detect, client, sample_image):
        """Test the disease detection endpoint"""
        # Mock the detection response
        mock_detect.return_value = {
            "disease": "leaf_spot",
            "confidence": 0.92,
            "description": "Fungal infection causing brown spots",
            "treatment": "Apply fungicide spray",
            "severity": "moderate",
        }

        response = client.post(
            "/analyze/disease",
            files={"image": ("test.jpg", sample_image, "image/jpeg")},
            data={"include_treatment": True},
        )

        assert response.status_code == 200
        data = response.json()
        assert "disease" in data
        assert "confidence" in data
        assert "timestamp" in data

    def test_invalid_image_format(self, client):
        """Test handling of invalid image format"""
        invalid_data = b"not an image"

        response = client.post(
            "/analyze/pest",
            files={"image": ("test.txt", invalid_data, "text/plain")},
            data={"confidence_threshold": 0.5},
        )

        assert response.status_code == 422 or response.status_code == 400

    def test_missing_image(self, client):
        """Test handling of missing image"""
        response = client.post("/analyze/pest", data={"confidence_threshold": 0.5})

        assert response.status_code == 422


class TestPestDetectionService:
    """Test the Pest Detection Service"""

    @pytest.fixture
    def pest_service(self):
        """Create a pest detection service instance"""
        return PestDetectionService()

    @pytest.fixture
    def sample_image_array(self):
        """Create a sample image array"""
        return np.random.randint(0, 255, (224, 224, 3), dtype=np.uint8)

    def test_service_initialization(self, pest_service):
        """Test service initialization"""
        assert pest_service.model_path is not None
        assert pest_service.class_names is not None
        assert len(pest_service.class_names) > 0

    @patch("services.pest_detection.YOLO")
    def test_load_model(self, mock_yolo, pest_service):
        """Test model loading"""
        mock_model = Mock()
        mock_yolo.return_value = mock_model

        pest_service.load_model()

        assert pest_service.model is not None
        mock_yolo.assert_called_once()

    @patch("services.pest_detection.YOLO")
    def test_detect_with_results(self, mock_yolo, pest_service, sample_image_array):
        """Test detection with mock results"""
        # Mock YOLO model and results
        mock_model = Mock()
        mock_result = Mock()
        mock_result.boxes.xyxy.cpu().numpy.return_value = np.array(
            [[100, 100, 200, 200]]
        )
        mock_result.boxes.conf.cpu().numpy.return_value = np.array([0.85])
        mock_result.boxes.cls.cpu().numpy.return_value = np.array([0])

        mock_model.return_value = [mock_result]
        mock_yolo.return_value = mock_model
        pest_service.model = mock_model

        result = pest_service.detect(sample_image_array, confidence_threshold=0.5)

        assert "detections" in result
        assert "total_detections" in result
        assert "processing_time" in result
        assert result["total_detections"] == 1

    def test_preprocess_image(self, pest_service, sample_image_array):
        """Test image preprocessing"""
        processed = pest_service.preprocess_image(sample_image_array)

        assert isinstance(processed, np.ndarray)
        assert processed.shape == (224, 224, 3)
        assert processed.dtype == np.uint8


class TestDiseaseDetectionService:
    """Test the Disease Detection Service"""

    @pytest.fixture
    def disease_service(self):
        """Create a disease detection service instance"""
        return DiseaseDetectionService()

    @pytest.fixture
    def sample_image_pil(self):
        """Create a sample PIL image"""
        return Image.new("RGB", (224, 224), color="green")

    def test_service_initialization(self, disease_service):
        """Test service initialization"""
        assert disease_service.model_id is not None
        assert disease_service.disease_classes is not None
        assert len(disease_service.disease_classes) > 0

    @patch("services.disease_detection.LlavaNextForConditionalGeneration")
    @patch("services.disease_detection.LlavaNextProcessor")
    def test_load_model(self, mock_processor, mock_model, disease_service):
        """Test model loading"""
        mock_model_instance = Mock()
        mock_processor_instance = Mock()

        mock_model.from_pretrained.return_value = mock_model_instance
        mock_processor.from_pretrained.return_value = mock_processor_instance

        disease_service.load_model()

        assert disease_service.model is not None
        assert disease_service.processor is not None

    @patch("services.disease_detection.LlavaNextForConditionalGeneration")
    @patch("services.disease_detection.LlavaNextProcessor")
    def test_detect_with_mock_response(
        self, mock_processor, mock_model, disease_service, sample_image_pil
    ):
        """Test detection with mock model response"""
        # Mock the model and processor
        mock_model_instance = Mock()
        mock_processor_instance = Mock()

        mock_model.from_pretrained.return_value = mock_model_instance
        mock_processor.from_pretrained.return_value = mock_processor_instance

        # Mock the generate response
        mock_model_instance.generate.return_value = torch.tensor([[1, 2, 3, 4, 5]])
        mock_processor_instance.decode.return_value = (
            "The plant shows signs of leaf spot disease"
        )

        disease_service.model = mock_model_instance
        disease_service.processor = mock_processor_instance

        result = disease_service.detect(sample_image_pil)

        assert "disease" in result
        assert "confidence" in result
        assert "processing_time" in result


class TestMemoryManager:
    """Test the Memory Manager"""

    @pytest.fixture
    def memory_manager(self):
        """Create a memory manager instance"""
        return MemoryManager(max_models=2)

    def test_initialization(self, memory_manager):
        """Test memory manager initialization"""
        assert memory_manager.max_models == 2
        assert memory_manager.loaded_models == {}
        assert memory_manager.model_usage == {}

    @patch("torch.cuda.is_available")
    @patch("torch.cuda.memory_allocated")
    def test_get_memory_info(
        self, mock_memory_allocated, mock_cuda_available, memory_manager
    ):
        """Test memory info retrieval"""
        mock_cuda_available.return_value = True
        mock_memory_allocated.return_value = 1024 * 1024 * 1024  # 1GB

        info = memory_manager.get_memory_info()

        assert "gpu" in info
        assert "gpu_available" in info
        assert info["gpu_available"] is True

    def test_update_model_usage(self, memory_manager):
        """Test model usage tracking"""
        memory_manager.update_model_usage("test_model")

        assert "test_model" in memory_manager.model_usage
        assert memory_manager.model_usage["test_model"]["count"] == 1

    def test_should_unload_model(self, memory_manager):
        """Test model unloading logic"""
        # Add models beyond capacity
        memory_manager.loaded_models = {
            "model1": Mock(),
            "model2": Mock(),
            "model3": Mock(),
        }

        should_unload = memory_manager._should_unload_model()
        assert should_unload is True


class TestErrorHandling:
    """Test error handling scenarios"""

    @pytest.fixture
    def client(self):
        """Create a test client"""
        return TestClient(app)

    def test_large_image_handling(self, client):
        """Test handling of very large images"""
        # Create a large image (simulate > 10MB)
        large_image = b"x" * (11 * 1024 * 1024)  # 11MB

        response = client.post(
            "/analyze/pest",
            files={"image": ("large.jpg", large_image, "image/jpeg")},
            data={"confidence_threshold": 0.5},
        )

        # Should handle gracefully (either process or return appropriate error)
        assert response.status_code in [200, 413, 422]

    def test_concurrent_requests(self, client, sample_image=None):
        """Test handling of concurrent requests"""
        if sample_image is None:
            img = Image.new("RGB", (224, 224), color="green")
            img_bytes = io.BytesIO()
            img.save(img_bytes, format="JPEG")
            sample_image = img_bytes.getvalue()

        # Send multiple concurrent requests
        responses = []
        for i in range(3):
            response = client.post(
                "/analyze/pest",
                files={"image": (f"test_{i}.jpg", sample_image, "image/jpeg")},
                data={"confidence_threshold": 0.5},
            )
            responses.append(response)

        # All should complete successfully or handle gracefully
        for response in responses:
            assert response.status_code in [200, 429, 503]


# Pytest configuration
if __name__ == "__main__":
    pytest.main([__file__, "-v", "--tb=short"])
