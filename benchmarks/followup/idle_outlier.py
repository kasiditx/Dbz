#!/usr/bin/env python3
"""Idle-CPU outlier hunt: N idle runs; when a 1 s cputime delta >= 0.10 s is seen, capture a 5 s
`sample` of the main thread immediately. Usage: python3 idle_outlier.py <label> <runs> <secs> [app]"""
import pathlib, subprocess, sys, time, json

label, runs, secs = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
ROOT = pathlib.Path(__file__).resolve().parents[2]
APP = sys.argv[4] if len(sys.argv) > 4 else str(ROOT / "crates/poc-ui/target/release/dbz-ui-poc")
RAW = pathlib.Path(__file__).resolve().parent / "raw"; RAW.mkdir(exist_ok=True)

def cputime(t):
    m, s = t.split(":"); return int(m) * 60 + float(s)

def ps(pid):
    o = subprocess.run(["ps", "-o", "rss=,cputime=", "-p", str(pid)], capture_output=True, text=True).stdout.split()
    return (int(o[0]), cputime(o[1])) if len(o) == 2 else None

summary = []
for run in range(runs):
    p = subprocess.Popen([APP, "--rows", "100000", "--bench", "idle"], stdout=subprocess.PIPE, text=True)
    p.stdout.readline(); time.sleep(2)
    series, sampled = [], False
    prev = ps(p.pid)
    for i in range(secs):
        time.sleep(1)
        cur = ps(p.pid)
        if not cur or not prev: break
        d = cur[1] - prev[1]; series.append(round(d, 2))
        if d >= 0.10 and not sampled:
            sampled = True
            subprocess.run(["sample", str(p.pid), "5", "-file", str(RAW / f"{label}_run{run+1}_sample.txt")], capture_output=True)
        prev = cur
    p.terminate(); p.wait()
    total = sum(series)
    summary.append({"run": run + 1, "cpu_pct": 100 * total / max(len(series), 1), "series": series, "sampled": sampled})
    print(f"[{label}] run{run+1}: cpu={100*total/max(len(series),1):.2f}% bursts(>=0.10s/s)={sum(1 for x in series if x>=0.10)} sampled={sampled}", flush=True)
(RAW / f"{label}_idle.json").write_text(json.dumps(summary))
