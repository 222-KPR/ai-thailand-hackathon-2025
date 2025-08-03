"""
Pest Detection Service using YOLO11s model from Hugging Face
"""

import logging
from typing import Set, Dict, Any, Optional
from pathlib import Path
import asyncio
from functools import lru_cache

from ultralytics import YOLO
import cv2
import numpy as np
from PIL import Image

logger = logging.getLogger(__name__)


class PestDetectionService:
    """
    Pest detection service using YOLO11s model for agricultural pest identification
    """

    def __init__(self):
        self.model: Optional[YOLO] = None
        self.model_path = "https://huggingface.co/underdogquality/yolo11s-pest-detection/resolve/main/best.pt"
        self._model_loaded = False

    async def initialize_model(self):
        """Initialize the YOLO model asynchronously"""
        if self._model_loaded:
            return

        try:
            logger.info(f"Loading YOLO pest detection model from: {self.model_path}")
            # Run model loading in thread pool to avoid blocking
            loop = asyncio.get_event_loop()
            self.model = await loop.run_in_executor(None, YOLO, self.model_path)
            self._model_loaded = True
            logger.info("YOLO pest detection model loaded successfully")
        except Exception as e:
            logger.error(f"Failed to load YOLO model: {e}")
            raise RuntimeError(f"Model initialization failed: {e}")

    async def detect_pests(
        self,
        image_path: str,
        conf_threshold: float = 0.01,
        return_details: bool = False,
    ) -> Dict[str, Any]:
        """
        Detect pests in an image

        Parameters:
            image_path (str): Path to the image file
            conf_threshold (float): Confidence threshold for detection
            return_details (bool): Whether to return detailed detection info

        Returns:
            Dict[str, Any]: Detection results with pests found and optional details
        """
        if not self._model_loaded:
            await self.initialize_model()

        try:
            # Run inference in thread pool
            loop = asyncio.get_event_loop()
            results = await loop.run_in_executor(
                None, self._run_inference, image_path, conf_threshold
            )

            detected_pests = set()
            detection_details = []

            for result in results:
                if result.boxes is not None:
                    for box in result.boxes:
                        cls_id = int(box.cls[0].item())
                        confidence = float(box.conf[0].item())
                        class_name = self.model.names.get(cls_id, f"Class {cls_id}")

                        detected_pests.add(class_name)

                        if return_details:
                            # Get bounding box coordinates
                            x1, y1, x2, y2 = box.xyxy[0].tolist()
                            detection_details.append(
                                {
                                    "class_name": class_name,
                                    "confidence": confidence,
                                    "bbox": {"x1": x1, "y1": y1, "x2": x2, "y2": y2},
                                }
                            )

            # Prepare response
            response = {
                "detected_pests": list(detected_pests),
                "pest_count": len(detected_pests),
                "has_pests": len(detected_pests) > 0,
            }

            # Add Thai language summary
            if detected_pests:
                thai_summary = "แมลงศัตรูพืชที่ตรวจพบ: " + ", ".join(detected_pests)
            else:
                thai_summary = "ไม่พบแมลงศัตรูพืชในภาพนี้"

            response["thai_summary"] = thai_summary

            if return_details:
                response["detection_details"] = detection_details

            logger.info(f"Pest detection completed: {len(detected_pests)} pests found")
            return response

        except Exception as e:
            logger.error(f"Pest detection failed: {e}")
            raise RuntimeError(f"Pest detection failed: {e}")

    def _run_inference(self, image_path: str, conf_threshold: float):
        """Run YOLO inference (blocking operation)"""
        return self.model(image_path, conf=conf_threshold)

    async def detect_pests_from_bytes(
        self,
        image_bytes: bytes,
        conf_threshold: float = 0.01,
        return_details: bool = False,
    ) -> Dict[str, Any]:
        """
        Detect pests from image bytes

        Parameters:
            image_bytes (bytes): Image data as bytes
            conf_threshold (float): Confidence threshold for detection
            return_details (bool): Whether to return detailed detection info

        Returns:
            Dict[str, Any]: Detection results
        """
        if not self._model_loaded:
            await self.initialize_model()

        try:
            # Convert bytes to numpy array
            nparr = np.frombuffer(image_bytes, np.uint8)
            image = cv2.imdecode(nparr, cv2.IMREAD_COLOR)

            if image is None:
                raise ValueError("Invalid image data")

            # Run inference in thread pool
            loop = asyncio.get_event_loop()
            results = await loop.run_in_executor(
                None, self.model, image, conf_threshold
            )

            detected_pests = set()
            detection_details = []

            for result in results:
                if result.boxes is not None:
                    for box in result.boxes:
                        cls_id = int(box.cls[0].item())
                        confidence = float(box.conf[0].item())
                        class_name = self.model.names.get(cls_id, f"Class {cls_id}")

                        detected_pests.add(class_name)

                        if return_details:
                            x1, y1, x2, y2 = box.xyxy[0].tolist()
                            detection_details.append(
                                {
                                    "class_name": class_name,
                                    "confidence": confidence,
                                    "bbox": {"x1": x1, "y1": y1, "x2": x2, "y2": y2},
                                }
                            )

            # Prepare response
            response = {
                "detected_pests": list(detected_pests),
                "pest_count": len(detected_pests),
                "has_pests": len(detected_pests) > 0,
            }

            # Add Thai language summary
            if detected_pests:
                thai_summary = "แมลงศัตรูพืชที่ตรวจพบ: " + ", ".join(detected_pests)
            else:
                thai_summary = "ไม่พบแมลงศัตรูพืชในภาพนี้"

            response["thai_summary"] = thai_summary

            if return_details:
                response["detection_details"] = detection_details

            logger.info(
                f"Pest detection from bytes completed: {len(detected_pests)} pests found"
            )
            return response

        except Exception as e:
            logger.error(f"Pest detection from bytes failed: {e}")
            raise RuntimeError(f"Pest detection from bytes failed: {e}")

    async def health_check(self) -> Dict[str, Any]:
        """Check if the pest detection service is healthy"""
        try:
            if not self._model_loaded:
                await self.initialize_model()

            return {
                "status": "healthy",
                "model_loaded": self._model_loaded,
                "model_path": self.model_path,
                "available_classes": len(self.model.names) if self.model else 0,
            }
        except Exception as e:
            logger.error(f"Health check failed: {e}")
            return {"status": "unhealthy", "model_loaded": False, "error": str(e)}


# Global service instance
pest_detection_service = PestDetectionService()


async def get_pest_detection_service() -> PestDetectionService:
    """Get the global pest detection service instance"""
    return pest_detection_service
