use crate::edge::models::axiom::Erratum;
use async_trait::async_trait;
use bon::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Builder, PartialEq)]
pub struct AITag {
    pub log_id: Uuid,
    pub model_version: String,
    pub tag: String,
    pub confidence: f32,
    pub tagged_at: String,
}

#[derive(Debug, Erratum)]
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
    async fn classify(&self, log_id: Uuid, message: &str) -> Result<AITag, AIError>;
}

#[async_trait]
pub trait TagStreamPublisher: Send + Sync {
    async fn publish_patch(&self, tag: &AITag) -> Result<(), AIError>;
}
