#!/usr/bin/env python3
"""Release-only macOS sampling. UI actions are supplied separately through cua_repl.
No DB, no cache purge, no unrelated process termination. No shell expansion.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import plistlib
import shutil
import signal
import subprocess
import time
import tempfile
from ownership import assign

ROOT = Path(__file__).resolve().parents[2]
RAW = ROOT / 'benchmarks/t004a/raw'
TAURI = ROOT / 'crates/poc-tauri/src-tauri/target/release/bundle/macos/Dbz Tauri POC.app/Contents/MacOS/dbz-tauri-poc'
EGUI = ROOT / 'crates/poc-ui/target/release/bundle/Dbz Egui POC.app/Contents/MacOS/dbz-ui-poc'


def capture(args):
    p = subprocess.run(args, capture_output=True, text=True, timeout=20)
    return {'returncode': p.returncode, 'stdout': p.stdout.strip(), 'stderr': p.stderr.strip()}


def process_table():
    out = capture(['ps', '-axo', 'pid=,ppid=,rss=,time=,comm='])['stdout']
    result = {}
    for line in out.splitlines():
        fields = line.split(None, 4)
        if len(fields) != 5:
            continue
        pid, parent, rss, cpu, name = fields
        parts = cpu.split(':')
        seconds = sum(float(part) * 60 ** i for i, part in enumerate(reversed(parts)))
        result[int(pid)] = {'pid': int(pid), 'ppid': int(parent), 'rss_kib': int(rss), 'cpu_s': seconds, 'name': name}
    return result


def launch(stack, rows, label, mode='idle', seconds=0, large=False):
    session_dir = Path(tempfile.mkdtemp(prefix=f'dbz-t004a-{label}-'))
    events = session_dir / f'{label}-events.jsonl'
    frames = session_dir / f'{label}-frames.{"jsonl" if stack == "tauri" else "csv"}'
    args = [str(TAURI if stack == 'tauri' else EGUI), '--rows', str(rows), '--event-log', str(events)]
    if large:
        args.append('--large-sql')
    if stack == 'tauri':
        args += ['--measure', '--seconds', str(seconds), '--out', str(frames)]
    else:
        args += ['--comparison', '--bench', 'external' if seconds else 'idle', '--seconds', str(seconds), '--out', str(frames)]
    bundle = str(Path(args[0]).parents[2])
    stdout_path = session_dir / f'{label}-stdout.log'
    stderr_path = session_dir / f'{label}-stderr.log'
    stdout_path.write_text('')
    stderr_path.write_text('')
    start = time.perf_counter()
    launcher = subprocess.Popen(['open', '-n', '-W', '--stdout', str(stdout_path), '--stderr', str(stderr_path), '-a', bundle, '--args', *args[1:]], stdout=subprocess.DEVNULL, stderr=subprocess.PIPE)
    ready = None
    main_pid = None
    deadline = time.monotonic() + 30
    while time.monotonic() < deadline:
        output = stdout_path.read_text()
        match = re.search(r'READY pid=(\d+)', output)
        if match:
            ready = (time.perf_counter() - start) * 1000
            main_pid = int(match.group(1))
            break
        if launcher.poll() is not None:
            break
        time.sleep(.005)
    if ready is None:
        launcher.terminate()
        raise RuntimeError(f'{label}: no READY within 30 seconds: {launcher.stderr.read().decode()}')
    launcher.app_pid = main_pid
    launcher.session_dir = session_dir
    return launcher, None, {'stack': stack, 'rows': rows, 'mode': mode, 'pid': main_pid, 'ready_ms': ready,
                             'args': args, 'bundle': bundle, 'events': str(events), 'frames': str(frames), 'large_sql': large}


def stop(process, _stderr):
    try:
        os.kill(process.app_pid, signal.SIGTERM)
    except ProcessLookupError:
        pass
    try:
        process.wait(timeout=5)
    except subprocess.TimeoutExpired:
        process.terminate()
        process.wait(timeout=5)
    process.stderr.close()
    for source in process.session_dir.iterdir():
        if source.is_file(): shutil.copy2(source, RAW / source.name)


def prepare_egui_bundle():
    EGUI.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(ROOT / 'crates/poc-ui/target/release/dbz-ui-poc', EGUI)
    (EGUI.parent.parent / 'Info.plist').write_bytes(plistlib.dumps({
        'CFBundleExecutable': 'dbz-ui-poc', 'CFBundleIdentifier': 'dev.dbz.t004a.egui',
        'CFBundleName': 'Dbz Egui POC', 'CFBundlePackageType': 'APPL', 'NSHighResolutionCapable': True,
    }))
    subprocess.run(['codesign', '--force', '--sign', '-', str(EGUI.parents[2])], check=True, capture_output=True)


def state(path):
    try:
        lines = Path(path).read_text().splitlines()
    except FileNotFoundError:
        return None
    for line in reversed(lines):
        try:
            return json.loads(line)
        except json.JSONDecodeError:
            continue
    return None


def environment():
    info = {name: capture(cmd) for name, cmd in {
        'os': ['sw_vers'], 'cpu': ['sysctl', '-n', 'machdep.cpu.brand_string'],
        'ram': ['sysctl', '-n', 'hw.memsize'], 'cpus': ['sysctl', '-n', 'hw.logicalcpu'],
        'power': ['pmset', '-g', 'batt'], 'display': ['system_profiler', 'SPDisplaysDataType'],
        'rustc': ['/opt/homebrew/opt/rustup/bin/rustc', '--version'], 'node': ['node', '--version'],
        'head': ['git', 'rev-parse', 'HEAD'], 'status': ['git', 'status', '--short'],
    }.items()}
    info['artifacts'] = {s: {'path': str(p), 'bytes': p.stat().st_size, 'sha256': hashlib.sha256(p.read_bytes()).hexdigest()} for s, p in [('tauri', TAURI), ('egui', EGUI)]}
    info['recorded_at'] = time.strftime('%Y-%m-%dT%H:%M:%S%z')
    (RAW / 'environment.json').write_text(json.dumps(info, indent=2))


def startup():
    prewarm = []
    for stack in ['egui', 'tauri']:
        process, err, row = launch(stack, 100000, f'prewarm-{stack}')
        stop(process, err)
        prewarm.append(row)
        time.sleep(.7)
    (RAW / 'prewarm.json').write_text(json.dumps(prewarm, indent=2))
    results = []
    for run in range(10):
        # Alternate order to reduce drift; preserve first run/cache outliers.
        for stack in (['egui', 'tauri'] if run % 2 == 0 else ['tauri', 'egui']):
            p, err, row = launch(stack, 100000, f'startup-{stack}-{run}')
            row['run'] = run
            row['state'] = state(row['events'])
            stop(p, err)
            results.append(row)
            (RAW / 'startup.json').write_text(json.dumps(results, indent=2))
            print(stack, run, round(row['ready_ms'], 2), flush=True)
            time.sleep(.7)


def sample_session(stack, rows, label, duration, active=False, large=False):
    if any(RAW.glob(f'{label}-*')):
        raise ValueError(f'Label already has raw artifacts: {label}; choose a new label')
    before = process_table()
    p, err, meta = launch(stack, rows, label, 'active' if active else 'idle', duration if active else 0, large)
    meta['power_at_start'] = capture(['pmset', '-g', 'batt'])
    meta['baseline_webkit_pids'] = [pid for pid, item in before.items() if 'WebKit' in item['name']]
    meta['state'] = state(meta['events'])
    meta['label'] = label
    (RAW / 'session.json').write_text(json.dumps(meta, indent=2))
    print(json.dumps(meta), flush=True)
    samples = []
    start = time.monotonic()
    try:
        while time.monotonic() - start < duration + (1 if active else 0):
            ownership = assign(p.app_pid)
            table = process_table()
            main = table.get(p.app_pid)
            candidates = [v for pid, v in table.items() if pid == p.app_pid or v['ppid'] == p.app_pid or ('WebKit' in v['name'] and pid not in before)]
            samples.append({'wall_ns': time.time_ns(), 'elapsed_s': time.monotonic() - start, 'main': main, 'candidates': candidates, 'ownership': ownership, 'attributed': [table[pid] for pid in ownership['pids'] if pid in table]})
            (RAW / f'{label}-memory.json').write_text(json.dumps({'metadata': meta, 'samples': samples}, indent=2))
            if p.poll() is not None:
                break
            time.sleep(1)
        if active:
            deadline = time.monotonic() + 20
            while time.monotonic() < deadline and p.poll() is None:
                frame_path = Path(meta['frames'])
                if frame_path.exists() and frame_path.stat().st_size > 0:
                    break
                time.sleep(.2)
        if samples and not active:
            pids = samples[-1]['ownership']['pids']
            if p.app_pid in pids and samples[-1]['main'] is not None:
                command = ['footprint', *[item for pid in pids for item in ['-p', str(pid)]], '-j', str(RAW / f'{label}-footprint.json')]
                measured = capture(command)
            else:
                measured = {'status': 'NOT MEASURED', 'reason': 'main process exited or ownership unresolved'}
            (RAW / f'{label}-footprint.txt').write_text(json.dumps(measured, indent=2))
        meta['power_at_end'] = capture(['pmset', '-g', 'batt'])
        meta['final_state'] = state(meta['events'])
        (RAW / f'{label}-session.json').write_text(json.dumps(meta, indent=2))
    finally:
        stop(p, err)


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('mode', choices=['env', 'startup', 'idle', 'active'])
    parser.add_argument('--stack', choices=['tauri', 'egui'], default='tauri')
    parser.add_argument('--rows', type=int, choices=[1000, 100000, 1000000], default=100000)
    parser.add_argument('--label', default='active')
    parser.add_argument('--seconds', type=float, default=65)
    parser.add_argument('--large-sql', action='store_true')
    args = parser.parse_args()
    RAW.mkdir(exist_ok=True)
    prepare_egui_bundle()
    if args.mode == 'env': environment()
    elif args.mode == 'startup': startup()
    else: sample_session(args.stack, args.rows, args.label, args.seconds, args.mode == 'active', args.large_sql)
