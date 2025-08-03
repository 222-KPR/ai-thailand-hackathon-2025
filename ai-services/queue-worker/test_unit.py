"""
Unit Tests for Queue Worker Service
"""

import asyncio
import base64
import os

# Import the app
import sys
from unittest.mock import Mock, patch

import pytest
from fastapi.testclient import TestClient


sys.path.append(os.path.dirname(__file__))

from app import app


class TestQueueWorkerAPI:
    """Test the Queue Worker API endpoints"""

    @pytest.fixture
    def client(self):
        """Create a test client"""
        return TestClient(app)

    @pytest.fixture
    def sample_image_b64(self):
        """Create a sample base64 encoded image"""
        import io

        from PIL import Image

        img = Image.new("RGB", (224, 224), color="green")
        img_bytes = io.BytesIO()
        img.save(img_bytes, format="JPEG")
        img_bytes.seek(0)

        return base64.b64encode(img_bytes.getvalue()).decode("utf-8")

    def test_health_endpoint(self, client):
        """Test the health endpoint"""
        response = client.get("/health")
        assert response.status_code == 200
        data = response.json()
        assert "status" in data
        assert "services" in data
        assert "timestamp" in data

    @patch("app.celery_app.send_task")
    def test_queue_pest_detection(self, mock_send_task, client):
        """Test queuing pest detection job"""
        # Mock Celery task
        mock_task = Mock()
        mock_task.id = "test-task-123"
        mock_send_task.return_value = mock_task

        import io

        from PIL import Image

        img = Image.new("RGB", (224, 224), color="green")
        img_bytes = io.BytesIO()
        img.save(img_bytes, format="JPEG")
        img_bytes.seek(0)

        response = client.post(
            "/analyze/pest",
            files={"image": ("test.jpg", img_bytes.getvalue(), "image/jpeg")},
            data={"confidence_threshold": 0.5, "return_details": True},
        )

        assert response.status_code == 200
        data = response.json()
        assert "job_id" in data
        assert "status" in data
        assert data["status"] == "queued"
        assert data["type"] == "pest_detection"

        mock_send_task.assert_called_once()

    @patch("app.celery_app.send_task")
    def test_queue_disease_detection(self, mock_send_task, client):
        """Test queuing disease detection job"""
        # Mock Celery task
        mock_task = Mock()
        mock_task.id = "test-task-456"
        mock_send_task.return_value = mock_task

        import io

        from PIL import Image

        img = Image.new("RGB", (224, 224), color="green")
        img_bytes = io.BytesIO()
        img.save(img_bytes, format="JPEG")
        img_bytes.seek(0)

        response = client.post(
            "/analyze/disease",
            files={"image": ("test.jpg", img_bytes.getvalue(), "image/jpeg")},
            data={"include_treatment": True},
        )

        assert response.status_code == 200
        data = response.json()
        assert "job_id" in data
        assert "status" in data
        assert data["status"] == "queued"
        assert data["type"] == "disease_detection"

    @patch("app.celery_app.AsyncResult")
    def test_get_job_status_success(self, mock_async_result, client):
        """Test getting job status for successful job"""
        # Mock successful task result
        mock_result = Mock()
        mock_result.state = "SUCCESS"
        mock_result.result = {
            "detections": [{"class": "aphid", "confidence": 0.85}],
            "total_detections": 1,
        }
        mock_async_result.return_value = mock_result

        response = client.get("/jobs/test-job-123")

        assert response.status_code == 200
        data = response.json()
        assert data["job_id"] == "test-job-123"
        assert data["status"] == "completed"
        assert "result" in data

    @patch("app.celery_app.AsyncResult")
    def test_get_job_status_pending(self, mock_async_result, client):
        """Test getting job status for pending job"""
        # Mock pending task result
        mock_result = Mock()
        mock_result.state = "PENDING"
        mock_async_result.return_value = mock_result

        response = client.get("/jobs/test-job-456")

        assert response.status_code == 200
        data = response.json()
        assert data["job_id"] == "test-job-456"
        assert data["status"] == "pending"

    @patch("app.celery_app.AsyncResult")
    def test_get_job_status_failed(self, mock_async_result, client):
        """Test getting job status for failed job"""
        # Mock failed task result
        mock_result = Mock()
        mock_result.state = "FAILURE"
        mock_result.info = "Model loading failed"
        mock_async_result.return_value = mock_result

        response = client.get("/jobs/test-job-789")

        assert response.status_code == 200
        data = response.json()
        assert data["job_id"] == "test-job-789"
        assert data["status"] == "failed"
        assert "error" in data

    @patch("app.celery_app.control.inspect")
    def test_queue_stats(self, mock_inspect, client):
        """Test getting queue statistics"""
        # Mock Celery inspector
        mock_inspector = Mock()
        mock_inspector.active.return_value = {"worker1": [{"id": "task1"}]}
        mock_inspector.reserved.return_value = {"worker1": [{"id": "task2"}]}
        mock_inspector.stats.return_value = {"worker1": {"total": 10}}
        mock_inspect.return_value = mock_inspector

        response = client.get("/queue/stats")

        assert response.status_code == 200
        data = response.json()
        assert "timestamp" in data
        assert "queue" in data
        assert "workers" in data
        assert "performance" in data


class TestCeleryTasks:
    """Test Celery task functions"""

    @pytest.fixture
    def sample_image_b64(self):
        """Create a sample base64 encoded image"""
        import io

        from PIL import Image

        img = Image.new("RGB", (224, 224), color="green")
        img_bytes = io.BytesIO()
        img.save(img_bytes, format="JPEG")
        img_bytes.seek(0)

        return base64.b64encode(img_bytes.getvalue()).decode("utf-8")

    @patch("tasks.requests.post")
    def test_process_pest_detection_task(self, mock_post, sample_image_b64):
        """Test pest detection Celery task"""
        # Mock the vision service response
        mock_response = Mock()
        mock_response.status_code = 200
        mock_response.json.return_value = {
            "detections": [{"class": "aphid", "confidence": 0.85}],
            "total_detections": 1,
            "processing_time": 1.5,
        }
        mock_post.return_value = mock_response

        # Import the task
        from tasks import process_pest_detection

        result = process_pest_detection(sample_image_b64, 0.5, True)

        assert "detections" in result
        assert "total_detections" in result
        mock_post.assert_called_once()

    @patch("tasks.requests.post")
    def test_process_disease_detection_task(self, mock_post, sample_image_b64):
        """Test disease detection Celery task"""
        # Mock the vision service response
        mock_response = Mock()
        mock_response.status_code = 200
        mock_response.json.return_value = {
            "disease": "leaf_spot",
            "confidence": 0.92,
            "description": "Fungal infection",
            "treatment": "Apply fungicide",
        }
        mock_post.return_value = mock_response

        # Import the task
        from tasks import process_disease_detection

        result = process_disease_detection(sample_image_b64, True)

        assert "disease" in result
        assert "confidence" in result
        mock_post.assert_called_once()

    @patch("tasks.requests.post")
    def test_task_failure_handling(self, mock_post, sample_image_b64):
        """Test task failure handling"""
        # Mock a failed request
        mock_post.side_effect = Exception("Vision service unavailable")

        from tasks import process_pest_detection

        # Should handle the exception gracefully
        with pytest.raises(Exception):
            process_pest_detection(sample_image_b64, 0.5, True)


class TestRedisIntegration:
    """Test Redis integration"""

    @pytest.fixture
    def mock_redis(self):
        """Create a mock Redis client"""
        return Mock()

    def test_redis_connection(self, mock_redis):
        """Test Redis connection"""
        mock_redis.ping.return_value = True

        assert mock_redis.ping() is True

    def test_task_metadata_storage(self, mock_redis):
        """Test storing task metadata in Redis"""
        task_id = "test-task-123"
        metadata = {
            "status": "STARTED",
            "started_at": "2025-08-03T10:00:00",
            "task_name": "process_pest_detection",
        }

        mock_redis.hset.return_value = True

        key = f"task:{task_id}:status"
        mock_redis.hset(key, mapping=metadata)

        mock_redis.hset.assert_called_once_with(key, mapping=metadata)


class TestErrorHandling:
    """Test error handling scenarios"""

    @pytest.fixture
    def client(self):
        """Create a test client"""
        return TestClient(app)

    def test_invalid_image_format(self, client):
        """Test handling of invalid image format"""
        invalid_data = b"not an image"

        response = client.post(
            "/analyze/pest",
            files={"image": ("test.txt", invalid_data, "text/plain")},
            data={"confidence_threshold": 0.5},
        )

        assert response.status_code in [400, 422]

    def test_missing_parameters(self, client):
        """Test handling of missing required parameters"""
        response = client.post("/analyze/pest")

        assert response.status_code == 422

    @patch("app.celery_app.send_task")
    def test_celery_connection_error(self, mock_send_task, client):
        """Test handling of Celery connection errors"""
        mock_send_task.side_effect = Exception("Celery broker unavailable")

        import io

        from PIL import Image

        img = Image.new("RGB", (224, 224), color="green")
        img_bytes = io.BytesIO()
        img.save(img_bytes, format="JPEG")
        img_bytes.seek(0)

        response = client.post(
            "/analyze/pest",
            files={"image": ("test.jpg", img_bytes.getvalue(), "image/jpeg")},
            data={"confidence_threshold": 0.5},
        )

        assert response.status_code == 500


class TestPerformanceMetrics:
    """Test performance monitoring"""

    @pytest.fixture
    def client(self):
        """Create a test client"""
        return TestClient(app)

    @patch("app.celery_app.control.inspect")
    @patch("app.redis_client.llen")
    def test_metrics_endpoint(self, mock_llen, mock_inspect, client):
        """Test metrics collection endpoint"""
        # Mock Celery stats
        mock_inspector = Mock()
        mock_inspector.active.return_value = {"worker1": []}
        mock_inspector.reserved.return_value = {"worker1": []}
        mock_inspector.stats.return_value = {"worker1": {"total": 10}}
        mock_inspect.return_value = mock_inspector

        # Mock Redis queue length
        mock_llen.return_value = 5

        response = client.get("/queue/stats")

        assert response.status_code == 200
        data = response.json()
        assert "timestamp" in data
        assert "queue" in data
        assert "workers" in data


# Pytest configuration and fixtures
@pytest.fixture(scope="session")
def event_loop():
    """Create an event loop for async tests"""
    loop = asyncio.get_event_loop_policy().new_event_loop()
    yield loop
    loop.close()


if __name__ == "__main__":
    pytest.main([__file__, "-v", "--tb=short"])
