# 12 — REA Research Protocol (authorized, clean-room)

## Why REA?
REA can help inspect shipped application behavior when source/docs/normal tests do not answer a specific question. Its skill is `reverse-engineer-anything`; MCP setup via official `npx rea-agents@latest setup`. **Not the default approach** for DBeaver Community because public source is available.

## Order of evidence preference
1. Upstream official public docs / source / license and stated behavior.
2. Observable UI black-box tests on legitimately obtained copy, with synthetic databases.
3. Interoperability investigation using REA only when authorized and essential.
4. Independently authored requirements/acceptance tests, implemented **without copying disallowed source or assets**.

## Approved research questions
- When a user clicks a table node, what metadata loads first? Record UI delay and request count from test DB if visible.
- How do common clients communicate long-running query state and cancellation uncertainty?
- Which Oracle object groups are surfaced and how quickly can an authorized user navigate packages?
- How do editors choose current SQL statement vs selected SQL with PL/SQL blocks?
- How do clients prevent accidental write in a production-marked connection?

## Explicit prohibited practices
- Unauthorized access or evasion of EULA, passwords, license/DRM checks, anti-tampering or authentication.
- Copying proprietary implementations, commercial-only code, UI assets, product names/icons, and private credentials.
- Intercepting third-party users' network traffic or extracting real data from production environments.
- Bulk cloning entire app code, automated decompilation intended to redistribute protected source.

## REA experiment template
```markdown
# REA-EXP-000 — Short title
- Target / lawful source / edition / version / checksum:
- Authorization / constraints:
- Question / hypothesis:
- Test environment (OS, test DB, synthetic data):
- Sources consulted before REA:
- Method: public docs / black-box / authorized REA
- Step-by-step reproduction:
- Observed behavior (facts only):
- Evidence (screenshots, logs, references, redacted):
- Interpretation / uncertainty:
- Our independently written feature requirement:
- Non-copying implementation notes:
- Reviewer / date:
```

## Research → spec → code handoff
Researcher outputs behavior spec and a test. Implementer sees spec + test + *permitted public documentation* but not proprietary decompiled code; maintain provenance. This is a process discipline, not a guarantee of legal immunity; consult legal advice for sensitive cases.

## Source-specific plan
- **DBeaver Community:** use Apache-2.0 public source as primary; comply with license if copying covered code. Avoid commercial binaries.
- **Gridline:** study Apache-2.0 implementation patterns publicly; copying introduces notices/attribution obligations.
- **Oracle SQL Developer:** rely on public user documentation and observable authorized behavior; no internal code/asset copying.
- **ArtCraft/PhotoCraft:** use architectural inspiration, not branding or protected assets.

## Output gate
Every claim must name source/version and observation; unknowns marked unknown. Only project-specific requirements enter code tickets, not a memory dump of reverse-engineered internals.
