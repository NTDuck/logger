use axiom::Erratum;
use thiserror::Error;
use async_trait::async_trait;
use ::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq)]
pub struct AlertConfig {
    pub config_id: Uuid,
    pub threshold: u64,
    pub window_seconds: u64,
    pub created_at: String,
}

#[derive(::core::fmt::Debug, ::axiom::Erratum, ::thiserror::Error)]
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
    ) -> ::axiom::result::Fallible<::core::result::Result<bool, ::std::vec::Vec<AlertError>>>;

    async fn commit(&self, fingerprint: &str) -> ::axiom::result::Fallible<::core::result::Result<(), ::std::vec::Vec<AlertError>>>;
    async fn rollback(&self, fingerprint: &str) -> ::axiom::result::Fallible<::core::result::Result<(), ::std::vec::Vec<AlertError>>>;
}

#[async_trait]
pub trait AlertNotifier: Send + Sync {
    async fn notify(&self, message: &str) -> ::axiom::result::Fallible<::core::result::Result<(), ::std::vec::Vec<AlertError>>>;
}

#[async_trait]
pub trait ConfigSubscriber: Send + Sync {
    async fn fetch_initial(&self) -> ::axiom::result::Fallible<::core::result::Result<AlertConfig, ::std::vec::Vec<AlertError>>>;
    async fn subscribe(&self) -> ::axiom::result::Fallible<::core::result::Result<tokio::sync::mpsc::Receiver<AlertConfig>, ::std::vec::Vec<AlertError>>>;
}
