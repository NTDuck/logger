### Log Asset Management Module

* **Data Structure Management:** Manages the data structure of a log record, which includes: `Application_Name`, `Log_Level` (`INFO`, `WARN`, `ERROR`, `CRITICAL`), `Message`, `Timestamp`, and `Trace_ID`.
* **Log Status Management:** Tracks and manages the processing lifecycle states of a log (e.g., *Raw Log Received*, *Normalized*, *Archived*).

### High-Speed Ingestion Matrix Module

* **High-Throughput API:** Design a high-concurrency API to continuously ingest log streams emitted from other applications (for example, every single user action in an external app triggers a log submission).
* **Message Queue Buffering:** The system must immediately push all raw log data into a Message Queue acting as a buffer for load balancing. This prevents system failure, as executing a direct SQL `INSERT` statement for every incoming log record would overload and crash the database within minutes.

### Log Parsing & Filtering Engine Module

* **Data Processing:** Background workers consume data from the Message Queue, parse the strings, clean the data, and persist it into the database.
* **Critical Alert Trigger:** If a worker detects a log record with a severity level of `ERROR` or `CRITICAL`, the system immediately triggers a critical alert event and routes it to a high-priority queue.

### Alert Locking Mechanism Module (Alert Coordination & Deduplication)

* **Real-time Notification:** Upon receiving a critical error event, the system automatically pushes real-time notifications via WebSockets to the operations engineer's monitoring dashboard and sends a message to Telegram.
* **Redis Deduplication:** Implements an alert deduplication mechanism using Redis. If the same error occurs consecutively 100 times within 1 minute, the system fires only a single notification to prevent alert fatigue for the engineers.

### Real-time Log Viewer Module (Visual Administration)

* **Live Stream View:** A user interface that displays a continuous, real-time scrolling stream of logs.
* **Instant Filtering:** Supports quick filtering by application type or error severity level without requiring a page reload.

### Additional Features

* **Role-Based Access Control (RBAC):** Engineers can only view logs belonging to the applications they manage.
* **System Configuration:** System Administrators have the authority to configure alert thresholds.
