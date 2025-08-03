use crate::{AppError, AppResult};
use lapin::{
    options::BasicPublishOptions, BasicProperties, Channel, Connection, ConnectionProperties,
    types::FieldTable,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{error, info, warn};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionAnalysisMessage {
    pub job_id: Uuid,
    pub analysis_type: AnalysisType,
    pub file_path: String,
    pub file_hash: String,
    pub metadata: ImageMetadata,
    pub parameters: AnalysisParameters,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalysisType {
    PestDetection,
    DiseaseDetection,
    Comprehensive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageMetadata {
    pub size_bytes: u64,
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub original_filename: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisParameters {
    pub confidence_threshold: Option<f32>,
    pub return_details: Option<bool>,
    pub custom_prompt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobStatus {
    pub job_id: Uuid,
    pub status: JobStatusType,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
    pub progress: Option<serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobStatusType {
    Queued,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

/// RabbitMQ service for handling message queuing
pub struct RabbitMQService {
    connection: Connection,
    channel: Channel,
    queue_name: String,
    exchange_name: String,
    routing_key: String,
}

impl RabbitMQService {
    /// Create a new RabbitMQ service instance
    pub async fn new(config: &crate::config::RabbitMQConfig) -> AppResult<Self> {
        info!("Connecting to RabbitMQ at {}", config.url);

        // Create connection
        let connection = Connection::connect(
            &config.url,
            ConnectionProperties::default()
                .with_connection_name("api-gateway".into())
                .with_executor(tokio::runtime::Handle::current())
                .with_reactor(tokio::runtime::Handle::current()),
        )
        .await
        .map_err(|e| {
            error!("Failed to connect to RabbitMQ: {}", e);
            AppError::Service(ServiceError::ConnectionFailed(e.to_string()))
        })?;

        info!("Connected to RabbitMQ successfully");

        // Create channel
        let channel = connection
            .create_channel()
            .await
            .map_err(|e| {
                error!("Failed to create RabbitMQ channel: {}", e);
                AppError::Service(ServiceError::ChannelCreationFailed(e.to_string()))
            })?;

        // Set QoS
        channel
            .basic_qos(config.prefetch_count, lapin::options::BasicQosOptions::default())
            .await
            .map_err(|e| {
                error!("Failed to set QoS: {}", e);
                AppError::Service(ServiceError::QosSettingFailed(e.to_string()))
            })?;

        // Declare exchange
        channel
            .exchange_declare(
                &config.exchange_name,
                lapin::ExchangeKind::Direct,
                lapin::options::ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| {
                error!("Failed to declare exchange: {}", e);
                AppError::Service(ServiceError::ExchangeDeclarationFailed(e.to_string()))
            })?;

        // Declare queue
        channel
            .queue_declare(
                &config.queue_name,
                lapin::options::QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| {
                error!("Failed to declare queue: {}", e);
                AppError::Service(ServiceError::QueueDeclarationFailed(e.to_string()))
            })?;

        // Bind queue to exchange
        channel
            .queue_bind(
                &config.queue_name,
                &config.exchange_name,
                &config.routing_key,
                FieldTable::default(),
            )
            .await
            .map_err(|e| {
                error!("Failed to bind queue: {}", e);
                AppError::Service(ServiceError::QueueBindingFailed(e.to_string()))
            })?;

        info!("RabbitMQ service initialized successfully");

        Ok(Self {
            connection,
            channel,
            queue_name: config.queue_name.clone(),
            exchange_name: config.exchange_name.clone(),
            routing_key: config.routing_key.clone(),
        })
    }

    /// Publish a vision analysis message to the queue
    pub async fn publish_vision_analysis(&self, message: VisionAnalysisMessage) -> AppResult<()> {
        let payload = serde_json::to_vec(&message).map_err(|e| {
            error!("Failed to serialize message: {}", e);
            AppError::Service(ServiceError::SerializationFailed(e.to_string()))
        })?;

        let properties = BasicProperties::default()
            .with_message_id(lapin::types::ShortString::from(message.job_id.to_string()))
            .with_timestamp(lapin::types::Timestamp::from(
                message.timestamp.timestamp() as u64
            ));

        self.channel
            .basic_publish(
                &self.exchange_name,
                &self.routing_key,
                BasicPublishOptions::default(),
                &payload,
                properties,
            )
            .await
            .map_err(|e| {
                error!("Failed to publish message: {}", e);
                AppError::Service(ServiceError::PublishFailed(e.to_string()))
            })?;

        info!("Published vision analysis message: {}", message.job_id);
        Ok(())
    }

    /// Check if the service is healthy
    pub async fn health_check(&self) -> bool {
        match self.connection.status() {
            lapin::ConnectionState::Connected => {
                info!("RabbitMQ connection is healthy");
                true
            }
            _ => {
                warn!("RabbitMQ connection is not healthy");
                false
            }
        }
    }

    /// Get connection status
    pub fn connection_status(&self) -> lapin::ConnectionState {
        self.connection.status()
    }

    /// Close the connection
    pub async fn close(self) -> AppResult<()> {
        info!("Closing RabbitMQ connection");
        self.connection.close(0, "shutdown").await.map_err(|e| {
            error!("Failed to close RabbitMQ connection: {}", e);
            AppError::Service(ServiceError::ConnectionCloseFailed(e.to_string()))
        })?;
        Ok(())
    }
}

/// Service error types
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Channel creation failed: {0}")]
    ChannelCreationFailed(String),

    #[error("QoS setting failed: {0}")]
    QosSettingFailed(String),

    #[error("Exchange declaration failed: {0}")]
    ExchangeDeclarationFailed(String),

    #[error("Queue declaration failed: {0}")]
    QueueDeclarationFailed(String),

    #[error("Queue binding failed: {0}")]
    QueueBindingFailed(String),

    #[error("Message serialization failed: {0}")]
    SerializationFailed(String),

    #[error("Message publish failed: {0}")]
    PublishFailed(String),

    #[error("Connection close failed: {0}")]
    ConnectionCloseFailed(String),
} 