#!/usr/bin/env python3
"""T-003 benchmark harness (macOS). Raw samples only; summarize.py derives percentiles.
Usage: python3 benchmarks/run_t003.py [--quick]
Every launched app opens a real window. Peak RSS/CPU come from wait4() rusage of the child.
"""
import json, os, platform, subprocess, sys, time, datetime, pathlib

ROOT = pathlib.Path(__file__).resolve().parent.parent
BIN = ROOT / "crates/poc-ui/target/release/dbz-ui-poc"
QUICK = "--quick" in sys.argv
OUT = ROOT / "benchmarks/results" / ("quick-" if QUICK else "") / datetime.datetime.now().strftime("%Y%m%d-%H%M%S")
OUT.mkdir(parents=True, exist_ok=True)

def sh(*a):
    return subprocess.run(a, capture_output=True, text=True).stdout.strip()

def env_info():
    info = {
        "date": datetime.datetime.now().isoformat(),
        "macos": sh("sw_vers", "-productVersion"),
        "cpu": sh("sysctl", "-n", "machdep.cpu.brand_string"),
        "ncpu": sh("sysctl", "-n", "hw.ncpu"),
        "mem_bytes": sh("sysctl", "-n", "hw.memsize"),
        "rustc": sh("rustc", "--version"),
        "git_head": sh("git", "-C", str(ROOT), "rev-parse", "HEAD"),
        "git_dirty": bool(sh("git", "-C", str(ROOT), "status", "--porcelain")),
        "power": sh("pmset", "-g", "batt").splitlines()[0] if sh("pmset", "-g", "batt") else "unknown",
        "display": sh("system_profiler", "SPDisplaysDataType").replace("\n", " | ")[:400],
        "binary": str(BIN),
        "binary_bytes": BIN.stat().st_size,
        "renderer": "eframe default (wgpu/metal on macOS; not explicitly pinned)",
        "argv": sys.argv,
    }
    (OUT / "env.json").write_text(json.dumps(info, indent=2))
    return info

def run(args, name):
    t0 = time.perf_counter()
    p = subprocess.Popen([str(BIN), *args], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    ready = None
    line = p.stdout.readline()
    if line.startswith("READY"):
        ready = (time.perf_counter() - t0) * 1000
    _, _, ru = os.wait4(p.pid, 0)
    wall = time.perf_counter() - t0
    rec = {"name": name, "args": args, "ext_ready_ms": ready, "ready_line": line.strip(),
           "wall_s": wall, "cpu_user_s": ru.ru_utime, "cpu_sys_s": ru.ru_stime,
           "peak_rss_mib": ru.ru_maxrss / (1024 * 1024)}  # macOS: bytes
    return rec

def cputime(txt):
    m, sec = txt.split(":")
    return int(m) * 60 + float(sec)

def idle_run(rows, secs):
    p = subprocess.Popen([str(BIN), "--rows", str(rows), "--bench", "idle"], stdout=subprocess.PIPE, text=True)
    t0 = time.perf_counter()
    line = p.stdout.readline()
    ready = (time.perf_counter() - t0) * 1000 if line.startswith("READY") else None
    samples = []
    time.sleep(2)  # let startup frames settle; then sample for `secs`
    for i in range(secs):
        # macOS ps %cpu is a lifetime average, so record cumulative cputime and derive the delta.
        out = sh("ps", "-o", "rss=,cputime=", "-p", str(p.pid)).split()
        samples.append({"t_s": i, "wall": time.perf_counter(), "rss_kib": int(out[0]), "cpu_s": cputime(out[1])} if len(out) == 2 else {"t_s": i, "rss_kib": None, "cpu_s": None})
        time.sleep(1)
    p.terminate(); p.wait()
    return {"rows": rows, "ext_ready_ms": ready, "samples": samples}

def main():
    env = env_info()
    reps = 2 if QUICK else 5
    results = {"startup": [], "scenarios": [], "idle": []}
    for i in range(10 if not QUICK else 2):
        results["startup"].append(run(["--rows", "100000", "--bench", "startup"], f"startup#{i+1}"))
        print("startup", i + 1, results["startup"][-1]["ready_line"], flush=True)
    for rows in (100_000, 1_000_000):
        for sc in ("scroll", "nav"):
            for i in range(reps):
                csv = OUT / f"{sc}_{rows}_{i+1}.csv"
                rec = run(["--rows", str(rows), "--bench", sc, "--out", str(csv)], f"{sc}/{rows}/{i+1}")
                rec["csv"] = csv.name; rec["rows"] = rows; rec["scenario"] = sc
                results["scenarios"].append(rec); print(rec["name"], f"peak {rec['peak_rss_mib']:.0f} MiB", flush=True)
    for i in range(reps):
        csv = OUT / f"resize_100000_{i+1}.csv"
        rec = run(["--rows", "100000", "--bench", "resize", "--out", str(csv)], f"resize/100000/{i+1}")
        rec["csv"] = csv.name; rec["rows"] = 100000; rec["scenario"] = "resize"
        results["scenarios"].append(rec); print(rec["name"], flush=True)
    idle_secs = 10 if QUICK else 60
    plan = [(1_000, 1), (100_000, 1 if QUICK else 3), (1_000_000, 1)]
    for rows, n in plan:
        for i in range(n):
            results["idle"].append(idle_run(rows, idle_secs)); print("idle", rows, i + 1, flush=True)
    (OUT / "results.json").write_text(json.dumps(results, indent=2))
    print("WROTE", OUT)

main()
