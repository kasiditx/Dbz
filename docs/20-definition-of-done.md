# 20 — Definition of Done / Quality Gates

## Every ticket
- [ ] Scope IDs/AC documented; ticket depends on only approved decisions.
- [ ] Relevant skills checked and invoked if available and permitted; Caveman FULL applied to responses.
- [ ] Code compiles; tests actually run with exact command/results; new regression tests written.
- [ ] Invalid input, errors, cancellations and recoveries tested.
- [ ] No credentials in code, logs, fixtures, screenshots or issue text.
- [ ] Driver-specific behavior doesn't leak into generic core API without capability flag.
- [ ] No full-table eager fetch or UI thread blocking introduced.
- [ ] Changes reviewed independently (Claude/Codex), findings resolved or tracked.
- [ ] Docs/scope/ADR updated if behavior changed.

## Oracle MVP gate
- [ ] Authorized Oracle 19c database connected successfully in real integration test.
- [ ] Host/service auth failures handled safely, secure stored profiles verified.
- [ ] Tables/views/columns explorer lazy loading, quoted object metadata correct.
- [ ] Typed SELECT with binds, PL/SQL basic execution if claimed, paging and long value preview tested.
- [ ] SQL error + query cancellation states distinguish confirmed/uncertain.
- [ ] Read-only and manual transaction behavior tested; no silent commit.
- [ ] UI interactive during metadata/query/LOB loads.

## Beta / v1 gate
- [ ] Database support matrix and versions tested, no unsupported claims.
- [ ] Production write prevention and conflicts tested for each supported DB.
- [ ] Perf targets measured and published with raw samples; exceptions documented.
- [ ] Accessibility and multi-display scale UX tested on release OS.
- [ ] License/NOTICE, SBOM, dependency vulnerability and install/uninstall QA complete.
- [ ] No unresolved critical data-loss, credential leakage or unauthorized network behavior.
- [ ] README/download artifacts reproducible and attributed.

## Not Done if...
- Only mock Oracle tests pass and README says Oracle 19c supported.
- UI screenshot looks pretty but editor/query/grid cannot complete a real workflow.
- Benchmark is one run with no environment details or competitor workload mismatch.
- Any test skipped is presented as passed.
- Credentials are persisted in plaintext when secure storage unavailable.
- AGENTS.md says skill used but tool invocation absent/unavailable.
