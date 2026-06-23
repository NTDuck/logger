Feature: AI Consumer Classification

  Scenario: Logs are successfully classified and published to stream
    Given a batch of log payloads have been published to "logs-normalized"
    And the ONNX runtime is initialized with a valid model
    When the Fetcher Task polls and pushes a batch of messages to the mpsc channel
    And the Processor Task extracts the message bodies and invokes the AIClassifier::classify method
    Then the classify call MUST return AITags with valid tags and confidences
    And the Processor Task MUST call TagStreamPublisher::publish_patch for each tag
    And the Processor Task MUST commit the Redpanda consumer offsets ONLY after all publish_patch calls succeed
    And logger_events_processed_total with status="success" MUST be incremented by the batch size OUTSIDE of any retry loops

  Scenario: ONNX classification fails
    Given a log payload has been published to "logs-normalized"
    And the ONNX model returns an inference error
    When the Processor Task attempts to classify the message
    Then the classify call MUST return an InferenceError
    And the Processor Task MUST NOT include this tag in publish_patch
    And logger_events_processed_total with status="error" MUST be incremented by 1
    And the Processor Task MUST commit the offset to skip the poison message

  Scenario: ai-tags-stream publish fails (Decoupled Backpressure)
    Given the tags have been successfully classified
    When the Processor Task attempts to call publish_patch
    And the rdkafka producer returns a StreamPublishError
    Then the Processor Task MUST enter a backoff loop retrying publish_patch in place
    And the retry sleep MUST be selectable against the CancellationToken
    And the mpsc channel MUST fill up, naturally blocking the Fetcher Task via TCP backpressure
    And the Fetcher Task MUST NOT call consumer.recv() while blocked
    And the telemetry counter MUST NOT be incremented during the retry loop
    And upon successful retry, the Processor Task MUST proceed to commit offsets
