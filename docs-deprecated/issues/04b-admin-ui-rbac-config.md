# Admin UI + RBAC + Config Hot-Reload

## What to build

Build the management interface for API Keys, RBAC `AccessScopes`, and Alert Thresholds. Implement the Redis Pub/Sub config hot-reloading mechanism so Workers and WebSocket Servers dynamically update their rules without restarting.

## Acceptance criteria

- [ ] Admin API and UI to manage users, keys, and thresholds in PostgreSQL.
- [ ] Admin API updates Redis Cache and publishes invalidation events via Pub/Sub.
- [ ] WebSocket server enforces `AccessScope` and forcibly disconnects clients when `rbac:invalidated` is received.
- [ ] Worker pulls dynamic Alert Thresholds from the Redis Hash, hot-reloading via Pub/Sub (with a 30s polling fallback).

## Blocked by

- 01-core-ingestion.md
- 02-live-stream-web-ui.md
- 04a-api-auth-rate-limiting.md
