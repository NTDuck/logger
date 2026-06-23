use cucumber::World;

mod steps;

#[tokio::main]
async fn main() {
    steps::ai_consumer_steps::AIWorld::cucumber()
        .run_and_exit("tests/features/ai_consumer.feature")
        .await;
}
