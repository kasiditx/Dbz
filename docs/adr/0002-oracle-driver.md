# ADR-0002 — Oracle Driver Strategy

**Status:** Proposed / Pending Oracle 19c live test

## Context
Oracle is a must-have. Core application aims to avoid Java and vendor client installation. Oracle-provided `rust-oracledb` Pure Rust thin client lists server-version support (12,18,19,21,26 families) but API/release maturity and feature coverage must be tested. Oracle 11g may require OCI/thick strategy.

## Candidate 1: rust-oracledb Thin
- Pure Rust, no Oracle Client libraries; favorable packaging. Maintained by Oracle.
- Risk: prerelease maturity and feature-specific gaps; verify bind, PL/SQL, metadata, LOB, cancel, TLS, session read-only, transaction behavior per pinned version.

## Candidate 2: rust-oracle OCI / ODPI-C
- Compatibility alternative for environments thin cannot cover.
- Tradeoff: external Oracle Client libs + platform installer/licensing + deployment overhead; may not be appropriate default for lightweight client.

## Experiment
1. Compile minimal POC with pinned crate and toolchain.
2. Auth to **authorized Oracle 19c** by service name. Test malformed creds, no plaintext logs.
3. Typed SELECT binds, arbitrary precision NUMBER, TIMESTAMP/TZ, CLOB/BLOB bounded read.
4. PL/SQL block, tx rollback, cancel long query, network loss.
5. Record driver behavior, unsupported APIs and total dependency/packaging burden.
6. If results fail required P0 tests, prototype OCI **only for missing capabilities**, not instant core rewrite.

## Decision
**Thin first, no blanket compatibility claims.** Full acceptance conditional on POC evidence and support matrix. 11g is outside baseline scope.

## Evidence log
- 2026-10-26: `oracledb` 26.0.0-beta.4 compiles in POC; blocking API; no cancel fn in source; live tests NOT TESTED. See `docs/evidence/T-002-oracle-driver.md`.
