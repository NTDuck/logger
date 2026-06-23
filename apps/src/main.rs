use clap::Parser;
use logger::edge::actors::{ingest_logs, AppState};
use logger::edge::adapters::KafkaLogProducer;
use prometheus::{Counter, IntCounterVec, Registry};
use rdkafka::config::ClientConfig;
use rdkafka::producer::FutureProducer;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    role: String,

    #[arg(long, default_value = "127.0.0.1:9092")]
    kafka_brokers: String,

    #[arg(long, env = "JWT_PUBLIC_KEY")]
    jwt_public_key: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    if args.role != "edge" {
        return Ok(());
    }

    let cancel_token = CancellationToken::new();

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &args.kafka_brokers)
        .set("message.timeout.ms", "5000")
        .create()?;

    let kafka_producer = Arc::new(KafkaLogProducer::new(producer));

    let jwt_key_bytes = Arc::new(args.jwt_public_key.into_bytes());

    let registry = Registry::new();
    let ingest_bytes_total = Counter::new("logger_ingest_bytes_total", "Total raw bytes ingested")?;
    let events_processed_total = IntCounterVec::new(
        prometheus::Opts::new("logger_events_processed_total", "Events processed"),
        &["stage", "status"],
    )?;

    registry.register(Box::new(ingest_bytes_total.clone()))?;
    registry.register(Box::new(events_processed_total.clone()))?;

    let state = AppState {
        producer: kafka_producer,
        jwt_public_key: jwt_key_bytes,
        ingest_bytes_total,
        events_processed_total,
        cancel_token: cancel_token.clone(),
    };

    let router = axum::Router::new()
        .route("/v1/logs", axum::routing::post(ingest_logs))
        .layer(axum::extract::DefaultBodyLimit::max(256 * 1024))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    ::tracing::info!("Edge receiver listening on 0.0.0.0:8080");

    let cancel_token_clone = cancel_token.clone();

    // Wire graceful shutdown
    axum::serve(listener, router)
        .with_graceful_shutdown(async move {
            cancel_token_clone.cancelled().await;
        })
        .await?;

    Ok(())
}
