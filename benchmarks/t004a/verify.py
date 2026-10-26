"""Run reproducible local checks and preserve exact exit codes/output."""
import json
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'benchmarks/t004a/raw'
env = dict(os.environ)
env['PATH'] = '/opt/homebrew/opt/rustup/bin:' + env['PATH']
commands = []
for crate in ['poc-workload', 'poc-ui', 'poc-tauri/src-tauri']:
    manifest = f'crates/{crate}/Cargo.toml'
    for args in [['fmt', '--check'], ['test', '--locked'], ['clippy', '--locked', '--all-targets', '--', '-D', 'warnings']]:
        # cargo fmt manifest option must precede the formatter's -- separator.
        commands.append(['cargo', args[0], '--manifest-path', manifest, *args[1:]])
commands += [['npm', 'run', name, '--prefix', 'crates/poc-tauri'] for name in ['typecheck', 'lint', 'test', 'build']]
commands += [['python3', '-m', 'unittest', 'discover', '-s', 'benchmarks/t004a', '-p', 'test_*.py']]
results = []
for index, cmd in enumerate(commands):
    log = OUT / f'check-{index:02}.log'
    with log.open('w') as f:
        proc = subprocess.run(cmd, cwd=ROOT, env=env, stdout=f, stderr=subprocess.STDOUT)
    results.append({'command': cmd, 'exit_code': proc.returncode, 'log': str(log.relative_to(ROOT))})
    print(proc.returncode, ' '.join(cmd), flush=True)
(OUT / 'test-results.json').write_text(json.dumps(results, indent=2))
raise SystemExit(any(item['exit_code'] for item in results))
