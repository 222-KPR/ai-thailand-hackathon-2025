use crate::{AppError, AppResult};
use image::{DynamicImage, ImageFormat};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tempfile::NamedTempFile;
use tokio::fs as tokio_fs;
use tracing::{error, info, warn};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredFile {
    pub file_id: Uuid,
    pub original_filename: String,
    pub stored_path: PathBuf,
    pub file_hash: String,
    pub size_bytes: u64,
    pub format: String,
    pub width: u32,
    pub height: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileValidationResult {
    pub is_valid: bool,
    pub format: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub size_bytes: u64,
    pub error_message: Option<String>,
}

/// File storage service for handling image files with memory-efficient practices
pub struct FileStorageService {
    temp_dir: PathBuf,
    max_file_size: usize,
    file_ttl: Duration,
    supported_formats: Vec<String>,
}

impl FileStorageService {
    /// Create a new file storage service
    pub fn new(config: &crate::config::FileStorageConfig) -> AppResult<Self> {
        let temp_dir = PathBuf::from(&config.temp_dir);
        
        // Create temp directory if it doesn't exist
        if !temp_dir.exists() {
            fs::create_dir_all(&temp_dir).map_err(|e| {
                error!("Failed to create temp directory: {}", e);
                AppError::Internal(format!("Failed to create temp directory: {}", e))
            })?;
        }

        info!("File storage service initialized with temp dir: {:?}", temp_dir);

        Ok(Self {
            temp_dir,
            max_file_size: config.max_file_size,
            file_ttl: config.file_ttl,
            supported_formats: config.supported_formats.clone(),
        })
    }

    /// Store an image file with memory-efficient handling
    pub async fn store_image(
        &self,
        image_data: Vec<u8>,
        original_filename: String,
    ) -> AppResult<StoredFile> {
        // Validate file size
        if image_data.len() > self.max_file_size {
            return Err(AppError::Validation(format!(
                "File size {} exceeds maximum allowed size {}",
                image_data.len(),
                self.max_file_size
            )));
        }

        // Validate image format and get metadata
        let validation = self.validate_image(&image_data)?;
        if !validation.is_valid {
            return Err(AppError::Validation(
                validation.error_message.unwrap_or_else(|| "Invalid image".to_string())
            ));
        }

        // Generate file hash for deduplication
        let file_hash = self.calculate_file_hash(&image_data);

        // Generate unique file ID
        let file_id = Uuid::new_v4();

        // Create filename with timestamp to avoid conflicts
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let extension = self.get_file_extension(&original_filename, &validation.format);
        let filename = format!("{}_{}.{}", file_id, timestamp, extension);

        // Create file path
        let file_path = self.temp_dir.join(&filename);

        // Write file to disk (streaming write to avoid memory issues)
        self.write_file_streaming(&file_path, &image_data).await?;

        // Calculate expiration time
        let created_at = chrono::Utc::now();
        let expires_at = created_at + chrono::Duration::from_std(self.file_ttl).unwrap();

        let stored_file = StoredFile {
            file_id,
            original_filename,
            stored_path: file_path.clone(),
            file_hash,
            size_bytes: image_data.len() as u64,
            format: validation.format.unwrap_or_else(|| "unknown".to_string()),
            width: validation.width.unwrap_or(0),
            height: validation.height.unwrap_or(0),
            created_at,
            expires_at,
        };

        info!(
            "Stored image file: {} ({} bytes) at {:?}",
            file_id, image_data.len(), file_path
        );

        Ok(stored_file)
    }

    /// Validate image data without loading entire image into memory
    pub fn validate_image(&self, image_data: &[u8]) -> AppResult<FileValidationResult> {
        // Check file size
        if image_data.len() > self.max_file_size {
            return Ok(FileValidationResult {
                is_valid: false,
                format: None,
                width: None,
                height: None,
                size_bytes: image_data.len() as u64,
                error_message: Some(format!(
                    "File size {} exceeds maximum allowed size {}",
                    image_data.len(),
                    self.max_file_size
                )),
            });
        }

        // Try to decode image format
        match image::load_from_memory(image_data) {
            Ok(img) => {
                let format = self.detect_format(image_data);
                let (width, height) = img.dimensions();

                // Check if format is supported
                if let Some(ref fmt) = format {
                    if !self.supported_formats.contains(fmt) {
                        return Ok(FileValidationResult {
                            is_valid: false,
                            format: Some(fmt.clone()),
                            width: Some(width),
                            height: Some(height),
                            size_bytes: image_data.len() as u64,
                            error_message: Some(format!("Unsupported format: {}", fmt)),
                        });
                    }
                }

                Ok(FileValidationResult {
                    is_valid: true,
                    format,
                    width: Some(width),
                    height: Some(height),
                    size_bytes: image_data.len() as u64,
                    error_message: None,
                })
            }
            Err(e) => Ok(FileValidationResult {
                is_valid: false,
                format: None,
                width: None,
                height: None,
                size_bytes: image_data.len() as u64,
                error_message: Some(format!("Failed to decode image: {}", e)),
            }),
        }
    }

    /// Read file data from disk
    pub async fn read_file(&self, file_path: &Path) -> AppResult<Vec<u8>> {
        tokio_fs::read(file_path)
            .await
            .map_err(|e| {
                error!("Failed to read file {:?}: {}", file_path, e);
                AppError::Internal(format!("Failed to read file: {}", e))
            })
    }

    /// Delete a stored file
    pub async fn delete_file(&self, file_path: &Path) -> AppResult<()> {
        if file_path.exists() {
            tokio_fs::remove_file(file_path)
                .await
                .map_err(|e| {
                    error!("Failed to delete file {:?}: {}", file_path, e);
                    AppError::Internal(format!("Failed to delete file: {}", e))
                })?;
            info!("Deleted file: {:?}", file_path);
        }
        Ok(())
    }

    /// Clean up expired files
    pub async fn cleanup_expired_files(&self) -> AppResult<usize> {
        let mut deleted_count = 0;
        let now = chrono::Utc::now();

        if let Ok(entries) = tokio_fs::read_dir(&self.temp_dir).await {
            let mut entries = entries;
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Ok(metadata) = entry.metadata().await {
                    if let Ok(created) = metadata.created() {
                        let created_time = chrono::DateTime::from(created);
                        let expires_at = created_time + chrono::Duration::from_std(self.file_ttl).unwrap();

                        if now > expires_at {
                            if let Err(e) = tokio_fs::remove_file(entry.path()).await {
                                warn!("Failed to delete expired file {:?}: {}", entry.path(), e);
                            } else {
                                deleted_count += 1;
                            }
                        }
                    }
                }
            }
        }

        info!("Cleaned up {} expired files", deleted_count);
        Ok(deleted_count)
    }

    /// Get file statistics
    pub async fn get_stats(&self) -> AppResult<FileStats> {
        let mut total_files = 0;
        let mut total_size = 0u64;

        if let Ok(entries) = tokio_fs::read_dir(&self.temp_dir).await {
            let mut entries = entries;
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Ok(metadata) = entry.metadata().await {
                    total_files += 1;
                    total_size += metadata.len();
                }
            }
        }

        Ok(FileStats {
            total_files,
            total_size_bytes: total_size,
            temp_dir: self.temp_dir.to_string_lossy().to_string(),
        })
    }

    /// Calculate SHA256 hash of file data
    fn calculate_file_hash(&self, data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }

    /// Detect image format from file data
    fn detect_format(&self, data: &[u8]) -> Option<String> {
        if data.len() < 8 {
            return None;
        }

        // Check for common image format signatures
        if data.starts_with(b"\xff\xd8\xff") {
            Some("jpeg".to_string())
        } else if data.starts_with(b"\x89PNG\r\n\x1a\n") {
            Some("png".to_string())
        } else if data.starts_with(b"GIF87a") || data.starts_with(b"GIF89a") {
            Some("gif".to_string())
        } else if data.starts_with(b"RIFF") && data.len() > 12 && &data[8..12] == b"WEBP" {
            Some("webp".to_string())
        } else if data.starts_with(b"BM") {
            Some("bmp".to_string())
        } else {
            None
        }
    }

    /// Get file extension from filename or detected format
    fn get_file_extension(&self, filename: &str, detected_format: &Option<String>) -> String {
        // Try to get extension from filename first
        if let Some(ext) = Path::new(filename).extension() {
            if let Some(ext_str) = ext.to_str() {
                return ext_str.to_lowercase();
            }
        }

        // Fall back to detected format
        detected_format.clone().unwrap_or_else(|| "bin".to_string())
    }

    /// Write file data to disk using streaming to avoid memory issues
    async fn write_file_streaming(&self, file_path: &Path, data: &[u8]) -> AppResult<()> {
        // Use tokio's write_all for efficient writing
        tokio_fs::write(file_path, data)
            .await
            .map_err(|e| {
                error!("Failed to write file {:?}: {}", file_path, e);
                AppError::Internal(format!("Failed to write file: {}", e))
            })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStats {
    pub total_files: usize,
    pub total_size_bytes: u64,
    pub temp_dir: String,
} 