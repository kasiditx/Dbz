# T-003 — Native UI editor + large grid POC (M0)

Date: 2026-10-26 · Base commit: `7086980` (T-002) · Code: `crates/poc-ui` (egui/eframe 0.37.0, throwaway prototype) · Raw data: `benchmarks/results/20261026-210512/` · Scripts: `benchmarks/run_t003.py`, `benchmarks/summarize.py`
Skills actually invoked via Skill tool: `caveman`, `design-an-interface`, `prototype`, `tdd`, `code-review`. Not invoked: `design-system`, `qa` (no benefit beyond the fallbacks below). Sub-agents were not spawned (design-an-interface / code-review suggest them; done inline instead — fallback, so no independent second reviewer).

## Verdict (scope-limited)
POC builds and runs; virtualization verified with data; headline numbers are **on one machine only** (Apple M4 Pro, macOS, battery power), **not** the Windows 11 baseline of `08-performance.md`. This is **not** evidence that egui beats Tauri: no Tauri spike exists. ADR-0001 stays **Pending**; T-004 may use this as the egui half of the comparison.

## What was built
Rust lib `poc_ui::model` (no egui, no DB dependency) + egui app (`app.rs`) + bench driver.
Dark/Light theme (⌘⇧L) · multiline SQL editor with token highlighting (Oracle-flavoured: keywords, strings, `--`/`/* */`, `:binds`, `"quoted"`) · editor tabs (⌘T/⌘W/Ctrl+Tab) · mock explorer tree (arrow-key nav) · virtualized grid (custom painter over `ScrollArea::show_viewport`, row + column culling, sticky header, selection, ⌘C copy of full cell) · resizable explorer (egui panel) and editor/results splitter (own `Split` model, ratio kept on window resize) · status bar (run state, Ln/Col, rows drawn, cells/frame, ui() CPU p50/p95, frame Δ p50/p95, RSS, focus) · Thai + very long text (300+ char column, display capped at 90 chars, copy is full) · mock query run on a **worker thread** with simulated 400 ms latency and Esc cancel (UI thread never waits). No database connection exists anywhere in `poc-ui`.

## Tests (actually run)
Commands (in `crates/poc-ui`, PATH needs `/opt/homebrew/opt/rustup/bin`): `cargo fmt --all -- --check` → clean; `cargo clippy --all-targets -- -D warnings` → clean; `cargo test` → **51 passed, 0 failed, 0 ignored**.
Coverage by seam: grid model (visible-range math incl. 1M rows, only-visible-cells-read counter test, selection/keyboard clamps, scroll-to-reveal), tab state, split/resize ratio, SQL tokenizer (Thai, tiling, comments), statement-under-cursor + PL/SQL block boundary + `line_col`, percentile metrics, explorer navigation.
TDD: red observed for both batches (24 and 27 failing against `unimplemented!()` stubs, 0 → green). Two tests failed after first implementation: one test expectation was wrong (Thai column count 7 vs real 8) and one impl bug (`DUAL` treated as keyword); both fixed and noted here. Stubs-then-tests were written per module batch, not strictly one test at a time (horizontal within a module) — deviation from the tdd skill's vertical-slice advice.
**NOT TESTED / gaps:** no egui-level component test (PocApp needs a `CreationContext`); real OS keyboard/mouse events never driven end-to-end (nav benchmark calls the model directly); mouse click selection, splitter drag by hand, tab buttons, focus handoff (key-focus fix applied after review) are **unverified by test**; IME/Thai input **NOT TESTED**; screen reader **NOT TESTED**.

## Virtualization evidence
- Unit: `only_visible_cells_are_read_from_source` — 1M-row source, cells read = visible rows × 20 cols (<1000).
- Runtime: max cells painted per frame = **540–560**, max rows drawn **27–28**, for both 100k and 1M rows (`summary.md`, all frames pooled).
- Memory: peak RSS 114–124 MiB for 100k and 1M rows alike; idle RSS 111–114 MiB for 1k/100k/1M. Rows are generated on demand (`SyntheticSource` stores 2 usizes), so this shows the *renderer* does not scale with rows; it says nothing about real result-set memory (T-104).
- Known limit: egui scroll offsets are f32; at 1M × 22 px = 22 M px the f32 step is 2 px, so row positioning is still correct but fine-grained scrolling is coarse. Scrolling to the very bottom was exercised by the sweep but not visually inspected. >~16 M rows would need a custom scroll model.

## Performance (measured; raw in results dir)
Machine: Apple M4 Pro, 12 logical CPUs, 24 GiB RAM, macOS 26.5.2, rustc 1.99.1, release profile, **battery power**, git base 7086980 with uncommitted work (dirty). Windows 11 baseline NOT MEASURED. Binary 14.8 MB (release, unstripped).

| Metric (spec ID) | Result | Spec target | Notes |
|---|---|---|---|
| Warm startup, spawn → frame-6 READY (PERF-02), n=10 | p50 **200 ms**, p95 749 ms (first run 749, others 191–209) | ≤1.0 s | external wall clock; READY printed at 6th frame |
| main() → first ui() | p50 110 ms, p95 156 ms | – | internal |
| Cold startup (PERF-01) | **NOT MEASURED** | ≤2.0 s | no cache purge possible without root |
| Idle RSS (PERF-03), 60 s | 111–114 MiB (1k/100k×3/1M) | ≤200 MiB | RSS includes GPU/Metal mappings; measured via `ps` |
| Idle CPU, 60 s | 0.5–6.7 % per run (1k: 0.74, 100k: 1.4/6.7/0.5, 1M: 0.80) | – | focused editor caret blink repaints; high variance, cause of the 6.7 % run not diagnosed |
| Peak RSS, scrolling/nav | 114–124 MiB | – | `wait4` rusage, 5 runs each |
| Avg CPU during continuous-repaint scroll/nav | 33–37 % of one core | – | repaints at display rate; not an idle figure |
| ui() CPU per frame, scroll slow / sweep | p95 1.0 / 2.5 ms, max 1.4 / 3.4 ms | – | CPU time inside `ui()` only (excludes GPU/present) |
| Frame Δ (interval between `ui()` starts), scroll | p50 8.3 ms, p95 9.3–9.6 ms, max 25.7 ms (n=1500–1510 per row, 100k and 1M) | PERF-05 p95 ≤33 ms | p50 ≈ 8.3 ms is consistent with a 120 Hz display (not verified); this is vsync-paced, so it bounds smoothness, not render cost |
| Keyboard nav step (model + render), 100k / 1M | ui() p95 2.3 ms; frame Δ p95 9.5 ms | PERF-04 p95 ≤50 ms | **not OS input→display latency** (NOT MEASURED); keys injected into the model |
| Window resize (programmatic `InnerSize` every frame) | ui() p95 0.69 ms but frame Δ **p95 ≈ 1009 ms** (~20 % of frames stall ≈1 s) | – | see risks; 242 distinct sizes confirmed in CSV; interactive drag resize **NOT MEASURED** |
| 100k vs 1M rows | no measurable difference in the above | – | expected: synthetic source |
Repetitions: startup 10, scroll/nav/resize 5 per dataset, idle 1–3. Pooled percentiles over all frames; no samples dropped. Outliers kept (startup run 1 = 749 ms; scroll slow max 25.7 ms).
Post-benchmark changes: clippy auto-fixes (collapsible ifs, 6 sites) and the editor-focus release. Benchmarked binary = pre-change build; behaviour of rendering path unchanged; a startup smoke run of the final binary succeeded (READY printed). Benchmarks were not re-run.

## Architecture findings
1. UI/core split holds: `model` has zero egui/DB deps and holds all tested logic; `app.rs` is a thin consumer. `RowSource` trait is the seam a real driver page-cache will implement (T-104). No change to the T-002 code or driver contract.
2. `oracledb` is blocking (T-002) → worker-thread + channel pattern demonstrated with the mock query; cancel is flag-based (mock only; says nothing about Oracle cancel).
3. egui 0.37 specifics: App trait is `ui(&mut Ui, ..)`, `Panel` replaces SidePanel/TopBottomPanel, layouter takes `&dyn TextBuffer`. API churn between minors is a maintenance cost to weigh in ADR-0001.
4. Thai needs a system font loaded at runtime (`Ayuthaya.ttf`, macOS only, not bundled — redistribution licence unreviewed). Combining-mark rendering looks plausible in screenshots but spacing in long Thai strings looks loose; IME not tested.
5. Custom editor features not provided by egui's `TextEdit`: no line numbers, no autocomplete, no multi-cursor, no bracket matching, no undo-grouping control. Highlighting re-tokenizes the whole buffer every frame the editor repaints (fine for small buffers; large scripts untested).

## Accessibility findings (not validated)
- AccessKit (macOS adapter crate present in the dependency tree) is the egui path to screen readers; **no VoiceOver test was run**, so no claim either way. Custom-painted grid cells are not individual accessible nodes (they are painted, not widgets) → likely inaccessible to screen readers without extra work. Risk: high for the grid.
- Focus visible: selection uses fill + 2 px ring (grey when grid unfocused); status bar shows focus owner. Contrast ratios not measured.
- Shortcuts: F6 cycle, Alt+1/2/3, ⌘↵, Esc, ⌘T/W, Ctrl+Tab(±Shift), ⌘⇧L, grid arrows/PgUp/PgDn/Home/End/⌘↑/⌘↓. Reduced-motion and UI scale not implemented. HiDPI: screenshots are 2560×1600 (Retina); Windows 125/150 % NOT TESTED.

## Risks / known limitations
- Programmatic resize shows ~1 s stalls; cause undiagnosed (outside `ui()`; could be winit/macOS/harness artefact). Manual drag-resize must be measured before ADR-0001 relies on resize quality.
- Idle CPU is non-zero (caret blink) and one 6.7 % outlier unexplained.
- Only macOS/M4 Pro measured; battery power; no Windows baseline; no competitor measurements (no DBeaver/Tauri comparison) → no "faster than" claim is made or supported.
- Dependency audit not done: eframe 0.37.0 (MIT OR Apache-2.0, MSRV 1.95) + png 0.18.1; `Cargo.lock` lists 426 packages; per-crate licences, CVEs and sizes NOT AUDITED (`cargo audit/deny` not installed).
- Screenshots are of the app's own mock data; no sensitive data. Files: `benchmarks/results/shots/{dark_100k,light_1m}.png`.
- Cell display truncates at 90 chars; real LOB handling out of scope.
- `app.rs` mixes UI, benchmark scripting and screenshot hooks (Divergent Change smell) — acceptable for a throwaway POC, must not be promoted to production as is.

## Code review (inline fallback, two axes)
**Standards (AGENTS.md / SKILLS.md / DoD):** no secrets; no DB I/O on UI thread; dependency facts recorded (audit gap flagged); fmt/clippy clean. Smells (judgement calls): Divergent Change in `app.rs`; tuple `last_range` (Primitive Obsession); duplicated visible-column culling conditions in `grid()`. Fixed during review: dead `else if` branch in editor, key events reaching both TextEdit and grid when focus moved (now releases text focus; not E2E-verified).
**Spec (the 10 requested features):** 1–5, 8–10 implemented and exercised by screenshot/bench/unit tests; 6 keyboard nav implemented, grid logic unit-tested, **real key events untested**; 7 resizable panels implemented, **drag untested by hand**. Scope creep: none beyond POC (mock run/cancel was requested by the "mock results" + non-blocking constraint). Missing evidence: Tauri comparison (out of scope by instruction), cold startup, interactive resize/latency, screen reader.

## Readiness for T-004
Partial. Egui-side evidence exists for startup, memory, virtualization and frame pacing on one machine. Before ADR-0001 can be decided: (a) build the same-workload Tauri/CodeMirror spike or explicitly drop the comparison, (b) measure manual resize + real input latency, (c) VoiceOver/accessibility spike for the grid, (d) repeat on the Windows baseline. ADR-0002 is blocked on live Oracle, independent of T-003.
