# Project Conventions and Coding Standards

This document captures the holistic, in-depth, idiosyncratic, and exhaustive coding conventions, standards, architectures, and design styles extracted from the analyzed repositories (`volunteer-hub`, `breadcrumbs`, `litmus`, `walkman`, and `dcli`). All developments in this workspace must strictly follow these rules to maintain project cohesion.

---

## 1. Directory Structure & Architecture

The codebase strictly adheres to **Clean Architecture / Hexagonal Architecture** combined with **Domain-Driven Design (DDD)** principles.

### Layout Hierarchy
Projects are split into decoupled crates or packages within a workspace:
- **`core/domain`**: Holds pure business models, value objects, and domain-level errors. It must be free of third-party framework dependencies (except serialization libraries like `serde` and code generation helpers like `bon` or `sealed`).
- **`core/use-cases`**: Encapsulates application-specific business logic.
  - **`boundaries/`**: Defines the request models, response models, and boundary traits (interfaces) describing the input and output gates.
  - **`interactors/`**: The implementation of the boundaries that orchestrates the flow of data to and from entities, utilizing gateways.
- **`infrastructures/`** or **`gateways-impl/`**: Database implementations (e.g. SurrealDB, PostgreSQL, or in-memory repositories), configuration loaders, and external integrations (e.g. command executors).
- **`bindings/`** (specifically `wasm-bindings`): Bridges the Rust core application layer to the target environment (e.g. compiling to WebAssembly for Deno or browser contexts).
- **`axiom` / `axiom-derive`**: Internal shared library crates that define cross-cutting concerns (common traits, macros, extensions, and error serialization protocols).

---

## 2. Rust Syntactic & Import Conventions

### 2.1 Leading Double Colons (`::`)
To keep imports uniform and prevent shadowing conflicts, **fully qualified absolute paths starting with `::` are strictly preferred** for standard/core/alloc libraries, external third-party crates, and internal workspace dependencies.
- **Standard Library / Core**: Use `::std::sync::Arc`, `::core::option::Option`, `::std::vec::Vec`, `::core::default::Default::default()`.
- **Macro Invocations**: Use `::core::compile_error!`, `::std::format!`, `::std::vec!`.
- **External Crates**: Use `::serde::Serialize`, `::bon::Builder`, `::async_trait::async_trait`.
- **Workspace Dependencies**: Use `::domain::UserRole` or `::use_cases::boundaries::*`.
- **Leading `::` on `use` Statements**: Even import statements qualify external items with a leading `::`:
  ```rust
  use ::async_trait::async_trait;
  use ::domain::ChannelUrl;
  use ::futures::prelude::*;
  ```
- *Exception*: Paths starting with the `crate` keyword cannot have a leading `::` (e.g. use `use crate::boundaries::*`).

### 2.2 Minimal Imports & Trait Re-exporting
- Only import when absolutely necessary (e.g., bringing a trait into scope to use its methods).
- When bringing extension traits into scope, import them anonymously using `as _` to prevent namespace pollution:
  ```rust
  pub use crate::convert::IntoType as _;
  pub use crate::iter::IteratorExt as _;
  ```

### 2.3 Type Aliasing & Data-Type Design
- **Common Type Aliases**: Implement a central `aliases` module to wrap commonly reused types and standard wrappers:
  - `aliases::string::String` = `::std::borrow::Cow<'static, str>` (for zero-copy string management).
  - `aliases::path::Path` = `::std::borrow::Cow<'static, ::std::path::Path>`.
  - `aliases::result::Fallible<T = ()>` = `::core::result::Result<T, ::anyhow::Error>`.
  - `aliases::time::Timestamp` = `::chrono::DateTime<::chrono::Utc>` or `::chrono::NaiveDateTime`.
- **DDD Value Objects & Identifiers**: Encapsulate identifiers or values in single-field tuple structs (newtypes):
  ```rust
  #[derive(Debug, Clone)]
  pub struct VideoId(MaybeOwnedString);
  ```
  Implement `::std::ops::Deref` targeting the inner type (`MaybeOwnedString`), and provide `From` implementations in both directions to allow seamless conversions:
  ```rust
  impl ::std::ops::Deref for VideoId {
      type Target = MaybeOwnedString;
      fn deref(&self) -> &Self::Target { &self.0 }
  }
  ```

### 2.4 Ergonomic Builders
Use the `bon::Builder` code-generation tool to implement robust builder APIs.
- Configure builders with standard automatic conversions: `#[builder(on(_, into))]` or `#[builder(on(::aliases::string::String, into))]`.
- Enable builders on standard constructor functions by annotating `new` methods directly:
  ```rust
  #[::bon::bon]
  impl TaskDescription {
      #[builder(on(::aliases::string::String, into))]
      pub fn new(value: ::aliases::string::String) -> ::core::result::Result<Self, TaskDescriptionError> { ... }
  }
  ```

---

## 3. Local Blocks & Functional Programming Style

### 3.1 Expression-Oriented & Functional Style
Prefer an expression-oriented, functional style for local code blocks using loops, iterators, pattern matching, streams, and collection adapters.
- **Slice Patterns**: Use slice patterns and binding destructuring (e.g., `[firsts @ .., last]`) to handle arrays and slices elegantly.
- **Iterator Adapters**: Chain mapping and filtering actions:
  ```rust
  stdout
      .filter_map(|line| async { VideoDownloadEvent::from_line(line) })
      .map(Ok)
      .try_for_each(|event| async { video_download_events_tx.send(event) })
  ```
- **Zero-Copy Normalization**: When handling `Cow` types, structure transformations to avoid re-allocation if the input does not change:
  ```rust
  fn normalize(value: ::std::borrow::Cow<'static, str>) -> ::std::borrow::Cow<'static, str> {
      match value {
          ::std::borrow::Cow::Borrowed(val) => {
              let trimmed = val.trim();
              if trimmed.len() == val.len() {
                  ::std::borrow::Cow::Borrowed(trimmed)
              } else {
                  ::std::borrow::Cow::Owned(trimmed.to_string())
              }
          },
          ::std::borrow::Cow::Owned(val) => ::std::borrow::Cow::Owned(val.trim().to_string()),
      }
  }
  ```

---

## 4. Error Handling Architecture

Application error handling is divided into two distinct levels representing unexpected system failures and expected business logic validation failures.

### 4.1 Double-Wrapped Boundary Results
Use a double-wrapped signature on application boundaries and interactors:
```rust
async fn apply(
    self: ::std::sync::Arc<Self>, request: CreateEventRequest,
) -> ::axiom::result::Fallible<CreateEventResponse>;
```
1. **Outer layer (`Fallible<T>`)**: Represents infrastructure, database, or unexpected failures. It aliases `::core::result::Result<T, ::anyhow::Error>`. Infrastructure errors bubble up to this layer using the `?` operator.
2. **Inner layer (`CreateEventResponse`)**: Represents expected domain-level validation and business logic outcomes. It wraps `::core::result::Result<OkResponse, ::std::vec::Vec<ErrResponse>>`. A `Vec` of error responses is preferred to allow collecting multiple validation errors simultaneously.

### 4.2 Local Wrapping Macros
Define interactor-level macros `ok!`, `err!`, and `errs!` to cleanly format boundary returns:
```rust
macro_rules! ok {
    ($($ok:tt)*) => { ::axiom::result::Fallible::Ok(Response::Ok($($ok)*)) };
}
macro_rules! errs {
    ($($errs:tt)*) => { ::axiom::result::Fallible::Ok(Response::Err($($errs)*)) };
}
macro_rules! err {
    ($($err:tt)*) => { ::axiom::result::Fallible::Ok(Response::Err(::std::vec![ErrResponse::$($err)*])) }
}
```

### 4.3 Custom Error Serialization (`Erratum`)
Domain validation errors (represented as enums) must serialize into structured JSON payloads matching this schema:
```json
{
  "error": "kebab-case-error-code",
  "message": "Human-readable description derived from #[error(...)] attributes",
  "data": { "field1": "details" }
}
```
This is achieved via the `::axiom::Erratum` custom derive macro, which automatically implements `Serialize` for the error enums and maps variant fields into the nested `data` container.

---

## 5. WebAssembly & Frontend Integration

### 5.1 Deno and Deno Tasks
- Frontend applications are developed using modern **Svelte 5** (utilizing runes like `$state` and `$props` and rendering via `{@render children()}`) and TypeScript, executed and bundled through **Deno**.
- Compiling the Rust core to WASM is centralized in a Deno task:
  ```json
  "bind-wasm": "wasm-pack build ./backend/bindings/wasm-bindings --out-dir ../output --out-name <pkg-name> --target deno"
  ```

### 5.2 Promise Type Mapping
- In Rust's WASM-bindings, async exports return `Promise<T>` which resolves to a JS Promise.
- `Promise<T>` is aliased in Rust as:
  ```rust
  pub type Promise<T> = ::core::result::Result<T, ::wasm_bindgen::JsValue>;
  ```
- Implement `IntoPromise` on `Fallible` to translate unexpected anyhow errors and expected domain validation errors into structured `JsValue` rejections.

---

## 6. Testing Conventions

### 6.1 Behavior-Driven Development (BDD)
BDD is preferred for both unit and integration tests.
- **Fluent Litmus API**: For testing without external Gherkin (`.feature`) files, use the `litmus` builder:
  ```rust
  ::litmus::Feature::new()
      .scenario(::litmus::Scenario::<World>::new()
          .given("an empty repository", |_| {})
          .when("inserting user `Alice`", |repo| repo.save("Alice"))
          .then("it contains `Alice`", |repo| ::litmus::assert!(repo.contains("Alice"))))
  ```
  Use `#[rustfmt::skip]` on features/suites definition calls to preserve alignment.
- **Local BDD Macros**: When defining standard integration tests, write local macros `given!`, `when!`, and `then!` to encapsulate steps:
  ```rust
  given!(an empty repository);
  when!(adding task {0});
  then!(the repository contains task {0});
  ```

### 6.2 Parameterized Test Suites (`suite!` Macro)
To run the exact same behavioral test suites against multiple adapter implementations (e.g. an in-memory repository and a SurrealDB repository), wrap tests in a parameterized `suite!` macro:
```rust
task_repository_suite::suite!(handle = InMemoryTaskRepositoryWorldHandle);
```
The macro generates test functions at compile-time bound to the specified test environment handle.

---

## 7. Formatting, Linting, & DevOps

### 7.1 Rust Formatting (`.rustfmt.toml`)
Rust code formatting rules are highly opinionated and strictly enforced:
- **`imports_granularity = "Item"`**: Forces imports to be listed individually (one per line).
- **`group_imports = "StdExternalCrate"`**: Groups imports into Standard Library, External Crates, and Local Crate imports, separated by empty lines.
- **`merge_derives = false`**: Forces multiple derive attributes to be annotated on separate lines:
  ```rust
  #[derive(::core::fmt::Debug)]
  #[derive(::core::clone::Clone)]
  ```
- **`use_small_heuristics = "Off"`**: Disables collapsing structures into single lines.
- **`max_width = 120`**: Maximum character width per line.
- **`format_strings = true`** and **`wrap_comments = true`**.

### 7.2 Rust Linting (`.clippy.toml` / workspace)
- Clippy rules are defined at the workspace level.
- Common Clippy configurations include:
  - `too-many-arguments-threshold = 3` (flag functions with more than 3 arguments).
  - `max-trait-bounds = 42`.
  - Enforce check of private items: `check-private-items = true`.
- Common allowed warnings in workspace:
  - `derived_hash_with_manual_eq = "allow"`
  - `new_without_default = "allow"`
  - `missing_safety_doc = "allow"`
  - `let_and_return = "allow"`

### 7.3 Frontend Styles & Prettier
TypeScript/Svelte code is styled using **Tailwind CSS v4** and **Skeleton UI v3**.
- Formatting configurations in `.prettierrc`:
  - `"useTabs": false`
  - `"singleQuote": false`
  - `"trailingComma": "all"`
  - `"printWidth": 100`
  - Plugins: `prettier-plugin-svelte`, `prettier-plugin-tailwindcss`.

### 7.4 DevOps & CI/CD Pipelines
All projects run robust verification jobs via modularized GitHub Actions workflows:
- **GitHub Workflow Reusability**: General workflow files (e.g. `ci.yml`, `cd.yml`) delegate their jobs to specific, reusable workflow definitions (`lint.yml`, `build.yml`, `test.yml`, `dependencies-check.yml`, `publish.yml`).
- **Caching**: Actions utilize `Swatinem/rust-cache@v2` to minimize compilation overhead.
- **Compilation Checks**: Quick compiler verification is performed using `cargo check --workspace --all-targets --all-features` across a matrix of `stable`, `beta`, and `nightly` channels on `ubuntu-latest`, `macos-latest`, and `windows-latest`.
- **Fast Testing**: Tests are executed via `nextest` tool with:
  ```bash
  cargo nextest run --workspace --all-targets --all-features --no-tests=pass
  ```
  This is executed across the OS matrix and Rust channel matrix.
- **Security Scans**: Dependency verification is performed using `cargo deny` (via `EmbarkStudios/cargo-deny-action@v2` with `continue-on-error: ${{ matrix.check == 'advisories' }}`) and `cargo-audit` (via `taiki-e/install-action@v2`).
- **Continuous Delivery**: Merges to the default branch trigger automated publish steps that compile release binaries (`cargo build --release`) across the OS matrix and upload them to GitHub artifacts.
