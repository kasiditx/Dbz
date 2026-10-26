# 14 — Database Driver Contract (design draft)

## Goal
All Database Engine adapters implement same **capability-driven** interface, while keeping SQL dialect-specific semantics. Reusing typed core makes desktop UI and later CLI/automation consistent; never hide an unsupported feature behind misleading generic method.

## Candidate traits (pseudocode, not compile-ready Rust)
```rust
trait DatabaseDriver {
    fn capabilities(&self, server_version: &DbVersion) -> Capabilities;
    async fn connect(&self, profile: PublicProfile, secret: SecretRef) -> Result<Session, DbError>;
}
trait Session {
    async fn list_schemas(&self, page: PageCursor) -> Result<MetadataPage, DbError>;
    async fn list_objects(&self, scope: SchemaRef, kind: ObjectKind, page: PageCursor)
        -> Result<MetadataPage, DbError>;
    async fn execute(&self, request: QueryRequest) -> Result<QueryHandle, DbError>;
    async fn commit(&self) -> Result<(), DbError>;
    async fn rollback(&self) -> Result<(), DbError>;
    async fn close(&self) -> Result<(), DbError>;
}
trait QueryHandle {
    async fn next_page(&mut self, count: usize) -> Result<RowPage, DbError>;
    async fn cancel(&self) -> Result<CancelOutcome, DbError>;
}
```

> Implementation may use generic async futures or boxed trait objects depending on Rust object safety/driver needs; actual interface defined and reviewed in ADR-0002.

## Typed data
- `DbValue`: `Null | Boolean | Int | ExactDecimal(String/BigDecimal) | Text | DateTimeWithZone | BinaryPreview | LobHandle | Json | Array | VendorSpecific`.
- Preserve native type name, scale, precision, nullability. Never silently convert Oracle NUMBER to float.
- `QualifiedIdentifier`: object parts and quoting rules stored separately from SQL bind values.

## Capability matrix contract
```text
supports_service_name
supports_sid
supports_tls_verify
supports_named_binds
supports_plsql_blocks
supports_server_side_paging
supports_cursor_fetch
supports_cancel_requested
supports_cancel_confirmed
supports_read_only_session
supports_staged_update
supports_schema_metadata
supports_lob_streaming
supports_execution_plan
```

Capabilities may depend on driver version, server version, permissions, and connection mode. Prefer negotiated/tested feature flags rather than static vendor labels.

## Query lifecycle
`Queued -> Running -> Streaming -> Complete | Error | CancelRequested -> CancelConfirmed/Uncertain`. Error is typed with vendor code, connection/session recoverability and safe user message. UI never blocks awaiting synchronous network call.

## Transactions
- Session carries `TxState`: none, active, dirty, committed, rolled-back, uncertain.
- Auto-commit differences represented explicitly. Oracle DDL implicit commits need warning and state reconciliation.
- Separate `readOnlyEnforcement`: `server`, `client`, `unknown`, reflected to UI.

## Paging
- `RowPage` has `rows`, `next_cursor` (optional), `has_more` (tri-state: yes/no/unknown), `column_schema`, `row_identity` when reliable.
- Arbitrary queries may use cursor streaming, not rewritten OFFSET SQL. Table browsing supports dialect-aware server paging and stable ordering where possible.
- Results are bounded: max pages in memory, LRU eviction, capped cell preview, on-demand LOB.

## Cancellation
`CancelOutcome = Confirmed | RequestedUnconfirmed | Unsupported | AlreadyFinished`; must map accurately. Async resource cleanup and session safety tests mandatory.

## Driver test conformance
For each engine: connection/auth/TLS, schema browse, named/positional binds, error typing, pagination, cancel behavior, transaction rollback, type round trip, Unicode and quoted identifiers. Mark unsupported cases explicitly rather than failing silently.

## Implementation discipline
Never use dynamic SQL interpolation for values; never assume shared SQL DDL. Pure driver adaptors cannot import GUI crate types. CLI/API automation subject to same read-only/transaction constraints.
