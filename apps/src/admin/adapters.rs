use crate::admin::models::{AdminError, AlertConfig, ConfigWriter};
use redis::AsyncCommands;
use reqwest::Client;
use ::std::sync::Arc;
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
    async fn append_config(
        &self,
        config: AlertConfig,
    ) -> ::axiom::result::Fallible<::core::result::Result<(), ::std::vec::Vec<AdminError>>> {
        let json_payload = match serde_json::to_string(&config) {
            Ok(v) => v,
            Err(e) => {
                ::tracing::error!(error = %e, "Failed to serialize AlertConfig");
                return ::axiom::err!(AdminError::WriteFailed(e.to_string()));
            }
        };

        let url = format!(
            "{}?query=INSERT INTO alert_configs FORMAT JSONEachRow",
            self.ch_url
        );

        let response = match self
            .ch_client
            .post(&url)
            .body(json_payload)
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => {
                ::tracing::error!(error = %e, "ClickHouse append_config INSERT failed");
                return ::axiom::err!(AdminError::WriteFailed(e.to_string()));
            }
        };

        if !response.status().is_success() {
            let status = response.status();
            ::tracing::error!(status = %status, "ClickHouse returned non-success status");
            return ::axiom::err!(AdminError::WriteFailed(format!(
                "HTTP {}",
                status
            )));
        }

        ::tracing::debug!(config_id = %config.config_id, "Config row appended to ClickHouse alert_configs table");
        ::axiom::ok!(())
    }

    #[::tracing::instrument(skip_all)]
    async fn publish_update_event(
        &self,
        config: AlertConfig,
    ) -> ::axiom::result::Fallible<::core::result::Result<(), ::std::vec::Vec<AdminError>>> {
        let json_payload = match serde_json::to_string(&config) {
            Ok(v) => v,
            Err(e) => {
                ::tracing::error!(error = %e, "Failed to serialize AlertConfig for Redis");
                return ::axiom::err!(AdminError::BroadcastFailed(
                    e.to_string()
                ));
            }
        };

        let mut conn = self.redis_conn.lock().await;

        match conn.publish::<_, _, ()>("admin:config_updates", json_payload).await {
            Ok(_) => (),
            Err(e) => {
                ::tracing::error!(error = %e, "Redis PUBLISH to admin:config_updates failed");
                return ::axiom::err!(AdminError::BroadcastFailed(
                    e.to_string()
                ));
            }
        };

        ::tracing::debug!(config_id = %config.config_id, channel = "admin:config_updates", "Config update event published to Redis Pub/Sub");
        ::axiom::ok!(())
    }
}
