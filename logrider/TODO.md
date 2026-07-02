Based on your original project specification, here's what can be done directly with Redpanda/Redpanda Connect versus what requires custom code:

## ✅ Can Be Done with Redpanda/Redpanda Connect (No Code)

### 1. High-Speed Ingestion Matrix
- **HTTP API endpoint** - Use `http_server` input with `/ingest` path
- **WebSocket ingestion** - Use `http_server` with `ws_path`
- **Load balancing via Message Queue** - Kafka output writes to `raw-logs` topic
- **JWT Authentication** - Configure `http_server` with `jwt` or `oauth` settings
- **Automatic batching** - Built-in batch processing for Kafka writes

### 2. Log Parsing & Filtering Engine
- **Data cleaning/normalization** - Bloblang mappings in `mapping` processor
- **Character extraction** - `string().trim()`, `string().uppercase()`, etc.
- **Field validation** - Check for existence, type, and format
- **Conditional routing** - Use `if` statements to route ERROR/CRITICAL logs to different topics
- **Priority queue** - Produce to a separate `critical-alerts` topic
- **Dead letter queue** - Send invalid logs to an error topic

### 3. Real-time Log Viewer
- **WebSocket streaming** - `http_server` output with `ws_path`
- **Filtering by application/level** - `mapping` processor can filter messages server-side before WebSocket delivery
- **Live stream updates** - Kafka consumer automatically pushes new messages to WebSocket

### 4. Database Integration
- **ClickHouse insertion** - Built-in ClickHouse output with batch settings (`batch_size`, `batch_period`)
- **Batch writes** - Configure batch size (e.g., 10,000 rows) and flush period (e.g., 5 seconds)
- **PostgreSQL output** - Built-in if you choose PostgreSQL

### 5. Log Retention Policy
- **Delete old logs** - ClickHouse TTL (Time To Live) or partition deletion via scheduled `clickhouse-client` queries
- **Compress info logs** - ClickHouse's built-in compression + TTL to move old data to different storage

### 6. Permission Control
- **Basic RBAC** - Redpanda's OIDC/JWT support for role-based access
- **Topic-level permissions** - Redpanda ACLs can limit which applications can produce/consume

---

## ❌ Requires Custom Code (You Write)

### 1. AI-Powered Log Analysis & Classification (llama-cpp)
- **AI classification** - Need a custom service (Rust/Go/Python) that consumes from `normalized-logs`, runs the Llama model, and produces to `classified-logs`
- **Model inference** - Redpanda doesn't have AI capabilities built-in
- **Batch classification** - You'd write logic to classify logs in batches for efficiency

### 2. Alert Locking Mechanism (Redis Deduplication)
- **Deduplication logic** - Need a custom service that checks Redis for recent alerts before triggering
- **Redis operations** - `SETNX` or similar for lock mechanism
- **Alert throttling** - Logic to count errors per minute and only send one notification

### 3. Real-time Notifications (Telegram, WebSocket to ops dashboard)
- **Telegram bot integration** - Custom code to call Telegram API
- **Ops dashboard WebSocket** - Separate WebSocket for ops vs. general log viewer
- **Alert formatting** - Custom formatting for notifications

### 4. Web UI (Frontend)
- **Historical log loading** - AJAX/HTTP request to fetch past logs from ClickHouse
- **Real-time WebSocket client** - JavaScript to connect to `/logs/stream`
- **Filtering UI** - Application/level dropdowns that modify the WebSocket subscription
- **Dashboard UI** - Charts, error rates, status indicators

### 5. Application Health Analytics Reporting
- **Aggregation queries** - Write ClickHouse SQL queries for hourly error rates
- **Chart generation** - Frontend charting (Chart.js, D3, etc.)
- **Least stable systems identification** - Logic to rank applications by error frequency

### 6. Permission Control (Advanced)
- **User authentication** - Login/signup flow for engineers and admins
- **Application-specific permissions** - Logic to filter logs based on which apps the engineer manages
- **Admin threshold configuration** - UI + backend for saving alert thresholds

### 7. Old Log Cleanup (Advanced Retention)
- **Cron/scheduled job** - Need to schedule periodic cleanup if not using ClickHouse TTL
- **Selective deletion** - Logic to only delete INFO-level logs older than 7 days, not ERROR/CRITICAL
- **Compression jobs** - Custom logic to compress archived logs

---

## 🏗️ Summary Architecture

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
| AI Classification | ❌ | ✅ Llama-cpp service |
| Alert Deduplication (Redis) | ❌ | ✅ Custom worker |
| Telegram/WebSocket Alerts | ❌ | ✅ Custom worker |
| Web UI/Frontend | ❌ | ✅ HTML/JavaScript |
| Health Analytics | ❌ | ✅ Custom queries + frontend |
| User Permissions (Basic) | ✅ ACLs | |
| User Permissions (Advanced) | ❌ | ✅ Custom auth logic |
| Old Log Cleanup | ✅ ClickHouse TTL | |
| Selective Compression | ❌ | ✅ Custom job |

---

## 💡 Minimal Code Required

**Absolute minimum** you need to write:
1. **AI classification service** - Required for the bonus feature
2. **Alert deduplication + notification service** - Required for alerts
3. **Web frontend** - Required for Log Viewer

**Everything else** can be configured in YAML. This is a **very low-code** solution!

## 🎯 Recommended Implementation Order

1. **YAML-only first** (Week 1):
   - Set up ingestion, normalization, stream, ClickHouse storage
   - Get logs flowing from HTTP/WebSocket → ClickHouse
   - Basic WebSocket streaming to a simple HTML page

2. **Add custom services** (Week 2-3):
   - AI classification service
   - Alert deduplication + notification service
   - Enhanced web dashboard with charts

3. **Polish** (Week 4):
   - User permissions
   - Advanced retention policies
   - Analytics reporting
