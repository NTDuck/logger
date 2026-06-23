Feature: Alert Consumer v9 Rate-Limited Notification

  Scenario: High-priority errors are deduplicated transactionally.
    Given the Config Listener fetches the initial threshold (e.g. 100 per 60s) from the Admin API.
    When 150 errors with matching fingerprints are consumed by the Fetcher and pushed to the mpsc channel.
    Then the Processor reads from the mpsc channel and computes the SHA-256 fingerprint.
    And executes a Redis Lua script to reserve a token (reserve_and_check).
    And fires exactly 1 notification to Telegram.
    And commits the token via commit() upon HTTP 2xx success.
    And batches the remaining 49 errors into a digest message (Batching Fallback) because the threshold was breached.
    And commits the Kafka offset.
    And increments logger_alerts_fired_total by 1.
    And increments logger_events_processed_total OUTSIDE of any retry loop.

  Scenario: Telegram API transient failure triggers Mechanical Backpressure.
    Given the Token Bucket reserved a token.
    When the Telegram HTTP call returns 503.
    Then the Processor task enters an inner retry loop.
    And the Processor sleeps using tokio::time::sleep wrapped in a tokio::select! alongside CancellationToken::cancelled().
    And the bounded mpsc channel fills up, blocking the Fetcher task naturally.
    And librdkafka autonomously maintains the background broker heartbeat without polling recv().
    And if all retries fail, it executes rollback() on the Token Bucket.
    And logger_events_processed_total is incremented exactly once as an error, NOT per retry.

  Scenario: Config Split-Brain is avoided via State Reconciliation.
    Given the Alert Consumer process starts.
    When it attempts to load configuration.
    Then it MUST NOT use hardcoded defaults.
    And MUST fetch the source-of-truth from the Admin API via HTTP GET.
    And only then subscribe to Redis Pub/Sub for live updates.
