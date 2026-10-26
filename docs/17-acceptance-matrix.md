# 17 — Acceptance Traceability Matrix

**Status for every row starts `NOT STARTED`; update with test ID, build revision and evidence.** Do not mark PASS based on mock-only run for DB integration requirements.

| Req ID | Minimum proof | Test lane | Release gate | Status |
|---|---|---|---|---|
| FR-001 / AC-001 | Oracle connect/test/disconnect, bad auth error safe | Oracle integration + UI | M1 | NOT STARTED |
| FR-002 / AC-002 | OS secure store or session-only; no plaintext | Security + OS E2E | M1 | NOT STARTED |
| FR-003 / AC-003 | Lazy schema loads, error/retry | Oracle + UI | M1 | NOT STARTED |
| FR-004 / AC-004 | Run selection and tab switch responsive | Editor/Oracle E2E | M1 | NOT STARTED |
| FR-005 / AC-005 | Typed binds, DB SQL error, elapsed times | Oracle integration | M1 | NOT STARTED |
| FR-006 / AC-006 | Page100 + virtual grid 100k scenario | Unit + perf + Oracle | M1 | NOT STARTED |
| FR-007 / AC-007 | Commit/rollback verified, prod guards | DB integration + security | M2 | NOT STARTED |
| FR-008 / AC-008 | Staged edit preview and conflict | E2E integration | Beta | NOT STARTED |
| FR-009 / AC-009 | Cancel UI state, no freeze, server status honest | Oracle integration + UI | M1 | NOT STARTED |
| FR-010 / AC-010 | Streamed export/escape/cancel | Unit + integration | Beta | NOT STARTED |
| FR-012 / AC-012 | Keyboard-only connect/execute | UI E2E | Beta | NOT STARTED |
| OR-001 | Oracle 19c service connect | Live 19c | M1 | NOT STARTED |
| OR-004 | Tables/views/columns/indexes/PK/FK | Live 19c | M1 | NOT STARTED |
| OR-005 | NUMBER and bind round trips | Live 19c | M1 | NOT STARTED |
| OR-006 | PL/SQL anonymous block and errors | Live 19c | M1 | NOT STARTED |
| OR-007 | Cursor/paging stable semantics | Live 19c | M1 | NOT STARTED |
| OR-008 | Explicit transaction/rollback | Live 19c | M2 | NOT STARTED |
| OR-009 | Date/Timestamp TZ/LOB/NULL round trip | Live 19c | M1/Beta | NOT STARTED |
| OR-012 | Cancel/timeouts/uncertain state | Live 19c | M1 | NOT STARTED |
| OR-014 | Quoted schema identifiers | Live 19c | M1 | NOT STARTED |
| NFR-001 | Startup cold p50 budget | perf machine | Beta | NOT STARTED |
| NFR-002 | Idle RAM budget | perf machine | Beta | NOT STARTED |
| NFR-003 | No UI thread stalls during DB work | perf/UI test | M1 | NOT STARTED |
| NFR-004 | Secrets and commit safety | security suite | M1/Beta | NOT STARTED |
| NFR-006 | No account or telemetry required | privacy UX | Beta | NOT STARTED |

## Evidence format
Each PASS needs `test name / environment / server version / agent skill invoked (optional) / date / commit / artifact`. `SKIP` must state reason and future gate. Bugs reference issue IDs.

## Release evidence condition
If database live tests are unavailable, keep status `NOT TESTED` and publicly label feature experimental/untested rather than `Supported`. Performance budgets require actual results under `08-performance.md`.
