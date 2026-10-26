# T-003 manual test checklist (items automation cannot decide)

Automated coverage: `benchmarks/followup/e2e_input.py` (real CGEvent keyboard/mouse; 35/35 on the final build; state read from the app's `--event-log`) and `cargo test` (68). Use this list for what remains. Status values: PASS / FAIL / NOT TESTED. Nothing below was executed by a human during this task unless marked.

| # | Check | How | Status |
|---|---|---|---|
| M1 | Thai input method composition (Thai Kedmanee/Pattachote): type "ก็", "ที่นี่" with the real IME, backspace behaviour on tone marks | System Settings → Keyboard → add Thai; type in editor | NOT TESTED |
| M2 | Combining-mark glyph placement and line height in editor + grid at 100/150 % scale | visual, compare screenshots | NOT TESTED (only default Retina screenshot seen) |
| M3 | VoiceOver: toolbar, tabs, explorer, editor, grid announce meaningfully; Tab/F6 order | VoiceOver on, walk the UI | NOT TESTED |
| M4 | Text selection by mouse drag, double-click word, triple-click line in editor | manual | NOT TESTED (keyboard selection + Cmd+A tested) |
| M5 | Copy from editor selection and paste into another app; paste Thai from another app | manual | PASS for Cmd+V paste of Thai/ASCII via pbcopy (E2E); cross-app copy-out NOT TESTED |
| M6 | Drag-resize of the window on an external monitor / different scale factor | manual | NOT TESTED |
| M7 | Hand-drag of explorer panel width (egui panel) | manual | NOT TESTED (editor/results splitter tested with real mouse) |
| M8 | Long idle (≥10 min) memory/CPU, sleep/wake of laptop | manual | NOT TESTED |
| M9 | Windows 11 baseline of docs/08 (125/150 % scaling, Narrator) | needs hardware | NOT TESTED |
| M10 | Trackpad momentum scrolling feel in grid at 100k/1M rows | manual | NOT TESTED (mouse-wheel line events tested) |
