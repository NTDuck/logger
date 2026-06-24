use crate::admin::logic::{build_alert_config, validate_admin_claims, validate_payload};
use crate::admin::models::{AdminConfigPayload, ConfigWriter};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::DecodingKey;
use prometheus::IntCounterVec;
use ::std::sync::Arc;
use tap::TapFallible;

#[derive(::core::clone::Clone)]
pub struct AdminAppState {
    pub writer: Arc<dyn ConfigWriter>,
    pub events_processed_total: IntCounterVec,
    pub decoding_key: Arc<DecodingKey>,
}

#[::tracing::instrument(skip_all)]
pub async fn admin_config_handler(
    State(state): State<AdminAppState>,
    headers: HeaderMap,
    Json(payload): Json<AdminConfigPayload>,
) -> Response {
    let auth_header = headers.get("Authorization").and_then(|h| h.to_str().ok());
    let token = if let Some(h) = auth_header {
        h.strip_prefix("Bearer ").unwrap_or(h)
    } else {
        ""
    };

    match validate_admin_claims(token, &state.decoding_key) {
        Ok(Ok(_)) => {}
        Ok(Err(_)) => {
            state
                .events_processed_total
                .with_label_values(&["admin", "error"])
                .inc();
            return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
        }
        Err(e) => {
            ::tracing::error!(error = %e, "validate_admin_claims failed");
            state
                .events_processed_total
                .with_label_values(&["admin", "error"])
                .inc();
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response();
        }
    }

    let payload = match validate_payload(payload) {
        Ok(Ok(p)) => p,
        Ok(Err(_)) => {
            state
                .events_processed_total
                .with_label_values(&["admin", "error"])
                .inc();
            return (StatusCode::BAD_REQUEST, "Invalid Payload").into_response();
        }
        Err(e) => {
            ::tracing::error!(error = %e, "validate_payload failed");
            state
                .events_processed_total
                .with_label_values(&["admin", "error"])
                .inc();
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response();
        }
    };

    let config = build_alert_config(payload);

    match state.writer.append_config(config.clone()).await {
        Ok(Ok(_)) => {}
        Ok(Err(errs)) => {
            ::tracing::error!(errors = ?errs, "Admin handler: append_config failed");
            state
                .events_processed_total
                .with_label_values(&["admin", "error"])
                .inc();
            let msg = errs.first().map(|e| e.to_string()).unwrap_or_default();
            return (StatusCode::BAD_GATEWAY, msg).into_response();
        }
        Err(e) => {
            ::tracing::error!(error = %e, "Admin handler: append_config system error");
            state
                .events_processed_total
                .with_label_values(&["admin", "error"])
                .inc();
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response();
        }
    }

    match state.writer.publish_update_event(config.clone()).await {
        Ok(Ok(_)) => {}
        Ok(Err(errs)) => {
            ::tracing::warn!(errors = ?errs, "Redis publish failed but ClickHouse write succeeded");
        }
        Err(e) => {
            ::tracing::warn!(error = %e, "Redis publish system error but ClickHouse write succeeded");
        }
    }

    state
        .events_processed_total
        .with_label_values(&["admin", "success"])
        .inc();

    ::tracing::debug!(config_id = %config.config_id, "Admin config update completed successfully");

    (
        StatusCode::CREATED,
        axum::Json(serde_json::json!({ "config_id": config.config_id })),
    )
        .into_response()
}
