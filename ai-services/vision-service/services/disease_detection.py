"""
Disease Detection Service using LLaVA model for plant leaf disease identification
"""

import logging
from typing import Dict, Any, Optional
import asyncio
import io

import torch
from PIL import Image
from transformers import (
    AutoProcessor,
    LlavaForConditionalGeneration,
    BitsAndBytesConfig,
)
import numpy as np

logger = logging.getLogger(__name__)


class DiseaseDetectionService:
    """
    Disease detection service using LLaVA model for plant leaf disease identification
    """

    def __init__(self):
        self.model: Optional[LlavaForConditionalGeneration] = None
        self.processor: Optional[AutoProcessor] = None
        self.model_id = "YuchengShi/LLaVA-v1.5-7B-Plant-Leaf-Diseases-Detection"
        self._model_loaded = False
        self._device = "cuda" if torch.cuda.is_available() else "cpu"

    async def initialize_model(self):
        """Initialize the LLaVA model asynchronously"""
        if self._model_loaded:
            return

        try:
            logger.info(f"Loading LLaVA disease detection model: {self.model_id}")
            logger.info(f"Using device: {self._device}")

            # Run model loading in thread pool to avoid blocking
            loop = asyncio.get_event_loop()

            # Load processor first
            self.processor = await loop.run_in_executor(
                None, AutoProcessor.from_pretrained, self.model_id
            )

            # Configure aggressive quantization for H100 16GB memory constraints
            quant_config = BitsAndBytesConfig(
                load_in_4bit=True,
                bnb_4bit_use_double_quant=True,
                bnb_4bit_compute_dtype=torch.float16,
                bnb_4bit_quant_type="nf4",
                llm_int8_enable_fp32_cpu_offload=True,  # Offload to CPU when needed
            )

            # Load model with aggressive memory optimization
            self.model = await loop.run_in_executor(
                None,
                lambda: LlavaForConditionalGeneration.from_pretrained(
                    self.model_id,
                    quantization_config=quant_config,
                    device_map="auto",
                    torch_dtype=torch.float16,  # Use FP16 to reduce memory
                    low_cpu_mem_usage=True,  # Reduce CPU memory during loading
                    max_memory={0: "12GB", "cpu": "4GB"},  # Limit GPU/CPU memory usage
                ),
            )

            self._model_loaded = True
            logger.info("LLaVA disease detection model loaded successfully")

        except Exception as e:
            logger.error(f"Failed to load LLaVA model: {e}")
            raise RuntimeError(f"Disease detection model initialization failed: {e}")

    async def detect_disease(
        self, image_bytes: bytes, custom_prompt: Optional[str] = None
    ) -> Dict[str, Any]:
        """
        Detect diseases in plant leaf images

        Parameters:
            image_bytes (bytes): Image data as bytes
            custom_prompt (str, optional): Custom prompt for analysis

        Returns:
            Dict[str, Any]: Disease detection results
        """
        if not self._model_loaded:
            await self.initialize_model()

        try:
            # Convert bytes to PIL Image
            image = Image.open(io.BytesIO(image_bytes))
            if image.mode != "RGB":
                image = image.convert("RGB")

            # Prepare conversation
            prompt_text = (
                custom_prompt
                or "What disease does this leaf have? Please provide a detailed analysis including the disease name, symptoms, severity, and treatment recommendations."
            )

            conversation = [
                {
                    "role": "user",
                    "content": [
                        {"type": "text", "text": prompt_text},
                        {"type": "image"},
                    ],
                },
            ]

            # Process conversation template
            prompt = self.processor.apply_chat_template(
                conversation, add_generation_prompt=True
            )

            # Run inference in thread pool
            loop = asyncio.get_event_loop()
            result = await loop.run_in_executor(
                None, self._run_inference, image, prompt
            )

            # Parse result
            response_text = result.strip()

            # Extract structured information (basic parsing)
            disease_info = self._parse_disease_response(response_text)

            logger.info(
                f"Disease detection completed: {disease_info.get('disease_name', 'Unknown')}"
            )

            return {
                "success": True,
                "raw_response": response_text,
                "disease_analysis": disease_info,
                "model": "LLaVA-v1.5-7B-Plant-Leaf-Diseases-Detection",
            }

        except Exception as e:
            logger.error(f"Disease detection failed: {e}")
            raise RuntimeError(f"Disease detection failed: {e}")

    def _run_inference(self, image: Image.Image, prompt: str) -> str:
        """Run LLaVA inference with aggressive memory optimization"""
        try:
            # Clear GPU cache before inference
            if torch.cuda.is_available():
                torch.cuda.empty_cache()

            # Resize image to reduce memory usage (max 512x512)
            max_size = 512
            if max(image.size) > max_size:
                ratio = max_size / max(image.size)
                new_size = tuple(int(dim * ratio) for dim in image.size)
                image = image.resize(new_size, Image.Resampling.LANCZOS)

            # Use context manager for memory cleanup
            with torch.no_grad():  # Disable gradient computation
                # Prepare inputs with memory optimization
                inputs = self.processor(
                    images=image,
                    text=prompt,
                    return_tensors="pt",
                    max_length=512,  # Limit input length
                    truncation=True,
                ).to(self.model.device, torch.float16)

                # Generate response with memory constraints
                output = self.model.generate(
                    **inputs,
                    max_new_tokens=150,  # Reduced from 200
                    do_sample=False,
                    temperature=0.1,
                    use_cache=True,
                    pad_token_id=self.processor.tokenizer.eos_token_id,
                )

                # Decode response
                response = self.processor.decode(
                    output[0][len(inputs["input_ids"][0]) :], skip_special_tokens=True
                )

                # Clean up tensors
                del inputs, output
                if torch.cuda.is_available():
                    torch.cuda.empty_cache()

            return response.strip()

        except Exception as e:
            logger.error(f"Inference failed: {e}")
            # Ensure cleanup on error
            if torch.cuda.is_available():
                torch.cuda.empty_cache()
            raise

    def _parse_disease_response(self, response_text: str) -> Dict[str, Any]:
        """Parse the disease detection response into structured data"""
        try:
            # Basic parsing - in production, you might want more sophisticated NLP
            response_lower = response_text.lower()

            # Extract disease name (basic keyword matching)
            diseases = [
                "leaf spot",
                "blight",
                "rust",
                "mosaic",
                "wilt",
                "mildew",
                "scorch",
                "rot",
                "canker",
                "anthracnose",
                "bacterial",
                "fungal",
                "viral",
                "healthy",
                "normal",
            ]

            detected_disease = "Unknown"
            for disease in diseases:
                if disease in response_lower:
                    detected_disease = disease.title()
                    break

            # Determine severity (basic keyword matching)
            severity = "Unknown"
            if any(
                word in response_lower for word in ["severe", "serious", "critical"]
            ):
                severity = "High"
            elif any(word in response_lower for word in ["moderate", "medium"]):
                severity = "Medium"
            elif any(word in response_lower for word in ["mild", "light", "early"]):
                severity = "Low"
            elif "healthy" in response_lower or "normal" in response_lower:
                severity = "None"
                detected_disease = "Healthy"

            # Generate Thai summary
            if detected_disease == "Healthy":
                thai_summary = "ใบพืชดูสุขภาพดี ไม่พบอาการของโรคพืช"
            else:
                thai_summary = f"พบอาการของโรคพืช: {detected_disease}"
                if severity != "Unknown":
                    severity_thai = {
                        "High": "รุนแรง",
                        "Medium": "ปานกลาง",
                        "Low": "เล็กน้อย",
                    }.get(severity, severity)
                    thai_summary += f" ระดับความรุนแรง: {severity_thai}"

            return {
                "disease_name": detected_disease,
                "severity": severity,
                "confidence": "High" if detected_disease != "Unknown" else "Low",
                "thai_summary": thai_summary,
                "recommendations": self._get_recommendations(
                    detected_disease, severity
                ),
                "is_healthy": detected_disease == "Healthy",
            }

        except Exception as e:
            logger.warning(f"Failed to parse disease response: {e}")
            return {
                "disease_name": "Unknown",
                "severity": "Unknown",
                "confidence": "Low",
                "thai_summary": "ไม่สามารถวิเคราะห์โรคพืชได้",
                "recommendations": ["ปรึกษาผู้เชี่ยวชาญด้านการเกษตร"],
                "is_healthy": False,
            }

    def _get_recommendations(self, disease: str, severity: str) -> list:
        """Get treatment recommendations based on disease and severity"""
        if disease == "Healthy":
            return ["ดูแลรักษาตามปกติ", "ตรวจสอบต้นพืชเป็นประจำ", "รดน้ำให้เพียงพอ"]

        recommendations = [
            "แยกต้นพืชที่เป็นโรคออกจากต้นอื่น",
            "ปรึกษาผู้เชี่ยวชาญด้านการเกษตร",
            "ใช้สารป้องกันกำจัดโรคตามคำแนะนำ",
        ]

        if severity == "High":
            recommendations.extend(["รีบดำเนินการรักษาทันที", "อาจต้องตัดใบที่เป็นโรคทิ้ง"])
        elif severity == "Low":
            recommendations.extend(["เฝ้าติดตามอาการอย่างใกล้ชิด", "ปรับปรุงการระบายอากาศ"])

        return recommendations

    async def health_check(self) -> Dict[str, Any]:
        """Check if the disease detection service is healthy"""
        try:
            return {
                "status": "healthy" if self._model_loaded else "loading",
                "model_loaded": self._model_loaded,
                "model_id": self.model_id,
                "device": self._device,
                "gpu_available": torch.cuda.is_available(),
            }
        except Exception as e:
            logger.error(f"Disease detection health check failed: {e}")
            return {"status": "unhealthy", "model_loaded": False, "error": str(e)}


# Global service instance
disease_detection_service = DiseaseDetectionService()


async def get_disease_detection_service() -> DiseaseDetectionService:
    """Get the global disease detection service instance"""
    return disease_detection_service
