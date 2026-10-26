# 13 — Open-Source Licensing / Clean-room / Provenance

**Not legal advice.** Have an appropriate reviewer check upstream licenses and project distribution obligations before public release.

## Product license proposal
- Choose **Apache-2.0** for original project code initially (patent grant, developer-friendly), with explicit owner approval before adding actual LICENSE file and copyright holder.
- Alternative MIT or dual license may be considered after license audit; **do not create fake copyright names/owners**.
- `README` must not assert a license has been finalized until official LICENSE exists.

## Dependencies and upstream
| Upstream | Use | Notes |
|---|---|---|
| DBeaver Community | public code/docs, behavior reference | Apache-2.0; preserve required notices if code reused |
| Gridline | reference design/driver ideas | Apache-2.0; if code adapted, follow provenance/NOTICE |
| `rust-oracledb` | Oracle Thin driver | UPL-1.0 OR Apache-2.0 per repository; verify pinned release |
| `rust-oracle` | OCI/thick fallback research | inspect current license and client redistribution separately |
| PhotoCraft/ArtCraft | philosophical reference | independently evaluate license, no asset copying |
| REA | investigation tool | MIT tool, distinct from rights in analyzed program |

## Clean-room discipline
- Feature ideas, workflows and independently written specs are allowed reference targets; **do not claim every reverse-engineering method is legally safe in every jurisdiction**.
- Prefer using open-source code lawfully with attribution over decompiling private programs when equivalent public source exists.
- Keep list of resources and copyright notices; do not rip SVG icons/themes/logos/trademarks from another app.
- When researching closed-source client, hand developer **behavioral requirements and tests** rather than proprietary implementation; document experiment version/source.
- Trademarks: choose original product name + logos; avoid confusingly similar branding.
- Respect EULAs, license terms, network/organization authorization and applicable law.

## License review gates
1. Collect direct + transitive dependency names, versions, sources, license IDs.
2. Verify native libraries and installer redistribution implications (especially OCI/Oracle Instant Client).
3. Check license compatibility with proposed project license; bundle required notices/attributions.
4. Keep SBOM, lockfiles, source attribution and third-party license inventory for release.
5. Check commercial font/icon/image usage separately from code license.
6. Publish sources for everything promised as Open Source; no hidden dependency on closed services.

## Public statements
- Correct: “Original open-source Database Client inspired by workflows found in multiple existing clients.”
- Incorrect without evidence: “Source-code reimplementation of DBeaver Enterprise” or “same features as all clients”.
- Speed claim needs measured benchmark, not aesthetic impression.

## Oracle caveat
Pure Rust thin driver aims to avoid Instant Client distribution; if OCI fallback ships, assess Oracle licensing, client packaging, platform libraries and runtime setup case-by-case. Driver support table must track versions and auth combinations tested.
