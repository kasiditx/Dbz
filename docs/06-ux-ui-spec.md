# 06 — UI / UX Design Specification

## Product feel
Modern developer tool, visually quiet, fast and discoverable. Dark/Light/System mode. No copy of competitor logo/layout assets/icons. Keyboard-first, but fully usable by mouse. Clear connection context. Text-heavy workflow should remain readable on 1080p and HiDPI.

## Workspace layout (wireframe)
```text
+-------------------------------------------------------------------+
| App  [Connection: Oracle DEV ▾] [Search / Command ⌘K]  [Settings]  |
+---------------+---------------------------------------------------+
| EXPLORER      | [SQL 1] [Table: ORDERS] [+]                       |
| Oracle DEV    |---------------------------------------------------|
|  Schemas ▾    | SELECT * FROM app.orders WHERE id = :id;           |
|   APP ▾       | [Run] [Cancel] [Read-only ✓] [Tx: none]            |
|    Tables ▾   |---------------------------------------------------|
|     ORDERS    | Results (100 rows/page)  Filter  Export           |
|     USERS     | +------+---------+-------+                       |
|    Views      | | ID   | STATUS  | TOTAL |                       |
|    Packages   | +------+---------+-------+                       |
+---------------+---------------------------------------------------+
| Connected • Oracle 19c • DEV | Query 86ms | Page 1 / unknown     |
+-------------------------------------------------------------------+
```

## Navigation
- Sidebar: connections and schema objects; expandable node lazy load; current node can be refreshed.
- Editor: one or more tabs, syntax colors, current connection banner, run selection/current statement; Ctrl/Cmd+Enter run.
- Result panel: typed columns, sort/filter, keyboard arrow nav, copy selection, explicit paging status.
- Footer/status: connection, environment, transaction mode, query duration, async loading/cancel status.
- Command palette: Ctrl/Cmd+K; search actions/connections/tables; fuzzy match bounded.

## High-priority screens
1. **Welcome / connection quick start:** choose engine, create first connection; no account requirement.
2. **Connection editor:** engine-specific fields; secrets toggle; Test; understandable error.
3. **Workspace:** Explorer/editor/results with resizable panes and clear active connection.
4. **Data editor:** staged diff, SQL preview, conflict warning, explicit commit/rollback.
5. **Query progress:** running progress/stop; cancel requested/confirmed distinct states.
6. **Settings:** font/size/theme/keyboard, fetch page size, privacy/history, connection timeouts.

## Oracle UX specifics
- Service Name primary with tooltip explaining SID difference; advanced descriptor optional.
- Database tree groups `Tables`, `Views`, `Materialized Views`, `Packages`, `Procedures`, `Functions`, `Sequences`, `Synonyms`, `Triggers`, `Indexes`.
- PL/SQL tab shows errors/DB codes; packages/procedures can open definition where grants allow.
- Oracle `NUMBER` shows exact precision string, `DATE` includes time, LOB preview is capped.

## Design tokens (initial proposal; tune in prototype)
- Spacing scale 4/8/12/16/24px; density `comfortable / compact`.
- Font: system UI + readable monospaced editor font; fallback available on each OS.
- Focus ring/high-contrast states; no color-only status encoding.
- Semantic colors: success, warning, danger, info, muted; avoid decorative gradients that lower legibility.
- Minimum clickable area target ~32px desktop; scalable for accessibility.

## Interaction states
Every async component: `idle | loading | partial | success | empty | error | retrying | cancelled`. Never leave indefinite spinner without cancel/retry/timeout policy.

## Accessibility
- Keyboard and visible focus; shortcuts advertised and override where possible.
- Screen-reader accessibility path must be evaluated on actual UI stack before v1 claim. Native egui and Tauri have different tradeoffs; record ADR.
- Zoom/UI scaling, reduced motion preference, color contrast; long strings truncation + full copy.

## UX acceptance tasks
- **UX-01:** Create Oracle DEV connection in under 2 minutes with known credentials, no documentation help.
- **UX-02:** Navigate to table and display first page in at most 4 high-level actions after connection.
- **UX-03:** Run selected SQL without running whole tab; keyboard-only path exists.
- **UX-04:** Prod write attempt triggers clear warning/permission guard.
- **UX-05:** Query running for >10s never locks window; cancel acknowledged visibly.
- **UX-06:** 100k-row simulated grid scroll maintains target frame time without eagerly rendering hidden rows.

## Visual QA
Capture screenshots on Windows 100%, 125%, 150% scaling and macOS Retina (when supported). Check 1080p and laptop small viewport; keyboard-only run; light/dark contrast; 300-char column; Unicode/Thai; null/binary/decimal.
