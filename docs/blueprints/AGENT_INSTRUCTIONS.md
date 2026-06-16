# Agent Implementation Guide: High-Throughput Logger

## 1. Introduction & Agent Mindset
You are an autonomous engineering agent tasked with building a mission-critical, high-throughput logging platform. You must approach this codebase with a specific mindset:
- **Performance-First:** You are building a system that must handle thousands of logs per second. Every line of code you write must optimize for zero-copy parsing, minimal heap allocations, and maximum throughput.
- **Strict Adherence to Idiosyncrasies:** This codebase uses a very specific dialect of Rust. You are explicitly forbidden from writing standard procedural MVC code or importing standard library types using `use std::...`. You must follow the exact Clean Architecture and syntactic rules detailed below.
- **Narrative Flow over Scaffolding:** Do not just scaffold empty files. Understand the domain (TimescaleDB, Redis Streams, WebSockets) and implement the vertical slices completely end-to-end.

## 2. The Domain Architecture
We are building a highly decoupled ingestion pipeline:
1. **Ingestion API (Rust):** Accepts JSON payloads (`LogRecord`). Validates them strictly. Pushes them to a Redis Stream (`logs:raw`). It includes a Token Bucket rate limiter per application and a global `MAXLEN` circuit breaker.
2. **Main Worker (Rust):** Consumes `logs:raw`. It parses the logs and performs batched `INSERT`s into a TimescaleDB hypertable (`logs`). It also calculates error signatures and deduplicates alerts using a sliding window counter in Redis.
3. **Web UI & Streaming:** The Worker pipelines parsed logs to a `logs:parsed` Redis Stream. A stateless WebSocket server reads this stream from the tail and broadcasts it to a Svelte frontend. 
4. **Analytics:** We use TimescaleDB Continuous Aggregates to pre-compute hourly error rates. A Grafana container connects directly to TimescaleDB to visualize this without custom UI code.

## 3. Strict Coding Conventions & Dialect

You must write Rust code exactly as described below. Failure to do so will break the project's strict `.clippy.toml` and `.rustfmt.toml` rules.

### Workspace Separation
You must separate concerns into distinct Cargo Workspace members:
- `core/domain/`: Pure business entities and pure errors. No I/O crates.
- `core/use-cases/`: Defines `boundaries.rs` (Input/Output traits) and `gateways.rs` (External I/O traits).
- `infrastructures/`: Contains the Axum REST handlers, TimescaleDB queries, and Redis clients.
- `configurations/`: The composition root (`main.rs`). It instantiates the infrastructures and injects them into the use-cases via `std::sync::Arc`.

### The Global Pathing Rule (Crucial)
You must **never** use `use` statements for standard library or external crate items in core logic. You must use fully qualified absolute paths, and you must **always prefix paths with `::`**.
- **Correct**: `::core::fmt::Debug`, `::std::sync::Arc`, `::core::result::Result`
- **Forbidden**: `core::fmt::Debug`, `std::sync::Arc`, `use std::sync::Arc;`

### Type Aliases & Minimal Allocations
Avoid raw `String` or `Vec` in boundaries. Use the centralized aliases:
- `pub type MaybeOwnedString = ::std::borrow::Cow<'static, str>;`
- `pub type Fallible<T = ()> = ::core::result::Result<T, ::anyhow::Error>;`
- `pub type Timestamp = ::chrono::NaiveDateTime;`

### Functional Programming & Builders
- Prefer functional programming (iterators, `map`, `fold`) over imperative `for` loops.
- Use the `bon` crate for all struct construction. `#[derive(::bon::Builder)]` with `#[builder(on(..., into))]` is mandatory to ergonomically handle `Cow` string conversions.
- Dependency Injection is purely manual constructor injection using `::std::sync::Arc<dyn Trait + ::core::marker::Send + ::core::marker::Sync>`. No DI frameworks.

### Error Handling
- Domain level: Pure Enums.
- Boundary level: Define strict error enums using `#[derive(::thiserror::Error)]`.
- Map Domain errors to Boundary errors using explicit `impl ::core::convert::From<DomainError> for BoundaryError`.

## 4. Implementation Steps (Vertical Slices)

When instructed to build a feature, refer to the `docs/issues/` directory, but execute them with this holistic understanding:
1. **Core Ingestion:** Build the Axum API and the TimescaleDB worker first. Prove data flows from HTTP to PostgreSQL.
2. **Live Streaming:** Implement the stateless `XREAD BLOCK` tailing in the WebSocket server and push to Svelte.
3. **Alerting & Resilience:** Implement the Redis sliding window for alert deduplication. Move Telegram dispatch to an asynchronous worker queue (`alerts:failed` with exponential backoff to `alerts:dead`).
4. **Auth & RBAC:** Implement `X-API-Key` validation and the hot-reloading Redis Hash polling for alert thresholds.

**Final Note to Agent:** Do not act like a generic code generator. Act as a senior Rust engineer who deeply understands Zero-Copy, Clean Architecture, and the specific `::` global pathing dialect of this project. Read the `CONTEXT.md` and `docs/schema.sql` before writing any code.
