# T-003 follow-up — Accessibility feasibility note (egui custom grid) and font licence

Date: 2026-10-26 · Scope: `crates/poc-ui` (egui/eframe 0.37). **No accessibility support is claimed.** VoiceOver was NOT run.

## What was actually checked
1. `eframe` 0.37 has its `accesskit` feature on by default (`cargo tree -e features`), egui-winit pulls `accesskit_macos`/`accesskit_winit`.
2. macOS accessibility tree of the running window via System Events (`entire contents of window 1`, `benchmarks/followup/raw/ax_tree_dump*.txt`): only window chrome (close/minimise/fullscreen buttons, title text, one `AXGroup`) was visible; **no toolbar buttons, editor, explorer rows, or grid cells appeared**, including when `ctx.enable_accesskit()` was forced (`--accesskit`). This does **not** prove VoiceOver would see nothing: System Events is a generic AX client and may not trigger AccessKit activation the way VoiceOver does. Result: **inconclusive, not a pass**.
3. Source check: egui 0.37 exposes `Context::accesskit_node_builder(id, f)` and AccessKit roles `Grid`, `Row`, `Cell`, `GridCell`, so a custom grid *can* publish nodes. egui's own widgets (labels, buttons, `TextEdit`) publish nodes automatically; text drawn with `Painter::text` (our grid cells, header, tab close glyphs) publishes **nothing**.

## Not tested (and why)
- VoiceOver reading order, focus announcements, editor caret/selection reading: needs VoiceOver running and a human/ASR check; enabling VoiceOver is a system setting change (not done).
- Windows Narrator/NVDA, Linux Orca: no such machines here.
- Contrast ratios, reduced-motion, UI scale: not measured / not implemented.
- Thai IME composition (preedit) with the real input method: injected Unicode key events do not exercise the IME.

## Assessment of the custom-painted grid (expectation, to be verified)
Cells are painted, so they are almost certainly invisible to assistive tech today. Approach if egui is kept:
| Option | What | Rough technical cost | Risk |
|---|---|---|---|
| A. Publish AccessKit grid for the visible window | Each frame build `Grid` → `Row` → `GridCell` nodes for only the ~28 visible rows × visible columns (matches our virtualization), with `row_index`, `column_index`, `row_count`, `column_count`, text value, focus on selected cell, and action handlers for scroll/focus | medium: ~1–2 weeks incl. VoiceOver verification on macOS + Narrator on Windows; per-frame node churn must stay small (≈500 nodes/frame measured as painted cells) | node-id stability across scroll; VoiceOver may announce wrongly for virtual rows without `set_size/pos_in_set`; extra CPU per frame unmeasured |
| B. Off-screen "summary" node + text-export action | One node describing the result ("100000 rows, row 3 col 2 = …") and a keyboard-accessible "copy cell"/"read cell" | low: days | weak UX; not equivalent to table navigation |
| C. Switch the grid to a web view (Tauri path) | Use DOM table/ARIA grid + virtualization library | cost sits in the Tauri comparison (T-004), not here | memory/startup trade-offs unknown |
Recommendation for T-004: treat A as the egui-side accessibility cost line item and require a VoiceOver spike before any accessibility claim; do not pick a UI stack on accessibility grounds until that spike (or the web equivalent) has been run.

## Ayuthaya.ttf licence risk
- The POC loads `/System/Library/Fonts/Supplemental/Ayuthaya.ttf` **at runtime from the OS** only to get Thai glyphs; the file is **not copied into the repo or binary** (verified: nothing under `crates/` or `benchmarks/` contains font data; `strings` on the font shows "Copyright 1992-2011 by Apple Inc. All rights reserved").
- Consequences: (1) it must never be bundled or redistributed; (2) the path is macOS-only, so Windows/Linux Thai text will fall back to tofu until a font strategy exists; (3) egui's `system_fonts` feature (egui_system_font_provider is in the dependency tree) may offer OS font lookup without bundling — not evaluated.
- For a cross-platform build the project needs a redistributable Thai font. Candidates to evaluate (licence NOT reviewed here, nothing downloaded): an SIL OFL 1.1 font such as Noto Sans Thai or Sarabun (note: a "THSarabun" set exists in the user's font folder; its licence terms were not checked and it was not used). Open item: pick, review licence and size impact, add NOTICE entry.
