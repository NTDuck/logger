use axiom::Erratum;
use thiserror::Error;
use bon::Builder;

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(::axiom::string::String, into))]
pub struct WsClientConfig {
    pub allowed_apps: Vec<String>,
    pub is_admin: bool,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(::axiom::string::String, into))]
pub struct BroadcastMessage {
    pub app_name: String,
    pub payload: String,
}

#[derive(::core::fmt::Debug, ::axiom::Erratum, ::thiserror::Error)]
pub enum WSError {
    #[error("Invalid token")]
    InvalidToken,

    #[error("Forbidden")]
    Forbidden,

    #[error("Connection dropped")]
    ConnectionDropped,

    #[error("Lagging client")]
    LaggingClient,

    #[error("Egress channel full")]
    EgressChannelFull,

    #[error("Send failure: {0}")]
    SendFailure(String),

    #[error("Consumer error: {0}")]
    ConsumerError(String),
}
