# ADR-0001 — Native UI Stack

**Status:** Proposed / Pending M0 evidence

## Context
User prioritizes modern professional UI, fast startup, low RAM, smooth grid, keyboard navigation and accessibility. Earlier idea: pure Rust egui/eframe + wgpu. Alternative Tauri 2 + React/TypeScript. Each has important tradeoffs.

## Candidates
### A) Native Rust egui/eframe + wgpu
Pros: single Rust build stack, no WebView runtime dependency, direct Rust core integration, compact binary possible. Risks: high-quality SQL editor, IME/Thai, clipboard, accessibility, screen-reader integration and desktop ergonomics may need substantial custom work.

### B) Tauri 2 + Rust + React / CodeMirror
Pros: mature editor/virtual grid/accessibility web ecosystem, rapid polished UI iteration. Risks: OS WebView behavior/version variability and JS/UI runtime memory overhead. Backend remains Rust.

## Spike acceptance
1. Build identical demo workspace: 3 panes, SQL code editor, 100k virtual row grid, tab switching, keyboard shortcuts.
2. Measure startup/idle memory/frame responsiveness on baseline Windows machine, 10 warm/cold runs.
3. Inspect IME/Thai, clipboard, selection, focus, accessibility, HiDPI, packaging size.
4. Score developer maintenance and dependency security; document tested versions and time cost.
5. Select the approach meeting **UX and resource goals together**; don't assume native is always faster.

## Decision
**Pending.** Do not hard-code eframe before experiment. Rust Core/driver boundary remains stable whichever UI chosen. Revisit if real-user UX/accessible UI issues outweigh measured benefits.

## Evidence log
- 2026-10-26: egui/eframe 0.37 POC measured on one macOS machine only; no Tauri spike yet; status remains Pending. See `docs/evidence/T-003-native-ui.md`.
- 2026-10-26 (follow-up): egui measurements on final build (input→state p50 ≈26 ms, live-resize 0 stalls in 22 real-mouse runs, ui() p95 ≤2.5 ms, idle 0.75 % CPU / ~115 MiB); non-interactive resize triggers ~1 s drawable stalls (CoreAnimation/wgpu). Accessibility, Thai IME and Windows unverified. Still **Pending**; Tauri same-workload comparison proposed (docs/evidence/T-003-followup.md).

- 2026-10-26 (T-004a): runnable Tauri 2/React/CodeMirror/TanStack experiment and a shared workload now exist. See `docs/evidence/T-004a-tauri-comparison.md` for release measurements, attributed helper memory, actual UI tests, differences and limitations. **Status remains Pending**; macOS-only evidence does not decide cross-platform stack selection.
