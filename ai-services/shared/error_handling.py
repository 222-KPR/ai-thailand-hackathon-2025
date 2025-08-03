"""
Comprehensive Error Handling and Recovery System
"""

import asyncio
import json
import logging
import time
import traceback
from collections.abc import Callable
from dataclasses import dataclass
from enum import Enum
from functools import wraps
from typing import Any, Dict, List, Optional

# Enhanced logging setup
logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s - %(name)s - %(levelname)s - %(message)s",
    handlers=[logging.FileHandler("/app/logs/errors.log"), logging.StreamHandler()],
)

logger = logging.getLogger(__name__)


class ErrorSeverity(Enum):
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"


class ErrorCategory(Enum):
    MODEL_ERROR = "model_error"
    PROCESSING_ERROR = "processing_error"
    VALIDATION_ERROR = "validation_error"
    RESOURCE_ERROR = "resource_error"
    NETWORK_ERROR = "network_error"
    SYSTEM_ERROR = "system_error"


@dataclass
class ErrorRecord:
    """Detailed error record for tracking and analysis"""

    timestamp: str
    error_id: str
    category: ErrorCategory
    severity: ErrorSeverity
    message: str
    traceback: str
    context: dict[str, Any]
    recovery_attempted: bool = False
    recovery_successful: bool = False
    retry_count: int = 0


class ErrorTracker:
    """Track and analyze error patterns"""

    def __init__(self, max_records: int = 1000):
        self.max_records = max_records
        self.error_records: list[ErrorRecord] = []
        self.error_counts = {}

    def record_error(
        self,
        error: Exception,
        category: ErrorCategory,
        severity: ErrorSeverity,
        context: dict[str, Any] = None,
    ) -> str:
        """Record an error with full context"""
        import uuid

        error_id = str(uuid.uuid4())[:8]

        record = ErrorRecord(
            timestamp=time.strftime("%Y-%m-%d %H:%M:%S"),
            error_id=error_id,
            category=category,
            severity=severity,
            message=str(error),
            traceback=traceback.format_exc(),
            context=context or {},
            retry_count=0,
        )

        self.error_records.append(record)

        # Maintain max records limit
        if len(self.error_records) > self.max_records:
            self.error_records = self.error_records[-self.max_records :]

        # Update error counts
        error_key = f"{category.value}:{type(error).__name__}"
        self.error_counts[error_key] = self.error_counts.get(error_key, 0) + 1

        # Log based on severity
        log_message = f"Error {error_id}: {category.value} - {error!s}"
        if severity == ErrorSeverity.CRITICAL:
            logger.critical(
                log_message, extra={"error_id": error_id, "context": context}
            )
        elif severity == ErrorSeverity.HIGH:
            logger.error(log_message, extra={"error_id": error_id, "context": context})
        elif severity == ErrorSeverity.MEDIUM:
            logger.warning(
                log_message, extra={"error_id": error_id, "context": context}
            )
        else:
            logger.info(log_message, extra={"error_id": error_id, "context": context})

        return error_id

    def get_error_statistics(self, hours: int = 24) -> dict[str, Any]:
        """Get error statistics for the specified time period"""
        import time
        from datetime import datetime, timedelta

        cutoff_time = datetime.now() - timedelta(hours=hours)

        recent_errors = [
            record
            for record in self.error_records
            if datetime.strptime(record.timestamp, "%Y-%m-%d %H:%M:%S") >= cutoff_time
        ]

        # Count by category and severity
        category_counts = {}
        severity_counts = {}

        for record in recent_errors:
            category_counts[record.category.value] = (
                category_counts.get(record.category.value, 0) + 1
            )
            severity_counts[record.severity.value] = (
                severity_counts.get(record.severity.value, 0) + 1
            )

        # Calculate error rate (errors per hour)
        error_rate = len(recent_errors) / max(hours, 1)

        return {
            "time_period_hours": hours,
            "total_errors": len(recent_errors),
            "error_rate_per_hour": round(error_rate, 2),
            "by_category": category_counts,
            "by_severity": severity_counts,
            "most_common_errors": dict(
                sorted(self.error_counts.items(), key=lambda x: x[1], reverse=True)[:5]
            ),
        }


# Global error tracker
error_tracker = ErrorTracker()


def handle_exceptions(
    category: ErrorCategory,
    severity: ErrorSeverity = ErrorSeverity.MEDIUM,
    retry_attempts: int = 0,
    recovery_func: Callable | None = None,
):
    """Decorator for comprehensive exception handling"""

    def decorator(func):
        @wraps(func)
        async def async_wrapper(*args, **kwargs):
            attempt = 0
            last_error = None

            while attempt <= retry_attempts:
                try:
                    return await func(*args, **kwargs)
                except Exception as e:
                    last_error = e
                    attempt += 1

                    # Record error
                    context = {
                        "function": func.__name__,
                        "attempt": attempt,
                        "max_attempts": retry_attempts + 1,
                        "args_count": len(args),
                        "kwargs_keys": list(kwargs.keys()),
                    }

                    error_id = error_tracker.record_error(
                        e, category, severity, context
                    )

                    # Try recovery if function provided
                    if recovery_func and attempt <= retry_attempts:
                        try:
                            await recovery_func(e, context)
                            logger.info(f"Recovery attempted for error {error_id}")
                        except Exception as recovery_error:
                            logger.error(
                                f"Recovery failed for error {error_id}: {recovery_error}"
                            )

                    # Wait before retry (exponential backoff)
                    if attempt <= retry_attempts:
                        wait_time = min(2 ** (attempt - 1), 30)  # Max 30 seconds
                        logger.info(
                            f"Retrying in {wait_time} seconds (attempt {attempt}/{retry_attempts + 1})"
                        )
                        await asyncio.sleep(wait_time)

            # All retries exhausted
            logger.error(f"All retry attempts exhausted for {func.__name__}")
            raise last_error

        @wraps(func)
        def sync_wrapper(*args, **kwargs):
            attempt = 0
            last_error = None

            while attempt <= retry_attempts:
                try:
                    return func(*args, **kwargs)
                except Exception as e:
                    last_error = e
                    attempt += 1

                    context = {
                        "function": func.__name__,
                        "attempt": attempt,
                        "max_attempts": retry_attempts + 1,
                    }

                    error_id = error_tracker.record_error(
                        e, category, severity, context
                    )

                    if attempt <= retry_attempts:
                        wait_time = min(2 ** (attempt - 1), 30)
                        logger.info(f"Retrying in {wait_time} seconds")
                        time.sleep(wait_time)

            raise last_error

        return async_wrapper if asyncio.iscoroutinefunction(func) else sync_wrapper

    return decorator


class CircuitBreaker:
    """Circuit breaker pattern for fault tolerance"""

    def __init__(self, failure_threshold: int = 5, timeout_seconds: int = 60):
        self.failure_threshold = failure_threshold
        self.timeout_seconds = timeout_seconds
        self.failure_count = 0
        self.last_failure_time = 0
        self.state = "CLOSED"  # CLOSED, OPEN, HALF_OPEN

    def call(self, func, *args, **kwargs):
        """Execute function with circuit breaker protection"""
        if self.state == "OPEN":
            if time.time() - self.last_failure_time > self.timeout_seconds:
                self.state = "HALF_OPEN"
                logger.info("Circuit breaker transitioning to HALF_OPEN")
            else:
                raise Exception("Circuit breaker is OPEN - service unavailable")

        try:
            result = func(*args, **kwargs)

            # Success - reset failure count
            if self.state == "HALF_OPEN":
                self.state = "CLOSED"
                logger.info("Circuit breaker CLOSED - service recovered")

            self.failure_count = 0
            return result

        except Exception as e:
            self.failure_count += 1
            self.last_failure_time = time.time()

            if self.failure_count >= self.failure_threshold:
                self.state = "OPEN"
                logger.error("Circuit breaker OPEN - too many failures")

            raise e


# Recovery strategies
async def model_recovery_strategy(error: Exception, context: dict[str, Any]):
    """Recovery strategy for model errors"""
    logger.info("Attempting model recovery...")

    # Clear GPU cache if CUDA error
    if "cuda" in str(error).lower() or "gpu" in str(error).lower():
        try:
            import torch

            if torch.cuda.is_available():
                torch.cuda.empty_cache()
                logger.info("GPU cache cleared")
        except ImportError:
            pass

    # Add memory cleanup
    import gc

    gc.collect()


async def resource_recovery_strategy(error: Exception, context: dict[str, Any]):
    """Recovery strategy for resource errors"""
    logger.info("Attempting resource recovery...")

    # Memory cleanup
    import gc

    gc.collect()

    # Wait for resources to free up
    await asyncio.sleep(5)


# Error response formatter
def format_error_response(error: Exception, error_id: str) -> dict[str, Any]:
    """Format error for API response"""
    return {
        "error": True,
        "error_id": error_id,
        "message": "An error occurred while processing your request",
        "type": type(error).__name__,
        "timestamp": time.strftime("%Y-%m-%d %H:%M:%S"),
        "detail": "Please contact support if this error persists",
    }


# Health check with error awareness
def get_system_health() -> dict[str, Any]:
    """Get system health including error statistics"""
    error_stats = error_tracker.get_error_statistics(hours=1)

    # Determine health status
    health_status = "healthy"
    if error_stats["total_errors"] > 10:
        health_status = "degraded"
    if error_stats.get("by_severity", {}).get("critical", 0) > 0:
        health_status = "unhealthy"

    return {
        "status": health_status,
        "error_statistics": error_stats,
        "timestamp": time.strftime("%Y-%m-%d %H:%M:%S"),
    }
