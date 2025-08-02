use redis::{Client, RedisError};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub max_connections: u32,
    pub connection_timeout_seconds: u64,
    pub command_timeout_seconds: u64,
    pub retry_attempts: u32,
    pub retry_delay_ms: u64,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            max_connections: 10,
            connection_timeout_seconds: 5,
            command_timeout_seconds: 10,
            retry_attempts: 3,
            retry_delay_ms: 100,
        }
    }
}

impl RedisConfig {
    /// Create a Redis client with the configured settings
    pub async fn create_client(&self) -> Result<Client, RedisError> {
        let client = Client::open(self.url.as_str())?;
        
        // Test connection
        let mut conn = client.get_async_connection().await?;
        redis::cmd("PING").query_async::<_, String>(&mut conn).await?;
        
        Ok(client)
    }
    
    /// Test Redis connection
    pub async fn test_connection(&self) -> Result<(), RedisError> {
        let client = self.create_client().await?;
        let mut conn = client.get_async_connection().await?;
        redis::cmd("PING").query_async::<_, String>(&mut conn).await?;
        Ok(())
    }
    
    /// Get Redis URL without password for logging
    pub fn safe_url(&self) -> String {
        if let Ok(mut url) = url::Url::parse(&self.url) {
            if url.password().is_some() {
                let _ = url.set_password(Some("***"));
            }
            url.to_string()
        } else {
            "invalid_url".to_string()
        }
    }
}

/// Redis key prefixes for different data types
pub struct RedisKeys;

impl RedisKeys {
    pub const USER_SESSION: &'static str = "session:user:";
    pub const RATE_LIMIT: &'static str = "rate_limit:";
    pub const CACHE_DIAGNOSIS: &'static str = "cache:diagnosis:";
    pub const CACHE_LLM_RESPONSE: &'static str = "cache:llm:";
    pub const QUEUE_VISION: &'static str = "queue:vision";
    pub const QUEUE_LLM: &'static str = "queue:llm";
    pub const CIRCUIT_BREAKER: &'static str = "circuit_breaker:";
    pub const SERVICE_HEALTH: &'static str = "health:service:";
    
    /// Generate a user session key
    pub fn user_session(user_id: &uuid::Uuid) -> String {
        format!("{}{}", Self::USER_SESSION, user_id)
    }
    
    /// Generate a rate limit key
    pub fn rate_limit(user_id: &uuid::Uuid, endpoint: &str) -> String {
        format!("{}{}:{}", Self::RATE_LIMIT, user_id, endpoint)
    }
    
    /// Generate a diagnosis cache key
    pub fn diagnosis_cache(image_hash: &str) -> String {
        format!("{}{}", Self::CACHE_DIAGNOSIS, image_hash)
    }
    
    /// Generate an LLM response cache key
    pub fn llm_cache(prompt_hash: &str) -> String {
        format!("{}{}", Self::CACHE_LLM_RESPONSE, prompt_hash)
    }
    
    /// Generate a circuit breaker key
    pub fn circuit_breaker(service_name: &str) -> String {
        format!("{}{}", Self::CIRCUIT_BREAKER, service_name)
    }
    
    /// Generate a service health key
    pub fn service_health(service_name: &str) -> String {
        format!("{}{}", Self::SERVICE_HEALTH, service_name)
    }
}