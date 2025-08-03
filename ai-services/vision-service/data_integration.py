"""
Data Management Integration for Vision Service
"""

import os
import sys


sys.path.append(os.path.join(os.path.dirname(__file__), "..", "shared"))

import logging

from data_manager import DataManager
from fastapi import BackgroundTasks


logger = logging.getLogger(__name__)

# Initialize data manager
data_manager = DataManager("/app/data/vision-service")


async def store_request_data(
    image_data: bytes, filename: str, request_metadata: dict
) -> str:
    """Store incoming image with metadata"""
    try:
        metadata = {
            "original_filename": filename,
            "content_type": request_metadata.get("content_type", "unknown"),
            "file_size": len(image_data),
            "request_timestamp": request_metadata.get("timestamp"),
            "user_agent": request_metadata.get("user_agent", "unknown"),
        }

        return data_manager.store_image(image_data, filename, metadata)
    except Exception as e:
        logger.error(f"Failed to store request data: {e}")
        raise


async def store_analysis_result(
    result_data: dict, analysis_type: str, image_id: str
) -> str:
    """Store analysis result with linking to original image"""
    try:
        return data_manager.store_result(result_data, analysis_type, image_id)
    except Exception as e:
        logger.error(f"Failed to store analysis result: {e}")
        raise


def cleanup_old_data_background(background_tasks: BackgroundTasks):
    """Schedule background cleanup task"""
    background_tasks.add_task(data_manager.cleanup_expired_data)


def get_storage_metrics() -> dict:
    """Get current storage metrics"""
    return data_manager.get_storage_stats()
