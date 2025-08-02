use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

/// Configuration for internal microservices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicesConfig {
    pub vision: ServiceConfig,
    pub llm: ServiceConfig,
    pub discovery: ServiceDiscoveryConfig,
    pub circuit_breaker: CircuitBreakerConfig,
    pub retry: RetryConfig,
}

impl Default for ServicesConfig {
    fn default() -> Self {
        Self {
            vision: ServiceConfig {
                name: "vision-service".to_string(),
                base_url: env::var("VISION_SERVICE_URL")
                    .unwrap_or_else(|_| "http://vision-service:8001".to_string()),
                timeout_seconds: 30,
                health_check_path: "/health".to_string(),
                health_check_interval_seconds: 30,
                max_concurrent_requests: 10,
            },
            llm: ServiceConfig {
                name: "llm-service".to_string(),
                base_url: env::var("LLM_SERVICE_URL")
                    .unwrap_or_else(|_| "http://llm-service:8002".to_string()),
                timeout_seconds: 60,
                health_check_path: "/health".to_string(),
                health_check_interval_seconds: 30,
                max_concurrent_requests: 5,
            },
            discovery: ServiceDiscoveryConfig::default(),
            circuit_breaker: CircuitBreakerConfig::default(),
            retry: RetryConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub base_url: String,
    pub timeout_seconds: u64,
    pub health_check_path: String,
    pub health_check_interval_seconds: u64,
    pub max_concurrent_requests: usize,
}

impl ServiceConfig {
    pub fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout_seconds)
    }
    
    pub fn health_check_interval(&self) -> Duration {
        Duration::from_secs(self.health_check_interval_seconds)
    }
    
    pub fn health_check_url(&self) -> String {
        format!("{}{}", self.base_url, self.health_check_path)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscoveryConfig {
    pub enabled: bool,
    pub registry_url: Option<String>,
    pub service_name: String,
    pub service_port: u16,
    pub health_check_interval_seconds: u64,
    pub deregister_on_shutdown: bool,
}

impl Default for ServiceDiscoveryConfig {
    fn default() -> Self {
        Self {
            enabled: false, // Disabled for hackathon - using static service URLs
            registry_url: env::var("CONSUL_URL").ok(),
            service_name: "api-gateway".to_string(),
            service_port: 3000,
            health_check_interval_seconds: 30,
            deregister_on_shutdown: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    pub recovery_timeout_seconds: u64,
    pub half_open_max_calls: u32,
    pub min_request_threshold: u32,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            recovery_timeout_seconds: 60,
            half_open_max_calls: 3,
            min_request_threshold: 10,
        }
    }
}

impl CircuitBreakerConfig {
    pub fn recovery_timeout(&self) -> Duration {
        Duration::from_secs(self.recovery_timeout_seconds)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub multiplier: f64,
    pub jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 100,
            max_delay_ms: 5000,
            multiplier: 2.0,
            jitter: true,
        }
    }
}

impl RetryConfig {
    pub fn initial_delay(&self) -> Duration {
        Duration::from_millis(self.initial_delay_ms)
    }
    
    pub fn max_delay(&self) -> Duration {
        Duration::from_millis(self.max_delay_ms)
    }
    
    /// Calculate delay for a given attempt (0-indexed)
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        let base_delay = self.initial_delay_ms as f64 * self.multiplier.powi(attempt as i32);
        let capped_delay = base_delay.min(self.max_delay_ms as f64);
        
        let final_delay = if self.jitter {
            // Add ±25% jitter
            let jitter_factor = 1.0 + (rand::random::<f64>() - 0.5) * 0.5;
            capped_delay * jitter_factor
        } else {
            capped_delay
        };
        
        Duration::from_millis(final_delay as u64)
    }
}