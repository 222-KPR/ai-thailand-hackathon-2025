"""
Advanced Performance Monitoring and Optimization
"""

import asyncio
import logging
import time
from collections import deque
from contextlib import contextmanager
from dataclasses import dataclass
from typing import Any, Dict, List, Optional

import numpy as np
import psutil
import torch

logger = logging.getLogger(__name__)


@dataclass
class PerformanceMetrics:
    """Performance metrics data structure"""

    timestamp: float
    cpu_percent: float
    memory_percent: float
    gpu_memory_mb: float | None
    gpu_utilization: float | None
    inference_time_ms: float
    queue_size: int
    active_tasks: int


class PerformanceMonitor:
    """Real-time performance monitoring and optimization"""

    def __init__(self, max_history: int = 1000):
        self.max_history = max_history
        self.metrics_history = deque(maxlen=max_history)
        self.inference_times = deque(maxlen=100)
        self.gpu_available = torch.cuda.is_available()

    @contextmanager
    def measure_inference(self, task_name: str = "inference"):
        """Context manager to measure inference time"""
        start_time = time.time()
        start_memory = self.get_memory_usage()

        try:
            yield
        finally:
            end_time = time.time()
            inference_time = (end_time - start_time) * 1000  # Convert to ms

            self.inference_times.append(inference_time)

            # Log performance
            logger.info(
                f"Performance: {task_name}",
                extra={
                    "inference_time_ms": round(inference_time, 2),
                    "memory_delta_mb": round(self.get_memory_usage() - start_memory, 2),
                },
            )

    def get_memory_usage(self) -> float:
        """Get current memory usage in MB"""
        process = psutil.Process()
        return process.memory_info().rss / 1024 / 1024

    def get_gpu_metrics(self) -> dict[str, float | None]:
        """Get GPU metrics if available"""
        if not self.gpu_available:
            return {"memory_mb": None, "utilization": None}

        try:
            gpu_memory = torch.cuda.memory_allocated() / 1024 / 1024  # MB
            gpu_utilization = (
                torch.cuda.utilization() if hasattr(torch.cuda, "utilization") else None
            )

            return {"memory_mb": round(gpu_memory, 2), "utilization": gpu_utilization}
        except Exception as e:
            logger.warning(f"Failed to get GPU metrics: {e}")
            return {"memory_mb": None, "utilization": None}

    def collect_metrics(
        self, queue_size: int = 0, active_tasks: int = 0
    ) -> PerformanceMetrics:
        """Collect current performance metrics"""
        try:
            # System metrics
            cpu_percent = psutil.cpu_percent(interval=0.1)
            memory_percent = psutil.virtual_memory().percent

            # GPU metrics
            gpu_metrics = self.get_gpu_metrics()

            # Inference time (average of recent measurements)
            avg_inference_time = (
                np.mean(list(self.inference_times)) if self.inference_times else 0
            )

            metrics = PerformanceMetrics(
                timestamp=time.time(),
                cpu_percent=cpu_percent,
                memory_percent=memory_percent,
                gpu_memory_mb=gpu_metrics["memory_mb"],
                gpu_utilization=gpu_metrics["utilization"],
                inference_time_ms=round(avg_inference_time, 2),
                queue_size=queue_size,
                active_tasks=active_tasks,
            )

            self.metrics_history.append(metrics)
            return metrics

        except Exception as e:
            logger.error(f"Failed to collect metrics: {e}")
            return PerformanceMetrics(
                timestamp=time.time(),
                cpu_percent=0,
                memory_percent=0,
                gpu_memory_mb=None,
                gpu_utilization=None,
                inference_time_ms=0,
                queue_size=0,
                active_tasks=0,
            )

    def get_performance_summary(self, window_minutes: int = 10) -> dict[str, Any]:
        """Get performance summary for the specified time window"""
        try:
            current_time = time.time()
            cutoff_time = current_time - (window_minutes * 60)

            # Filter recent metrics
            recent_metrics = [
                m for m in self.metrics_history if m.timestamp >= cutoff_time
            ]

            if not recent_metrics:
                return {"error": "No recent metrics available"}

            # Calculate statistics
            cpu_values = [m.cpu_percent for m in recent_metrics]
            memory_values = [m.memory_percent for m in recent_metrics]
            inference_values = [
                m.inference_time_ms for m in recent_metrics if m.inference_time_ms > 0
            ]

            summary = {
                "time_window_minutes": window_minutes,
                "sample_count": len(recent_metrics),
                "cpu": {
                    "avg": round(np.mean(cpu_values), 2),
                    "max": round(np.max(cpu_values), 2),
                    "min": round(np.min(cpu_values), 2),
                },
                "memory": {
                    "avg": round(np.mean(memory_values), 2),
                    "max": round(np.max(memory_values), 2),
                    "min": round(np.min(memory_values), 2),
                },
                "inference": {
                    "avg_ms": round(np.mean(inference_values), 2)
                    if inference_values
                    else 0,
                    "max_ms": round(np.max(inference_values), 2)
                    if inference_values
                    else 0,
                    "min_ms": round(np.min(inference_values), 2)
                    if inference_values
                    else 0,
                    "p95_ms": round(np.percentile(inference_values, 95), 2)
                    if inference_values
                    else 0,
                },
            }

            # Add GPU metrics if available
            gpu_memory_values = [
                m.gpu_memory_mb for m in recent_metrics if m.gpu_memory_mb is not None
            ]
            if gpu_memory_values:
                summary["gpu"] = {
                    "avg_memory_mb": round(np.mean(gpu_memory_values), 2),
                    "max_memory_mb": round(np.max(gpu_memory_values), 2),
                    "min_memory_mb": round(np.min(gpu_memory_values), 2),
                }

            # Performance health indicators
            summary["health"] = {
                "cpu_status": "high" if summary["cpu"]["avg"] > 80 else "normal",
                "memory_status": "high" if summary["memory"]["avg"] > 85 else "normal",
                "inference_status": "slow"
                if summary["inference"]["avg_ms"] > 5000
                else "normal",
            }

            return summary

        except Exception as e:
            logger.error(f"Failed to generate performance summary: {e}")
            return {"error": str(e)}


class ModelOptimizer:
    """Model optimization utilities"""

    @staticmethod
    def optimize_model_for_inference(model):
        """Apply inference optimizations to model"""
        try:
            # Set to evaluation mode
            model.eval()

            # Disable gradient computation
            for param in model.parameters():
                param.requires_grad = False

            # Enable fusion optimizations if available
            if hasattr(torch.jit, "optimized_execution"):
                torch.jit.optimized_execution(True)

            # Try to use torch.compile if available (PyTorch 2.0+)
            if hasattr(torch, "compile"):
                try:
                    model = torch.compile(model, mode="reduce-overhead")
                    logger.info("Applied torch.compile optimization")
                except Exception as e:
                    logger.warning(f"torch.compile failed: {e}")

            logger.info("Model optimized for inference")
            return model

        except Exception as e:
            logger.error(f"Model optimization failed: {e}")
            return model

    @staticmethod
    def get_model_memory_usage(model) -> dict[str, float]:
        """Calculate model memory usage"""
        try:
            param_size = 0
            buffer_size = 0

            for param in model.parameters():
                param_size += param.nelement() * param.element_size()

            for buffer in model.buffers():
                buffer_size += buffer.nelement() * buffer.element_size()

            total_size = param_size + buffer_size

            return {
                "parameters_mb": round(param_size / 1024 / 1024, 2),
                "buffers_mb": round(buffer_size / 1024 / 1024, 2),
                "total_mb": round(total_size / 1024 / 1024, 2),
            }

        except Exception as e:
            logger.error(f"Failed to calculate model memory: {e}")
            return {"error": str(e)}


class BatchProcessor:
    """Optimized batch processing for multiple images"""

    def __init__(self, max_batch_size: int = 4, timeout_seconds: float = 2.0):
        self.max_batch_size = max_batch_size
        self.timeout_seconds = timeout_seconds
        self.pending_requests = []
        self.processing_lock = asyncio.Lock()

    async def process_batch(self, request_data: dict[str, Any], process_func) -> Any:
        """Add request to batch and process when ready"""
        async with self.processing_lock:
            self.pending_requests.append(request_data)

            # Process batch if full or timeout reached
            if (
                len(self.pending_requests) >= self.max_batch_size
                or self._should_process_timeout()
            ):
                batch = self.pending_requests.copy()
                self.pending_requests.clear()

                try:
                    # Process batch
                    results = await process_func(batch)
                    return results
                except Exception as e:
                    logger.error(f"Batch processing failed: {e}")
                    raise

            return None  # Added to batch, will be processed later

    def _should_process_timeout(self) -> bool:
        """Check if batch should be processed due to timeout"""
        if not self.pending_requests:
            return False

        # Implementation would need timestamp tracking
        # Simplified for this example
        return len(self.pending_requests) > 0
