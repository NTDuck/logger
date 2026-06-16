# Tech Stack Blueprint

This document details the exact architectural, structural, stylistic, and highly idiosyncratic conventions extracted from the `dcli`, `walkman`, and `breadcrumbs` repositories. Autonomous agents must strictly adhere to these conventions when building the Rust backend—**no exceptions**.

## 1. Strict Workspace Architecture
The backend is structured as a strict Cargo Workspace implementing Clean Architecture. MVC (`src/models`, `src/handlers`) is strictly forbidden.

- **`core/domain/`**: Pure Rust. No external I/O crates. Contains business entities, state machines, and pure domain errors.
- **`core/use-cases/`**:
  - `boundaries.rs`: Defines Input/Output traits (e.g., `CreateTaskBoundary` or `Accept<Request>`) and Request/Response DTOs.
  - `gateways.rs`: Defines traits for external I/O (e.g., `TaskRepository`, `UuidGenerator`).
  - `interactors.rs`: Implementations of business logic. Structs here implement `Boundary` traits and have `Gateway` traits injected via `std::sync::Arc`.
  - `models.rs`: Intermediate structures.
- **`infrastructures/`**: Contains physical implementations.
  - `adapters/`: Primary adapters like REST API handlers (Axum).
  - `gateways-impl/`: Secondary adapters like Database implementations (TimescaleDB).
- **`configurations/`**: The composition root (`main.rs`). Instantiates `gateways-impl`, injects them into `interactors`, and wires interactors to `adapters`.

## 2. Idiosyncratic Type Aliases & Primitives
You must use a centralized `aliases` crate (or `utils::aliases` module) for foundational types. Raw `String` or `Vec` are actively discouraged in boundaries.

- **Errors**: `pub type Fallible<T = ()> = ::core::result::Result<T, ::anyhow::Error>;`
- **Strings/Collections (Cow)**: 
  - `pub type MaybeOwnedString = ::std::borrow::Cow<'static, str>;`
  - `pub type MaybeOwnedPath = ::std::borrow::Cow<'static, ::std::path::Path>;`
  - `pub type MaybeOwnedVec<T> = ::std::borrow::Cow<'static, [T]>;`
- **Streams**: `pub type BoxedStream<T> = ::std::pin::Pin<::std::boxed::Box<dyn ::futures::Stream<Item = T> + ::core::marker::Send>>;`
- **Time**: `pub type Timestamp = ::chrono::NaiveDateTime;`

## 3. Strict Syntax, Style & Performance

### Performance-First & Functional Paradigms
- **Performance-first**: Every design choice must prioritize zero-copy parsing, minimal heap allocations, and maximum throughput.
- **Functional Style**: Prefer functional programming patterns (e.g., iterators, `map`, `filter`, `fold`) over imperative `for` loops and mutable state whenever possible.

### Fully Qualified Global Paths (No Imports)
You must **never** use `use` statements for standard library or external crate items in core logic. You must use fully qualified absolute paths. 
Furthermore, you must **always prefix paths with `::`** to ensure macro hygiene and prevent local scope shadowing.
- **Correct**: `::core::fmt::Debug`, `::std::sync::Arc`, `::core::result::Result`, `::anyhow::Result`
- **Forbidden**: `core::fmt::Debug`, `std::sync::Arc`, `use std::sync::Arc;`

### Bon Builders
Constructors must be generated using the `bon` crate. `#[builder(on(..., into))]` is heavily used to handle conversions (especially for `Cow` types).
```rust
#[derive(::bon::Builder)]
#[builder(on(::aliases::string::String, into))]
pub struct CreateTaskRequest {
    pub description: ::aliases::string::String,
}
```

### Derives & Feature Gating
Derives must not be merged. Serde and Wasm bindings are heavily feature-gated.
```rust
#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[derive(::bon::Builder)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct MyRequest { ... }
```

## 4. Linting & Formatting Rules (`.clippy.toml` & `.rustfmt.toml`)
The project utilizes extremely strict formatting and linting rules.

### `rustfmt` Configuration:
- `max_width = 120`
- `merge_derives = false` (Every derive must be on its own line)
- `imports_granularity = "Item"` and `group_imports = "StdExternalCrate"`
- `struct_lit_single_line = false`
- `use_small_heuristics = "Off"`

### `clippy` Configuration:
- `absolute-paths-max-segments = 0` (Enforces absolute path usage)
- `max-trait-bounds = 42`
- `too-many-arguments-threshold = 3` (Forces the use of config structs / `bon` builders instead of long argument lists)
- `struct-field-name-threshold = 0` and `enum-variant-name-threshold = 0`

## 5. Dependency Injection & Error Handling
- **No DI Frameworks**: Dependency injection is purely manual Constructor Injection using `::std::sync::Arc<dyn Trait + ::core::marker::Send + ::core::marker::Sync>`.
- **Async Traits**: Use the `#[async_trait]` macro for all Boundary and Gateway interfaces.
- **Errors**: Map Domain errors (pure Enums) to Boundary errors (`thiserror` enums) using explicit `impl ::core::convert::From<DomainError> for BoundaryError`.

## 6. Frontend Strategy
- **Grafana**: Zero-code operational dashboards, connected directly to TimescaleDB.
- **Svelte**: Custom Web UI served statically or via Nginx, handling WebSockets and the Admin panel.
