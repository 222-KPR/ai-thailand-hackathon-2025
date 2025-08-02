// Camera components module for AI4Thai Crop Guardian - 2025 Design System
// Exports all camera-related components

pub mod camera_capture;

pub use camera_capture::{
    CameraCapture, CameraCaptureProps, CameraState, CameraFacing,
    generate_camera_css
};

// Re-export commonly used types
pub use camera_capture::{CameraState as CamState, CameraFacing as CamFacing};
