-- Enable TimescaleDB extension
CREATE EXTENSION IF NOT EXISTS timescaledb;

-- Create Log Levels Enum
CREATE TYPE log_level_enum AS ENUM ('DEBUG', 'INFO', 'WARN', 'ERROR', 'CRITICAL');

-- Logs Table
CREATE TABLE logs (
    id UUID DEFAULT gen_random_uuid(),
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    application_name VARCHAR(255) NOT NULL,
    log_level log_level_enum NOT NULL,
    message TEXT NOT NULL,
    trace_id VARCHAR(255),
    error_code VARCHAR(255),
    PRIMARY KEY (id, timestamp)
);

-- Convert to hypertable
SELECT create_hypertable('logs', 'timestamp');

-- Indexes
CREATE INDEX ix_logs_timestamp_brin ON logs USING BRIN (timestamp);
CREATE INDEX ix_logs_app_level_time ON logs USING btree (application_name, log_level, timestamp DESC);

-- Continuous Aggregate Materialized View: app_error_rate_hourly
CREATE MATERIALIZED VIEW app_error_rate_hourly
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('1 hour', timestamp) AS bucket,
    application_name,
    COUNT(*) AS error_count
FROM logs
WHERE log_level IN ('ERROR', 'CRITICAL')
GROUP BY bucket, application_name;

-- RBAC: User Application Access
CREATE TABLE user_app_access (
    user_id UUID NOT NULL,
    application_name VARCHAR(255) NOT NULL,
    granted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, application_name)
);

-- AI Insights Table
CREATE TABLE log_ai_insights (
    log_id UUID NOT NULL,
    log_timestamp TIMESTAMPTZ NOT NULL,
    classification VARCHAR(255) NOT NULL,
    severity_hint VARCHAR(50),
    suggested_fix TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (log_id, log_timestamp)
);

-- Make AI insights a hypertable as well for time-series scaling and retention policies
SELECT create_hypertable('log_ai_insights', 'log_timestamp');
