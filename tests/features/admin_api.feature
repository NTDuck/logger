Feature: Admin API Alert Configuration Management

  Scenario: Admin successfully updates threshold configuration.
    Given an Admin user authenticated with a valid JWT containing the admin role claim.
    And they have prepared a configuration payload with threshold 100 and window_seconds 60.
    When they submit a POST request to "/v1/admin/config" with the configuration payload.
    Then the system MUST generate a new config_id and created_at timestamp.
    And the system MUST append the AlertConfig row to the ClickHouse "alert_configs" MergeTree table.
    And the system MUST publish the serialized AlertConfig to the Redis Pub/Sub channel "admin:config_updates".
    And the system MUST respond with HTTP 201 Created.
    And the metric logger_events_processed_total with labels stage="admin" and status="success" MUST be incremented by 1 (counted exactly once).

  Scenario: Unauthenticated request is rejected.
    Given a request with no JWT token or an invalid JWT token.
    When the request is sent to POST "/v1/admin/config".
    Then the system MUST respond with HTTP 401 Unauthorized.
    And the metric logger_events_processed_total with labels stage="admin" and status="error" MUST be incremented by 1.

  Scenario: Request with missing admin claim is rejected.
    Given a valid JWT that does NOT contain the admin role claim.
    When the request is sent to POST "/v1/admin/config".
    Then the system MUST respond with HTTP 401 Unauthorized.
    And the metric logger_events_processed_total with labels stage="admin" and status="error" MUST be incremented by 1.

  Scenario: ClickHouse write failure is handled gracefully.
    Given a valid admin JWT and a valid configuration payload.
    When the ClickHouse INSERT fails (network error, timeout, non-200 response).
    Then the system MUST respond with HTTP 502 Bad Gateway.
    And the metric logger_events_processed_total with labels stage="admin" and status="error" MUST be incremented by 1.

  Scenario: Redis publish failure does not block the response.
    Given a valid admin JWT and a valid configuration payload.
    And the ClickHouse INSERT succeeds.
    When the Redis PUBLISH fails.
    Then the system MUST still respond with HTTP 201 Created (the config is persisted; notification is best-effort).
    And the metric logger_events_processed_total with labels stage="admin" and status="success" MUST be incremented by 1.
    And a tracing error span MUST be emitted for the Redis failure.
