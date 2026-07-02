# Logrider

Logrider is a high-performance log ingestion and processing pipeline.

## Setup

Ensure you have Docker and `docker-compose` installed.

1. An environment file `.env` has been configured in the root with necessary ports and URIs.
2. Start the backing services via Docker Compose:
   ```bash
   cd persist
   docker-compose up -d
   ```
   This will start Redpanda, Redis, ClickHouse (with auto-initialization for the `logrider` DB), PostgreSQL, and the Redpanda Connect (Benthos) pipelines.

3. Start the web server:
   ```bash
   cd server
   npm install
   npm start
   ```
## Testing

Run the included test script to fire 500 POST requests in under 2 seconds to the `/api/logs` endpoint:
```bash
./scripts/test.sh
```
