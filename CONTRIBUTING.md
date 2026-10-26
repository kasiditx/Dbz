# CONTRIBUTING — Open-Source Database Client

Welcome! This project is **at blueprint stage**. Existing docs define intended requirements; no executable application source has been delivered yet.

## Before contributing
1. Read `README.md`, `docs/00-vision.md`, `docs/02-scope.md` and `docs/20-definition-of-done.md`.
2. Open issue/ticket before major features, new drivers, changes in license/UI architecture.
3. Prefer a single user-observable vertical slice over refactoring multiple layers at once.
4. Check whether new behavior requires an ADR; write tests and update docs when necessary.

## Coding principles
- Rust core separate from UI; driver-specific behavior behind capability flags.
- Do not log credentials or user query results; provide safe error messages.
- Always add unit/regression tests for changed behavior, and integration test for claimed DB support.
- No generated code or copied icons/assets from proprietary apps without rights.
- Don't claim an Oracle feature works until tested on supported Oracle version. Clearly mark mocks/skips.

## Pull requests
Include: Goal, requirement/ticket IDs, user-visible demo, risks, exact tests run, relevant before/after benchmark, screenshots redacted if UI, database/version tested, license impacts, rollback.

## Code of conduct
Be respectful, critique code not people, and avoid posting personal/company database information. A project Code of Conduct may be adopted before first public release.

## Licensing
License **proposed Apache-2.0, not finalized**. Contributors should not assume specific grant until maintainer adds approved `LICENSE`. Third-party code retains upstream obligations. See `docs/13-license-and-clean-room.md`.
