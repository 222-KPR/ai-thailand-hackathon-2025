use crate::{
    ApiResponse, AppResult, AppState, AppError,
    services::{rabbitmq::*, file_storage::*},
};
use axum::{
    extract::{Multipart, State},
    response::Json,
};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct VisionAnalysisResponse {
    pub job_id: Uuid,
    pub status: String,
    pub message: String,
    pub estimated_processing_time: u64, // seconds
}

#[derive(Debug, Serialize)]
pub struct JobStatusResponse {
    pub job_id: Uuid,
    pub status: String,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
    pub progress: Option<serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AnalysisParameters {
    pub confidence_threshold: Option<f32>,
    pub return_details: Option<bool>,
    pub custom_prompt: Option<String>,
}

/// Queue vision analysis job with memory-efficient file handling
pub async fn queue_vision_analysis(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> AppResult<Json<ApiResponse<VisionAnalysisResponse>>> {
    let start_time = std::time::Instant::now();
    let job_id = Uuid::new_v4();

    info!("Starting vision analysis job: {}", job_id);

    // Extract multipart data
    let mut image_data: Option<Vec<u8>> = None;
    let mut original_filename: Option<String> = None;
    let mut analysis_type: Option<String> = None;
    let mut confidence_threshold: Option<f32> = None;
    let mut return_details: Option<bool> = None;
    let mut custom_prompt: Option<String> = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        error!("Failed to read multipart field: {}", e);
        AppError::Validation(format!("Failed to read multipart data: {}", e))
    })? {
        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "image" => {
                // Read image data in chunks to avoid memory issues
                let mut data = Vec::new();
                while let Some(chunk) = field.chunk().await.map_err(|e| {
                    error!("Failed to read image chunk: {}", e);
                    AppError::Validation(format!("Failed to read image data: {}", e))
                })? {
                    data.extend_from_slice(&chunk);
                    
                    // Check size limit during streaming
                    if data.len() > state.config.file_storage.max_file_size {
                        return Err(AppError::Validation(format!(
                            "Image size exceeds maximum allowed size of {} bytes",
                            state.config.file_storage.max_file_size
                        )));
                    }
                }
                image_data = Some(data);
            }
            "filename" => {
                original_filename = Some(
                    field.text().await.map_err(|e| {
                        error!("Failed to read filename: {}", e);
                        AppError::Validation(format!("Failed to read filename: {}", e))
                    })?
                );
            }
            "analysis_type" => {
                analysis_type = Some(
                    field.text().await.map_err(|e| {
                        error!("Failed to read analysis type: {}", e);
                        AppError::Validation(format!("Failed to read analysis type: {}", e))
                    })?
                );
            }
            "confidence_threshold" => {
                let threshold_str = field.text().await.map_err(|e| {
                    error!("Failed to read confidence threshold: {}", e);
                    AppError::Validation(format!("Failed to read confidence threshold: {}", e))
                })?;
                confidence_threshold = threshold_str.parse().ok();
            }
            "return_details" => {
                let details_str = field.text().await.map_err(|e| {
                    error!("Failed to read return details: {}", e);
                    AppError::Validation(format!("Failed to read return details: {}", e))
                })?;
                return_details = Some(details_str.parse().unwrap_or(false));
            }
            "custom_prompt" => {
                custom_prompt = Some(
                    field.text().await.map_err(|e| {
                        error!("Failed to read custom prompt: {}", e);
                        AppError::Validation(format!("Failed to read custom prompt: {}", e))
                    })?
                );
            }
            _ => {
                warn!("Unknown multipart field: {}", field_name);
            }
        }
    }

    // Validate required fields
    let image_data = image_data.ok_or_else(|| {
        AppError::Validation("Image file is required".to_string())
    })?;

    let original_filename = original_filename.unwrap_or_else(|| "unknown.jpg".to_string());
    let analysis_type = analysis_type.unwrap_or_else(|| "comprehensive".to_string());

    // Store image file to disk (memory-efficient)
    let stored_file = state.file_storage_service.store_image(image_data, original_filename).await?;

    // Create analysis parameters
    let parameters = AnalysisParameters {
        confidence_threshold,
        return_details,
        custom_prompt,
    };

    // Convert analysis type string to enum
    let analysis_type_enum = match analysis_type.to_lowercase().as_str() {
        "pest" | "pest_detection" => AnalysisType::PestDetection,
        "disease" | "disease_detection" => AnalysisType::DiseaseDetection,
        "comprehensive" | "full" => AnalysisType::Comprehensive,
        _ => {
            warn!("Unknown analysis type: {}, defaulting to comprehensive", analysis_type);
            AnalysisType::Comprehensive
        }
    };

    // Create RabbitMQ message
    let message = VisionAnalysisMessage {
        job_id,
        analysis_type: analysis_type_enum,
        file_path: stored_file.stored_path.to_string_lossy().to_string(),
        file_hash: stored_file.file_hash,
        metadata: ImageMetadata {
            size_bytes: stored_file.size_bytes,
            width: stored_file.width,
            height: stored_file.height,
            format: stored_file.format,
            original_filename: stored_file.original_filename,
        },
        parameters: rabbitmq::AnalysisParameters {
            confidence_threshold: parameters.confidence_threshold,
            return_details: parameters.return_details,
            custom_prompt: parameters.custom_prompt,
        },
        timestamp: chrono::Utc::now(),
    };

    // Publish message to RabbitMQ
    state.rabbitmq_service.publish_vision_analysis(message).await?;

    // Store job status in Redis for tracking
    let job_status = JobStatus {
        job_id,
        status: JobStatusType::Queued,
        result: None,
        error: None,
        progress: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let mut redis_conn = state.get_redis().await.map_err(|e| {
        error!("Failed to get Redis connection: {}", e);
        AppError::Redis(e)
    })?;

    let status_key = format!("job:{}", job_id);
    let status_json = serde_json::to_string(&job_status).map_err(|e| {
        error!("Failed to serialize job status: {}", e);
        AppError::Internal(format!("Failed to serialize job status: {}", e))
    })?;

    redis::cmd("SETEX")
        .arg(&status_key)
        .arg(86400) // 24 hours TTL
        .arg(&status_json)
        .execute_async(&mut redis_conn)
        .await
        .map_err(|e| {
            error!("Failed to store job status in Redis: {}", e);
            AppError::Redis(e)
        })?;

    let processing_time = start_time.elapsed();
    info!(
        "Vision analysis job queued successfully: {} (took {:?})",
        job_id, processing_time
    );

    // Estimate processing time based on analysis type
    let estimated_processing_time = match analysis_type_enum {
        AnalysisType::PestDetection => 30,      // 30 seconds
        AnalysisType::DiseaseDetection => 60,   // 1 minute
        AnalysisType::Comprehensive => 90,      // 1.5 minutes
    };

    let response = VisionAnalysisResponse {
        job_id,
        status: "queued".to_string(),
        message: "Vision analysis job has been queued successfully".to_string(),
        estimated_processing_time,
    };

    Ok(Json(ApiResponse::success_with_message(
        response,
        format!("Job queued successfully. Estimated processing time: {} seconds", estimated_processing_time)
    )))
}

/// Get job status from Redis
pub async fn get_job_status(
    State(state): State<AppState>,
    axum::extract::Path(job_id): axum::extract::Path<Uuid>,
) -> AppResult<Json<ApiResponse<JobStatusResponse>>> {
    info!("Getting job status: {}", job_id);

    let mut redis_conn = state.get_redis().await.map_err(|e| {
        error!("Failed to get Redis connection: {}", e);
        AppError::Redis(e)
    })?;

    let status_key = format!("job:{}", job_id);
    let status_json: Option<String> = redis::cmd("GET")
        .arg(&status_key)
        .query_async(&mut redis_conn)
        .await
        .map_err(|e| {
            error!("Failed to get job status from Redis: {}", e);
            AppError::Redis(e)
        })?;

    match status_json {
        Some(json) => {
            let job_status: JobStatus = serde_json::from_str(&json).map_err(|e| {
                error!("Failed to deserialize job status: {}", e);
                AppError::Internal(format!("Failed to deserialize job status: {}", e))
            })?;

            let response = JobStatusResponse {
                job_id: job_status.job_id,
                status: match job_status.status {
                    JobStatusType::Queued => "queued".to_string(),
                    JobStatusType::Processing => "processing".to_string(),
                    JobStatusType::Completed => "completed".to_string(),
                    JobStatusType::Failed => "failed".to_string(),
                    JobStatusType::Cancelled => "cancelled".to_string(),
                },
                result: job_status.result,
                error: job_status.error,
                progress: job_status.progress,
                created_at: job_status.created_at,
                updated_at: job_status.updated_at,
            };

            Ok(Json(ApiResponse::success(response)))
        }
        None => {
            Err(AppError::Validation(format!("Job not found: {}", job_id)))
        }
    }
}

/// Cancel a job
pub async fn cancel_job(
    State(state): State<AppState>,
    axum::extract::Path(job_id): axum::extract::Path<Uuid>,
) -> AppResult<Json<ApiResponse<serde_json::Value>>> {
    info!("Cancelling job: {}", job_id);

    let mut redis_conn = state.get_redis().await.map_err(|e| {
        error!("Failed to get Redis connection: {}", e);
        AppError::Redis(e)
    })?;

    let status_key = format!("job:{}", job_id);
    let status_json: Option<String> = redis::cmd("GET")
        .arg(&status_key)
        .query_async(&mut redis_conn)
        .await
        .map_err(|e| {
            error!("Failed to get job status from Redis: {}", e);
            AppError::Redis(e)
        })?;

    match status_json {
        Some(json) => {
            let mut job_status: JobStatus = serde_json::from_str(&json).map_err(|e| {
                error!("Failed to deserialize job status: {}", e);
                AppError::Internal(format!("Failed to deserialize job status: {}", e))
            })?;

            // Only allow cancellation of queued jobs
            match job_status.status {
                JobStatusType::Queued => {
                    job_status.status = JobStatusType::Cancelled;
                    job_status.updated_at = chrono::Utc::now();

                    let updated_json = serde_json::to_string(&job_status).map_err(|e| {
                        error!("Failed to serialize updated job status: {}", e);
                        AppError::Internal(format!("Failed to serialize job status: {}", e))
                    })?;

                    redis::cmd("SETEX")
                        .arg(&status_key)
                        .arg(86400) // 24 hours TTL
                        .arg(&updated_json)
                        .execute_async(&mut redis_conn)
                        .await
                        .map_err(|e| {
                            error!("Failed to update job status in Redis: {}", e);
                            AppError::Redis(e)
                        })?;

                    info!("Job cancelled successfully: {}", job_id);

                    Ok(Json(ApiResponse::success_with_message(
                        serde_json::json!({ "job_id": job_id, "status": "cancelled" }),
                        "Job cancelled successfully"
                    )))
                }
                _ => {
                    Err(AppError::Validation(format!(
                        "Cannot cancel job in status: {:?}",
                        job_status.status
                    )))
                }
            }
        }
        None => {
            Err(AppError::Validation(format!("Job not found: {}", job_id)))
        }
    }
}

/// Get file storage statistics
pub async fn get_file_stats(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<FileStats>>> {
    info!("Getting file storage statistics");

    let stats = state.file_storage_service.get_stats().await?;

    Ok(Json(ApiResponse::success(stats)))
}

/// Clean up expired files
pub async fn cleanup_files(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<serde_json::Value>>> {
    info!("Cleaning up expired files");

    let deleted_count = state.file_storage_service.cleanup_expired_files().await?;

    Ok(Json(ApiResponse::success_with_message(
        serde_json::json!({ "deleted_files": deleted_count }),
        format!("Cleaned up {} expired files", deleted_count)
    )))
} 