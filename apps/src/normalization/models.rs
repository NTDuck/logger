use bon::Builder;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Builder, PartialEq)]
pub struct NormalizedLog {
    pub log_id: Uuid,
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub app_name: String,
    pub error_code: Option<String>,
    pub attribute_keys: Vec<String>,
    pub attribute_values_string: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DLQEnvelope {
    pub failed_at: String,
    pub error_reason: String,
    pub worker_id: String,
    pub original_payload_truncated: String,
    pub sha256_hash: String,
}

#[bon::bon]
impl DLQEnvelope {
    #[builder]
    pub fn new(
        failed_at: String,
        error_reason: String,
        worker_id: String,
        raw_payload: &[u8],
    ) -> Self {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(raw_payload);
        let sha256_hash = hasher
            .finalize()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();

        let truncate_len = std::cmp::min(raw_payload.len(), 2048);
        let original_payload_truncated =
            String::from_utf8_lossy(&raw_payload[..truncate_len]).into_owned();

        Self {
            failed_at,
            error_reason,
            worker_id,
            original_payload_truncated,
            sha256_hash,
        }
    }
}

#[derive(Debug, Error)]
pub enum NormalizationError {
    #[error("Poison Pill: {0}")]
    PoisonPill(String),
    #[error("Regex Failure: {0}")]
    RegexFailure(String),
    #[error("Serialization Error: {0}")]
    SerializationError(String),
    #[error("Produce Error: {0}")]
    ProduceError(String),
}
