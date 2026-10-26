# ADR-0003 — Open Source Provenance & Licensing

**Status:** Proposed / Requires owner decision before release

## Context
Brand-new database client inspired by competitor UX. DBeaver Community and Gridline have publicly available Apache-2.0 code; proprietary app/assets are distinct. REA is for limited lawful behavioral research, not wholesale decompilation-and-copying.

## Proposal
- Original code Apache-2.0 (subject to owner's final choice and dependency audit).
- Keep record of copied/adapted OSS snippets with license headers and attribution/NOTICE obligations.
- For closed-source behaviors use independently written requirements and black-box tests. Do not copy assets, proprietary implementations or trademarks.
- Publish SPDX/dependency inventory and security disclosure.

## Decision gate
Validate license compatibility, third-party notices, packaging obligations and final rights holder before adding LICENSE/copyright attribution and public release. Legal counsel where appropriate.
