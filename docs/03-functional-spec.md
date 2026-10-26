# 03 — Functional Specification (cross-database)

## Requirement notation
- `FR-*`: cross-DB feature; `OR-*`: Oracle-specific; `NFR-*`: non-functional.
- `AC-*`: acceptance scenario. A feature is complete only if its ACs run and pass on stated environments.

## FR-001: Connections
- Create/edit/delete profile: `name`, `engine`, host, port, database/service path, optional role, environment tag, read-only flag, TLS options.
- Test Connection must return handshake/auth/TLS error with actionable info and **no password** in logs.
- Profiles editable without revealing stored secrets; password save opt-in into OS secure store. Reconnection uses bounded retry with user control.
- No DB connection automatically opened at app startup unless user explicitly opted in (post-MVP).
- **AC-001:** Invalid credentials return redacted auth error without process crash; valid Oracle 19c test succeeds on permitted test host.

## FR-002: Secrets / connection visibility
- Keychain/DPAPI/libsecret or secure OS alternative; no passwords/keys in app config, CLI args, JSON crash report, SQL history.
- Explicit profile tags `DEV` / `STAGING` / `PROD`, distinct visual banner for PROD.
- **AC-002:** OS store unavailable: refuse save or offer session-only credentials; plaintext file never written.

## FR-003: Explorer
- Connection → schemas → object groups → tables/views → columns/keys/indexes. Fetch children **on expand only**.
- Per-node loading/cancel/error/retry states; metadata cache lifetime limited and refreshable.
- Search objects with server-side scope or incremental search, not full metadata preload.
- **AC-003:** 5k+ object schemas remain navigable; refresh one node does not refetch whole connection tree.

## FR-004: SQL Editor
- Multiple query tabs per connection; SQL syntax highlighting per dialect; selection/statement execution; format optional.
- Unsaved tab state preserved where appropriate; ensure query text not logged to third-party telemetry.
- Editor sends query and typed bind values via core API; no string interpolation for parameters.
- **AC-004:** Run selection only executes selection; tab switching during a running query never blocks UI.

## FR-005: Query execution
- Render rowsets, affected-row counts, warnings, DB error code and location if available; expose elapsed client request timing clearly.
- Query concurrency controlled per connection; transparent transaction state; no surprise implicit commit on query tab close.
- **AC-005:** Bad SQL gives actionable error and leaves application usable; DB/network timeout surfaces a recoverable state.

## FR-006: Data grid
- Default `100` rows/page (configurable within safe bounds), server-side sort/filter/paging; UI virtualization independent of fetch.
- Type-aware cells: NULL distinct from empty string, timestamps and timezones displayed precisely, decimal numeric values not converted to lossy float, binary metadata only.
- Long text/LOB preview **bounded**; view/download on demand.
- **AC-006:** Scrolling a 100k-row *simulated data source* does not render 100k UI cells or fetch full dataset; measured in perf protocol.

## FR-007: Transaction control
- Explicit `auto-commit OFF` semantics for staged edits; transaction indicator displays active/dirty/in-doubt state.
- Read-only mode must be enforced where supported by DB session/credentials, not only SQL keyword matching. Client-side guards are defense-in-depth.
- **AC-007:** Rollback restores original table; connection close with dirty tx requires confirm/rollback choice, never silent commit.

## FR-008: Staged edit review (Beta)
- Editable only with stable row identity: PK/unique identity or vendor-supported row locator with explicit limits.
- Show diff + generated parameterized SQL and bind placeholders, affected rows expected. Use optimistic lock where feasible.
- **AC-008:** Concurrent row update conflict surfaced rather than silently overwriting; user can discard all staged changes.

## FR-009: Cancellation
- Driver capability reports `cancel=confirmed | requested | unsupported`; request should not freeze UI.
- Query status separates client abort from server-confirmed cancellation.
- **AC-009:** Long-running query can be cancelled where driver allows; UI input remains responsive during attempt.

## FR-010: Export
- CSV/JSON for selected rows / current page / full filtered result. **Scope chosen explicitly before export**.
- Stream into temp file with bounded memory; cancellation cleans up temp file, then atomic rename on success.
- **AC-010:** Export 100k rows with bounded memory and verify escaping/Unicode/NULL semantics.

## FR-011: History & saved queries
- Local history opt-in for queries, grouped by connection; never store bind values or sensitive result data by default.
- Search, rename, delete; user can disable history per connection, clear history.

## FR-012: Keyboard and theme
- Command palette, connection switcher, new query tab, run/stop, commit/rollback shortcuts.
- Dark/Light/System; keyboard focus visible; shortcuts configurable or documented.
- **AC-012:** End-to-end connection navigation and SELECT run without mouse after profile exists.

## Error UI contract
| Category | Show | Must not show |
|---|---|---|
| Auth/TLS | DB name, phase, DB code, remediation | password, token, wallet secret |
| SQL error | message, line/column if available, code | hidden bind values in public logs |
| Connection lost | retry/disconnect and tx uncertainty | claim mutation rolled back if unknown |
| Cancel | cancel requested / confirmed / unsupported | assert server stopped when unverified |

## Cross-DB dialect boundary
Formatting/completion/metadata introspection/paging/DDL/transaction capabilities vary per driver. Do not assume Oracle supports `LIMIT`, SQLite exposes schemas like PostgreSQL, or Redis returns SQL rowsets. Feature flags must be version- and capability-based.
