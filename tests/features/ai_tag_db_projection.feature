Feature: AI Tag Database Projection

  Scenario: Batch of AI tags is written to ClickHouse.
    Given a batch of AI tags consumed from the "ai-tags-stream" topic.
    When the accumulator reaches the flush threshold.
    Then the system MUST send a JSONEachRow POST request to ClickHouse.
    And the metric logger_events_processed_total MUST be incremented with stage="ai-tag-db" and status="success".

  Scenario: ClickHouse is offline causing backpressure.
    Given ClickHouse is unreachable.
    When the processor attempts to flush the AI tags.
    Then the system MUST retry indefinitely with exponential backoff.
    And Task A MUST block due to mpsc channel backpressure.
