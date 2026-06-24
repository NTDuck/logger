use axiom::Erratum;
use thiserror::Error;
use bon::Builder;
use ::serde::{Deserialize, Serialize};

#[derive(::core::fmt::Debug, ::serde::Deserialize)]
pub struct AdminConfigPayload {
    pub threshold: u64,
    pub window_seconds: u64,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder, ::serde::Serialize)]
#[builder(on(::axiom::string::String, into))]
pub struct AlertConfig {
    pub config_id: String,
    pub threshold: u64,
    pub window_seconds: u64,
    pub created_at: String,
}

#[derive(::core::fmt::Debug, ::axiom::Erratum, ::thiserror::Error)]
pub enum AdminError {
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Invalid payload")]
    InvalidPayload,

    #[error("Write failed: {0}")]
    WriteFailed(String),

    #[error("Broadcast failed: {0}")]
    BroadcastFailed(String),
}

#[async_trait::async_trait]
pub trait ConfigWriter: Send + Sync {
    async fn append_config(
        &self,
        config: AlertConfig,
    ) -> ::axiom::result::Fallible<::core::result::Result<(), ::std::vec::Vec<AdminError>>>;
    async fn publish_update_event(
        &self,
        config: AlertConfig,
    ) -> ::axiom::result::Fallible<::core::result::Result<(), ::std::vec::Vec<AdminError>>>;
}
