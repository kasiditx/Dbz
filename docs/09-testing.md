# 09 — Test Strategy / CI Quality

## Test pyramid
1. **Unit tests**: Rust domain, driver capability routing, SQL paging generation, bind mapping, config migration, secret redaction, cancellation states.
2. **Component tests**: editor statement boundary, table grid virtualization/paging, async UI state reducers.
3. **Driver integration**: Oracle 19c (required for Oracle claim), PostgreSQL, SQLite, MySQL/MariaDB as scheduled.
4. **E2E desktop**: connection, schema browser, SQL selected text, data edit/rollback, export, long-query cancel; platform-specific automation where tooling supports it.
5. **Benchmark**: startup/RSS/first row/scroll/cancellation as `08-performance.md`.
6. **Security**: credentials, TLS, SQL parameterization, transaction safety, dependency licensing.

## Database test matrix
| DB | Must test | Optional |
|---|---|---|
| Oracle 19c | connect, bind SQL, paging, PL/SQL, metadata, typed values, tx, long-query | TCPS/Wallet if test infra present |
| Oracle 12c/21c/26ai | mark supported only with evidence | role-based metadata scenarios |
| PostgreSQL | connect, schemas, typed paging, tx, cancel | dump/restore not v1 |
| SQLite | file DB, explorer, paging, tx, edit safety | in-memory-only stress |
| MySQL/MariaDB | Beta connect/query/schema/data edit | extra engine variants |

## Test data & isolation
- Local / test-only DBs. Never use company production DB or PII as fixtures.
- Generate synthetic deterministic data; fixtures checked into repo contain no secrets.
- Per-test schema/database namespace; setup and cleanup run even when test fails.
- Credentials from environment/OS keychain/secret store, never source; `.env.example` with fake placeholders only.
- Tags: `unit`, `integration-oracle`, `integration-postgres`, `integration-mysql`, `ui-e2e`, `perf`.

## Oracle integration scenarios
- Host correct, credentials wrong, wallet invalid, network drop.
- SQL syntax error/ORA code; cancellation of long query and UI responsiveness.
- Bind variables and unusual column names, numbers, date/time/LOB/NULL.
- Transaction rollback, explicit commit, DDL implicit-commit warning.
- Privilege-restricted user browsing USER_*/ALL_*; no DBA privilege assumption.
- Row concurrent-update conflicts; session timeout + reconnect.

## CI plan
| Lane | Trigger | Must pass |
|---|---|---|
| Formatting/lint | every PR | `cargo fmt --check`, clippy where project supports |
| Unit | every PR | Rust tests + UI component tests |
| Static security | every PR | secret scan + dependency policy + source-license checks |
| SQLite E2E | PR/nightly | local reproducible fixture |
| PostgreSQL/MySQL | PR/nightly | dockerized disposable integration DBs where possible |
| Oracle 19c | scheduled/authorized runner | real permitted fixture; avoid falsely calling OCI/Oracle image free for all CI |
| Perf & desktop E2E | nightly/release | stable hardware / tracked baselines |
| Release matrix | candidate | OS builds, signing, installer, basic smoke tests |

## Coverage standard
Coverage percentage alone is insufficient. Every P0 FR/OR item has one unit or component test plus at least one integration/E2E flow where practical. Critical safety workflows need negative tests.

## Failure handling
- Any failed test must be reported with command, environment and reproduction; don't mark green unless rerun passes.
- Skipped Oracle live tests **must remain visible** (`NOT TESTED`) and block Oracle-support release claim.
- Flaky tests identified separately; don't simply rerun until lucky, diagnose timing/environment.
- All fixed bugs receive regression test referencing issue/requirement.

## Test quality
Each AC must be falsifiable against starting code. A scenario already passing before feature implementation does not prove ticket work. Prefer vertical integration from UI action to driver result where feasible.

## T-004a prototype validation
`python3 benchmarks/t004a/verify.py` records exact local test/fmt/lint/typecheck/build commands and results. `benchmarks/t004a/cua-workload.mjs` drives real desktop actions through CUA; `harness.py` collects process/frame/state evidence. Read the comparison report for QA failures, corrections, precise timing gaps and platform limitations. No live database test is implied.
