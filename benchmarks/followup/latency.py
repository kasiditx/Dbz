#!/usr/bin/env python3
"""Input -> app-state latency with real OS key events.
latency = (time the app finished the ui() frame that applied the key, from --event-log ts_ns)
          - (CLOCK_REALTIME just before the injector posts the CGEvent).
EXCLUDES GPU present and display scan-out (not observable without a camera/photodiode) and includes
injector scheduling noise. Usage: python3 latency.py <inject> <label> <rows> [app]"""
import json, math, pathlib, subprocess, sys, time

INJ, label, rows = sys.argv[1], sys.argv[2], sys.argv[3]
ROOT = pathlib.Path(__file__).resolve().parents[2]
APP = sys.argv[4] if len(sys.argv) > 4 else str(ROOT / "crates/poc-ui/target/release/dbz-ui-poc")
LOG = pathlib.Path(INJ).parent / f"lat_{label}.log"
OUT = pathlib.Path(__file__).resolve().parent / "raw"; OUT.mkdir(exist_ok=True)
N = 100

def pct(v, p):
    v = sorted(v); n = len(v)
    return v[max(math.ceil(p / 100 * n), 1) - 1] if n else float("nan")

def lines():
    out = []
    for l in LOG.read_text().splitlines():
        try: out.append(json.loads(l))
        except Exception: pass
    return out

p = subprocess.Popen([APP, "--rows", rows, "--event-log", str(LOG)], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
try:
    time.sleep(2.5)
    subprocess.run(["osascript", "-e", f'tell application "System Events" to set frontmost of (first process whose unix id is {p.pid}) to true'], capture_output=True)
    time.sleep(0.8)
    subprocess.run([INJ, "key", "20", "alt"], capture_output=True); time.sleep(0.4)  # Alt+3 -> grid
    grid, posts = [], []
    for i in range(N):
        ts = int(subprocess.run([INJ, "key", "125"], capture_output=True, text=True).stdout.strip())
        posts.append(ts); time.sleep(0.15)
    time.sleep(0.5)
    L = lines()
    for i, ts in enumerate(posts):
        want = i + 1
        hit = next((l for l in L if l["sel"][0] == want and l["ts_ns"] >= ts), None)
        grid.append((hit["ts_ns"] - ts) / 1e6 if hit else None)
    # editor typing: focus editor, press 'x' 100 times via Unicode events
    subprocess.run([INJ, "key", "19", "alt"], capture_output=True); time.sleep(0.4)
    base = lines()[-1]["chars"]; posts2 = []
    for i in range(N):
        t0 = int(subprocess.run([INJ, "type", "x"], capture_output=True, text=True).stdout.strip())
        posts2.append(t0); time.sleep(0.1)
    time.sleep(0.5)
    L = lines(); ed = []
    for i, ts in enumerate(posts2):
        hit = next((l for l in L if l["chars"] >= base + i + 1 and l["ts_ns"] >= ts), None)
        ed.append((hit["ts_ns"] - ts) / 1e6 if hit else None)
finally:
    p.terminate()

res = {"label": label, "rows": rows, "grid_down_ms": grid, "editor_type_ms": ed}
(OUT / f"latency_{label}.json").write_text(json.dumps(res))
for name, v in (("grid ArrowDown", grid), ("editor typing (Unicode key event)", ed)):
    ok = [x for x in v if x is not None]
    print(f"[{label}] {name}: n={len(ok)}/{len(v)} p50={pct(ok,50):.1f} p95={pct(ok,95):.1f} max={max(ok):.1f} ms")
