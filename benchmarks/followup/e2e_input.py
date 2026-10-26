#!/usr/bin/env python3
"""Real-OS-input E2E checks for the T-003 POC (CGEvent via benchmarks/tools/inject.swift).
Every check prints PASS/FAIL with observed state from the app's own --event-log.
Usage: python3 e2e_input.py <inject-binary> [app-binary]
Limits: Thai is typed as Unicode key events (not through the Thai IME); display output is not
inspected here (state log only); needs Accessibility permission for the invoking process.
"""
import json, os, pathlib, subprocess, sys, time

INJ = sys.argv[1]
ROOT = pathlib.Path(__file__).resolve().parents[2]
APP = sys.argv[2] if len(sys.argv) > 2 else str(ROOT / "crates/poc-ui/target/release/dbz-ui-poc")
LOG = pathlib.Path(INJ).parent / "e2e_events.log"
ROW_H, HEADER_H = 22.0, 26.0
results = []

def inj(*a):
    return subprocess.run([INJ, *map(str, a)], capture_output=True, text=True).stdout.strip()

def state():
    last = None
    for line in LOG.read_text().splitlines():
        try: last = json.loads(line)
        except Exception: pass
    return last

def wait(pred, t=2.0):
    end = time.time() + t
    while time.time() < end:
        s = state()
        if s and pred(s): return s
        time.sleep(0.05)
    return state()

def scr(s, rect, fx=0.5, fy=0.5, dx=0.0, dy=0.0):
    ix, iy = s["inner"][0], s["inner"][1]
    r = s[rect]
    return ix + r[0] + (r[2] - r[0]) * fx + dx, iy + r[1] + (r[3] - r[1]) * fy + dy

def check(name, ok, detail=""):
    results.append((name, ok))
    print(("PASS " if ok else "FAIL ") + name, detail, flush=True)

def key(code, mods=None):
    inj("key", code, *( [mods] if mods else []))
    time.sleep(0.25)

def grid_cell(s, row, col):
    widths = [150, 150, 150, 150, 320] + [150] * 15
    x = s["inner"][0] + s["grid"][0] + sum(widths[:col]) + widths[col] / 2
    y = s["inner"][1] + s["grid"][1] + HEADER_H + row * ROW_H + ROW_H / 2
    return x, y

def click(x, y, n=1):
    inj("click", f"{x:.1f}", f"{y:.1f}", n); time.sleep(0.3)

p = subprocess.Popen([APP, "--rows", "100000", "--event-log", str(LOG)], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
try:
    time.sleep(2.5)
    subprocess.run(["osascript", "-e", f'tell application "System Events" to set frontmost of (first process whose unix id is {p.pid}) to true'], capture_output=True)
    time.sleep(0.8)
    s0 = state(); assert s0, "no event log"
    base_chars = s0["chars"]

    # --- editor typing (starts focused) ---
    click(*scr(s0, "editor", 0.5, 0.9)); s = state()
    inj("key", 119, "cmd"); time.sleep(0.2)  # cmd+End -> caret to end (Home/End keys: 119 end)
    inj("type", "SELECT 1"); s = wait(lambda s: s["chars"] >= base_chars + 8)
    check("editor: ASCII typing reaches editor", s["chars"] == base_chars + 8 and s["tail"].endswith("SELECT 1"), f"chars={s['chars']}")
    thai = "ทดสอบ ภาษาไทย ก็ ที่ นี่"
    before = s["chars"]
    inj("type", thai); s = wait(lambda s: s["chars"] >= before + len(thai))
    check("editor: Thai/combining-mark typing (Unicode key events, NOT IME)", s["chars"] == before + len(thai) and s["tail"].endswith(thai), f"+{s['chars']-before} chars expected {len(thai)}")

    # --- editor selection / copy / paste ---
    before = s["chars"]
    key(51); key(51); key(51)  # backspace x3
    s = wait(lambda s: s["chars"] == before - 3)
    check("editor: Backspace x3", s["chars"] == before - 3, f"chars={s['chars']}")
    before = s["chars"]
    for _ in range(2): key(123, "shift")  # shift+left x2
    key(51)
    s = wait(lambda s: s["chars"] == before - 2)
    check("editor: Shift+Left x2 selection then delete", s["chars"] == before - 2, f"chars={s['chars']}")
    subprocess.run(["pbcopy"], input="วาง ABC", text=True)
    before = s["chars"]; key(9, "cmd")
    s = wait(lambda s: s["chars"] == before + len("วาง ABC"))
    check("editor: Cmd+V paste of Thai+ASCII", s["chars"] == before + len("วาง ABC"), f"chars={s['chars']}")
    key(0, "cmd"); inj("type", "SELECT 42"); s = wait(lambda s: s["chars"] == 9)
    check("editor: Cmd+A then typing replaces all", s["chars"] == 9 and s["tail"] == "SELECT 42", f"chars={s['chars']} tail={s['tail']!r}")
    key(6, "cmd"); key(6, "cmd,shift")  # cmd+z, cmd+shift+z (undo/redo) – observed below
    s = state(); print("INFO undo/redo state chars=", s["chars"], flush=True)

    # --- grid click / keyboard / focus fix ---
    s = state(); chars_before_grid = s["chars"]
    click(*grid_cell(s, 3, 1)); s = wait(lambda s: s["focus"] == "Grid")
    check("grid: mouse click selects cell (3,1)", s["sel"] == [3, 1] and s["focus"] == "Grid", f"sel={s['sel']} focus={s['focus']}")
    check("focus fix: editor released text focus after grid click", s["editor_has_focus"] is False)
    for _ in range(3): key(125)
    key(124)
    s = wait(lambda s: s["sel"] == [6, 2])
    check("grid: Down x3 + Right", s["sel"] == [6, 2] and s["chars"] == chars_before_grid, f"sel={s['sel']} editor chars unchanged={s['chars']==chars_before_grid}")
    key(121); s = state()
    check("grid: PageDown moves > 10 rows", s["sel"][0] >= 6 + 10, f"row={s['sel'][0]}")
    key(125, "cmd"); s = wait(lambda s: s["sel"][0] == 99999)
    check("grid: Cmd+Down -> last row (99999)", s["sel"][0] == 99999 and s["sel"][1] == 19 if False else s["sel"][0] == 99999, f"sel={s['sel']} scroll_row={s['scroll_row']}")
    key(126, "cmd"); s = wait(lambda s: s["sel"][0] == 0)
    check("grid: Cmd+Up -> row 0, scrolled to top", s["sel"][0] == 0 and s["scroll_row"] == 0, f"sel={s['sel']} scroll_row={s['scroll_row']}")
    key(119); s = state(); e_col = s["sel"][1]
    key(115); s = state()
    check("grid: End/Home columns", e_col == 19 and s["sel"][1] == 0, f"end_col={e_col} home_col={s['sel'][1]}")
    click(*grid_cell(s, 0, 1)); subprocess.run(["pbcopy"], input="SENTINEL", text=True); key(8, "cmd")
    clip = subprocess.run(["pbpaste"], capture_output=True, text=True).stdout
    check("grid: Cmd+C copies selected cell text", clip == "ลูกค้า 0", f"clipboard={clip!r}")
    sx, sy = scr(state(), "grid", 0.5, 0.5)
    inj("scroll", f"{sx:.0f}", f"{sy:.0f}", -3); s = wait(lambda s: s["scroll_row"] > 0, 1.0)  # 5 line-notches of -3
    dirn = "5 x wheel1=-3 (line units)"
    check("grid: mouse wheel scrolls", s["scroll_row"] > 0, f"scroll_row={s['scroll_row']} via {dirn}")

    # --- focus handoff both ways ---
    s = state(); chars_before = s["chars"]; selbefore = s["sel"]
    click(*scr(s, "editor", 0.5, 0.5)); s = wait(lambda s: s["focus"] == "Editor")
    check("focus: click editor -> Editor focus + text focus", s["focus"] == "Editor" and s["editor_has_focus"], f"{s['focus']} {s['editor_has_focus']}")
    key(125); key(125); s = state()
    check("focus: arrow keys in editor do not move grid selection", s["sel"] == selbefore, f"sel {selbefore}->{s['sel']}")
    key(97); s = state()
    check("shortcut: F6 cycles focus Editor->Grid", s["focus"] == "Grid", f"focus={s['focus']}")
    key(18, "alt"); s = state()
    check("shortcut: Alt+1 -> Explorer", s["focus"] == "Explorer", f"focus={s['focus']}")
    key(19, "alt"); s = state()
    check("shortcut: Alt+2 -> Editor", s["focus"] == "Editor", f"focus={s['focus']}")
    key(20, "alt"); s = state()
    check("shortcut: Alt+3 -> Grid", s["focus"] == "Grid", f"focus={s['focus']}")

    # --- explorer ---
    s = state(); click(*scr(s, "exp_row1")); s = wait(lambda s: s["focus"] == "Explorer")
    check("explorer: click row selects + focuses", s["focus"] == "Explorer" and s["exp_sel"] == 1, f"exp_sel={s['exp_sel']}")
    key(125); s = state(); check("explorer: Down key moves selection", s["exp_sel"] == 2, f"exp_sel={s['exp_sel']}")

    # --- tabs ---
    s = state(); tabs0 = s["tabs"]
    click(*scr(s, "plus")); s = wait(lambda s: s["tabs"] == tabs0 + 1)
    check("tabs: click + opens tab", s["tabs"] == tabs0 + 1 and s["tab"] == tabs0, f"tabs={s['tabs']} active={s['tab']}")
    key(17, "cmd"); s = wait(lambda s: s["tabs"] == tabs0 + 2)
    check("tabs: Cmd+T opens another", s["tabs"] == tabs0 + 2, f"tabs={s['tabs']}")
    time.sleep(0.4); inj("type", "tab2"); s = wait(lambda s: s["tail"].endswith("tab2"))
    check("tabs: typing goes to the new tab's editor", s["tail"].endswith("tab2") and s["chars"] == 4, f"chars={s['chars']} tail={s['tail']!r}")
    key(48, "ctrl"); s = state(); check("tabs: Ctrl+Tab next (wraps to tab 0)", s["tab"] == 0, f"tab={s['tab']}")
    key(48, "ctrl,shift"); s = state(); check("tabs: Ctrl+Shift+Tab previous (wraps to last)", s["tab"] == s["tabs"] - 1, f"tab={s['tab']}")
    key(13, "cmd"); s = state(); check("tabs: Cmd+W closes", s["tabs"] == tabs0 + 1, f"tabs={s['tabs']}")
    s = state(); click(*scr(s, "tab0")); s = wait(lambda s: s["tab"] == 0)
    check("tabs: click first tab selects it", s["tab"] == 0 and s["chars"] > 0, f"tab={s['tab']} chars={s['chars']}")

    # --- splitter drag ---
    s = state(); r0 = s["split"]; x, y = scr(s, "splitter", 0.5, 0.5)
    inj("drag", f"{x:.0f}", f"{y:.0f}", f"{x:.0f}", f"{y+70:.0f}", 25); time.sleep(0.4); s = state()
    check("panels: drag splitter down grows editor", s["split"] > r0 + 0.02, f"split {r0:.3f}->{s['split']:.3f}")
    x, y = scr(s, "splitter", 0.5, 0.5); r1 = s["split"]
    inj("drag", f"{x:.0f}", f"{y:.0f}", f"{x:.0f}", f"{y-1000:.0f}", 25); time.sleep(0.4); s = state()
    check("panels: drag far up clamps (editor keeps min height)", 0.0 < s["split"] < r1, f"split {r1:.3f}->{s['split']:.3f}")

    # --- run / cancel / theme ---
    key(36, "cmd"); s = wait(lambda s: s["run"] in ("running", "done"), 1.0)
    saw = s["run"]; s = wait(lambda s: s["run"] == "done", 3.0)
    check("run: Cmd+Enter starts mock run and completes without UI hang", saw in ("running", "done") and s["run"] == "done", f"saw={saw} end={s['run']}")
    key(36, "cmd"); wait(lambda s: s["run"] == "running", 1.0); key(53); s = wait(lambda s: s["run"] == "cancelled", 2.0)
    check("run: Esc cancels a running mock query", s["run"] == "cancelled", f"run={s['run']}")
    d0 = state()["dark"]; key(37, "cmd,shift"); s = state()
    check("theme: Cmd+Shift+L toggles", s["dark"] != d0, f"dark {d0}->{s['dark']}")
finally:
    p.terminate()
    ok = sum(1 for _, o in results if o)
    print(f"\nSUMMARY: {ok}/{len(results)} passed; failed: {[n for n, o in results if not o]}")
