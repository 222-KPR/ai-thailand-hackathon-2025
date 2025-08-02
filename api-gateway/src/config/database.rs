use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout_seconds: u64,
    pub idle_timeout_seconds: u64,
    pub max_lifetime_seconds: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgresql://postgres:password@localhost:5432/ai4thai".to_string()
            }),
            max_connections: 20,
            min_connections: 5,
            connection_timeout_seconds: 30,
            idle_timeout_seconds: 600,
            max_lifetime_seconds: 1800,
        }
    }
}

impl DatabaseConfig {
    /// Create a PostgreSQL connection pool with the configured settings
    pub async fn create_pool(&self) -> Result<PgPool, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(self.max_connections)
            .min_connections(self.min_connections)
            .acquire_timeout(Duration::from_secs(self.connection_timeout_seconds))
            .idle_timeout(Duration::from_secs(self.idle_timeout_seconds))
            .max_lifetime(Duration::from_secs(self.max_lifetime_seconds))
            .connect(&self.url)
            .await
    }
    
    /// Test database connection
    pub async fn test_connection(&self) -> Result<(), sqlx::Error> {
        let pool = self.create_pool().await?;
        sqlx::query("SELECT 1").fetch_one(&pool).await?;
        pool.close().await;
        Ok(())
    }
    
    /// Get database URL without password for logging
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