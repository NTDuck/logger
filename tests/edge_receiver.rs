mod steps;
use cucumber::World;
use steps::edge_receiver_steps::EdgeWorld;

#[tokio::main]
async fn main() {
    EdgeWorld::cucumber()
        .run_and_exit("tests/features/edge_receiver.feature")
        .await;
}
