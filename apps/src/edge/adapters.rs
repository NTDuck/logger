use crate::edge::models::{DomainLog, EdgeError};
use async_trait::async_trait;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use tap::TapFallible;

#[async_trait]
pub trait LogProducer: Send + Sync {
    async fn produce(&self, domain_log: &DomainLog) -> ::axiom::result::Fallible<::core::result::Result<(), ::std::vec::Vec<EdgeError>>>;
}

pub struct KafkaLogProducer {
    producer: FutureProducer,
}

impl KafkaLogProducer {
    pub fn new(producer: FutureProducer) -> Self {
        Self { producer }
    }
}

#[async_trait]
impl LogProducer for KafkaLogProducer {
    #[::tracing::instrument(skip_all)]
    async fn produce(&self, domain_log: &DomainLog) -> ::axiom::result::Fallible<::core::result::Result<(), ::std::vec::Vec<EdgeError>>> {
        let payload = match serde_json::to_vec(domain_log) {
            Ok(p) => p,
            Err(e) => return ::axiom::err!(EdgeError::KafkaProduceError(e.to_string())),
        };

        let record = FutureRecord::to("logs-raw")
            .payload(&payload)
            .key(&domain_log.app_name);

        match self.producer.send(record, Timeout::Never).await {
            Ok(_) => {
                ::tracing::debug!(
                    topic = "logs-raw",
                    app_name = %domain_log.app_name,
                    "Produced DomainLog to logs-raw"
                );
                ::axiom::ok!(())
            }
            Err((e, _)) => {
                ::tracing::error!(error = %e, "Kafka produce to logs-raw failed");
                ::axiom::err!(EdgeError::KafkaProduceError(e.to_string()))
            }
        }
    }
}
