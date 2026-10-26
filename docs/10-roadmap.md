# 10 — Roadmap / Milestone Gates

No calendar dates promised. Proceed when **exit gates** are met. Work in small verifiable vertical slices using `11-backlog.md`.

## M0 — Foundations & feasibility (research first)
- Read all docs; verify license and dependency policy.
- Oracle Thin-driver minimal CLI POC: `test_connection`, `SELECT :bind`, typed fetch, close; record supported versions/limitations. If no authorized Oracle instance, report incomplete.
- Native UI POC egui/eframe: editor + virtualized 100k-row grid + tabs; benchmark startup/RAM/interaction. Compare small Tauri spike only if cost/quality questions persist.
- Driver API draft, typed errors, capability matrix, secret store design.
- ADR-0001 UI choice and ADR-0002 Oracle connection strategy decision with evidence.
- Create baseline benchmark scripts and OS build recipe, fill acceptance matrix.

**Gate:** working minimal artifacts tested; documented unknowns; first 2 tickets unblocked. No false Oracle compatibility claim.

## M1 — Oracle read-only vertical slice
- Connect/test/save profile securely, visible PROD / read-only tags.
- Open Oracle schema/table/column list lazily.
- SQL editor SELECT with typed binds, basic highlighting, run selected statement.
- Paginated data grid 100 rows, typed columns, cancel/requested/confirmed states.
- Unit/UI Oracle integration tests and no-ui-freeze tests.

**Gate:** Oracle 19c real-test read-only workflow (or report M1 incomplete when server unavailable). Error handling and secret redaction pass.

## M2 — Full core workflow
- Manual transaction control and safe mutation guards; staged edit preview foundations.
- PostgreSQL + SQLite connect/browse/SQL/grid through same driver API.
- Multi-tabs, saved query/history opt-in, basic CSV/JSON export.
- Docs and typed cross-driver contracts; data grid robustness.

**Gate:** 3 engines pass shared conformance suite at their declared support level.

## M3 — Beta / polish
- MySQL/MariaDB full core support.
- Oracle packages/procedures/function explorer, large LOB UI, dialect correctness.
- Staged insert/update/delete with conflict detection; server-side filters/sort.
- UI theme, keyboard palette, accessibility findings fixed or documented.
- Security audit, perf budget evidence, platform installers beta.

**Gate:** no known data-loss/credential vulnerabilities, benchmark reproducible, v1 risk board manageable.

## M4 — Public Beta to v1.0
- Regression suite on supported DB versions, reproducible builds, crash-free handling.
- Licensing/NOTICE/SBOM, README and migration docs, release notes, checksum/signing if available.
- Community issue templates, triage SLA expectations, security contact.
- Compare reference clients only with measured apples-to-apples scenarios.

**Gate:** `20-definition-of-done.md` + `17-acceptance-matrix.md` complete. Publish claims only for verified capabilities.

## Change management
- Roadmap not a wish list. Any new feature competes with M1–M4 capacity; update scope and decision log before engineering.
- When one tech risk blocks release (e.g. Oracle thin driver missing feature), isolate fallback rather than rewriting whole app in panic.
