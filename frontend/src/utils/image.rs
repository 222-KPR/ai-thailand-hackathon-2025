// Copyright (c) 2025 AI4Thai Crop Guardian
// Licensed under the MIT License

//! Image utility functions for file handling and validation

use web_sys::File;
use serde::{Deserialize, Serialize};

/// File preview information for UI display
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilePreviewInfo {
    pub name: String,
    pub size: String,
    pub type_: String,
    pub last_modified: u64,
}

/// Image validation error types
#[derive(Debug, Clone, PartialEq)]
pub enum ImageValidationError {
    FileTooLarge { size: u64, max_size: u64 },
    InvalidMimeType { mime_type: String },
    InvalidExtension { extension: String },
    EmptyFile,
    UnreadableFile,
}

impl std::fmt::Display for ImageValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageValidationError::FileTooLarge { size, max_size } => {
                write!(f, "File too large: {} exceeds maximum size of {}",
                       format_file_size(*size), format_file_size(*max_size))
            }
            ImageValidationError::InvalidMimeType { mime_type } => {
                write!(f, "Invalid file type: {}. Only JPG, PNG, and WebP images are supported", mime_type)
            }
            ImageValidationError::InvalidExtension { extension } => {
                write!(f, "Invalid file extension: .{}. Only .jpg, .jpeg, .png, .webp are allowed", extension)
            }
            ImageValidationError::EmptyFile => {
                write!(f, "File is empty")
            }
            ImageValidationError::UnreadableFile => {
                write!(f, "File cannot be read")
            }
        }
    }
}

/// Image upload configuration
pub struct ImageUploadConfig {
    pub max_size_bytes: u64,
    pub allowed_mime_types: Vec<String>,
    pub allowed_extensions: Vec<String>,
}

impl Default for ImageUploadConfig {
    fn default() -> Self {
        Self {
            max_size_bytes: 10 * 1024 * 1024, // 10MB
            allowed_mime_types: vec![
                "image/jpeg".to_string(),
                "image/jpg".to_string(),
                "image/png".to_string(),
                "image/webp".to_string(),
            ],
            allowed_extensions: vec![
                "jpg".to_string(),
                "jpeg".to_string(),
                "png".to_string(),
                "webp".to_string(),
            ],
        }
    }
}

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

/// Validate image file with comprehensive checks
pub fn validate_image_file(file: &File) -> Result<(), ImageValidationError> {
    validate_image_file_with_config(file, &ImageUploadConfig::default())
}

/// Validate image file with custom configuration
pub fn validate_image_file_with_config(file: &File, config: &ImageUploadConfig) -> Result<(), ImageValidationError> {
    // Check file size
    let size = file.size() as u64;
    if size == 0 {
        return Err(ImageValidationError::EmptyFile);
    }

    if size > config.max_size_bytes {
        return Err(ImageValidationError::FileTooLarge {
            size,
            max_size: config.max_size_bytes,
        });
    }

    // Check MIME type
    let mime_type = file.type_();
    if !config.allowed_mime_types.contains(&mime_type) {
        return Err(ImageValidationError::InvalidMimeType { mime_type });
    }

    // Check file extension
    let filename = file.name();
    if let Some(extension) = get_file_extension(&filename) {
        if !config.allowed_extensions.contains(&extension.to_lowercase()) {
            return Err(ImageValidationError::InvalidExtension { extension });
        }
    } else {
        return Err(ImageValidationError::InvalidExtension {
            extension: "none".to_string(),
        });
    }

    Ok(())
}

/// Get file preview information for UI display
pub fn get_file_preview_info(file: &File) -> FilePreviewInfo {
    FilePreviewInfo {
        name: file.name(),
        size: format_file_size(file.size() as u64),
        type_: file.type_(),
        last_modified: file.last_modified() as u64,
    }
}

/// Validate image file type (legacy function for compatibility)
pub fn is_valid_image_type(extension: &str) -> bool {
    let config = ImageUploadConfig::default();
    config.allowed_extensions.contains(&extension.to_lowercase())
}

/// Validate image file size (legacy function for compatibility)
pub fn is_valid_image_size(size: u64) -> bool {
    let config = ImageUploadConfig::default();
    size <= config.max_size_bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_extension() {
        assert_eq!(get_file_extension("image.jpg"), Some("jpg".to_string()));
        assert_eq!(get_file_extension("photo.JPEG"), Some("jpeg".to_string()));
        assert_eq!(get_file_extension("file.png"), Some("png".to_string()));
        assert_eq!(get_file_extension("document.webp"), Some("webp".to_string()));
        assert_eq!(get_file_extension("no_extension"), Some("no_extension".to_string()));
        assert_eq!(get_file_extension(""), Some("".to_string()));
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "0.0 B");
        assert_eq!(format_file_size(512), "512.0 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1536), "1.5 KB");
        assert_eq!(format_file_size(1048576), "1.0 MB");
        assert_eq!(format_file_size(1073741824), "1.0 GB");
    }

    #[test]
    fn test_is_valid_image_type() {
        assert!(is_valid_image_type("jpg"));
        assert!(is_valid_image_type("jpeg"));
        assert!(is_valid_image_type("png"));
        assert!(is_valid_image_type("webp"));
        assert!(is_valid_image_type("JPG"));
        assert!(!is_valid_image_type("gif"));
        assert!(!is_valid_image_type("bmp"));
        assert!(!is_valid_image_type("pdf"));
    }

    #[test]
    fn test_is_valid_image_size() {
        assert!(is_valid_image_size(1024)); // 1KB
        assert!(is_valid_image_size(5 * 1024 * 1024)); // 5MB
        assert!(is_valid_image_size(10 * 1024 * 1024)); // 10MB (max)
        assert!(!is_valid_image_size(11 * 1024 * 1024)); // 11MB (too large)
    }

    #[test]
    fn test_image_validation_error_display() {
        let error = ImageValidationError::FileTooLarge {
            size: 15 * 1024 * 1024,
            max_size: 10 * 1024 * 1024,
        };
        assert!(error.to_string().contains("File too large"));
        assert!(error.to_string().contains("15.0 MB"));
        assert!(error.to_string().contains("10.0 MB"));

        let error = ImageValidationError::InvalidMimeType {
            mime_type: "application/pdf".to_string(),
        };
        assert!(error.to_string().contains("Invalid file type"));
        assert!(error.to_string().contains("application/pdf"));

        let error = ImageValidationError::InvalidExtension {
            extension: "gif".to_string(),
        };
        assert!(error.to_string().contains("Invalid file extension"));
        assert!(error.to_string().contains(".gif"));

        let error = ImageValidationError::EmptyFile;
        assert_eq!(error.to_string(), "File is empty");

        let error = ImageValidationError::UnreadableFile;
        assert_eq!(error.to_string(), "File cannot be read");
    }
}
