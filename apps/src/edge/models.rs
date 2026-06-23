pub mod axiom {
    pub use thiserror::Error as Erratum;
}

use bon::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Builder, Clone)]
pub struct DomainLog {
    #[builder(default = Uuid::now_v7())]
    pub log_id: Uuid,
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub app_name: String,
    pub error_code: Option<String>,
    pub attribute_keys: Vec<String>,
    pub attribute_values_string: Vec<String>,
}

#[derive(Debug, axiom::Erratum)]
pub enum EdgeError {
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Forbidden")]
    Forbidden,
    #[error("BadRequest: {0}")]
    BadRequest(String),
    #[error("PayloadTooLarge")]
    PayloadTooLarge,
    #[error("KafkaProduceError: {0}")]
    KafkaProduceError(String),
}

#[derive(Debug, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub app_grants: Vec<String>,
    pub exp: u64,
}
