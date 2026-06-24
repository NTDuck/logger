use axiom::Erratum;
use thiserror::Error;
use bon::Builder;
use ::serde::{Deserialize, Serialize};

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder, ::serde::Serialize, ::serde::Deserialize)]
#[builder(on(::axiom::string::String, into))]
pub struct AITagMessage {
    pub log_id: String,
    pub tag: String,
    pub confidence_score: f64,
}

#[derive(::core::fmt::Debug, ::axiom::Erratum, ::thiserror::Error)]
pub enum AITagDBError {
    #[error("Write failed: {0}")]
    WriteFailed(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),
}

#[async_trait::async_trait]
pub trait AITagClickHouseWriter: Send + Sync {
    async fn write_batch(
        &self,
        tags: Vec<AITagMessage>,
    ) -> ::axiom::result::Fallible<::core::result::Result<(), ::std::vec::Vec<AITagDBError>>>;
}
