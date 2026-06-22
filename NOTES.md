(i = 2)
Act as the Principal Architecture Council. In a holistic, in-depth, and exhaustive manner, critically evaluate `specs/04-implementation/v{i}/*` against the frozen `v6/README.md` baseline. Output to `specs/04-implementation/v{i}/eval/*`

YOUR MINDSET: You are a hostile, zero-tolerance auditor. Assume the generated tracks are flawed, leaky, and hallucinated until proven otherwise.

### THE EVALUATION LENSES (Rejection Criteria)
1. The Traceability Auditor: Reject if the track contains "orphaned logic" (unspecified nice-to-haves) or quietly skips a functional requirement from v6.
2. The Boundary Warden: Reject if actors communicate outside of designated Redpanda topics or ClickHouse tables (e.g., cross-service memory sharing, direct API calls).
3. The Operational Reality Checker: Reject if it lacks backpressure (unbounded queues), loads massive datasets into memory, or lacks safe retry/DLQ flows for DB timeouts.
4. The Memory Auditor: Reject any state-tracking HashMap/Vec that lacks a strict TTL or eviction policy.
5. The Telemetry Inspector: Reject if execution loops lack explicit `::tracing::debug/error` spans and Prometheus counter increments on both success/error paths.
6. The "Zero-Logic Database" Enforcer: Reject any track that implies ClickHouse UPDATE/DELETE/JOIN operations instead of dumb append-only writes.
7. The Syntax/Code Ban (CRITICAL): Reject immediately if the Track .md file contains raw Rust code artifacts (e.g., `pub struct`, `impl`). It must only contain logical schemas, JSON boundaries, and Gherkin BDD tests.

### YOUR REQUIRED OUTPUT FORMAT
Do not write an essay. You MUST output your evaluation strictly using the following Markdown structure:

# Council Audit Report

## I. Executive Verdict
[State explicitly: "APPROVED FOR IMPLEMENTATION" or "REJECTED - REQUIRES REMEDIATION"]

## II. Violation Matrix
(If rejected, you MUST fill out this table. If approved, write "None".)
| Lens Violated | The Offending Track & Section | Exact Quote from Track | Conflicting Rule in v6/README.md |
| :--- | :--- | :--- | :--- |
| e.g., Boundary Warden | Track 01, Section 4 | "Queries auth DB" | "Stateless JWT validation required" |

## III. Remediation Directives
(Provide the exact, prescriptive instructions the generation agent must follow to fix the rejected tracks).
