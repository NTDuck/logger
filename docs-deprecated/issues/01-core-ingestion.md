# Core Ingestion (API → Queue → Worker → DB)

## What to build

Deliver the fundamental end-to-end ingestion pipeline. An HTTP API accepts a JSON `LogRecord`, drops invalid payloads, and appends them to a Redis Stream (`logs:raw`). It includes the circuit breaker (MAXLEN + 503) to protect Redis. A separate Rust Worker consumes this stream and bulk inserts the parsed logs into a TimescaleDB hypertable.

## Acceptance criteria

- [ ] Rust API exposes `POST /api/v1/ingest`.
- [ ] API strictly validates `Application_Name`, `Log_Level` (Enum), and `Message`. Returns HTTP 422 if invalid.
- [ ] API appends valid logs to `logs:raw` and returns HTTP 202.
- [ ] API implements the `MAXLEN` circuit breaker, returning HTTP 503 if the queue exceeds the threshold.
- [ ] Rust Worker consumes `logs:raw` and performs batched `INSERT`s into the TimescaleDB `logs` hypertable.
- [ ] A `curl` request to the API results in a row visible in the PostgreSQL database.

## Blocked by

- None - can start immediately
