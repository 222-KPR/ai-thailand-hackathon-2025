"""
Enhanced Data Management System
"""

import asyncio
import hashlib
import json
import logging
import os
import shutil
import time
from dataclasses import asdict, dataclass
from datetime import datetime, timedelta
from enum import Enum
from pathlib import Path
from typing import Any, Dict, List, Optional

logger = logging.getLogger(__name__)


class DataType(Enum):
    IMAGE = "image"
    RESULT = "result"
    CACHE = "cache"
    LOG = "log"
    TEMP = "temp"


@dataclass
class DataRecord:
    """Data record with metadata"""

    id: str
    type: DataType
    path: str
    size_bytes: int
    created_at: str
    accessed_at: str
    metadata: dict[str, Any]
    retention_days: int = 7


class DataManager:
    """Enhanced data management with lifecycle policies"""

    def __init__(self, base_path: str = "/app/data"):
        self.base_path = Path(base_path)
        self.metadata_file = self.base_path / "metadata.json"
        self.setup_directories()
        self.load_metadata()

    def setup_directories(self):
        """Setup data directory structure"""
        directories = [
            "images/incoming",
            "images/processed",
            "results/pest_detection",
            "results/disease_detection",
            "cache",
            "logs",
            "temp",
        ]

        for directory in directories:
            (self.base_path / directory).mkdir(parents=True, exist_ok=True)

    def load_metadata(self):
        """Load data metadata from file"""
        try:
            if self.metadata_file.exists():
                with open(self.metadata_file) as f:
                    data = json.load(f)
                    self.records = {k: DataRecord(**v) for k, v in data.items()}
            else:
                self.records = {}
        except Exception as e:
            logger.error(f"Failed to load metadata: {e}")
            self.records = {}

    def save_metadata(self):
        """Save metadata to file"""
        try:
            with open(self.metadata_file, "w") as f:
                data = {k: asdict(v) for k, v in self.records.items()}
                json.dump(data, f, indent=2)
        except Exception as e:
            logger.error(f"Failed to save metadata: {e}")

    def store_image(
        self, image_data: bytes, filename: str, metadata: dict[str, Any]
    ) -> str:
        """Store image with metadata tracking"""
        try:
            # Generate unique ID
            file_hash = hashlib.sha256(image_data).hexdigest()[:16]
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            file_id = f"{timestamp}_{file_hash}"

            # Store file
            file_path = self.base_path / "images" / "incoming" / f"{file_id}_{filename}"
            with open(file_path, "wb") as f:
                f.write(image_data)

            # Create record
            record = DataRecord(
                id=file_id,
                type=DataType.IMAGE,
                path=str(file_path),
                size_bytes=len(image_data),
                created_at=datetime.now().isoformat(),
                accessed_at=datetime.now().isoformat(),
                metadata=metadata,
                retention_days=30,  # Images kept longer
            )

            self.records[file_id] = record
            self.save_metadata()

            logger.info(f"Stored image: {file_id}, size: {len(image_data)} bytes")
            return file_id

        except Exception as e:
            logger.error(f"Failed to store image: {e}")
            raise

    def store_result(
        self, result_data: dict[str, Any], data_type: str, related_image_id: str
    ) -> str:
        """Store analysis result with linking"""
        try:
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            result_id = f"result_{timestamp}_{related_image_id}"

            # Store result
            result_dir = self.base_path / "results" / data_type
            result_path = result_dir / f"{result_id}.json"

            with open(result_path, "w") as f:
                json.dump(
                    {
                        "result": result_data,
                        "image_id": related_image_id,
                        "analysis_type": data_type,
                        "timestamp": datetime.now().isoformat(),
                    },
                    f,
                    indent=2,
                )

            # Create record
            record = DataRecord(
                id=result_id,
                type=DataType.RESULT,
                path=str(result_path),
                size_bytes=os.path.getsize(result_path),
                created_at=datetime.now().isoformat(),
                accessed_at=datetime.now().isoformat(),
                metadata={
                    "analysis_type": data_type,
                    "related_image_id": related_image_id,
                    "detection_count": len(result_data.get("detections", [])),
                },
            )

            self.records[result_id] = record
            self.save_metadata()

            return result_id

        except Exception as e:
            logger.error(f"Failed to store result: {e}")
            raise

    def get_data(self, data_id: str) -> bytes | None:
        """Retrieve data by ID"""
        try:
            if data_id not in self.records:
                return None

            record = self.records[data_id]
            file_path = Path(record.path)

            if not file_path.exists():
                logger.warning(f"File not found: {file_path}")
                return None

            # Update access time
            record.accessed_at = datetime.now().isoformat()
            self.save_metadata()

            with open(file_path, "rb") as f:
                return f.read()

        except Exception as e:
            logger.error(f"Failed to get data {data_id}: {e}")
            return None

    def cleanup_expired_data(self):
        """Clean up expired data based on retention policies"""
        try:
            current_time = datetime.now()
            expired_records = []

            for record_id, record in self.records.items():
                created_at = datetime.fromisoformat(record.created_at)
                age_days = (current_time - created_at).days

                if age_days > record.retention_days:
                    expired_records.append(record_id)

            # Remove expired files and records
            removed_count = 0
            freed_bytes = 0

            for record_id in expired_records:
                record = self.records[record_id]
                file_path = Path(record.path)

                try:
                    if file_path.exists():
                        freed_bytes += file_path.stat().st_size
                        file_path.unlink()
                        removed_count += 1

                    del self.records[record_id]
                except Exception as e:
                    logger.error(f"Failed to remove {file_path}: {e}")

            self.save_metadata()

            logger.info(
                f"Cleanup completed: {removed_count} files, {freed_bytes / 1024 / 1024:.2f} MB freed"
            )
            return {
                "removed_files": removed_count,
                "freed_mb": round(freed_bytes / 1024 / 1024, 2),
            }

        except Exception as e:
            logger.error(f"Cleanup failed: {e}")
            return {"error": str(e)}

    def get_storage_stats(self) -> dict[str, Any]:
        """Get comprehensive storage statistics"""
        try:
            stats = {
                "total_records": len(self.records),
                "by_type": {},
                "total_size_mb": 0,
                "oldest_record": None,
                "newest_record": None,
            }

            # Calculate stats by type
            for record in self.records.values():
                data_type = record.type.value
                if data_type not in stats["by_type"]:
                    stats["by_type"][data_type] = {"count": 0, "size_mb": 0}

                stats["by_type"][data_type]["count"] += 1
                stats["by_type"][data_type]["size_mb"] += (
                    record.size_bytes / 1024 / 1024
                )
                stats["total_size_mb"] += record.size_bytes / 1024 / 1024

            # Find oldest and newest records
            if self.records:
                sorted_records = sorted(
                    self.records.values(), key=lambda r: r.created_at
                )
                stats["oldest_record"] = sorted_records[0].created_at
                stats["newest_record"] = sorted_records[-1].created_at

            # Round values
            stats["total_size_mb"] = round(stats["total_size_mb"], 2)
            for type_stats in stats["by_type"].values():
                type_stats["size_mb"] = round(type_stats["size_mb"], 2)

            # Disk usage
            disk_usage = shutil.disk_usage(self.base_path)
            stats["disk"] = {
                "total_gb": round(disk_usage.total / 1024 / 1024 / 1024, 2),
                "used_gb": round(disk_usage.used / 1024 / 1024 / 1024, 2),
                "free_gb": round(disk_usage.free / 1024 / 1024 / 1024, 2),
                "usage_percent": round((disk_usage.used / disk_usage.total) * 100, 2),
            }

            return stats

        except Exception as e:
            logger.error(f"Failed to get storage stats: {e}")
            return {"error": str(e)}


# Background cleanup task
async def background_cleanup_task(data_manager: DataManager):
    """Background task for regular data cleanup"""
    while True:
        try:
            # Run cleanup every hour
            await asyncio.sleep(3600)
            data_manager.cleanup_expired_data()
        except Exception as e:
            logger.error(f"Background cleanup error: {e}")
            await asyncio.sleep(600)  # Retry in 10 minutes
