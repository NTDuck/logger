use cucumber::{given, then, when, World};

#[derive(Debug, Default, World)]
#[allow(dead_code)]
pub struct AITagDBWorld {
    pub ready: bool,
}

#[given(expr = "a batch of AI tags consumed from the \"ai-tags-stream\" topic.")]
async fn a_batch_of_ai_tags_consumed_from_the_ai_tags_stream_topic(_w: &mut AITagDBWorld) {
    panic!("pending");
}

#[when(expr = "the accumulator reaches the flush threshold.")]
async fn the_accumulator_reaches_the_flush_threshold(_w: &mut AITagDBWorld) {
    panic!("pending");
}

#[then(expr = "the system MUST send a JSONEachRow POST request to ClickHouse.")]
async fn the_system_must_send_a_json_each_row_post_request_to_click_house(_w: &mut AITagDBWorld) {
    panic!("pending");
}

#[then(
    regex = r#"^the metric logger_events_processed_total MUST be incremented with stage="([^"]*)" and status="([^"]*)"\.$"#
)]
async fn the_metric_logger_events_processed_total_must_be_incremented_with_stage_and_status(
    _w: &mut AITagDBWorld,
    _stage: String,
    _status: String,
) {
    panic!("pending");
}

#[given(expr = "ClickHouse is unreachable.")]
async fn clickhouse_is_unreachable(_w: &mut AITagDBWorld) {
    panic!("pending");
}

#[when(expr = "the processor attempts to flush the AI tags.")]
async fn the_processor_attempts_to_flush_the_ai_tags(_w: &mut AITagDBWorld) {
    panic!("pending");
}

#[then(expr = "the system MUST retry indefinitely with exponential backoff.")]
async fn the_system_must_retry_indefinitely_with_exponential_backoff(_w: &mut AITagDBWorld) {
    panic!("pending");
}

#[then(expr = "Task A MUST block due to mpsc channel backpressure.")]
async fn task_a_must_block_due_to_mpsc_channel_backpressure(_w: &mut AITagDBWorld) {
    panic!("pending");
}
