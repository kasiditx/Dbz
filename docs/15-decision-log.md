# 15 — Decision Log / Open Questions

| Decision | Current direction | Status | Evidence/Next action |
|---|---|---|---|
| Core language | Rust | Proposed | capability and performance design |
| UI | egui/eframe + wgpu candidate | **Unconfirmed** | M0 Native UX/editor/grid spike, ADR-0001 |
| UI alternative | Tauri 2 + React/TypeScript | Experiment available; decision Pending | T-004a same-workload macOS evidence; Windows/accessibility gates remain |
| Oracle Primary | `oracle/rust-oracledb` thin | **Unverified live** | POC + 19c test, ADR-0002 |
| Oracle OCI fallback | `kubo/rust-oracle` adapter | Deferred | only if documented gap/11g need |
| Database support MVP | Oracle 19c, PostgreSQL, SQLite | Proposed | integration fixtures + version support matrix |
| MySQL/MariaDB | Beta | Planned | driver conformance after core |
| Redis/SQL Server | post v1 | Out-of-scope | demand / effort analysis |
| OSS license | Apache-2.0 proposed | **Approval pending** | dependency audit and copyright holder |
| Secret storage | OS credential providers | Accepted principle | platform prototype needed |
| Paging | virtual grid + bounded fetch | Accepted principle | test arbitrary SQL cursors |
| LOB | lazy view, capped preview | Accepted principle | Oracle type stress test |
| Read-only | server-enforced where possible + UI | Accepted principle | test each driver |
| Telemetry | none by default | Accepted principle | release docs / opt-in policy |

## Decision update template
```markdown
## DEC-YYYYMMDD-01: Title
- Context:
- Options:
- Evidence / measurements:
- Decision:
- Alternatives rejected and why:
- Consequences / security implications:
- Revisit trigger:
- Owner / date:
- Related ADR / tickets:
```

## Important unknowns to address early
1. Does thin Oracle driver expose reliable cancellation semantics and LOB streaming for required versions?
2. Does native egui deliver SQL editor UX and accessibility target without excessive custom engineering?
3. Can Oracle metadata load be efficient under least-privileged accounts?
4. How to expose transaction state safely across Oracle/PostgreSQL/SQLite differences?
5. Which OS versions/installers are feasible with test hardware and CI?

No unknown is silently assumed solved. Mark `Accepted` only after evidence/owner decision.
