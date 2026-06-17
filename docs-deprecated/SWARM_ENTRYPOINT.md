# Swarm Orchestration Entry Point

**Target:** The Lead Orchestrator Agent
**Mission:** Initialize the repository workspace and orchestrate a swarm of autonomous subagents to build the High-Throughput Logger according to the blueprints.

## 1. Initialization (Lead Agent Task)
Before spawning the swarm, you (the Lead Agent) must prepare the shared workspace:
1. Initialize the root Cargo Workspace (`Cargo.toml`) that includes `core/domain`, `core/use-cases`, `infrastructures`, and `configurations`.
2. Apply the strict formatting and linting rules by creating `.clippy.toml` and `.rustfmt.toml` at the root, mapping them to the rules described in `docs/blueprints/tech-stack.md`.
3. Create the centralized `aliases` crate inside the workspace.

## 2. Subagent Spawning Sequence
You will use the `invoke_subagent` tool to spawn worker agents in phases. **Worker agents must run in "share" workspace mode** so they can independently commit to different branches or directories without corrupting the main context, or they must coordinate locks if working directly on main.

For each issue, inject the following payload into the subagent prompt.

### Phase 1: Core Foundation
Spawn a subagent to tackle Issue 01:
**Prompt to Subagent:**
> "You are the Core Backend Engineer. Read `docs/blueprints/AGENT_INSTRUCTIONS.md` to understand your coding constraints (strictly use `::` global paths, `bon` builders, and `Cow` aliases). Read `docs/blueprints/tech-stack.md` and `docs/blueprints/schema.sql`.
> Your task is to resolve `docs/issues/01-core-ingestion.md`. 
> Build the Axum API and the TimescaleDB Rust Worker. Use `docs/infrastructure/scripts/test_ingestion.sh` to verify your circuit breaker works. Report back when acceptance criteria are met."

*Wait for Phase 1 to complete before proceeding to Phase 2.*

### Phase 2: Live UI, Alerting, and Analytics
Once Issue 01 is merged, spawn three parallel subagents to tackle Issues 02, 03, and 05.

**Subagent 2A Prompt (Issue 02):**
> "Read `AGENT_INSTRUCTIONS.md`. Your task is `docs/issues/02-live-stream-web-ui.md`. Implement the stateless WebSocket server in Rust tailing `logs:parsed`, and build the static Svelte UI frontend to consume it."

**Subagent 2B Prompt (Issue 03):**
> "Read `AGENT_INSTRUCTIONS.md`. Your task is `docs/issues/03-telegram-alerting.md`. Implement the sliding window `INCR` in Redis and the initial Telegram HTTP dispatch."

**Subagent 2C Prompt (Issue 05):**
> "Read `AGENT_INSTRUCTIONS.md`. Your task is `docs/issues/05-health-analytics.md`. Create the API endpoint that merges raw logs with the TimescaleDB continuous aggregate, and visualize it in Grafana (update `docker-compose.yml` if necessary)."

*Wait for Phase 2 subagents to complete.*

### Phase 3: Auth, Resilience, and AI
Spawn parallel subagents for the final enhancements.

**Subagent 3A Prompt (Issue 04a & 04b):**
> "Read `AGENT_INSTRUCTIONS.md`. Your task is to implement `04a-api-auth-rate-limiting.md` and `04b-admin-ui-rbac-config.md`. Build the `X-API-Key` middleware, the Redis Token Bucket, and the Admin UI in Svelte."

**Subagent 3B Prompt (Issue 06):**
> "Read `AGENT_INSTRUCTIONS.md`. Your task is `06-alert-retry-dlq.md`. Decouple the Telegram alerting into a dedicated async queue (`alerts:failed` -> `alerts:dead`)."

**Subagent 3C Prompt (Issue 07):**
> "Read `AGENT_INSTRUCTIONS.md`. Your task is `07-ai-pipeline.md`. Build the lossy `logs:for_ai` async queue and the dedicated AI Worker using the OpenAI API."

## 3. Verification & Completion
As subagents report back, you must review their changes. Use your own `run_command` tools to run `cargo clippy` and `cargo fmt --check` to ensure they did not violate the `absolute-paths-max-segments = 0` rule or insert any prohibited `use std::...` imports. 
If a subagent fails a lint, reply to their message with the compiler errors and instruct them to fix it. Do not write the code for them.
