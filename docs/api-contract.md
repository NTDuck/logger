# API Contract

## Ingestion API

### `POST /api/v1/ingest`
Ingests a single log record.

**Headers:**
- `X-API-Key`: (Required) Application-specific API key.

**Request Body:** (`application/json`)
Must strictly validate against `docs/json-schema.json`.

**Responses:**
- `202 Accepted`: Log accepted and enqueued successfully.
- `401 Unauthorized`: Missing or invalid `X-API-Key` header.
- `422 Unprocessable Entity`: Strict validation failed (missing required fields, invalid types, or unexpected properties).
- `429 Too Many Requests`: Rate limit exceeded for this API Key. Includes `Retry-After` header.
- `503 Service Unavailable`: Global circuit breaker engaged (ingestion queue full).

## Retrieval APIs

### `GET /api/logs`
Retrieves catch-up logs for the UI.

**Headers:**
- `Authorization`: Bearer token (User auth)

**Query Parameters:**
- `application_name`: (Optional) Filter by application name. Subject to RBAC filtering.
- `since`: (Optional) ISO8601 timestamp to fetch logs created after this time.
- `level`: (Optional) Filter by log level.
- `limit`: (Optional) Max logs to return (default: 100, max: 1000).

**Responses:**
- `200 OK`: Returns an array of log objects.
- `403 Forbidden`: User lacks RBAC access to the requested application(s).

### `GET /api/health-analytics`
Retrieves aggregated health metrics, merging continuous aggregate history with recent raw logs.

**Query Parameters:**
- `application_name`: (Required) Application to query.
- `window`: (Optional) Time window (e.g., `24h`, `7d`).

**Responses:**
- `200 OK`: Returns hourly error rates and recent raw error logs.

## Admin APIs

### `POST /api/admin/api-keys`
Generate a new API key for an application.
**Request Body:** `{"application_name": "string", "rate_limit_per_minute": "integer"}`

### `POST /api/admin/rbac`
Grant user access to an application's logs.
**Request Body:** `{"user_id": "string", "application_name": "string"}`

### `PUT /api/admin/alert-thresholds`
Update alert thresholds.
**Request Body:**
```json
{
  "application_name": "string",
  "log_level": "string",
  "threshold_count": "integer",
  "window_seconds": "integer"
}
```
