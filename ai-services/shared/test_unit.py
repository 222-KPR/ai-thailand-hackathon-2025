"""
Unit Tests for Shared Utilities
"""

import json
import os
import shutil

# Import shared utilities
import sys
import tempfile
from datetime import datetime, timedelta
from pathlib import Path
from unittest.mock import Mock, patch

import pytest

sys.path.append(os.path.dirname(__file__))

try:
    from data_manager import DataManager, DataRecord, DataType
    from error_handling import (
        CircuitBreaker,
        ErrorCategory,
        ErrorHandler,
        ErrorSeverity,
    )
    from performance_monitor import PerformanceMetrics, PerformanceMonitor
except ImportError:
    # If imports fail, create minimal test stubs
    pytest.skip("Shared utilities not available", allow_module_level=True)


class TestDataManager:
    """Test the Data Manager utility"""

    @pytest.fixture
    def temp_data_dir(self):
        """Create a temporary directory for testing"""
        temp_dir = tempfile.mkdtemp()
        yield temp_dir
        shutil.rmtree(temp_dir)

    @pytest.fixture
    def data_manager(self, temp_data_dir):
        """Create a data manager instance with temp directory"""
        return DataManager(temp_data_dir)

    def test_initialization(self, data_manager, temp_data_dir):
        """Test data manager initialization"""
        assert data_manager.base_path == Path(temp_data_dir)
        assert data_manager.metadata_file.exists()

        # Check directory structure
        expected_dirs = [
            "images/incoming",
            "images/processed",
            "results/pest_detection",
            "results/disease_detection",
            "cache",
            "logs",
            "temp",
        ]

        for directory in expected_dirs:
            assert (data_manager.base_path / directory).exists()

    def test_store_image(self, data_manager):
        """Test storing an image with metadata"""
        image_data = b"fake image data"
        filename = "test.jpg"
        metadata = {
            "content_type": "image/jpeg",
            "timestamp": datetime.now().isoformat(),
        }

        file_id = data_manager.store_image(image_data, filename, metadata)

        assert file_id is not None
        assert file_id in data_manager.records

        record = data_manager.records[file_id]
        assert record.type == DataType.IMAGE
        assert record.size_bytes == len(image_data)
        assert Path(record.path).exists()

    def test_store_result(self, data_manager):
        """Test storing analysis result"""
        result_data = {
            "detections": [{"class": "aphid", "confidence": 0.85}],
            "total_detections": 1,
        }
        data_type = "pest_detection"
        related_image_id = "test_image_123"

        result_id = data_manager.store_result(result_data, data_type, related_image_id)

        assert result_id is not None
        assert result_id in data_manager.records

        record = data_manager.records[result_id]
        assert record.type == DataType.RESULT
        assert Path(record.path).exists()

        # Check stored data
        with open(record.path) as f:
            stored_data = json.load(f)
            assert stored_data["result"] == result_data
            assert stored_data["image_id"] == related_image_id

    def test_get_data(self, data_manager):
        """Test retrieving stored data"""
        # Store test data first
        test_data = b"test data"
        file_id = data_manager.store_image(test_data, "test.jpg", {})

        # Retrieve data
        retrieved_data = data_manager.get_data(file_id)

        assert retrieved_data == test_data

    def test_cleanup_expired_data(self, data_manager):
        """Test cleaning up expired data"""
        # Store data with short retention
        image_data = b"old image data"
        file_id = data_manager.store_image(image_data, "old.jpg", {})

        # Manually set creation time to past
        record = data_manager.records[file_id]
        old_time = datetime.now() - timedelta(days=10)
        record.created_at = old_time.isoformat()
        record.retention_days = 7
        data_manager.save_metadata()

        # Run cleanup
        result = data_manager.cleanup_expired_data()

        assert "removed_files" in result
        assert result["removed_files"] > 0
        assert file_id not in data_manager.records

    def test_get_storage_stats(self, data_manager):
        """Test getting storage statistics"""
        # Store some test data
        data_manager.store_image(b"test image 1", "test1.jpg", {})
        data_manager.store_image(b"test image 2", "test2.jpg", {})

        stats = data_manager.get_storage_stats()

        assert "total_records" in stats
        assert "by_type" in stats
        assert "total_size_mb" in stats
        assert stats["total_records"] >= 2


class TestErrorHandler:
    """Test the Error Handler utility"""

    @pytest.fixture
    def error_handler(self):
        """Create an error handler instance"""
        return ErrorHandler()

    def test_format_error_response(self, error_handler):
        """Test error response formatting"""
        error = Exception("Test error")
        response = error_handler.format_error_response(
            error, ErrorCategory.MODEL_ERROR, ErrorSeverity.HIGH
        )

        assert "error" in response
        assert "category" in response
        assert "severity" in response
        assert "timestamp" in response
        assert response["error"] == "Test error"

    def test_log_error(self, error_handler):
        """Test error logging"""
        error = Exception("Test error")

        # Should not raise an exception
        error_handler.log_error(
            error, ErrorCategory.API_ERROR, ErrorSeverity.MEDIUM, {"context": "test"}
        )

    def test_should_retry(self, error_handler):
        """Test retry logic"""
        # Test retryable error
        retryable_error = ConnectionError("Connection failed")
        assert (
            error_handler.should_retry(retryable_error, attempt=1, max_attempts=3)
            is True
        )

        # Test non-retryable error
        non_retryable_error = ValueError("Invalid input")
        assert (
            error_handler.should_retry(non_retryable_error, attempt=1, max_attempts=3)
            is False
        )

        # Test max attempts reached
        assert (
            error_handler.should_retry(retryable_error, attempt=3, max_attempts=3)
            is False
        )


class TestCircuitBreaker:
    """Test the Circuit Breaker utility"""

    @pytest.fixture
    def circuit_breaker(self):
        """Create a circuit breaker instance"""
        return CircuitBreaker(failure_threshold=3, timeout=60)

    def test_initial_state(self, circuit_breaker):
        """Test initial circuit breaker state"""
        assert circuit_breaker.state == "CLOSED"
        assert circuit_breaker.failure_count == 0

    def test_record_success(self, circuit_breaker):
        """Test recording successful operations"""
        circuit_breaker.record_success()
        assert circuit_breaker.failure_count == 0
        assert circuit_breaker.state == "CLOSED"

    def test_record_failure(self, circuit_breaker):
        """Test recording failed operations"""
        # Record failures up to threshold
        for i in range(3):
            circuit_breaker.record_failure()

        # Should open circuit
        assert circuit_breaker.state == "OPEN"

    def test_can_execute(self, circuit_breaker):
        """Test execution permission"""
        # Initially should allow execution
        assert circuit_breaker.can_execute() is True

        # After failures, should block execution
        for i in range(3):
            circuit_breaker.record_failure()

        assert circuit_breaker.can_execute() is False


class TestPerformanceMonitor:
    """Test the Performance Monitor utility"""

    @pytest.fixture
    def performance_monitor(self):
        """Create a performance monitor instance"""
        return PerformanceMonitor()

    def test_initialization(self, performance_monitor):
        """Test performance monitor initialization"""
        assert performance_monitor.max_history == 1000
        assert len(performance_monitor.metrics_history) == 0
        assert len(performance_monitor.inference_times) == 0

    @patch("performance_monitor.psutil.cpu_percent")
    @patch("performance_monitor.psutil.virtual_memory")
    def test_collect_metrics(self, mock_memory, mock_cpu, performance_monitor):
        """Test metrics collection"""
        # Mock system metrics
        mock_cpu.return_value = 45.5
        mock_memory.return_value = Mock(percent=67.2)

        metrics = performance_monitor.collect_metrics(queue_size=5, active_tasks=2)

        assert isinstance(metrics, PerformanceMetrics)
        assert metrics.cpu_percent == 45.5
        assert metrics.memory_percent == 67.2
        assert metrics.queue_size == 5
        assert metrics.active_tasks == 2

    def test_measure_inference_context_manager(self, performance_monitor):
        """Test inference time measurement"""
        import time

        with performance_monitor.measure_inference("test_task"):
            time.sleep(0.1)  # Simulate work

        assert len(performance_monitor.inference_times) == 1
        assert performance_monitor.inference_times[0] >= 100  # At least 100ms

    def test_get_performance_summary(self, performance_monitor):
        """Test performance summary generation"""
        # Add some mock metrics
        import time

        current_time = time.time()

        for i in range(5):
            metrics = PerformanceMetrics(
                timestamp=current_time - (i * 60),  # 1 minute apart
                cpu_percent=50 + i,
                memory_percent=60 + i,
                gpu_memory_mb=None,
                gpu_utilization=None,
                inference_time_ms=1000 + (i * 100),
                queue_size=i,
                active_tasks=i,
            )
            performance_monitor.metrics_history.append(metrics)

        summary = performance_monitor.get_performance_summary(window_minutes=10)

        assert "cpu" in summary
        assert "memory" in summary
        assert "inference" in summary
        assert "health" in summary
        assert summary["sample_count"] == 5


# Test configuration
@pytest.fixture(scope="session")
def event_loop():
    """Create an event loop for async tests"""
    import asyncio

    loop = asyncio.get_event_loop_policy().new_event_loop()
    yield loop
    loop.close()


if __name__ == "__main__":
    pytest.main([__file__, "-v", "--tb=short"])
