pub mod external_apis;
pub mod redis;
pub mod services;

use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub redis: redis::RedisConfig,
    pub services: services::ServicesConfig,
    pub rabbitmq: RabbitMQConfig,
    pub file_storage: FileStorageConfig,
    pub external_apis: external_apis::ExternalApisConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub request_timeout: Duration,
    pub max_upload_size: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RabbitMQConfig {
    pub url: String,
    pub connection_timeout: Duration,
    pub channel_timeout: Duration,
    pub queue_name: String,
    pub exchange_name: String,
    pub routing_key: String,
    pub prefetch_count: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FileStorageConfig {
    pub temp_dir: String,
    pub max_file_size: usize,
    pub cleanup_interval: Duration,
    pub file_ttl: Duration,
    pub supported_formats: Vec<String>,
}

impl AppConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        let mut builder = config::Config::builder()
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 3000)?
            .set_default("server.request_timeout", 30)?
            .set_default("server.max_upload_size", 10485760)? // 10MB
            .set_default("redis.url", "redis://localhost:6379")?
            .set_default("services.vision_service_url", "http://localhost:2001")?
            .set_default("services.llm_service_url", "http://localhost:2002")?
            .set_default("services.queue_worker_url", "http://localhost:2003")?
            .set_default("rabbitmq.url", "amqp://guest:guest@localhost:5672")?
            .set_default("rabbitmq.connection_timeout", 30)?
            .set_default("rabbitmq.channel_timeout", 10)?
            .set_default("rabbitmq.queue_name", "vision_analysis_queue")?
            .set_default("rabbitmq.exchange_name", "vision_exchange")?
            .set_default("rabbitmq.routing_key", "vision.analysis")?
            .set_default("rabbitmq.prefetch_count", 1)?
            .set_default("file_storage.temp_dir", "/tmp/vision_uploads")?
            .set_default("file_storage.max_file_size", 10485760)? // 10MB
            .set_default("file_storage.cleanup_interval", 3600)? // 1 hour
            .set_default("file_storage.file_ttl", 86400)? // 24 hours
            .set_default("file_storage.supported_formats", vec!["jpg", "jpeg", "png", "webp", "bmp"])?
            .set_default("external_apis.ai4thai_api_key", "")?;

        // Load from environment variables
        builder = builder.add_source(config::Environment::default().separator("__"));

        // Load from config files if they exist
        if let Ok(config) = builder.build() {
            config.try_deserialize()
        } else {
            builder.build()?.try_deserialize()
        }
    }
}
