--conversation=69f60bab-0502-4707-96ef-4036ea43d713


I am currently beginning phase 3; I already called speckit-specify and outputed to 03-hardened. I would like to use the "council" skill (from skills.sh) to improve on the quality of its output, ensure consistency, ensure full context, etc. (think about more, and include everything). Gimme the prompt (in code block)

Some hardening on prompt design:
- 02-disambiguated has a README.md which references ```specs\02-disambiguated\adrs\0001-clickhouse-over-standard-sql.md
specs\02-disambiguated\adrs\0002-custom-rust-workers-for-ingestion.md
specs\02-disambiguated\adrs\0003-redpanda-native-over-mq-abstraction.md
specs\02-disambiguated\adrs\0004-dedicated-redpanda-topic-for-priority-queue.md
specs\02-disambiguated\adrs\0005-strict-schema-policies-on-attributes.md
specs\02-disambiguated\adrs\0006-attribute-projection-over-attribute-promotion.md
specs\02-disambiguated\adrs\0007-clickhouse-native-ttl-for-retention.md
specs\02-disambiguated\adrs\0008-sidecar-table-for-ai-metadata.md
specs\02-disambiguated\adrs\0009-stateless-authorization-boundary.md
specs\02-disambiguated\adrs\0010-dedicated-edge-receiver-service.md
specs\02-disambiguated\adrs\0011-clickhouse-materialized-views-for-analytics.md
specs\02-disambiguated\adrs\0012-alert-fingerprints-for-deterministic-deduplication.md
specs\02-disambiguated\adrs\0013-deployment-model-single-binary-across-containers.md
specs\02-disambiguated\adrs\0014-in-memory-materializer-for-websocket-scaling.md
specs\02-disambiguated\adrs\0015-control-plane-configuration-architecture.md
specs\02-disambiguated\adrs\0016-attribute-flattening-at-the-edge.md
specs\02-disambiguated\adrs\0017-pipeline-fan-out-for-ai-consumer.md
specs\02-disambiguated\adrs\0018-dead-letter-queue-for-poison-pills.md
specs\02-disambiguated\adrs\0019-abandon-pipeline-state-machine-for-live-stream.md
specs\02-disambiguated\adrs\0020-concrete-soa-over-clean-architecture.md
specs\02-disambiguated\adrs\0021-pragmatic-performance-over-micro-optimizations.md
specs\02-disambiguated\adrs\0022-telegram-integration-and-rate-limiting.md
specs\02-disambiguated\adrs\0023-tumbling-window-for-alert-deduplication.md
specs\02-disambiguated\adrs\0024-implicit-log-processing-status.md
specs\02-disambiguated\adrs\0025-jwt-claim-based-rbac-for-app-ownership.md```
- 01-origin has a README.md
- 00-conventions has README.md which references to ```specs\00-conventions\01-architecture-soa.md
specs\00-conventions\02-rust-syntax-style.md
specs\00-conventions\03-axiom-utilities.md
specs\00-conventions\04-error-handling.md
specs\00-conventions\05-testing-cucumber.md
specs\00-conventions\06-ci-cd-devops.md
specs\00-conventions\07-constitution-principles.md```
- DO NOT use artifacts in 01-origin, ONLY use it if you need a look-back on the initial business requirements to e.g. check if the generated artifact complies, since the 02-disambiguated's README.md is already very comprehensive
- in 03-hardened I would like a complete requirements, covering exhaustively & holistically user stories, functional requirements, non-functional requirements. Certain rules: (1) Only provide non-code artifacts (things like domain model, data models (since those belong to implementation details), Gherkin tests (anything that belongs to implementation details)), (2) EVERY boundary must be exhaustively and clearly (albeit might just be in NLP, not code) defined (since the system is shaped by its boundaries; i want the shape of the system to be indubitably formed at this phase), (3) must satisfy everything in 02-disambiguated and must not conflict with 01-origin. Output format should be a README.md with references. Give me the following prompts:
    - (1) ```/speckit-specify {...}``` to form initial artifacts
    - (2) ```/council {...}``` to incrementally and methodically improve on the quality of the artifacts, (based on the aforementioned criteria and your own deduction as well)
- in 04-implmenetation, go with your recommendation. This 
    - plan
    - actionable tasks
- development
- validation & QA
- documentation & deployment

specify preset add workflow-preset architecture-governance agent-parity-governance explicit-task-dependencies toc-navigation security-governance