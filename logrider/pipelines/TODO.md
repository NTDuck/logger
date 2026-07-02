- create ingest.yaml that consumes log from the HTTP endpoint (exposed by server), updates the webserver websocket, normalize (via mapping pipeline), then produces to topic logs-ingested
- create persist.yaml that consumes from topic logs-ingested and batch writes to clickhouse db, then updates the webserver websocket

| Component | Redpanda Built-in | Custom Code |
|-----------|-------------------|-------------|
| HTTP Ingestion API | ✅ `http_server` | |
| WebSocket Ingestion | ✅ `http_server` with `ws_path` | |
| Message Queue | ✅ Kafka topics | |
| JWT Authentication | ✅ OIDC/JWT config | |
| Log Normalization | ✅ Bloblang mapping | |
| Critical Alert Detection | ✅ Conditional routing | |
| Priority Queue | ✅ Separate topic | |
| WebSocket Streaming | ✅ `http_server` output | |
| ClickHouse Storage | ✅ ClickHouse output | |
| Old Log Cleanup | ✅ ClickHouse TTL | |
