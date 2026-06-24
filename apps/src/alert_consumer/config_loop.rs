use crate::alert_consumer::models::{AlertConfig, ConfigSubscriber};
use ::std::sync::Arc;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

pub async fn run_config_listener_task(
    subscriber: Arc<dyn ConfigSubscriber>,
    config_cache: Arc<RwLock<Option<AlertConfig>>>,
    cancel_token: CancellationToken,
    config_reconciliations_total: prometheus::IntCounter,
) {
    loop {
        // Break out completely if cancelled
        if cancel_token.is_cancelled() {
            break;
        }

        // State Reconciliation BEFORE subscribing
        match subscriber.fetch_initial().await {
            Ok(Ok(config)) => {
                let mut cache = config_cache.write().await;
                *cache = Some(config);
                config_reconciliations_total.inc();
                ::tracing::info!("State Reconciliation complete. Config fetched from Admin API.");
            }
            Ok(Err(errs)) => {
                ::tracing::error!(errors = ?errs, "Domain error fetching initial config. Retrying...");
                tokio::select! {
                    _ = tokio::time::sleep(::std::time::Duration::from_secs(5)) => continue,
                    _ = cancel_token.cancelled() => break,
                }
            }
            Err(e) => {
                ::tracing::error!(error = ?e, "System error fetching initial config. Retrying...");
                tokio::select! {
                    _ = tokio::time::sleep(::std::time::Duration::from_secs(5)) => continue,
                    _ = cancel_token.cancelled() => break,
                }
            }
        }

        // Now subscribe to Pub/Sub
        match subscriber.subscribe().await {
            Ok(Ok(mut rx)) => {
                ::tracing::info!("Successfully subscribed to Redis Pub/Sub");
                loop {
                    tokio::select! {
                        _ = cancel_token.cancelled() => {
                            return;
                        }
                        msg = rx.recv() => {
                            match msg {
                                Some(config) => {
                                    let mut cache = config_cache.write().await;
                                    *cache = Some(config);
                                    config_reconciliations_total.inc();
                                    ::tracing::info!("Live config update applied.");
                                }
                                None => {
                                    ::tracing::error!("Redis Pub/Sub channel closed. Reconnecting...");
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            Ok(Err(errs)) => {
                ::tracing::error!(errors = ?errs, "Domain error subscribing to Pub/Sub. Retrying...");
            }
            Err(e) => {
                ::tracing::error!(error = ?e, "System error subscribing to Pub/Sub. Retrying...");
            }
        }

        // Delay before full reconnection + reconciliation loop
        tokio::select! {
            _ = tokio::time::sleep(::std::time::Duration::from_secs(5)) => continue,
            _ = cancel_token.cancelled() => break,
        }
    }
}
