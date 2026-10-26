# T-004a — Tauri vs egui same-workload experiment

Date: 2026-10-26 (Asia/Bangkok). Base: `25711f5`, preserving `0d4972d`. Branch: `experiment/t004a-tauri`. Original request: `benchmarks/t004a/REQUEST.md`. **Throwaway UI experiments; not production clients. No database access.**

**Decision: ADR-0001 remains Pending. ADR-0002 remains Proposed / Pending live Oracle 19c. Platform Validation Pending: Windows and Linux. NOT TESTED: live Oracle, real Thai IME composition, VoiceOver.**

## Scope and implementation

Requirement seams: FR-003 explorer, FR-004 SQL editor, FR-005 mock execution, FR-006 grid, FR-009 cancellation, FR-012 keyboard/theme; PERF-02/03/04/05/09; UX-05/06. Oracle SQL highlighting is not Oracle parsing/execution validation.

- Isolated `crates/poc-tauri`: Tauri 2.12.5 Rust shell, React 19.3.1, TypeScript 5.9.3, Vite 8.3.5, CodeMirror 6, TanStack React Virtual 3.14.23. Exact dependency graphs in package-lock.json and Cargo.lock. TypeScript 7.0.2 was rejected by typescript-eslint's peer range; resolved without `--force` or legacy-peer bypass.
- Dark/light semantic CSS tokens, system UI/mono fonts, SQL/PLSQL highlighting, 3 initial tabs with separate undo/selection state, mock explorer, status bar, keyboard focus transfer, resizable sidebar/editor, bounded row/column DOM, full-value copy, NULL/Thai/Unicode/exact decimal/timestamp/long text.
- Rust mock worker waits asynchronously for 400 ms, allows one job, confirms cancellation, rejects concurrent jobs, exposes success/empty/error outcomes. No SQL is sent to a database. Frontend shows running/cancelling/completed/empty/error states; later attempts remain possible after failure/cancel.
- `crates/poc-workload` + `benchmarks/t004a/workload.json/query.sql`: deterministic comparison contract. Egui opts in with `--comparison`; default T-003 behavior remains intact. Oracle POC unchanged.
- Sources and built assets are local. CSP restricts script loading to self; no shell, network, DB or filesystem plugin. Diagnostic file paths are trusted CLI arguments, never frontend IPC arguments. Event logs contain only synthetic POC text snippets. No real-data usage is supported by this experiment.

## Same-workload contract and differences

Both: 1280×800 logical content area (raw geometry checked), 1K/100K/1M logical rows, 20 columns, 22 px rows, 150 px columns except NOTES 320 px, 3 tabs, identical 191-character SQL, 400 ms mock delay. Large SQL: 500 copies = 95,500 Unicode characters per tab. Both generate cells on demand; no million-row array or million-row DOM.

Tauri's native window config height is 832 on this macOS to provide 800 px WebView content after 32 px native chrome. Egui requests 800 inner height. An exploratory direct-executable startup series used mismatched content heights and is excluded (`exploratory-startup-direct.json`).

Material differences remain and must accompany any interpretation:

| Difference | Consequence |
|---|---|
| Egui grid starts near y=284.5 and has ~474.5 px height; Tauri near y=384 and ~388 px height at initial layout | More visible rows in egui. Window/data/action contract is matched; exact pane geometry/render work is not identical. This is an implementation comparison, not an isolated framework microbenchmark. |
| Egui materializes all 20 columns for each visible row, then culls painting; Tauri materializes only virtual columns | Per-frame cell-generation work differs; do not attribute every CPU difference solely to rendering technology. |
| TanStack keeps O(N) metadata (including 16 MB numeric backing for 1M rows); egui fixed-row arithmetic has constant-size source metadata | DOM stays bounded in both, but total memory does not have the same asymptotic behavior. |
| Initial caret is at the end in egui and at the start in CodeMirror; the same ASCII inputs land at different positions | Typing work is not perfectly matched; these transport-inclusive timings cannot establish a parser/editor speed winner. |
| CodeMirror has line numbers, selection/history, viewport editor rendering and incremental parser; egui text editor uses its existing whole-buffer highlighter | Same script/text actions, different editing implementations and feature quality. |
| Egui callback frame-start intervals vs WebView requestAnimationFrame callbacks | Both indicate scheduling gaps; neither is physical display latency. Absolute frame-rate differences may be OS/renderer scheduling. |
| Mock explorer hierarchy differs; empty/error chooser exists only in Tauri | Functional evidence, excluded from matched performance actions. |
| CUA input transport includes activation/dispatch/observation overhead | Call→state measurements are upper bounds, not precise OS-event→state or input→display. |

## Performance evidence

**Environment drift:** start metadata at 23:10:45 records battery 14%; a `pmset` summary at 23:22:47 records AC, and later power snapshots confirm charging. Exact switchover time was not captured. Earlier idle/power conditions cannot be asserted identical. The 1M active pair and final Tauri 100K rerun record AC at start/end. No power setting was changed by the experiment. Treat cross-run resource comparisons as exploratory, not controlled release gates.

Final values are generated from raw data in `benchmarks/t004a/raw/summary.json`; commands and definitions in `benchmarks/t004a/README.md`. Environment includes hardware, power/display, compiler/runtime versions, binary SHA-256 and artifact bytes. After removing unapproved license metadata from the original-code manifests, final release rebuilds produced **identical signed executable SHA-256** for both stacks; see `raw/rebuild-equivalence.json`. Release binaries embed assets; no dev server/debug build is decision evidence.

| Metric | egui | Tauri | Notes |
|---|---:|---:|---|
| Warm startup p50 / p95 (ms) | 229.70 / 264.95 | 519.77 / 554.18 | n=10 each; one prewarm excluded per stack |
| Idle 100K main RSS (MiB) | 118.39 | 103.95 | Median of three launches; power caveat applies |
| Idle 100K attributed-group RSS (MiB) | 157.27 | 291.05 | Median of three launches; power caveat applies |
| Idle 100K physical footprint (MiB) | 265.50 | 251.66 | Median of three launches; power caveat applies |
| Idle 100K group CPU (% of one logical CPU) | 0.76 | 3.65 | Median of three launches; power caveat applies |
| Exact OS input→state / display | NOT MEASURED | NOT MEASURED | CUA bounds below are a different metric |
| Mixed-session frame interval 100,000, p50 / p95 (ms) | 7.56 / 9.37 | 17.00 / 18.00 | Whole instrumented session, **not scroll-only** |
| Scroll sampled peak group RSS 100,000 (MiB) | 223.06 | 449.58 | 1 Hz; 6 / 2 samples in scroll-call window |
| Mixed-session frame interval 1,000,000, p50 / p95 (ms) | 8.33 / 9.62 | 17.00 / 18.00 | Whole instrumented session, **not scroll-only** |
| Scroll sampled peak group RSS 1,000,000 (MiB) | 218.94 | 484.41 | 1 Hz; 6 / 3 samples in scroll-call window |
| Real mouse window resize | 6 width changes per dataset | 6 width changes per dataset | Frame logs: 1280↔1120, height 800; Tauri width-only change does not emit an app-state snapshot |
| Accessibility | Keyboard/custom grid; semantic-grid gap | Keyboard/AX semantic cells verified | VoiceOver and real Thai IME NOT TESTED |
| Development complexity | Rust editor/highlighter/grid work | Rust IPC + TS/CSS + CodeMirror + virtual grid | Two language/package ecosystems; source review, not a timed productivity study |
| Release executable (MiB) | 14.13 | 10.73 | Local ad-hoc signed arm64; OS WebView/frameworks excluded |
| Local app bundle files (MiB) | 14.14 | 10.73 | Sum of file lengths, not a compressed installer |

### Idle dataset detail

All values MiB except CPU. For 100K, values are medians of three per-launch medians/snapshots; ranges show physical-footprint variability.

| Stack / rows | n | Main RSS | Group RSS | Physical footprint (range) | Group CPU % |
|---|---:|---:|---:|---:|---:|
| egui / 1,000 | 1 | 108.77 | 156.39 | 268.35 (268.35–268.35) | 0.79 |
| egui / 100,000 | 3 | 118.39 | 157.27 | 265.50 (258.63–272.27) | 0.76 |
| egui / 1,000,000 | 1 | 112.55 | 151.44 | 259.95 (259.95–259.95) | 0.76 |
| tauri / 1,000 | 1 | 97.92 | 245.62 | 235.96 (235.96–235.96) | 3.71 |
| tauri / 100,000 | 3 | 103.95 | 291.05 | 251.66 (153.00–253.15) | 3.65 |
| tauri / 1,000,000 | 1 | 104.25 | 278.47 | 263.05 (263.05–263.05) | 3.58 |

**RSS and footprint disagree:** Tauri has higher attributed RSS here, while physical footprint overlaps or is lower in some snapshots. Shared mappings, compressed/resident pages and helper lifecycle differ. Do not report a universal physical-memory winner. Both main RSS values hide substantial helper cost.

### Active counts and input observations

Input values below are CUA-call→first matched state **upper bounds**, p50 / p95 ms, with matched/attempted counts. Millisecond clock quantization can produce 0.00; that does not mean zero latency. Fast state coalescing and stale pre-action snapshots cause unmatched records; the subset has selection bias. Final states verify all 20 row moves, 20 tab switches and 20 inserted characters. Do not use these bounds to rank precise input latency or pass PERF-04.

| Dataset / action | egui p50 / p95; matched | Tauri p50 / p95; matched |
|---|---:|---:|
| 100,000 / grid-key | 2.21 / 2.82; 18/20 | 2.00 / 7.00; 14/20 |
| 100,000 / tab-switch | 2.73 / 3.55; 18/20 | 4.00 / 7.00; 16/20 |
| 100,000 / large-sql-typing | 8.87 / 10.13; 19/20 | 0.00 / 6.00; 14/20 |
| 100,000 / scroll | 6.23 / 7.47; 20/20 | 2.00 / 6.00; 18/20 |
| 100,000 / resize | 674.94 / 1227.88; 6/6 | NOT MEASURED / NOT MEASURED; 0/6 |
| 1,000,000 / grid-key | 2.54 / 6.80; 17/20 | 2.00 / 3.00; 12/20 |
| 1,000,000 / tab-switch | 2.50 / 3.25; 18/20 | 5.00 / 11.00; 17/20 |
| 1,000,000 / large-sql-typing | 7.87 / 10.61; 19/20 | 2.00 / 7.00; 17/20 |
| 1,000,000 / scroll | 6.74 / 10.69; 20/20 | 1.00 / 483.00; 18/20 |
| 1,000,000 / resize | 728.86 / 1220.79; 6/6 | NOT MEASURED / NOT MEASURED; 0/6 |

| Session | Frame samples | Max materialized cells (egui) / DOM cells (Tauri) | Max DOM elements | All-active sampled main / group RSS (MiB) |
|---|---:|---:|---:|---:|
| active-egui-100000-r2 | 17243 | 560 | N/A — native | 164.19 / 234.25 |
| active-egui-1000000-r1 | 14325 | 560 | N/A — native | 154.09 / 219.19 |
| active-tauri-100000-r2 | 7118 | 175 | 589 | 122.11 / 449.58 |
| active-tauri-1000000-r1 | 7110 | 175 | 589 | 124.27 / 484.41 |

Scroll-only frame percentiles: **NOT MEASURED**. Continuous mixed-window logging includes idle gaps and observer overhead. Both final runs reached scroll row 756 after 20 high-level scroll calls. The first Tauri 100K run closed before frame-file completion; preserved as `exploratory-flush-*`. Final Tauri runs allow up to 20 seconds after sampling for frame-file flush. No UI source change was needed.

Against the provisional 200 MiB **whole-client RSS** envelope, observed Tauri groups exceed it; egui idle groups are below it. This macOS experiment cannot certify the Windows baseline. Keep the stack decision open and investigate Tauri metadata/editor/helper costs before adopting it.

Cold startup: **NOT MEASURED**. No cache purge/root access or unverified cold labels. Precise OS-event→state, input→display, isolated JavaScript CPU, React render/commit CPU, Rust CPU and OS event-delivery stages: **NOT MEASURED**. Rust's mock 400 ms wait is wall delay, not CPU work. React production Profiler was not enabled. Do not compare egui ui() CPU to React commit time.

Warm startup: 10 runs per stack, alternating order, LaunchServices request→READY. READY means six UI/rAF callbacks and mounted UI, not verified monitor presentation. Polling contributes up to ~5 ms. One explicit prewarm per stack precedes these ten runs. All measured runs/outliers retained; earlier unconditioned launches are separate exploratory evidence. LaunchServices starts each app under its own responsibility identity; direct shell launch inherited Codex ownership and could not establish helper attribution.

Idle: five launches per stack (1K once, 100K three times, 1M once), 65-second sample windows; use RSS samples at ≥60 seconds. Cumulative CPU-time deltas, not instantaneous `ps %cpu`. No continuous rAF in Tauri idle mode; no app-requested continuous repaint in egui idle mode. Caret/OS work remains. The repo's ≥5 independent runs **per non-startup case** target is not met for every dataset; results are exploratory, not release certification.

Active: real desktop inputs through CUA; common keyboard, scrolling and mouse-resize sequence. Continuous frame logging is enabled for both during these windows. DOM querying, state serialization, IPC/file logging and CUA observation add diagnostic overhead. One complete 120-second launch per stack/dataset is reported. Exact sample counts, failed action matches, dimensions and timestamps are retained, not discarded. The native transport dispatches at different cadences per app; the same action count/order is not identical wall-time pacing. RSS is sampled at 1 Hz, so its maximum is an **observed sampled peak**, not a guarantee of the true high-water mark.

### Memory attribution

`ownership.py` uses read-only macOS responsibility API with pinned Apple/Chromium ABI sources, plus parent-chain evidence and coalition corroboration. Main PID, helper PID/name, responsibility and coalition fields are retained per sample. Helpers are attributed only when responsibility points to this app or a verified parent chain connects them; another app's WebKit helpers are excluded. Sharing the Codex launcher coalition is insufficient.

Attributable processes can include WebContent, Networking, GPU, audio SandboxHelper, SafariPlatformSupport.Helper and ThemeWidgetControlViewService. Report main RSS separately from summed attributed RSS. **RSS sums may double count shared pages.** This is a defensible observed group, not proof that every possible future helper has been captured. Polling discovers restarts/new helpers.

Representative idle 100K sample (`idle-tauri-100000-4-memory.json`, last sample):

| PID | Process | RSS MiB | Ownership evidence |
|---:|---|---:|---|
| 3118 | `dbz-tauri-poc` | 103.95 | main process |
| 3123 | `com.apple.WebKit.GPU` | 40.36 | responsibility API equals main PID |
| 3124 | `com.apple.WebKit.Networking` | 15.34 | responsibility API equals main PID |
| 3125 | `com.apple.WebKit.WebContent` | 79.80 | responsibility API equals main PID |
| 3126 | `com.apple.audio.SandboxHelper` | 12.72 | responsibility API equals main PID |
| 3128 | `com.apple.SafariPlatformSupport.Helper` | 38.94 | responsibility API equals main PID |

WebKit PIDs 23321/23331/23333 belong to responsible PID 23314, and PID 81309 to 81308; excluded. Their evidence remains in the raw sample.

`footprint` snapshots run after idle sampling, with explicit attributed PID list. Multi-process summary de-duplicates multiply-mapped objects; physical footprint and peaks are distinct from RSS. Process exit/unresolved ownership must produce NOT MEASURED, never an unscoped `footprint` invocation. Method-validation sample PID57930 is kept separately and is not substituted for the 60-second idle baseline.

No OS permission settings were changed. Self-responsible app launches could not redirect diagnostics into protected Documents paths (`open` error -10810); benchmark outputs instead use a private generated temporary directory and the harness copies them back. Local `.app` bundles are ad-hoc signed for execution; no user signing credentials or notarization were used.

## Functional, accessibility and test evidence

Automated local checks: exact commands, exit codes and logs in `raw/test-results.json`. Rust workload 2 tests; egui 69 tests (68 existing + comparison adapter); Tauri query 2 async tests; TypeScript 5 behavior tests; ownership/harness integrity 6 tests. All ran successfully. Rust fmt/Clippy, TypeScript typecheck/ESLint and production frontend/native builds ran. Vite warns about the ~600 KB frontend chunk; retained, not suppressed. Full `git diff --check` flags whitespace in exact raw logs/AX output and the SQL fixture's intentional final blank line; source-only check passes. Exclusions and raw warnings are recorded in `raw/final-diff-check.json`; no warning configuration was changed.

TDD evidence: workload missing-function red→green, TS generator stub red→green, navigation missing-export red→green, sticky-header reveal regression red→green. Query cancellation tests were added after initial implementation, so **not all new logic followed strict test-first order**. Original egui constant-size test failed when a mode flag enlarged its source; fixed by a separate ComparisonSource adapter, retaining the existing assertion and baseline source.

Real desktop observations (actual screenshots and state logs, no fabricated images):

- PageDown selection visibility failed before the header geometry fix; screenshot `grid-selection-red.png`. After fix, selected intermediate rows stay visible.
- Grid→editor copy verified with Thai `ลูกค้า 0` and the full 420-character NOTES value, including the generator's trailing space (`grid-copy-thai.json`, `grid-copy-long.json`). The initial long-copy expectation incorrectly trimmed that space; raw assertion retained separately.
- 1M row `999999`, column `19` visibly reached; 22,000,026 px scrollHeight observed, ~140 rendered cells at the bottom. Full copy uses logical selected cell, not truncated display.
- Run completion resets selection and both scroll axes; cancel acknowledged; UI remains interactive during mock wait.
- The existing egui Alt+3 shortcut changes focus but also inserts `£` into a focused editor on this keyboard layout; retained as a baseline functional gap. It adds one character during each grid-focus setup. Tauri prevents this insertion.
- ASCII typing, Thai paste with combining marks, select/copy/paste, new/close tabs, Ctrl+Tab/Ctrl+Shift+Tab, theme, focus shortcuts, explorer expand/collapse and sidebar/editor mouse drags were exercised.
- CUA `typeText` dropped Thai characters while ASCII arrived. Thai **paste** and display/copy passed; this does not establish real Thai keyboard/IME composition. The initial failed typing/copy assertions remain in raw QA, with later successful paste tests distinct.
- Native macOS select-menu animation can deliver `change` after an immediately injected Cmd+Enter. Tests must wait until the app outcome state commits before asserting that outcome. The initial premature-run and stale CUA-session checks are retained as harness diagnostics, not counted as final acceptance failures. Final logs show Empty with 0 cells and Error after confirmed outcome selection.
- Tauri AX tree exposes editor, tabs, explorer rows, splitters and virtual grid cells. This is a verified semantic tree, **not a VoiceOver usability claim**. Offscreen cells are virtual; full-grid screen-reader navigation remains unvalidated. Egui's previous AccessKit/custom-painted-grid gaps remain; the comparison build has not implemented a semantic grid.
- macOS Retina screenshots show system-font rendering and Thai. Visual appearance was inspected; no shaping conformance, Windows scaling, contrast audit or external-display test.

## Actual screenshots

Final measured release: [Tauri 1M](../../benchmarks/t004a/raw/active-tauri-1000000-r1.png), [egui 1M](../../benchmarks/t004a/raw/active-egui-1000000-r1.png). Functional QA: [light theme](../../benchmarks/t004a/raw/tauri-light.png), [1M last cell](../../benchmarks/t004a/raw/tauri-1m-bottom.png), [full grid-value copy](../../benchmarks/t004a/raw/tauri-grid-copy.png). QA screenshots are functional observations, not timing evidence.

## Independent review

Used two independent Codex subagents, supported by the current host; no user model config was changed. No external Codex CLI/model workaround was needed.

**Standards:** three initial findings: missing header geometry in grid navigation; closed-tab editor state reinserted by cleanup; stale key timestamp paired with unrelated commit. Fixed and re-reviewed. Two harness findings: reused temporary directory could leak stale frame files, and absent main PID could make footprint unscoped. Fixed with a unique directory, explicit-PID guard and refusal to reuse raw sample labels. No Oracle/secret or mutation issue identified in this review scope.

**Spec:** three initial findings: navigation visibility, rerun scroll reset, causal timing. Fixed; second review found no new implementation blocker. Precise input latency acceptance remains partial; raw CUA upper bounds cannot decide that gate. Pane/column-generation differences are explicit above. The reviews are independent source/seam reviews; reviewers did not pretend to have driven real OS input.

## Dependencies, licensing and maintenance

Primary-source research and version/license/MSRV/registry-size snapshot: `T-004a-research.md`. Lockfiles are authoritative. npm install/audit reported zero known vulnerabilities for the resolved graph at execution time. Cargo CVE/transitive license audit, complete release SBOM, notices, provenance and distribution signing remain pending; declared licenses are not a complete legal audit. Original-code license remains unselected; new manifests do not assert a license before owner approval. No font is redistributed; original simple SVG database icon created for this experiment.

Maintenance cost observed: Tauri needs a Rust command/capability boundary plus TS/CSS build and two lockfile ecosystems. CodeMirror supplies editor facilities that egui's custom editor currently lacks; grid keyboard/ARIA/virtual geometry still required app work and produced a real regression. Tauri's system WebView and helper ownership require OS-specific testing; egui's renderer also has previously measured macOS presentation caveats. Source line count alone is not a maintenance-cost estimate.

## Recommendation / next gate

Proceed with Tauri as a serious UX/editor candidate, while retaining egui for its lower observed RSS and idle CPU. Choose only after comparing the measured resource budget against the editor/accessibility work still needed. No universal “faster stack” claim is supported by one machine, instrumented sessions, unequal pane geometry/caret placement, power drift and unresolved exact input latency.

Next ticket: T-004b Windows 11 same-workload validation, five independent active repetitions per case, accurate event-dispatch latency, real Thai IME/VoiceOver or Windows screen-reader flows, matched pane geometry, and dependency/release audit. Then revisit ADR-0001. Oracle driver validation is separate and still requires an explicitly permitted live Oracle instance.

## Skills actually used

Loaded installed SKILL.md files: caveman (Full-style concise Thai), research (background primary-source investigation), prototype (isolated runnable experiment), design-system (three-layer tokens), tdd (red/green seams, deviations disclosed), code-review (independent Standards/Spec agents). `design-an-interface` loaded but is API-shape design, so its three-agent API-design workflow was not applied to this predetermined GUI benchmark. `qa` loaded but is GitHub issue intake, so local functional checklist used instead. No unavailable skill was claimed invoked; no issue-tracker setup/plugin installation was required for a local-only task.
