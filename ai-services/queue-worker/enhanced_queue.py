"""
Enhanced Queue Management with Advanced Features
"""

import time
from enum import Enum
from typing import Any

import redis
import structlog
from celery import Celery
from celery.signals import task_failure, task_postrun, task_prerun


logger = structlog.get_logger(__name__)


class TaskStatus(Enum):
    PENDING = "PENDING"
    STARTED = "STARTED"
    PROCESSING = "PROCESSING"
    SUCCESS = "SUCCESS"
    FAILURE = "FAILURE"
    RETRY = "RETRY"


class EnhancedQueue:
    """Enhanced queue management with monitoring and error handling"""

    def __init__(self, celery_app: Celery, redis_client: redis.Redis):
        self.celery = celery_app
        self.redis = redis_client
        self.setup_monitoring()

    def setup_monitoring(self):
        """Setup task monitoring and lifecycle hooks"""

        @task_prerun.connect
        def task_prerun_handler(
            sender=None, task_id=None, task=None, args=None, kwargs=None, **kwds
        ):
            """Log task start and update status"""
            logger.info("Task started", task_id=task_id, task_name=task.name)
            self.update_task_status(
                task_id,
                TaskStatus.STARTED,
                {"started_at": time.time(), "task_name": task.name},
            )

        @task_postrun.connect
        def task_postrun_handler(
            sender=None,
            task_id=None,
            task=None,
            args=None,
            kwargs=None,
            retval=None,
            state=None,
            **kwds,
        ):
            """Log task completion"""
            logger.info("Task completed", task_id=task_id, state=state)
            self.update_task_status(
                task_id,
                TaskStatus.SUCCESS if state == "SUCCESS" else TaskStatus.FAILURE,
                {
                    "completed_at": time.time(),
                    "result": retval if state == "SUCCESS" else None,
                },
            )

        @task_failure.connect
        def task_failure_handler(
            sender=None,
            task_id=None,
            exception=None,
            traceback=None,
            einfo=None,
            **kwds,
        ):
            """Handle task failures with detailed logging"""
            logger.error("Task failed", task_id=task_id, exception=str(exception))
            self.update_task_status(
                task_id,
                TaskStatus.FAILURE,
                {
                    "failed_at": time.time(),
                    "error": str(exception),
                    "traceback": str(traceback),
                },
            )

    def update_task_status(
        self, task_id: str, status: TaskStatus, metadata: dict[str, Any]
    ):
        """Update task status in Redis with metadata"""
        try:
            key = f"task:{task_id}:status"
            data = {"status": status.value, "updated_at": time.time(), **metadata}
            self.redis.hset(key, mapping=data)
            self.redis.expire(key, 3600)  # Expire after 1 hour
        except Exception as e:
            logger.error("Failed to update task status", task_id=task_id, error=str(e))

    def get_task_status(self, task_id: str) -> dict[str, Any] | None:
        """Get comprehensive task status"""
        try:
            key = f"task:{task_id}:status"
            data = self.redis.hgetall(key)
            if data:
                return {
                    k.decode() if isinstance(k, bytes) else k: v.decode()
                    if isinstance(v, bytes)
                    else v
                    for k, v in data.items()
                }
            return None
        except Exception as e:
            logger.error("Failed to get task status", task_id=task_id, error=str(e))
            return None

    def get_queue_metrics(self) -> dict[str, Any]:
        """Get queue performance metrics"""
        try:
            # Get active tasks
            active_tasks = self.celery.control.inspect().active()

            # Get queue lengths
            queue_lengths = {}
            for queue in ["celery", "priority", "background"]:
                length = self.redis.llen(queue)
                queue_lengths[queue] = length

            # Calculate processing times
            task_keys = self.redis.keys("task:*:status")
            recent_tasks = []
            for key in task_keys[-100:]:  # Last 100 tasks
                task_data = self.redis.hgetall(key)
                if task_data:
                    recent_tasks.append(task_data)

            avg_processing_time = 0
            if recent_tasks:
                processing_times = []
                for task in recent_tasks:
                    if b"started_at" in task and b"completed_at" in task:
                        start = float(task[b"started_at"])
                        end = float(task[b"completed_at"])
                        processing_times.append(end - start)

                if processing_times:
                    avg_processing_time = sum(processing_times) / len(processing_times)

            return {
                "active_tasks": sum(
                    len(tasks) for tasks in (active_tasks or {}).values()
                ),
                "queue_lengths": queue_lengths,
                "avg_processing_time_seconds": round(avg_processing_time, 2),
                "total_processed": len(recent_tasks),
            }
        except Exception as e:
            logger.error("Failed to get queue metrics", error=str(e))
            return {"error": str(e)}


# Priority queue configuration
def configure_priority_queues(celery_app: Celery):
    """Configure priority-based task routing"""

    celery_app.conf.update(
        task_routes={
            "tasks.process_image_priority": {"queue": "priority"},
            "tasks.process_image_background": {"queue": "background"},
            "tasks.cleanup_old_data": {"queue": "maintenance"},
        },
        task_annotations={
            "tasks.process_image_priority": {"rate_limit": "10/m"},
            "tasks.process_image_background": {"rate_limit": "5/m"},
        },
    )
