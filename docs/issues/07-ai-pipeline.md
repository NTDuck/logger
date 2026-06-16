# Asynchronous AI Pipeline

## What to build

Implement the asynchronous AI Worker to analyze `ERROR` logs. The Main Worker routes errors to `logs:for_ai` with strict `MAXLEN` lossy backpressure. The AI Worker consumes this stream, classifies errors using an LLM, and writes insights to PostgreSQL. The UI displays these tags and provides an on-demand "Deep Analyze" button.

## Acceptance criteria

- [ ] Main Worker pushes to `logs:for_ai` with `MAXLEN ~ 1000`.
- [ ] AI Worker consumes from tail (stateless) and performs LLM classification, saving to `log_ai_insights`.
- [ ] UI Live Stream merges insight data into the log display.
- [ ] UI provides "Analyze" button triggering a synchronous deep-dive LLM query.

## Blocked by

- 01-core-ingestion.md
- 02-live-stream-web-ui.md
