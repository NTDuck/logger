# Alert Retry + Dead Letter Queue

## What to build

Decouple the Telegram HTTP call from the main Worker. The main Worker pushes alerts to `alerts:pending`. A Dispatcher Worker consumes it. Failures are routed to `alerts:failed` with exponential backoff retries, and eventually to `alerts:dead` for manual review.

## Acceptance criteria

- [ ] Main Worker stops calling Telegram directly and pushes to `alerts:pending`.
- [ ] Dispatcher Worker consumes and sends the Telegram alert.
- [ ] On HTTP failure, Dispatcher routes to `alerts:failed` and retries with backoff (1s -> 16s).
- [ ] Max retries met results in payload moving to `alerts:dead`.

## Blocked by

- 03-telegram-alerting.md
