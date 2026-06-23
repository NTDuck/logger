use crate::admin::models::{AdminError, AlertConfig, ConfigWriter};
use redis::AsyncCommands;
use reqwest::Client;
use std::sync::Arc;
use tap::TapFallible;
use tokio::sync::Mutex;

pub struct AdminConfigWriter {
    pub ch_client: Client,
    pub ch_url: String,
    pub redis_conn: Arc<Mutex<redis::aio::MultiplexedConnection>>,
}

#[async_trait::async_trait]
impl ConfigWriter for AdminConfigWriter {
    #[::tracing::instrument(skip_all)]
    async fn append_config(&self, config: AlertConfig) -> Result<(), AdminError> {
        let json_payload = serde_json::to_string(&config)
            .tap_err(|e| ::tracing::error!(error = %e, "Failed to serialize AlertConfig"))
            .map_err(|e| AdminError::WriteFailed(e.to_string()))?;

        let url = format!(
            "{}?query=INSERT INTO alert_configs FORMAT JSONEachRow",
            self.ch_url
        );

        let response = self
            .ch_client
            .post(&url)
            .body(json_payload)
            .send()
            .await
            .tap_err(|e| ::tracing::error!(error = %e, "ClickHouse append_config INSERT failed"))
            .map_err(|e| AdminError::WriteFailed(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            ::tracing::error!(status = %status, "ClickHouse returned non-success status");
            return Err(AdminError::WriteFailed(format!("HTTP {}", status)));
        }

        ::tracing::debug!(config_id = %config.config_id, "Config row appended to ClickHouse alert_configs table");
        Ok(())
    }

    #[::tracing::instrument(skip_all)]
    async fn publish_update_event(&self, config: AlertConfig) -> Result<(), AdminError> {
        let json_payload = serde_json::to_string(&config)
            .tap_err(|e| ::tracing::error!(error = %e, "Failed to serialize AlertConfig for Redis"))
            .map_err(|e| AdminError::BroadcastFailed(e.to_string()))?;

        let mut conn = self.redis_conn.lock().await;

        let _: () = conn
            .publish("admin:config_updates", json_payload)
            .await
            .tap_err(
                |e| ::tracing::error!(error = %e, "Redis PUBLISH to admin:config_updates failed"),
            )
            .map_err(|e| AdminError::BroadcastFailed(e.to_string()))?;

        ::tracing::debug!(config_id = %config.config_id, channel = "admin:config_updates", "Config update event published to Redis Pub/Sub");
        Ok(())
    }
}
