#!/usr/bin/env python3
"""Summarize a results dir: python3 benchmarks/summarize.py benchmarks/results/<run>"""
import csv, json, sys, math, pathlib, collections

d = pathlib.Path(sys.argv[1])
res = json.loads((d / "results.json").read_text())
env = json.loads((d / "env.json").read_text())

def pct(v, p):  # nearest-rank, same definition as model::metrics::Samples
    v = sorted(v); n = len(v)
    return v[max(math.ceil(p / 100 * n), 1) - 1] if n else float("nan")

print(f"# Summary of {d.name}\n\nenv: {env['cpu']} · macOS {env['macos']} · {env['rustc']} · git {env['git_head'][:7]}{' (dirty)' if env['git_dirty'] else ''} · {env['power']}\n")

st = res["startup"]
if st:
    ext = [r["ext_ready_ms"] for r in st if r["ext_ready_ms"]]
    inn = [float(r["ready_line"].split("ready_ms=")[1]) for r in st if "ready_ms=" in r["ready_line"]]
    fu = [float(r["ready_line"].split("first_ui_ms=")[1].split()[0]) for r in st if "first_ui_ms=" in r["ready_line"]]
    print(f"## Startup (warm, n={len(ext)}; cold NOT MEASURED)\n")
    print("| metric | p50 ms | p95 ms | min | max |\n|---|---|---|---|---|")
    for name, v in (("process spawn → READY (external)", ext), ("main() → first ui() (internal)", fu), ("main() → frame 6 ready (internal)", inn)):
        if v: print(f"| {name} | {pct(v,50):.1f} | {pct(v,95):.1f} | {min(v):.1f} | {max(v):.1f} |")
    print("\nraw external ready ms:", [round(x, 1) for x in ext], "\n")

groups = collections.defaultdict(lambda: collections.defaultdict(lambda: {"cpu": [], "int": [], "cells": [], "vis": []}))
runs = collections.defaultdict(list)
for r in res["scenarios"]:
    key = (r["scenario"], r["rows"])
    runs[key].append(r)
    with open(d / r["csv"]) as f:
        for row in csv.DictReader(f):
            g = groups[key][row["phase"]]
            g["cpu"].append(float(row["cpu_ms"])); g["int"].append(float(row["interval_ms"]))
            g["cells"].append(int(row["cells"])); g["vis"].append(int(row["vis_rows"]))
            g.setdefault("w", set()).add((row["win_w"], row["win_h"]))

print("## Frame metrics (all frames of all runs pooled; warmup phase excluded)\n")
print("| scenario | rows | phase | frames | ui() cpu p50 | p95 | max | frame Δ p50 | p95 | max | max cells/frame | max rows drawn | distinct win sizes |\n|---|---|---|---|---|---|---|---|---|---|---|---|---|")
for key in sorted(groups):
    for ph, g in groups[key].items():
        if ph == "warmup": continue
        print(f"| {key[0]} | {key[1]} | {ph} | {len(g['cpu'])} | {pct(g['cpu'],50):.2f} | {pct(g['cpu'],95):.2f} | {max(g['cpu']):.2f} | {pct(g['int'],50):.2f} | {pct(g['int'],95):.2f} | {max(g['int']):.2f} | {max(g['cells'])} | {max(g['vis'])} | {len(g['w'])} |")

print("\n## Process-level (per run; wait4 rusage)\n")
print("| scenario | rows | runs | peak RSS MiB (min/med/max) | avg CPU % (cpu_time/wall) |\n|---|---|---|---|---|")
for key in sorted(runs):
    rs = runs[key]
    peaks = [r["peak_rss_mib"] for r in rs]
    cpu = [100 * (r["cpu_user_s"] + r["cpu_sys_s"]) / r["wall_s"] for r in rs]
    print(f"| {key[0]} | {key[1]} | {len(rs)} | {min(peaks):.0f} / {pct(peaks,50):.0f} / {max(peaks):.0f} | {pct(cpu,50):.0f} (min {min(cpu):.0f}, max {max(cpu):.0f}) |")

print("\n## Idle (no repaint requested by app; ps samples every 1 s after 2 s settle; focused editor caret blink may repaint)\n")
print("| rows | runs | secs | RSS MiB first→last (median run) | max RSS MiB | CPU % over window (cputime delta/wall) |\n|---|---|---|---|---|---|")
by = collections.defaultdict(list)
for r in res["idle"]: by[r["rows"]].append(r)
for rows, rs in sorted(by.items()):
    firsts, lasts, mx, cpus = [], [], [], []
    for r in rs:
        s = [x for x in r["samples"] if x["rss_kib"] is not None]
        if not s: continue
        firsts.append(s[0]["rss_kib"] / 1024); lasts.append(s[-1]["rss_kib"] / 1024)
        mx.append(max(x["rss_kib"] for x in s) / 1024)
        cpus.append(100 * (s[-1]["cpu_s"] - s[0]["cpu_s"]) / (s[-1]["wall"] - s[0]["wall"]))
    if firsts:
        print(f"| {rows} | {len(rs)} | {len(rs[0]['samples'])} | {pct(firsts,50):.0f} → {pct(lasts,50):.0f} | {max(mx):.0f} | {sum(cpus)/len(cpus):.2f} (runs: {[round(c,2) for c in cpus]}) |")
