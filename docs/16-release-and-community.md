# 16 — Open Source Release, Packaging & Community

## Packaging targets
- Windows x86-64 first (installer + portable variant if secure profile handling remains correct).
- macOS Apple Silicon/Intel later as tested, notarization/signing subject to project resources.
- Linux x86-64 later; desktop backend/runtime dependencies clearly listed.
- Release notes include supported DB versions, known limitations (particularly Oracle auth/cancel), checksums and source revision.

## Release checklist
- [ ] `LICENSE` exists with approved copyright holder; third-party NOTICE/attribution included.
- [ ] Dependency inventory, SBOM, OS/native redistribution check.
- [ ] Rust format/lint/unit/integration/E2E/security checks passing or documented; Oracle live matrix passes for claimed versions.
- [ ] Reproducible build recipe + artifact hash; signing where possible.
- [ ] Default security settings documented; profile migration tested; upgrade/downgrade limitations recorded.
- [ ] Startup/idle memory/large-grid benchmarks attached with reproducible conditions.
- [ ] No secrets, debug keys, bundled test credentials or PII in binary/source/assets.
- [ ] Changelog, screenshots, install/uninstall instructions, known issues.
- [ ] Manual smoke test on each supported OS and driver; accessibility scope honest.
- [ ] Security contact established; contributions and issue templates clear.

## Repo conventions
- Branches: `feature/T-103-query-editor`, `fix/issue-###`, `docs/...`.
- Commit: concise, meaningful, issue link; avoid huge mixed-purpose PRs.
- Issue labels: `bug`, `enhancement`, `oracle`, `ui-ux`, `performance`, `security`, `good-first-issue`, `blocked`, `needs-repro`.
- PR: affected requirements, screenshots if UI, tests run, fallback behavior, migration/security/perf impact.
- ADR for architecture changes, not every one-line code tweak.

## Public roadmap conduct
- Open issues and honest feature matrix; community can contribute provider drivers with contract tests.
- No false "DBeaver replacement" parity claim; publish supported features/version matrix and limitations.
- Responses to security reports prioritized privately, publication after remediation coordination.

## Release channels
- `nightly`: unstable, debug/help triage, never put user production connections at risk.
- `beta`: broad integration and performance QA, must avoid data-loss bugs.
- `stable v1.0`: gated on Definition of Done, Oracle live 19c verification and OSS compliance.

## Governance (starter)
Maintainer reviews feature scope, security-relevant code and releases. Contributors sign standard contribution acknowledgment via license terms only when necessary; do not impose CLA without explicit owner decision.
