# 05 — System Architecture

**Primary principle:** no database driver, SQL dialect or I/O logic embedded in UI widgets. Treat UI as a consumer of typed commands/events.

## Shape
```text
Desktop UI (Native egui/eframe candidate; ADR gate)
  |  UI action / cancel / rendering
Application Services (sessions, tabs, query jobs, connection profiles)
  |  typed async commands, events + capability negotiation
Core Domain (schema, rows, exact values, transactions, error types)
  |  Driver trait/API
Adapters: Oracle | PostgreSQL | SQLite | MySQL/MariaDB
  |  network/native DB protocol
User-authorized database server(s)

Cross-cutting: OS Secret Store, Config DB, Diagnostics, Benchmark, License/SBOM
```

## Rust workspace suggestion (future source code; not part of this doc-only pack)
```text
apps/desktop/           Native UI shell and presentation models
apps/cli/               Later: scriptable core; no v1 feature dependency
crates/app-core/        Session lifecycle, query jobs, async command orchestration
crates/domain/          DataType, Value, Row, SchemaObject, Error, Capability
crates/driver-api/      DatabaseDriver/Connection/QueryStream traits
crates/driver-oracle/   Oracle adapter + dialect + metadata
crates/driver-pg/       PostgreSQL adapter
crates/driver-sqlite/   SQLite adapter
crates/driver-mysql/    MySQL/MariaDB adapter
crates/editor/          SQL editor behavior / tokens / dialect boundaries
crates/grid/            Grid model, paging, edit staging, virtual viewport
crates/profile/         Config/settings + secure credential reference
crates/observability/   tracing/log redaction, perf instrumentation
crates/test-support/    drivers fixtures / mock query streams
```

## Domain model
- `ConnectionProfile`: id, label, dialect, endpoints, environment tag, security policy (secrets = refs, not values).
- `ConnectionSession`: connected/connecting/reconnecting/tx-open/tx-uncertain/disconnected; worker state.
- `QueryRequest`: statement/selection, dialect, typed binds, page policy, timeout, transaction intent.
- `QueryJob`: UUID, status, cancellation token, timestamps, rows fetched, errors.
- `ResultSet`: columns (logical type, native type metadata, nullable), bounded page/stream handles.
- `SchemaObject`: type, qualified identifier, capabilities, parent, loading state.
- `EditBatch`: staged inserts/updates/deletes, expected row identity, bind values, optimistic verification.

## Execution model
1. UI sends typed command (`test_connection`, `browse_schema`, `run_query`, `fetch_page`, `cancel_job`, `commit`, `rollback`).
2. Application layer validates session permissions and dispatches async job. Bounded queue prevents runaway concurrent work.
3. Driver executes, emitting `QueryStarted`, `SchemaLoaded`, `RowsPage`, `Progress`, `Cancelled/Failed/Finished`.
4. UI updates only visible cells via virtualization. Large values are lazy references or capped previews.
5. Cancellation is best-effort with driver-specific confirmation semantics; job state never lies.

## Driver contract & capability negotiation
See [`14-driver-contract.md`](14-driver-contract.md). Required capability groups: connect, metadata, query, paging/fetch, transactions, cancellation, staged write, type mappings, security/auth options. Unsupported behavior must be explicitly disabled in UI.

## Resource budgets
- No loading all schemas/tables/rows on initial connection.
- Pool minimal and per-profile configurable, default few connections rather than unlimited. Short-lived connection for metadata if necessary.
- IO outside UI thread; bounded task queue; cancellation token and timeout per job.
- Avoid exact row count unless requested; large LOB never preload.
- Instrument memory, query first-row latency, UI frame time and startup phases.

## Extensibility vs safety
- Drivers compile into app for V1, **not arbitrary untrusted dynamically loaded plugins**.
- Do not promise stable third-party plugin ABI until versioned contracts/security audit.
- MCP/automation future only through typed core commands with explicit read-only scopes; cannot bypass UI safeguards.

## Technology choices (subject to ADR)
- Core Rust, Tokio async, typed traits, tracing with redaction.
- Native UI experiment: egui/eframe + wgpu, SQL editor experiment.
- Alternative UI: Tauri 2 + TypeScript + CodeMirror/Monaco, benchmark against same workload; no hard lock before M0.
- Local settings: SQLite/TOML/JSON can store **nonsecret** state; secret references only.
- Oracle thin Rust driver first, OCI adapter if compatibility justifies it; avoid bundling native client by default.

## Threat boundaries
Desktop process trusts its own source and authorized settings, **not** query text, database strings, rich values or remote certificates. Handle untrusted SQL result data as data. Escaping required for any HTML/WebView fallback.
