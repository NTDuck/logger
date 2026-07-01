use futures::prelude::*;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use tokio::signal;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("group.id", "my-group")
        .set("auto.offset.reset", "earliest")
        .create()?;

    consumer.subscribe(&["foo"])?;

    println!("Listening for messages on topic 'foo'. Press Ctrl+C to stop.");

    let mut message_stream = consumer.stream();

    let shutdown = signal::ctrl_c();

    tokio::select! {
        _ = shutdown => {
            println!("Shutting down...");
            Ok(())
        }
        _ = async {
            while let Some(message_result) = message_stream.next().await {
                match message_result {
                    Ok(message) => {
                        if let Some(payload) = message.payload() {
                            match std::str::from_utf8(payload) {
                                Ok(text) => println!("Received: {}", text),
                                Err(e) => println!("Invalid UTF-8: {}", e),
                            }
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
        } => Ok(()),
    }
}
