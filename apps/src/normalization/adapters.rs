use crate::normalization::models::{DLQEnvelope, NormalizationError, NormalizedLog};
use async_trait::async_trait;
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::message::{Message, OwnedMessage};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use ::std::sync::Arc;
use tap::TapFallible;

macro_rules! try_local {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => return ::axiom::err!(e),
        }
    };
}

#[async_trait]
pub trait LogConsumer: Send + Sync {
    async fn consume(&self) -> ::axiom::result::Fallible<::core::result::Result<(Vec<u8>, OwnedMessage), ::std::vec::Vec<NormalizationError>>>;
    fn commit_offset(&self, message: &OwnedMessage) -> ::axiom::result::Fallible<::core::result::Result<(), ::std::vec::Vec<NormalizationError>>>;
}

pub struct KafkaLogConsumer {
    consumer: Arc<StreamConsumer>,
}

impl KafkaLogConsumer {
    pub fn new(consumer: Arc<StreamConsumer>) -> Self {
        Self { consumer }
    }
}

#[async_trait]
impl LogConsumer for KafkaLogConsumer {
    #[::tracing::instrument(skip_all)]
    async fn consume(&self) -> ::axiom::result::Fallible<::core::result::Result<(Vec<u8>, OwnedMessage), ::std::vec::Vec<NormalizationError>>> {
        let msg = self
            .consumer
            .recv()
            .await
            .tap_err(|e| ::tracing::error!(error = %e, "rdkafka consumer recv failed"))
            .map_err(|e| NormalizationError::ProduceError(e.to_string()))?;

        let payload = msg.payload().unwrap_or_default().to_vec();
        ::tracing::debug!(bytes = payload.len(), "consumed raw message from logs-raw");

        ::axiom::ok!((payload, msg.detach()))
    }

    #[::tracing::instrument(skip_all)]
    fn commit_offset(&self, message: &OwnedMessage) -> ::axiom::result::Fallible<::core::result::Result<(), ::std::vec::Vec<NormalizationError>>> {
        let mut tpl = rdkafka::TopicPartitionList::new();
        tpl.add_partition_offset(
            message.topic(),
            message.partition(),
            rdkafka::Offset::Offset(message.offset() + 1),
        )
        .map_err(|e| NormalizationError::ProduceError(e.to_string()))?;

        self.consumer
            .commit(&tpl, CommitMode::Async)
            .tap_err(|e| ::tracing::error!(error = %e, "rdkafka offset commit failed"))
            .map_err(|e| NormalizationError::ProduceError(e.to_string()))?;

        ::tracing::debug!("offset committed for logs-raw message");
        ::axiom::ok!(())
    }
}

#[async_trait]
pub trait NormalizedProducer: Send + Sync {
    async fn produce_normalized(&self, log: &NormalizedLog) -> ::axiom::result::Fallible<::core::result::Result<(), ::std::vec::Vec<NormalizationError>>>;
    async fn produce_alert(&self, log: &NormalizedLog) -> ::axiom::result::Fallible<::core::result::Result<(), ::std::vec::Vec<NormalizationError>>>;
    async fn produce_dlq(&self, envelope: &DLQEnvelope) -> ::axiom::result::Fallible<::core::result::Result<(), ::std::vec::Vec<NormalizationError>>>;
}

pub struct KafkaNormalizedProducer {
    producer: FutureProducer,
}

impl KafkaNormalizedProducer {
    pub fn new(producer: FutureProducer) -> Self {
        Self { producer }
    }
}

#[async_trait]
impl NormalizedProducer for KafkaNormalizedProducer {
    #[::tracing::instrument(skip_all)]
    async fn produce_normalized(&self, log: &NormalizedLog) -> ::axiom::result::Fallible<::core::result::Result<(), ::std::vec::Vec<NormalizationError>>> {
        let payload = serde_json::to_vec(log)
            .tap_err(|e| ::tracing::error!(error = %e, "serialization failed for NormalizedLog"))
            .map_err(|e| NormalizationError::SerializationError(e.to_string()))?;

        let key = log.log_id.to_string();
        let record = FutureRecord::to("logs-normalized")
            .payload(&payload)
            .key(&key);

        self.producer
            .send(record, Timeout::Never)
            .await
            .map_err(|(e, _)| e)
            .tap_err(|e| {
                ::tracing::error!(error = ?e, topic = "logs-normalized", "produce to logs-normalized failed")
            })
            .map_err(|e| NormalizationError::ProduceError(e.to_string()))?;

        ::tracing::debug!(topic = "logs-normalized", log_id = %log.log_id, "produced normalized log");
        ::axiom::ok!(())
    }

    #[::tracing::instrument(skip_all)]
    async fn produce_alert(&self, log: &NormalizedLog) -> ::axiom::result::Fallible<::core::result::Result<(), ::std::vec::Vec<NormalizationError>>> {
        let payload = serde_json::to_vec(log)
            .tap_err(
                |e| ::tracing::error!(error = %e, "serialization failed for alert NormalizedLog"),
            )
            .map_err(|e| NormalizationError::SerializationError(e.to_string()))?;

        let key = log.log_id.to_string();
        let record = FutureRecord::to("alerts-priority-stream")
            .payload(&payload)
            .key(&key);

        self.producer
            .send(record, Timeout::Never)
            .await
            .map_err(|(e, _)| e)
            .tap_err(|e| {
                ::tracing::error!(error = ?e, topic = "alerts-priority-stream", "produce to alerts-priority-stream failed")
            })
            .map_err(|e| NormalizationError::ProduceError(e.to_string()))?;

        ::tracing::debug!(topic = "alerts-priority-stream", log_id = %log.log_id, "produced alert duplicate");
        ::axiom::ok!(())
    }

    #[::tracing::instrument(skip_all)]
    async fn produce_dlq(&self, envelope: &DLQEnvelope) -> ::axiom::result::Fallible<::core::result::Result<(), ::std::vec::Vec<NormalizationError>>> {
        let payload = serde_json::to_vec(envelope)
            .tap_err(|e| ::tracing::error!(error = %e, "serialization failed for DLQEnvelope"))
            .map_err(|e| NormalizationError::SerializationError(e.to_string()))?;

        let key = &envelope.sha256_hash;
        let record = FutureRecord::to("logs-dlq").payload(&payload).key(key);

        self.producer
            .send(record, Timeout::Never)
            .await
            .map_err(|(e, _)| e)
            .tap_err(
                |e| ::tracing::error!(error = ?e, topic = "logs-dlq", "produce to logs-dlq failed"),
            )
            .map_err(|e| NormalizationError::ProduceError(e.to_string()))?;

        ::tracing::debug!(topic = "logs-dlq", sha256 = %envelope.sha256_hash, "produced DLQ envelope");
        ::axiom::ok!(())
    }
}
