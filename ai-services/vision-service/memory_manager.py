"""
Memory Management for AI4Thai Vision Service
Handles GPU memory optimization and cleanup for H100 16GB constraints
"""

import gc
import logging
import asyncio
from typing import Optional
import torch

logger = logging.getLogger(__name__)

class MemoryManager:
    """
    Memory manager for optimizing GPU/CPU memory usage in vision models
    """

    def __init__(self):
        self.gpu_available = torch.cuda.is_available()
        self.device_count = torch.cuda.device_count() if self.gpu_available else 0

    def get_memory_stats(self) -> dict:
        """Get current memory usage statistics"""
        stats = {
            "gpu_available": self.gpu_available,
            "gpu_count": self.device_count
        }

        if self.gpu_available:
            for i in range(self.device_count):
                memory_allocated = torch.cuda.memory_allocated(i) / 1024**3  # GB
                memory_cached = torch.cuda.memory_reserved(i) / 1024**3  # GB
                stats[f"gpu_{i}"] = {
                    "allocated_gb": round(memory_allocated, 2),
                    "cached_gb": round(memory_cached, 2)
                }

        return stats

    def cleanup_gpu_memory(self):
        """Aggressive GPU memory cleanup"""
        if not self.gpu_available:
            return

        try:
            # Force garbage collection
            gc.collect()

            # Clear PyTorch cache
            torch.cuda.empty_cache()

            # Synchronize CUDA operations
            torch.cuda.synchronize()

            logger.info("GPU memory cleanup completed")

        except Exception as e:
            logger.warning(f"GPU memory cleanup failed: {e}")

    def cleanup_cpu_memory(self):
        """CPU memory cleanup"""
        try:
            # Force Python garbage collection
            collected = gc.collect()
            logger.info(f"CPU memory cleanup: {collected} objects collected")

        except Exception as e:
            logger.warning(f"CPU memory cleanup failed: {e}")

    def full_cleanup(self):
        """Complete memory cleanup (GPU + CPU)"""
        logger.info("Starting full memory cleanup...")
        self.cleanup_cpu_memory()
        self.cleanup_gpu_memory()
        logger.info("Full memory cleanup completed")

    async def periodic_cleanup(self, interval_seconds: int = 300):
        """Run periodic memory cleanup (every 5 minutes by default)"""
        while True:
            try:
                await asyncio.sleep(interval_seconds)
                self.full_cleanup()

                # Log memory stats after cleanup
                stats = self.get_memory_stats()
                logger.info(f"Memory stats after cleanup: {stats}")

            except Exception as e:
                logger.error(f"Periodic cleanup failed: {e}")

    def check_memory_pressure(self, threshold_gb: float = 12.0) -> bool:
        """Check if GPU memory usage is above threshold"""
        if not self.gpu_available:
            return False

        try:
            for i in range(self.device_count):
                allocated_gb = torch.cuda.memory_allocated(i) / 1024**3
                if allocated_gb > threshold_gb:
                    logger.warning(f"GPU {i} memory pressure: {allocated_gb:.2f}GB > {threshold_gb}GB")
                    return True
            return False

        except Exception as e:
            logger.error(f"Memory pressure check failed: {e}")
            return False

    def optimize_for_inference(self):
        """Optimize memory settings for inference"""
        if not self.gpu_available:
            return

        try:
            # Set memory fraction to prevent OOM
            torch.cuda.set_per_process_memory_fraction(0.9)  # Use 90% of available GPU memory

            # Enable memory-efficient attention if available
            torch.backends.cuda.enable_flash_sdp(True)

            logger.info("Memory optimization for inference applied")

        except Exception as e:
            logger.warning(f"Memory optimization failed: {e}")

# Global memory manager instance
memory_manager = MemoryManager()

def get_memory_manager() -> MemoryManager:
    """Get the global memory manager instance"""
    return memory_manager
