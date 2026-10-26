# 08 — Performance Budgets & Benchmark Protocol

**All numbers below are engineering targets, not measured claims.** Do not claim “faster than DBeaver/Gridline” without matched experiment. Performance gate may be revised by ADR after M0 profiling.

## Baseline machine definition (minimum reproducible target)
- Windows 11, 4-core modern x86-64 CPU, 8 GiB RAM, SSD, 1920×1080, no extreme background load.
- Additional platform QA: Windows 10/11, macOS ARM64/Intel, Linux desktop as supported by runtime.
- Record CPU/RAM/SSD/GPU, OS build, app build hash, Rust version, UI renderer backend, DB server version, connection path, power plan, monitor scaling.

## Candidate budgets
| ID | Metric | Initial target | Measurement |
|---|---|---|---|
| PERF-01 | Cold startup: process launch → interactive empty workspace, p50 | ≤ 2.0 s | 10 runs, restarted cache environment documented |
| PERF-02 | Warm startup: p50 | ≤ 1.0 s | 10 runs |
| PERF-03 | Idle memory RSS (one idle workspace, no connected DB) | ≤ 200 MiB | measure after 60s, plus peak |
| PERF-04 | UI event responsiveness (no DB I/O blocking) p95 | ≤ 50ms | test input-to-update on representative task |
| PERF-05 | Large virtual grid scroll frame budget p95 | ≤ 33ms | >=100k simulated rows, 2k visible max |
| PERF-06 | Object Explorer expand metadata | no UI freeze | 5k objects; identify network-bound component |
| PERF-07 | Result set size | bounded client rows | default 100, no hidden full-table prefetch |
| PERF-08 | Query first-row | report **client overhead separately** | same server/data/SQL, warm conn |
| PERF-09 | Cancellation UI acknowledgment | ≤ 200ms target for button state | distinguish DB confirmed cancel |
| PERF-10 | LOB memory | does not scale with total LOB size | large CLOB/BLOB fixture + heap observation |

## What to time
1. Startup `process start`, `native UI created`, `first interactive frame`, `ready` recorded separately.
2. Idle RSS/private working set and CPU after 60s of inactivity; active peak during SQL/scroll.
3. SQL command: editor action → dispatch → DB request → first row → page complete → grid painted. Report server/network contribution when observable.
4. Browse: click schema → metadata roundtrip → first visible child list.
5. Grid: 100k/1M simulated and real DB rowsets; scroll, resize, filter, sort, copy.
6. Transaction: commit/rollback roundtrip; cancellation never blocks UI.

## Dataset plans
- **Synthetic UI:** 100k and 1M virtual rows, 20 columns, 2 long text columns, mixed NULL/Unicode/decimal/time values; does not fetch all in RAM.
- **Oracle dev fixture:** 100k rows, indexed/unindexed queries, NUMBER precision, timestamps, LOBs, multiple schemas.
- **Postgres/SQLite:** equivalent schema when practical; SQL plan and server behavior not identical.
- **Competitor comparison:** same hardware, OS, power state, database, table, query, page size, warm/cold distinction, allowed versions; metadata about Java heap for DBeaver.

## Protocol
1. Fix test build commit, clear/record app state, warm/cold distinction, connection pool status.
2. Run ≥10 repetitions for startup, ≥5 repetitions for other metrics, save raw samples.
3. Report median/p95, min/max when useful; identify outliers and do not drop without explanation.
4. Capture memory/CPU trace with tool name and version. Report whether measured on AC/battery.
5. Avoid loading different result sizes/row counts across products. If competitor can't express same workload, mark `not comparable`.
6. Baseline → change one variable → re-run. Large regressions require rollback or documented exception.

## Performance failure gates
- Perf sensitive pull request without before/after: cannot claim optimization, but may merge with marked outstanding benchmark if functionality critical.
- UI freeze or uncontrolled result/LOB allocation is release blocker.
- Any performance claim on README must link to reproducible benchmark environment, build, raw samples and script.

## Expected deliverables after M0
`benchmarks/README.md`, `benchmarks/hardware.md`, CSV/JSON raw samples, summary, repeatable scripts. Until those files exist, all listed budgets remain aspirational.
