# Security Policy (draft)

This is a planning repository, not a released application. Database clients handle high-value credentials and data; any future vulnerability affecting auth, secret storage, SQL generation, transaction handling, unsafe exports or update channels should receive priority review.

## Reporting
Until maintainer publishes a dedicated security contact, **do not include passwords, connection strings with secrets, production tables, exploit payloads containing real user data, or company details in public issues**. Request private contact via the repository maintainer's future security policy/privately maintained channel. Do not invent a security email address.

## Disclosure practice
- Provide minimal reproduction using synthetic data and a disposable DB.
- State affected build, OS, database driver/version, impact and expected/actual result.
- Coordinate remediation before making exploit details public, particularly credential extraction or destructive SQL pathways.

## Release blockers
Any reproducible, unmitigated credential leak, silent destructive commit/data loss, or malicious automatic download/update execution blocks a release. See `docs/07-security.md`.
