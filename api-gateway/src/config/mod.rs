pub mod external_apis;
pub mod redis;
pub mod services;

use serde::{Deserialize, Serialize};
use std::env;

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub redis: redis::RedisConfig,
    pub services: services::ServicesConfig,
    pub external_apis: external_apis::ExternalApisConfig,
    pub logging: LoggingConfig,
}

impl AppConfig {
    /// Load configuration from environment variables and config files
    pub fn load() -> Result<Self, config::ConfigError> {
        let config = config::Config::builder()
            // Start with default values
            .add_source(config::File::with_name("config/default").required(false))
            // Add environment-specific config
            .add_source(
                config::File::with_name(&format!(
                    "config/{}",
                    env::var("APP_ENV").unwrap_or_else(|_| "development".into())
                ))
                .required(false),
            )
            // Add local config file (gitignored)
            .add_source(config::File::with_name("config/local").required(false))
            // Override with environment variables
            .add_source(config::Environment::with_prefix("AI4THAI").separator("__"))
            .build()?;

        config.try_deserialize()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
    pub max_connections: Option<usize>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 3000,
            workers: None,
            max_connections: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub output: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
            format: "json".to_string(),
            output: "stdout".to_string(),
        }
    }
}
