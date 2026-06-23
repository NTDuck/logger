use cucumber::World;
use steps::ai_tag_db_projection_steps::AITagDBWorld;

mod steps;

#[tokio::main]
async fn main() {
    AITagDBWorld::cucumber()
        .run_and_exit("tests/features/ai_tag_db_projection.feature")
        .await;
}
