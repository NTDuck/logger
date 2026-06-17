# Live Stream + Web UI

## What to build

Implement the real-time log broadcast system. The Worker forwards parsed logs to a `logs:parsed` Redis Stream. A stateless WebSocket Server reads this stream from the tail (`XREAD`) and pushes logs to connected browser clients. The Web UI fetches historical logs from TimescaleDB on load, then connects via WebSocket for live updates.

## Acceptance criteria

- [ ] Worker pipelines valid logs to the `logs:parsed` stream.
- [ ] WebSocket Server tails `logs:parsed` using `XREAD BLOCK ... $` (stateless, no consumer groups).
- [ ] Web UI fetches catch-up logs from the API on initial load.
- [ ] Web UI connects to the WebSocket and dynamically appends new logs without page reload.

## Blocked by

- 01-core-ingestion.md
