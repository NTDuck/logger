use crate::ai_consumer::models::AITag;
use crate::normalization::models::NormalizedLog;

pub fn extract_message_body(log: &NormalizedLog) -> String {
    log.message.clone()
}

pub fn build_ai_tag(
    log_id: uuid::Uuid,
    model_version: String,
    tag: String,
    confidence: f32,
) -> AITag {
    AITag::builder()
        .log_id(log_id)
        .model_version(model_version)
        .tag(tag)
        .confidence(confidence)
        .tagged_at(chrono::Utc::now().to_rfc3339())
        .build()
}
