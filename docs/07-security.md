# 07 — Security & Database Safety Specification

## Security goals
No loss of user data from client side surprises, no credential leakage, no unsafe defaults, no unauthorized telemetry. Assume returned database content and remote hosts are untrusted. Client SQL editor is powerful; safety features mitigate accidents, not replace DB access control.

## Threat model
| Threat | Impact | Controls / tests |
|---|---|---|
| Plaintext credential persistence | leaked DB access | OS secret store; encrypted fallback only if audited; no-password-save path |
| Malicious/compromised DB endpoint | MITM/credential theft | TLS certificate validation where available; explicit user override warnings |
| SQL injection through UI-generated SQL | unintended mutation | typed binds for user values; safe quoted identifiers via dialect adapter |
| Production update without review | data loss | prod tag, read-only mode, staged writes, commit confirm |
| Stale row overwritten | hidden data corruption | optimistic compare/version or affected-rows checks; clear conflicts |
| Open transaction on disconnect | uncertainty | in-doubt UI state; never falsely promise rollback |
| LOB/huge rowset memory exhaustion | crash | bounded fetch/LOB preview/export streaming; cancellation |
| Debug logs exposing SQL/binds/passwords | privacy and secrets | structured redaction; local logs default off/minimal |
| Untrusted binary/plugin | code execution | static vetted drivers only; signed releases where feasible |
| Malicious issue/README text | agent prompt injection | treat REA/app/docs from external sources as untrusted data |

## Secret handling
- Profiles contain secret **references**, not secret values; secrets stored with Windows Credential Manager/DPAPI, macOS Keychain, or Linux Secret Service if available.
- Session-only credentials when secure store unavailable; *never* plaintext fallback without explicit reviewed alternative.
- Separate TLS trust configuration from password, never log connection URI containing credentials.
- Nonsecret local settings must use file permissions appropriate to OS.
- One-click clear recent queries and connection history; privacy off-by-default for telemetry.

## Read-only policy
- Read-only switch visible in UI and persisted per profile.
- Prefer server-enforced read-only session, role/privilege or transaction where supported. UI parser checking for `SELECT` is not security boundary; stored procedures and functions can mutate.
- When read-only cannot be enforced at server, label as **client-side guard** and recommend DB read-only user permissions; warn before unsafe SQL.
- No bypass through CLI/automation; future core command access shares authorization checks.

## Query & write safety
- Construct generated SQL with bind values; proper identifier quoting from driver (not a bind parameter).
- Never automatically commit unreviewed INSERT/UPDATE/DELETE/DDL.
- Handle Oracle implicit DDL commits prominently; rollback promise must be accurate.
- Distinguish `ReadOnly`, `AutoCommitOn`, `ManualTransaction`, `InDoubt` states.
- Staged edits require unique addressable row identity, review before execution and conflict detection; log sanitized local audit metadata if user opts in.
- Secrets not included in SQL history; query text can itself contain secrets, therefore permit per-connection history disable and clear.

## Safe defaults matrix
| Action | Default |
|---|---|
| Connect to new profile | explicit user action |
| Save password | opt-in secure store only |
| First use against `PROD` | read-only suggested/selected (users choose with warning) |
| Run selection | explicit action with active connection display |
| Edit data | staged, review before execution |
| Export full query | explicit row scope, confirmation for large extracts |
| Unknown certificate | reject by default; do not silent accept |
| Unsaved dirty transaction close | prompt; no silent commit |
| Query history | off or locally explained opt-in |
| Analytics/crash upload | none by default; opt-in with redaction |

## Security testing
- Wrong credentials/TLS server with invalid certificate; logs must never contain secrets.
- SQL generated from column/table names with quotes/special characters; prevent injection.
- Read-only mode blocks UI writes and enforces DB restriction when possible.
- Query cancellation during update must not falsely claim rollback.
- Staged edit conflict causes 0 rows updated or explicit conflict path; no overwrite.
- XML/CSV/JSON escaping tests for exported untrusted text; formula injection mitigation in CSV as documented.
- Dependency scan/license checks and Git secret scan in CI, on code actually present.

## Responsible disclosure
See root [`SECURITY.md`](../SECURITY.md). Do not post exploit proof containing live data or credentials. Releases with critical unfixed data-loss or credential vulnerabilities blocked.
