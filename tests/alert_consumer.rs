use cucumber::World;

mod steps;

#[tokio::main]
async fn main() {
    steps::alert_consumer_steps::AlertWorld::cucumber()
        .run_and_exit("tests/features/alert_consumer.feature")
        .await;
}
