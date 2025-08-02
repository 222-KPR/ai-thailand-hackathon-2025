// Copyright (c) 2025 AI4Thai Crop Guardian
// Licensed under the MIT License

//! Image utility functions for file handling and validation

/// Get file extension from filename
pub fn get_file_extension(filename: &str) -> Option<String> {
    if let Some(pos) = filename.rfind('.') {
        Some(filename[pos + 1..].to_lowercase())
    } else {
        Some(filename.to_string())
    }
}

/// Format file size in human readable format
pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.1} {}", size, UNITS[unit_index])
}

/// Validate image file type
pub fn is_valid_image_type(extension: &str) -> bool {
    matches!(extension.to_lowercase().as_str(), "jpg" | "jpeg" | "png" | "webp")
}

/// Validate image file size (max 10MB)
pub fn is_valid_image_size(size: u64) -> bool {
    size <= 10 * 1024 * 1024 // 10MB
}
