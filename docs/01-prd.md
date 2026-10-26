# 01 — Product Requirements Document (PRD)

**Status:** Draft v0.1. **Maintainer:** Project owner. **Scope baseline:** `02-scope.md`. **Requirements:** `03-functional-spec.md`, `04-oracle-spec.md`, `08-performance.md`.

## Summary
Cross-platform Open-Source Database Client. Rust core; **Native UI candidate egui/eframe**, selected only after M0 ADR. Oracle first-class. Fast launch/low RAM; keyboard-first; strong SQL editor and virtualized data grid. Offline-first except explicit DB network connections and optional update checks.

## Problem → Outcome mapping
| Pain / hypothesis | Expected outcome | Evidence needed |
|---|---|---|
| Heavy startup | app ready quickly | cold/warm p50/p95 benchmark |
| Navigating deep trees | lazy explorer + search | task usability test |
| Large tables freeze UI | server-side pagination + virtual grid | 100k/1M-row dataset tests |
| Oracle treated as second-class | packages, SQL/PLSQL, LOB support | Oracle 19c compatibility matrix |
| Accidental production writes | read-only indicator + staged writes | safety tests + UX tests |
| Poor error visibility | exact actionable typed errors | scripted failures |

## Product boundaries
- Client sends SQL to user-specified DB server, does not host DB server.
- Supports local connections and network database connections; account creation not required.
- Open Source license decision before public release (`13-license-and-clean-room.md`).
- Optional future CLI/MCP integration: keep core command-oriented, but **not MVP**.

## Functional requirements / priorities
| ID | Requirement | Priority | Release |
|---|---|---|---|
| FR-001 | Saved connection profiles, connect/test/close | P0 | MVP |
| FR-002 | Secure credential storage via OS provider | P0 | MVP |
| FR-003 | Database explorer lazy schema/table/column navigation | P0 | MVP |
| FR-004 | SQL editor with highlighting, multiple tabs, run selected/query | P0 | MVP |
| FR-005 | Parameters/bind values; errors with location when available | P0 | MVP |
| FR-006 | Virtualized result grid + server-driven paging | P0 | MVP |
| FR-007 | Explicit transaction, commit/rollback, read-only safety | P0 | MVP |
| FR-008 | Staged edit review for tables with addressable rows | P0 | Beta |
| FR-009 | Query cancel, timeouts, no UI freeze | P0 | MVP |
| FR-010 | CSV/JSON export of selected/queried rows | P1 | Beta |
| FR-011 | SQL history/search + saved queries | P1 | Beta |
| FR-012 | Keyboard command palette, theme, settings | P1 | Beta |
| FR-013 | PostgreSQL + SQLite + MySQL/MariaDB drivers | P0/P1 | See Scope |
| OR-001.. | Oracle requirements | P0 | `04-oracle-spec.md` |
| FR-014 | Connection-specific environment tags / prod warnings | P0 | MVP |
| FR-015 | Execution plan viewer (Oracle) | P1 | after MVP |
| FR-016 | import wizard/ER diagram/code completion metadata | P2 | post v1 |

## Non-functional requirements
- NFR-001 startup cold p50 ≤2s target on baseline (re-measure at M0)
- NFR-002 idle RSS ≤200MiB target, 60s after ready, one idle profile
- NFR-003 no long-running DB work on UI thread; scroll/frame responsiveness threshold in `08-performance.md`
- NFR-004 safe defaults, redacted logs, no silent destructive commits
- NFR-005 Windows first; macOS/Linux release only after platform test gates
- NFR-006 offline-first with no mandatory remote auth/telemetry
- NFR-007 versioned stable Driver Adapter API per milestone

## Essential workflows
**WF-01 Oracle first connection**: New Connection → Oracle → host+port+service → Test → save (password in OS keychain) → Connect → visible banner/tags.

**WF-02 Read a table**: Expand schema lazily → choose table → open data tab → first 100 rows + row-count state → filter/sort request server → scroll without UI freeze.

**WF-03 Run a query**: New SQL tab → enter parameterized query → Run → progress/cancel → result grid or typed error → explicit transaction state.

**WF-04 Edit safely**: choose uniquely addressable row → stage change → review SQL + binds → commit or discard → refresh; avoid optimistic editing without conflict detection.

**WF-05 Long query**: Run long Oracle query → UI remains responsive → Cancel → show confirmed cancellation / requested but unconfirmed state, never promise server termination without driver evidence.

## Release cut-line
- **M1** Oracle connection, read-only SELECT, explorer, paging, error/cancel POC; **MVP** Oracle workflows complete enough for real read-only daily use.
- **Beta** staged edits, additional DB drivers, exports, reliability, first-run UX.
- **v1.0** quality/security/compatibility gates verified; use support tiers accurately.

## Open decisions (not blockers for writing the plan)
- UI stack: native egui vs Tauri fallback, ADR-0001.
- Oracle thin driver feature coverage by tested server versions.
- Exact Oracle 12c and 26ai test availability; 19c required baseline.
- Package-based versus universal binary distributions on each OS.
- Final public OSS license choice after dependency audit.
