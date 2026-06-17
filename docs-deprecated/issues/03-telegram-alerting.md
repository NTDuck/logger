# Telegram Alerting

## What to build

Implement the deduplication logic and initial alerting. The Worker calculates an error `Signature` and implements an `INCR` sliding window in Redis. If the (temporarily hardcoded) threshold is met, the alert payload is dispatched via a basic HTTP call to the Telegram API.

## Acceptance criteria

- [ ] Worker generates a `Signature` from `ERROR/CRITICAL` logs (fuzzy message hashing if `Error_Code` missing).
- [ ] Worker implements minute-granularity `INCR` in Redis (`alert:count:{sig}:{min}`).
- [ ] Worker successfully dispatches an HTTP request to Telegram when the count hits the threshold.
- [ ] Mutes subsequent errors in the same minute window.

## Blocked by

- 01-core-ingestion.md
