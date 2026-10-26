# 02 — Scope / Boundaries / Release Cuts

Priority: **P0** must ship for promised release, **P1** useful next, **P2** future. A feature is not supported merely because a menu or driver stub exists.

## MVP (M1 + M2) — In scope
- [ ] **Oracle 19c live-tested**: connection by Service Name, secure credentials, test, disconnect, schema/table/column browsing.
- [ ] Read-only SQL/PLSQL execution where driver verified; typed result rendering.
- [ ] Basic SQL editor (highlight, run statement/selection, errors, multiple tabs).
- [ ] Result grid with **page size default 100**, page limits; lazy result/LOB handling.
- [ ] SQL cancellation requested + UI stays responsive; accurately report cancellation state.
- [ ] Transaction state panel, explicit commit/rollback; no implicit destructive commit.
- [ ] Clear production/read-only indicators + deny mutation when read-only is active.
- [ ] OS keychain integration or explicit safe no-save fallback; no plaintext persistence.
- [ ] Connection error handling + reconnect flows.
- [ ] PostgreSQL and SQLite after Oracle vertical slice (no false support claims).
- [ ] Targeted benchmark and regression suite.

## Public Beta — In scope
- [ ] MySQL/MariaDB connection, browse, query, data grid and key-based edits.
- [ ] Staged INSERT/UPDATE/DELETE with preview and optimistic conflict detection where possible.
- [ ] CSV/JSON export (streaming, explicit row scope); SQL history.
- [ ] Native keyboard palette, persistent workspace layout, accessible focus states.
- [ ] Oracle packages/procedures/functions/triggers/sequences metadata explorer.
- [ ] Oracle LOB viewer: truncation/pagination or on-demand reading, never eager load all.
- [ ] Windows installer + basic macOS/Linux builds if CI/QA verified.

## v1.0 — Must-have quality gates
- [ ] All claimed drivers tested against supported server versions with fixtures.
- [ ] Migration-safe profile/settings storage.
- [ ] No untriaged P0/P1 data-loss/security bugs.
- [ ] Build/release provenance, license/NOTICE, disclosure policy, contribution guide.
- [ ] Documented start-up/memory baselines, crash handling and cancellation limitations.

## Out of scope before v1
- Oracle 11g as confirmed supported (**requires separate compatibility research/driver**).
- Full DBA administration suite (backup/recovery, RMAN, partition management GUI, bulk DBA jobs).
- Embedded AI assistant/cloud account/mandatory telemetry.
- Redis (different data model; driver adapter may be added later).
- SQL Server support, ER modeling/diagram editor, database diff/migration designer.
- Reimplementation of commercial-only DBeaver/SQL Developer features not independently specified.
- Plugin marketplace, multi-user SaaS collaboration, centralized credential management.
- Automated destructive database schema migrations, arbitrary stored procedure debugger.

## Explicit database matrix
| Database | M1 POC | MVP | Beta | v1.0 claim |
|---|---:|---:|---:|---|
| Oracle 19c | P0 | P0 | P0 | Yes only after live tests |
| Oracle 12c/18c/21c/26ai | research | best effort | verification | Version-by-version evidence required |
| Oracle 11g | No | No | No | Not supported by default driver |
| PostgreSQL | later | P0 | P0 | Tested versions documented |
| SQLite | later | P0 | P0 | Tested versions documented |
| MySQL/MariaDB | No | optional dev preview | P0 | Tested versions documented |
| SQL Server / Redis | No | No | No | Future roadmap only |

## Scope change protocol
1. Create requirement/change request with who benefits and expected task flow.
2. Compare with performance, security, driver portability, release cut-line.
3. Prototype unknowns when risk is substantial.
4. ADR + traceable requirement + test + ticket before implementation.
5. Do not remove P0 work silently to add P2 features.
