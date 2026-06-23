use cucumber::World;
use steps::admin_steps::AdminWorld;

mod steps;

#[tokio::main]
async fn main() {
    AdminWorld::cucumber()
        .run_and_exit("tests/features/admin_api.feature")
        .await;
}
