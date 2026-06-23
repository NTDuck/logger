Feature: Edge Receiver Ingestion

  Scenario: Valid log payload is accepted and flattened
    Given a valid OTLP JSON payload with nested key-value attributes at depth 3
    And the payload size is under 256KB
    And a JWT with app_grants containing the payload's app_name
    When it is POSTed to "/v1/logs"
    Then the Edge Receiver MUST respond with HTTP 202
    And the payload MUST be iteratively parsed, flattened to dot-notation parallel arrays, and produced to "logs-raw" as a DomainLog

  Scenario: Payload exceeds depth limit
    Given a log payload containing attributes with a nesting depth of 6
    And a valid JWT
    When it is POSTed to "/v1/logs"
    Then the Edge Receiver MUST fail-fast immediately with HTTP 400
    And no message MUST be produced to "logs-raw"

  Scenario: Request payload exceeds maximum size limit
    Given a log payload with size exceeding 256KB
    When it is sent to the Edge Receiver
    Then it MUST be rejected with HTTP 413 Payload Too Large
    And no message MUST be produced to "logs-raw"

  Scenario: JWT is missing or invalid
    Given a request with no Authorization header (or an expired/malformed JWT)
    When it is POSTed to "/v1/logs"
    Then the Edge Receiver MUST respond with HTTP 401 Unauthorized

  Scenario: App name not in JWT grants
    Given a valid JWT with app_grants containing only "payment-api"
    And a payload with app_name "auth-service"
    When it is POSTed to "/v1/logs"
    Then the Edge Receiver MUST respond with HTTP 403 Forbidden

  Scenario: Attributes are flattened to dot-notation
    Given a payload with attributes containing nested objects like key "request" with value containing key "headers" with value containing key "host" with leaf value "example.com"
    When it is accepted by the Edge Receiver
    Then the produced DomainLog MUST contain attribute_keys including "request.headers.host" and the corresponding attribute_values_string entry MUST be "example.com"

  Scenario: Wildcard JWT grant allows any app_name
    Given a valid JWT with app_grants containing "*"
    And a payload with any arbitrary app_name
    When it is POSTed to "/v1/logs"
    Then the Edge Receiver MUST respond with HTTP 202

  Scenario: Payload attributes exceed memory guardrail limits
    Given a log payload containing an attribute object with 51 properties, or an array with 251 items, or a key exceeding 255 characters
    When it is POSTed to "/v1/logs"
    Then the Edge Receiver MUST fail-fast immediately with HTTP 400
    And no message MUST be produced to "logs-raw"
