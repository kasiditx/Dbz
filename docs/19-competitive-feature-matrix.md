# 19 — Reference Clients & Research Questions

**Not a verified comprehensive feature comparison**. Treat entries as directions for hands-on study rather than current competitive claims. Use upstream documentation and observed behavior before marking parity.

| Client | Reference strength | What to study | Don't do |
|---|---|---|---|
| DBeaver Community | wide SQL DB feature breadth | explorer, connection manager, SQL editor, result set workflows | copy trademarks/assets or presume Enterprise features OSS |
| Gridline | modern lightweight Tauri database GUI | interaction patterns, table grid, staged edits | assume Oracle supported or simply change styling |
| Oracle SQL Developer | Oracle developer workflows | service/SID, PL/SQL, packages, datatype/error handling | reverse private internals or ship proprietary code |
| ArtCraft / PhotoCraft | original OSS product inspired by mature software | command-oriented architecture, native UI and workflow familiarity | treat claim as license to copy protected implementation |

## Research matrix (fill with evidence)
| Scenario | DBeaver | Gridline | SQL Developer | Our acceptance target |
|---|---|---|---|---|
| Oracle test connection | test/version pending | verify unsupported | test/version pending | OR-001 |
| Oracle package explorer | test/version pending | N/A until researched | test/version pending | OR-010 |
| Native cold startup | benchmark pending | benchmark pending | benchmark pending | PERF-01 |
| Scroll 100k-result table | benchmark pending | benchmark pending | benchmark pending | PERF-05 |
| Cancel slow Oracle query | test pending | N/A until researched | test pending | OR-012 |
| Credentials stored safely | verify behavior | verify behavior | verify behavior | FR-002 |
| Staged data editing | verify behavior | verify behavior | verify behavior | FR-008 |

## Interview / usability tasks
Ask 3–5 developers (with consent) to perform: connect Oracle, inspect table definition, run bind SELECT, paginate, export, cancel slow query, and safely edit. Time task and record observed points of confusion without collecting secrets or real customer data.

## Evidence rule
Every future feature-parity claim must include source URL+version or recorded test date+environment. Do not silently turn `pending` into `yes`.
