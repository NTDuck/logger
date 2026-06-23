Feature: Database Batch Writer

  Scenario: Batch of normalized logs is written to ClickHouse on row count threshold
    Given a batch of 1000 messages has been consumed from logs-normalized.
    When the DB Writer batch accumulator reaches the row count threshold.
    Then it MUST serialize the batch as a JSONEachRow payload.
    And execute an HTTP POST INSERT to the ClickHouse logs table.
    And commit Redpanda consumer offsets only after receiving HTTP 200 from ClickHouse.
    And increment logger_events_processed_total with labels stage="db_writer" and status="success" by the batch size.

  Scenario: Batch is flushed on timer expiry
    Given fewer than 1000 messages have been consumed from logs-normalized.
    When 5 seconds elapse since the last flush.
    Then the DB Writer MUST flush the current partial batch using the same INSERT and commit sequence as Scenario 1.

  Scenario: ClickHouse is offline — backpressure and retry
    Given ClickHouse is unreachable or returns a non-2xx status.
    When the Processor Task attempts to write a batch.
    Then the Processor Task MUST enter an exponential backoff retry loop (base 1 second, max 60 seconds, with jitter) in place.
    And MUST block further processing, causing the bounded mpsc channel to fill up.
    And the Fetcher Task MUST naturally block on channel send via TCP backpressure, leaving pre-fetched messages safely inside librdkafka's internal queues while librdkafka autonomously maintains heartbeats.
    And MUST NOT commit Redpanda offsets during the retry loop.
    And the Processor Task MUST select on the cancellation token during sleep to ensure idempotent cancellation.
    And upon successful retry, the Processor Task resumes reading from the mpsc channel.

  Scenario: Deserialization failure on a consumed message
    Given a message from logs-normalized cannot be deserialized into NormalizedLog.
    When the DB Writer encounters the deserialization error.
    Then it MUST log the error via tracing::error with the partition, offset, and error description.
    And skip the message without adding it to the batch.
    And increment logger_events_processed_total with labels stage="db_writer" and status="error".
