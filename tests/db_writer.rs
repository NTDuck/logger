use cucumber::World;

mod steps;

#[tokio::main]
async fn main() {
    steps::db_writer_steps::DbWriterWorld::cucumber()
        .run_and_exit("tests/features/db_writer.feature")
        .await;
}
