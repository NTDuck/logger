use axiom::Erratum;
use thiserror::Error;
use async_trait::async_trait;
use bon::Builder;
use ::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::serde::Serialize, ::serde::Deserialize, ::bon::Builder, PartialEq)]
#[builder(on(::axiom::string::String, into))]
pub struct AITag {
    pub log_id: Uuid,
    pub model_version: String,
    pub tag: String,
    pub confidence: f32,
    pub tagged_at: String,
}

#[derive(::core::fmt::Debug, ::axiom::Erratum, ::thiserror::Error)]
pub enum AIError {
    #[error("InferenceError: {0}")]
    InferenceError(String),
    #[error("StreamPublishError: {0}")]
    StreamPublishError(String),
    #[error("ConsumerError: {0}")]
    ConsumerError(String),
}

#[async_trait]
pub trait AIClassifier: Send + Sync {
    async fn classify(&self, log_id: Uuid, message: &str) -> ::axiom::result::Fallible<::core::result::Result<AITag, ::std::vec::Vec<AIError>>>;
}

#[async_trait]
pub trait TagStreamPublisher: Send + Sync {
    async fn publish_patch(&self, tag: &AITag) -> ::axiom::result::Fallible<::core::result::Result<(), ::std::vec::Vec<AIError>>>;
}
