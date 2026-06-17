# API Auth + Rate Limiting

## What to build

Secure the ingestion API against noisy neighbors. Implement `X-API-Key` validation and token-bucket rate limiting per Application using Redis. For this slice, API keys and rate limits can be provisioned via environment variables or manual database inserts.

## Acceptance criteria

- [ ] API validates the `X-API-Key` header against the DB/Redis cache. Returns 401 if invalid.
- [ ] API enforces a Token Bucket rate limit per application via Redis `INCR`.
- [ ] API returns HTTP 429 with `Retry-After` header when limit is exceeded.

## Blocked by

- 01-core-ingestion.md
