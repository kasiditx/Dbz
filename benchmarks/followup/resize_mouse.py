#!/usr/bin/env python3
"""Interactive-style live resize: real mouse drags on the window's bottom-right corner (CGEvent).
Usage: python3 resize_mouse.py <inject> <label> <rows> [app-binary]
Writes raw/<label>.csv (per-frame, 'external' phase = whole run after 30 warm-up frames) and prints stalls."""
import csv, json, math, pathlib, subprocess, sys, time

INJ, label, rows = sys.argv[1], sys.argv[2], sys.argv[3]
ROOT = pathlib.Path(__file__).resolve().parents[2]
APP = sys.argv[4] if len(sys.argv) > 4 else str(ROOT / "crates/poc-ui/target/release/dbz-ui-poc")
RAW = pathlib.Path(__file__).resolve().parent / "raw"; RAW.mkdir(exist_ok=True)
LOG = pathlib.Path(INJ).parent / f"rm_{label}.log"; CSV = RAW / f"{label}.csv"

def pct(v, p):
    v = sorted(v); n = len(v)
    return v[max(math.ceil(p / 100 * n), 1) - 1] if n else float("nan")

def inner():
    last = None
    for l in LOG.read_text().splitlines():
        try: last = json.loads(l)
        except Exception: pass
    return last["inner"] if last else None

p = subprocess.Popen([APP, "--rows", rows, "--bench", "external", "--seconds", "26", "--out", str(CSV), "--event-log", str(LOG)],
                     stdout=subprocess.PIPE, text=True)
p.stdout.readline()
time.sleep(1.5)
subprocess.run(["osascript", "-e", f'tell application "System Events" to set frontmost of (first process whose unix id is {p.pid}) to true'], capture_output=True)
time.sleep(0.8)
sizes = set(); drags = 0
for i in range(14):
    r = inner()
    if r is None: break
    x, y = r[2] - 3, r[3] - 3
    dx, dy = (160, 90) if i % 2 == 0 else (-160, -90)
    subprocess.run([INJ, "drag", str(int(x)), str(int(y)), str(int(x + dx)), str(int(y + dy)), "30"], capture_output=True)
    drags += 1
    r2 = inner(); sizes.add((round(r2[2] - r2[0]), round(r2[3] - r2[1])))
    time.sleep(0.3)
p.wait(timeout=120)
rows_ = [r for r in csv.DictReader(open(CSV)) if r["phase"] == "external"]
iv = [float(r["interval_ms"]) for r in rows_]
st = [x for x in iv if x > 100]
wsz = {(r["win_w"], r["win_h"]) for r in rows_}
print(f"[{label}] drags={drags} distinct_window_sizes_in_csv={len(wsz)} frames={len(rows_)} stalls(>100ms)={len(st)} stall_total_ms={sum(st):.0f} "
      f"interval p50={pct(iv,50):.2f} p95={pct(iv,95):.2f} max={max(iv):.0f} | ui_cpu p95={pct([float(r['cpu_ms']) for r in rows_],95):.2f}")
