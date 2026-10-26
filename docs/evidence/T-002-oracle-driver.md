# T-002 — Oracle driver feasibility evidence (M0)

Date: 2026-10-26 · Host: macOS 26.5.2, Apple M4 Pro · rustc/cargo 1.99.1 · Skills: caveman (loaded); tdd/code-review/prototype skills NOT invoked via Skill tool — equivalent red-green-refactor + inline review followed (fallback).

## Verified (run on this machine)
- `oracledb` **26.0.0-beta.4** exists on crates.io; repo `oracle/rust-oracledb` active (last push 2026-10-19). License `UPL-1.0 OR Apache-2.0`, MSRV 1.89, optional `arrow` feature. Prerelease (beta) — maturity risk stands.
- API is **blocking/sync** (`oracledb::connect`, `Connection::query`, `Cursor: Iterator`). UI must call it from worker threads; async-trait driver contract (docs/14) needs a thread-bridge.
- 13 direct deps (aes, rustls, chrono, rand, sha2, uuid, whoami, ...); `cargo tree` for the POC = 123 lines. No Oracle Client libs needed to compile.
- Source inspection (crate src): **no cancel/break/interrupt function** found by name search; only `Connection::set_call_timeout`. Implication: `CancelOutcome::Unsupported` / timeout-only unless live test shows otherwise. Live behavior UNVERIFIED.
- `OracleNumber` implements Display/FromStr — exact NUMBER text possible without f64.
- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`: clean. `cargo test`: 24 passed, 1 ignored (live), 0 failed.
- Red phase observed for the first 19 tests (stubbed `unimplemented!()`); the quoted-identifier guard test was added together with its fix, red not observed.

## NOT TESTED
- **NOT TESTED: live Oracle** — connect, bind, typed fetch, wrong password, TLS, PL/SQL, tx, cancel, LOB, any version (incl. 19c). No authorized instance (user: none available).
- `cargo audit` / `cargo deny` not installed → CVE and license audit of the 114 locked packages NOT DONE.
- No performance measurement.

## POC artifacts
`crates/poc-oracle`: `config` (EZConnect build, injection chars rejected, Secret redaction), `error` (typed, redaction), `query` (client read-only guard, bounded fetch max_rows+1, exact NUMBER), `real` (oracledb adapter + error mapping), bin `poc-oracle`, ignored test `tests/live_oracle.rs` (fails if forced without env).
Run live later: set `DBZ_ORACLE_HOST/PORT/SERVICE/USER/PASSWORD` then `cargo test --test live_oracle -- --ignored` and `cargo run --bin poc-oracle`.

## Self-review findings (code-review fallback)
- Read-only guard is **client-side only** (`readOnlyEnforcement = client`): does not stop `SELECT ... FOR UPDATE` or functions with side effects; no `q'[ ]'` literal handling. Server-side read-only (user privileges / `SET TRANSACTION READ ONLY`) must be tested live.
- Adapter maps unknown column types to `Value::Unsupported`; LOB/TIMESTAMP/RAW mapping not implemented.
- `has_more` set only by caller truncation; adapter fetches max_rows+1 then `run_select` truncates — correct only if adapter returns >max_rows; unit-tested via mock only.
- Connection timeout is per-call (10 s); initial TCP connect timeout not configured.

## ADR-0002 impact
Status stays **Proposed**. Evidence supports "thin first" compile-feasibility only. Do not mark Accepted until live 19c results exist.
