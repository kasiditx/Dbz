# 04 — Oracle First-Class Support Spec

**Reference driver:** [oracle/rust-oracledb](https://github.com/oracle/rust-oracledb) (Pure Rust Thin, release maturity must be checked); **fallback exploration:** [kubo/rust-oracle](https://github.com/kubo/rust-oracle) / OCI requires native client libraries. Driver and DB version support cannot be assumed until tested.

## Compatibility policy
- Required live test baseline: **Oracle Database 19c** (authorized dev/test instance).
- Oracle Thin project's documentation identifies support for **12, 18, 19, 21, 26** family; **product claim remains per-version tested**, not blanket promise.
- Oracle **11g not in v1**; evaluate OCI/thick option later if actual demand. SID support, wallets, TCPS and auth modes require separate feature verification.
- Record client driver version, server DB version/edition, OS, auth mode in compatibility report.

## OR requirements
| ID | Requirement | Priority | Acceptance evidence |
|---|---|---|---|
| OR-001 | Connect with Host/Port/Service Name | P0 | success/error to real 19c test |
| OR-002 | SID/TNS alias support if driver allows | P1 | separate connection tests |
| OR-003 | Secure user/pass and TCPS options | P0/P1 | cert validation and wrong-cert tests |
| OR-004 | Schema/Tables/Views/Columns/PK/FK/Indexes | P0 | compare metadata against known fixture |
| OR-005 | SQL bind params, type mapping, SELECT/DML/DDL | P0 | parameterized integration tests |
| OR-006 | PL/SQL anonymous blocks, result/error handling | P0 | IN/OUT test if supported |
| OR-007 | Pagination and sorted/filterable result | P0 | stable ordering tests |
| OR-008 | Explicit COMMIT/ROLLBACK and read-only | P0 | before/after row verification |
| OR-009 | NUMBER/DATE/TIMESTAMP/TZ/RAW/CLOB/BLOB/NULL | P0/P1 | data round-trip fixture |
| OR-010 | Procedure/Function/Package/Trigger/Sequence explorer | P1 | metadata fixtures |
| OR-011 | Oracle Query execution plan viewer | P1 | read-only plan in permitted DB |
| OR-012 | Async cancel/timeout and tx uncertainty | P0 | long query controlled test |
| OR-013 | Role/privilege-aware navigation | P1 | least-privileged fixture |
| OR-014 | Quoted identifiers / schema-qualified names | P0 | mixed-case schema/table tests |

## Oracle-specific UX
**Connection form**: `Connection Name`, `Host`, `Port` (suggest default `1521`, editable), `Service Name` (primary), optional advanced connection descriptor, `Username`, `Password` (session-only or OS keychain), read-only, SSH tunnel optional later, TCPS/TLS advanced where supported. Label Service Name and SID clearly; they are not interchangeable.

**Object explorer**:
- schema groups: Tables, Views, Materialized Views, Indexes, Sequences, Synonyms, Procedures, Functions, Packages, Triggers; default load only high-value groups and fetch children on expansion.
- Metadata queries must use bind values, privilege-aware ALL_* / USER_* views; DBA_* views only when authorized; never assume `SELECT_CATALOG_ROLE`.

**SQL and PL/SQL**:
- Runs both SQL statements and PL/SQL blocks. Slash (`/`) behavior is a client batch separator in SQL*Plus-like tools, **not universally server SQL**; parser handles statement boundaries deliberately.
- DB bind variables use Oracle-style placeholders as supported; typed binds must map accurately. No interpolation of user data.
- SQL editor must distinguish SQL, PL/SQL block, comments, strings, quoted identifiers, and semicolons in PL/SQL blocks.

## Pagination protocol
- For compatible Oracle versions, use `OFFSET ... ROWS FETCH NEXT ... ROWS ONLY` with valid stable `ORDER BY` when needed. Do not silently append unstable ordering to arbitrary user SQL.
- For arbitrary query results, prefer cursor/fetch streaming; UI windowing and server pagination are different concerns. Preserve column order/types and repeated rows.
- Default page size 100; allow next page until exhaustion. Unknown total row count is valid; don't run a costly count on every request.
- A cancelled or failing fetch must return typed error and retain safe connection state (or mark state uncertain if needed).

## Type mapping test matrix
| Oracle type | UI representation | Round-trip rule |
|---|---|---|
| NUMBER | exact decimal string / arbitrary precision | never auto-cast to f64 for money/key |
| VARCHAR2/NVARCHAR2/CHAR | UTF-aware text | respect Oracle empty-string-is-NULL nuance |
| DATE | date + time fields (Oracle DATE has time) | timezone interpretation explicit |
| TIMESTAMP / WITH TZ | precision + offset | preserve offset, no silent timezone conversion |
| RAW / BLOB | size + hex preview/download | bounded fetch, lazy display |
| CLOB / NCLOB | truncated preview + on-demand view | cap memory; clarify encoding |
| NULL | distinct `<NULL>` visual | editable value distinct from empty text |
| ROWID | metadata/locator when applicable | never treat as globally stable PK |

## Transaction and DDL caveats
- Oracle DDL can cause implicit commits. Show warning and treat tx state carefully; never promise DDL is rollbackable.
- Read-only mode isn't just SQL text scanning. Use DB permissions/session constraints where possible; otherwise clearly communicate limitation and add UI guard.
- ORA errors display code, safe message and context. Do not expose credentials/connect descriptors with secrets.

## Integration fixture
Fixture tables for: PK, composite PK, no PK, NUMBER(38,*), Unicode, NULL/empty semantics, DATE/TZ, CLOB/BLOB payload of varying sizes, 100k rows, quoted identifiers, privileges, sequence, package/procedure, foreign key, concurrent edit conflict.

## Must-test scenarios (Oracle 19c)
1. Correct service, wrong service, wrong password, TLS verification failure.
2. `SELECT` with parameter, multiple data types, cursor paging and cancel.
3. Insert/update/delete in manual tx + ROLLBACK; verify exact data unchanged.
4. Date/time and NUMBER precision round-trip; CLOB/BLOB bounded preview.
5. Package metadata loads under limited user privileges.
6. Long query abort and disconnect with active transaction.
7. Database restart/network interruption without UI freeze or false success.

## Exit condition
**Do not mark Oracle supported until a permitted 19c test deployment passes OR-001, 004–009, 012, 014 as applicable; document unsupported/partial features explicitly.**
