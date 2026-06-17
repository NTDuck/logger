# Health Analytics Dashboard

## What to build

Implement the charting data pipeline using TimescaleDB Continuous Aggregates. Create a backend API that seamlessly merges the raw logs for the past 1-2 hours with the pre-aggregated materialized view for older data, presenting a unified chart to the Admin UI.

## Acceptance criteria

- [ ] Create TimescaleDB Continuous Aggregate view grouping errors by app, level, and 1-hour buckets.
- [ ] Backend API executes the "Merge Query" strategy (raw + aggregate) for real-time accuracy.
- [ ] UI visualizes the hourly error rates on a chart.

## Blocked by

- 01-core-ingestion.md
