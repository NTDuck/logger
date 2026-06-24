use axiom::Erratum;
use thiserror::Error;

#[derive(::core::fmt::Debug, ::axiom::Erratum, ::thiserror::Error)]
pub enum DbWriterError {
    #[error("ConnectionDropped: {0}")]
    ConnectionDropped(String),
    #[error("BatchInsertFailed: {0}")]
    BatchInsertFailed(String),
    #[error("DeserializationError: {0}")]
    DeserializationError(String),
    #[error("ConsumerError: {0}")]
    ConsumerError(String),
}
