use crate::edge::models::axiom::Erratum;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AlertConfig {
    pub config_id: Uuid,
    pub threshold: u64,
    pub window_seconds: u64,
    pub created_at: String,
}

#[derive(Debug, Erratum)]
pub enum AlertError {
    #[error("RedisError: {0}")]
    RedisError(String),
    #[error("TelegramError: {0}")]
    TelegramError(String),
    #[error("ConsumerError: {0}")]
    ConsumerError(String),
    #[error("ConfigSubscriptionError: {0}")]
    ConfigSubscriptionError(String),
}

#[async_trait]
pub trait RateLimiter: Send + Sync {
    async fn reserve_and_check(
        &self,
        fingerprint: &str,
        window_sec: u64,
        limit: u64,
        strict_ttl: u64,
    ) -> Result<bool, Vec<AlertError>>;

    async fn commit(&self, fingerprint: &str) -> Result<(), Vec<AlertError>>;
    async fn rollback(&self, fingerprint: &str) -> Result<(), Vec<AlertError>>;
}

#[async_trait]
pub trait AlertNotifier: Send + Sync {
    async fn notify(&self, message: &str) -> Result<(), Vec<AlertError>>;
}

#[async_trait]
pub trait ConfigSubscriber: Send + Sync {
    async fn fetch_initial(&self) -> Result<AlertConfig, Vec<AlertError>>;
    async fn subscribe(&self) -> Result<tokio::sync::mpsc::Receiver<AlertConfig>, Vec<AlertError>>;
}
