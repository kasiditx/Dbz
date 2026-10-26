#!/usr/bin/env python3
"""Resize-stall experiments. Usage: python3 resize_experiments.py <label> [binary]
Each case writes raw per-frame CSV into ./raw/<label>_<case>.csv and prints a one-line verdict.
Stall = frame interval > 100 ms. 'external' resizes via System Events (AX) from another process.
"""
import csv, math, os, pathlib, subprocess, sys, time

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parent.parent
label = sys.argv[1]
BIN = sys.argv[2] if len(sys.argv) > 2 else str(ROOT / "crates/poc-ui/target/release/dbz-ui-poc")
RAW = HERE / "raw"; RAW.mkdir(exist_ok=True)

def pct(v, p):
    v = sorted(v); n = len(v)
    return v[max(math.ceil(p / 100 * n), 1) - 1] if n else float("nan")

def analyze(path, phase):
    rows = [r for r in csv.DictReader(open(path)) if r["phase"] == phase]
    iv = [float(r["interval_ms"]) for r in rows]
    cpu = [float(r["cpu_ms"]) for r in rows]
    st = [x for x in iv if x > 100]
    sizes = {(r["win_w"], r["win_h"]) for r in rows}
    return (f"frames={len(rows)} sizes={len(sizes)} stalls(>100ms)={len(st)} stall_total_ms={sum(st):.0f} "
            f"interval p50={pct(iv,50):.2f} p95={pct(iv,95):.2f} max={max(iv):.0f} | ui_cpu p95={pct(cpu,95):.2f}")

def prog(case, rows, every, frames):
    out = RAW / f"{label}_{case}.csv"
    t0 = time.time()
    subprocess.run([BIN, "--rows", str(rows), "--bench", "resize", "--resize-every", str(every),
                    "--resize-frames", str(frames), "--out", str(out)], capture_output=True, timeout=300)
    print(f"[{label}] {case}: wall={time.time()-t0:.1f}s", analyze(out, "resize"), flush=True)

def external(case, rows, steps, delay):
    out = RAW / f"{label}_{case}.csv"
    p = subprocess.Popen([BIN, "--rows", str(rows), "--bench", "external", "--seconds", "20", "--out", str(out)],
                         stdout=subprocess.PIPE, text=True)
    p.stdout.readline()  # READY
    time.sleep(2.0)      # let window settle; first 30 frames are 'warmup'
    errs = []
    for i in range(steps):
        w = 1100 + int(300 * math.sin(i / 3)); h = 760 + int(140 * math.cos(i / 4))
        script = ('tell application "System Events" to tell (first process whose unix id is %d) '
                  'to set size of window 1 to {%d, %d}' % (p.pid, w, h))
        r = subprocess.run(["osascript", "-e", script], capture_output=True, text=True)
        if r.returncode != 0:
            errs.append(r.stderr.strip()[:120])
        time.sleep(delay)
    p.wait(timeout=120)
    print(f"[{label}] {case}: osascript_errors={len(errs)} {errs[:1]}", analyze(out, "external"), flush=True)

which = os.environ.get("CASES", "prog1,prog5,prog30,ext100k,ext1m").split(",")
if "prog1" in which: prog("prog_every1_100k", 100_000, 1, 120)
if "prog5" in which: prog("prog_every5_100k", 100_000, 5, 120)
if "prog30" in which: prog("prog_every30_100k", 100_000, 30, 120)
if "ext100k" in which: external("external_ax_100k", 100_000, 40, 0.15)
if "ext1m" in which: external("external_ax_1m", 1_000_000, 40, 0.15)
