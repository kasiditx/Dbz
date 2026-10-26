# 11 — Initial Backlog (vertical tracer-bullet tickets)

**Notation:** `T-###`, `Blocked by`, `Demo`, `Acceptance`, `Skills`. Not completed; no code exists yet. Split/adjust after M0. Tickets should contain end-to-end observable user behavior, not purely component tasks.

## M0 — evidence & decision tickets

### T-001 — Audit specs, license, installed skills
- **Blocked by:** none
- **Demo:** requirements consistency report and a documented list of available skills for Claude Code + Codex.
- **Acceptance:** inconsistencies enumerated and resolved or flagged; Caveman Full configured; no claim that unavailable skill ran.
- **Skills:** `research`, `codebase-recon`, `caveman`, `review`.

### T-002 — Oracle driver feasibility proof
- **Blocked by:** T-001
- **Demo:** command/test that connects to authorized Oracle 19c, binds one NUMBER/string parameter, fetches typed results; honest `NOT TESTED` when unavailable.
- **Acceptance:** supported versions and limitations documented; secrets excluded; error/timeout handled; output evidence attached.
- **Skills:** `research`, `prototype`, `tdd`, `code-review`.

### T-003 — Native UI editor + large grid POC
- **Follow-up (2026-10-26):** real-input E2E 35/35, 3 bugs fixed, resize stall root-caused (not app code), final-build benchmarks; see `docs/evidence/T-003-followup.md`. Proposed next: **T-004a Tauri same-workload comparison** (plan in that doc); ADR-0001 still Pending.
- **Status (2026-10-26): DONE WITH GAPS** — POC + 51 unit tests + macOS benchmarks; real-input latency, manual resize, accessibility, cold start, Windows baseline NOT MEASURED. Evidence: `docs/evidence/T-003-native-ui.md`.
- **Blocked by:** T-001
- **Demo:** desktop window with SQL tab, 100k simulated virtual rows, keyboard navigation, resizable pane.
- **Acceptance:** benchmark script reports startup/RSS/frame latency on specified hardware; no full-row allocation by viewport renderer; accessibility gaps listed.
- **Skills:** `design-an-interface`, `prototype`, `qa`.

### T-004 — Driver API / UI ADRs
- **Blocked by:** T-002, T-003
- **Demo:** reviewed ADR-0001/0002 and typed contract design linked to POC evidence.
- **Acceptance:** scope stays M1-focused; rejected alternatives, tradeoffs, rollback options documented.
- **Skills:** `research`, `code-review`.

## M1 — Oracle vertical slice

### T-101 — Oracle connection profile and safe store
- **Blocked by:** T-004
- **Demo:** connection editor, Test, Save (securely), Connect/Disconnect; PROD/read-only display.
- **Acceptance:** AC-001/002; no plaintext secrets; wrong-pass and lost-network error test.
- **Skills:** `implement`, `tdd`, `code-review`.

### T-102 — Oracle lazy explorer
- **Blocked by:** T-101
- **Demo:** select connection, expand schemas and tables, read columns.
- **Acceptance:** AC-003; no full metadata preload; least-privilege test; loading/error states.
- **Skills:** `implement`, `tdd`, `database-optimizer`.

### T-103 — Oracle SELECT through SQL editor
- **Blocked by:** T-101, T-003
- **Demo:** type `SELECT ... WHERE id = :id`, bind parameter, run selection; result/status/error.
- **Acceptance:** AC-004/005 and OR-005; SQL/PLSQL parsing boundary tests; output never blocks UI.
- **Skills:** `implement`, `tdd`, `code-review`.

### T-104 — Virtualized typed result set with paging
- **Blocked by:** T-103
- **Demo:** initial 100 rows; next page; NUMBER/TIMESTAMP/NULL/LOB previews; no memory spike.
- **Acceptance:** AC-006, Oracle datatype integration, PERF-05/07/10 instrumentation.
- **Skills:** `implement`, `tdd`, `database-optimizer`, `qa`.

### T-105 — Cancel, timeout, tx state
- **Blocked by:** T-103
- **Demo:** long query with cancel; indicator requested vs confirmed, UI still responsive.
- **Acceptance:** AC-009; no false server cancellation claim; long-query regression tested.
- **Skills:** `diagnosing-bugs`, `tdd`, `code-review`.

### T-106 — Oracle MVP hardening review
- **Blocked by:** T-102, T-104, T-105
- **Demo:** verified read-only Oracle workflow with evidence; compatibility table updated.
- **Acceptance:** all M1 claims have live Oracle results; skipped tests prominently marked; security + perf reports attached.
- **Skills:** `qa`, `review`, `code-review`.

## M2 / M3 candidate tickets (expand after M1)
- T-201 PostgreSQL shared driver conformance — blocked by T-104
- T-202 SQLite shared driver conformance — blocked by T-104
- T-203 Manual transaction/commit/rollback end-to-end — blocked by T-103
- T-204 Safe staged row editor / conflict detection — blocked by T-203
- T-205 Export current/selected/full query safely — blocked by T-104
- T-301 MySQL/MariaDB core driver conformance — blocked by M2
- T-302 Oracle package/PLSQL explorer — blocked by T-102
- T-303 Cross-platform release package + signed artifact policy — blocked by Beta gates

## Work item quality gate
Ticket must state **one user-observable demo**, falsifiable acceptance tests, dependencies, rollback/risk, and how completion will be proven. Build high-risk spike ticket separately when implementation approach unknown.
