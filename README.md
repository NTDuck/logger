# Logger Prototype

This repository contains the prototype implementation for the Log Collection and Application Error Monitoring System.

## Infrastructure Setup

**Local-First Prototype:** Currently, this prototype runs entirely in-memory and local-first. No Docker, Redis, Redpanda, or ClickHouse are required to run the basic ingestion and real-time viewing flows.

## Running the Components

The prototype is built as a Modular Monolith. You can run both the Edge Receiver (Ingestion) and WebSocket Server (View) in the same process, or isolate them using role-based entrypoint flags.

### Run All (Monolith Mode)
To run both the ingest component and viewer component simultaneously:
```bash
cd logger/server
cargo run -- --role all
```
*(The Edge receiver will listen on port 3000, and the WS server on port 3001. Logs are routed internally via broadcast channels.)*

### Run Edge Receiver Only
```bash
cd logger/server
cargo run -- --role edge
```

### Run WebSocket Viewer Only
```bash
cd logger/server
cargo run -- --role ws-server
```

## Testing the Prototype

### 1. Send logs via HTTP (Edge Receiver)
Use this `curl` command to post a log entry to the edge ingestion API. Make sure the Edge Receiver is running (port 3000).
*Note: Replace `<JWT_TOKEN>` with a valid JSON payload containing `"app_grants": ["payment-api"]` since the prototype parses raw JSON as a mock JWT.*

```bash
curl -X POST http://localhost:3000/v1/logs \
  -H "Authorization: Bearer {\"app_grants\":[\"payment-api\"]}" \
  -H "Content-Type: application/json" \
  -d '{
    "timestamp": "2026-07-01T12:00:00Z",
    "level": "ERROR",
    "message": "Payment failed",
    "app_name": "payment-api",
    "error_code": "PAY-101",
    "attributes": [
      { "key": "user_id", "value": { "id": "123" } }
    ]
  }'
```

### 2. Connect to the WebSocket Viewer
Test the real-time normalized log stream by connecting to the WebSocket Viewer on port 3001.
You can use `wscat` or a simple browser script:

```javascript
// Browser console test script
const mock_token = '{"app_grants":["payment-api"]}';
// Ensure the token is URL encoded if passing via query
const ws = new WebSocket(`ws://localhost:3001/v1/stream?token=${encodeURIComponent(mock_token)}`);

ws.onopen = () => {
  console.log("WebSocket connected.");
};

ws.onmessage = (event) => {
  console.log('Received normalized log:', event.data);
};

ws.onerror = (error) => {
  console.error('WebSocket Error:', error);
};
```