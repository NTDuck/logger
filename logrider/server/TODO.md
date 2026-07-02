## Updated Architecture: Web Server as Auth + Render Layer

### Data Flow Overview

```
[External Apps] 
      | (HTTP POST /api/logs)
      v
[Backend API - Your Web Server]
      | (Validates JWT, rate limits, publishes to Redpanda)
      v
[Redpanda] ---> [Log Processor Workers] ---> [ClickHouse]
      |                                      |
      | (ERROR/CRITICAL alerts)              | (SQL queries for analytics)
      v                                      v
[Redis] (Dedup)                    [Web Server - Dashboard]
      |                                      |
      | (Alert triggers)                     | (WebSocket connections)
      v                                      v
[Telegram/Admin UI]                [Engineer Browser - Real-time view]
```

---

## Complete Requirements (Web Server Focus)

### 1. APIs (Web Server Only)

| Endpoint | Method | Description | Auth | Response |
| :--- | :--- | :--- | :--- | :--- |
| **`/`** | GET | Landing page with login link | Public | HTML page |
| **`/health`** | GET | Health check (DB, Redis, Redpanda) | Public | `{"status": "healthy", "services": {...}}` |
| **`/login`** | POST | Authenticate, create JWT | Public | `{"token": "jwt", "redirect": "/dashboard"}` |
| **`/logout`** | POST | Invalidate JWT | JWT | `{"message": "Logged out"}` |
| **`/dashboard`** | GET | Main dashboard HTML | JWT | HTML page (no log data embedded) |
| **`/api/me`** | GET | Get current user info | JWT | `{"username": "...", "role": "...", "apps": [...]}` |
| **`/api/logs`** | POST | **Ingestion endpoint** (publishes to Redpanda) | JWT (or API key) | `{"status": "accepted", "topic": "logs-app_name"}` |
| **`/api/ws`** | WS | WebSocket endpoint for real-time logs | JWT (in query param) | Real-time log stream |
| **`/api/admin/config`** | GET/PUT | Admin: Alert thresholds (stored in Redis/DB) | Admin JWT | Threshold config |
| **`/api/admin/users`** | GET/POST/PUT/DELETE | Admin: User management | Admin JWT | User objects |
| **`/api/admin/apps`** | GET | List all applications with logs (from ClickHouse) | Admin JWT | `{"apps": ["auth", "payment", ...]}` |
| **`/api/stats`** | GET | Health analytics (queries ClickHouse) | JWT | Aggregated stats per app |

---

### 2. Web Server Responsibilities

#### What the Web Server Does ✅
| Responsibility | Implementation |
| :--- | :--- |
| **Authentication & Authorization** | JWT creation/validation, role-based access control |
| **User Management** | Store users in PostgreSQL (or ClickHouse if preferred) |
| **Configuration Management** | Alert thresholds, retention policies (Redis/PostgreSQL) |
| **Log Ingestion** | Validate logs, publish to Redpanda topics |
| **WebSocket Gateway** | Manage connections, stream logs from Redpanda/Redis to browsers |
| **Template Rendering** | Serve HTML pages (SSR or static with API calls) |
| **API Proxy (Optional)** | Forward analytics queries to ClickHouse |

#### What the Web Server Does NOT ❌
| Responsibility | Handled By |
| :--- | :--- |
| **Log Storage** | ClickHouse (columnar storage for fast queries) |
| **Log Processing/Parsing** | Separate Worker services (consume from Redpanda) |
| **Log Retention Cleanup** | ClickHouse TTL or scheduled jobs |
| **Deduplication Logic** | Redis (managed by Workers or separate Alert Engine) |
| **Alert Delivery** | Workers publish to Redis, Web Server reads from Redis |

---
---

### 12. Summary: Web Server Responsibilities

| Feature | Web Server Role |
| :--- | :--- |
| **Landing Page (`/`)** | Render HTML with login link |
| **Health Check (`/health`)** | Check Redis, PostgreSQL, Redpanda, ClickHouse connectivity |
| **Login (`/login`)** | Validate credentials, return JWT |
| **Dashboard (`/dashboard`)** | Render HTML with user info and WebSocket client |
| **User Info (`/api/me`)** | Return username, role, allowed apps (from PostgreSQL) |
| **Log Ingestion (`/api/logs`)** | Validate, publish to Redpanda (DOES NOT store) |
| **WebSocket (`/api/ws`)** | Forward Redis pub/sub messages to browsers |
| **Admin Config (`/api/admin/*`)** | Read/write settings in PostgreSQL/Redis |
| **Stats (`/api/stats`)** | Query ClickHouse for analytics data |

---

This architecture cleanly separates concerns:
- **Web Server:** Lightweight, handles auth, UI rendering, WebSocket gateway
- **Redpanda:** High-throughput message buffer
- **Workers:** Log processing, storage, alerting (heavy lifting)
- **ClickHouse:** Optimized log storage and analytics
- **Redis:** Caching, deduplication, pub/sub for real-time events
