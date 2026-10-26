"""Ten sequential idle windows; no UI automation or compilation during this suite."""
from harness import prepare_egui_bundle, environment, sample_session, startup

prepare_egui_bundle()
environment()
startup()
for run, rows in enumerate([1000, 100000, 1000000, 100000, 100000]):
    order = ['egui', 'tauri'] if run % 2 == 0 else ['tauri', 'egui']
    for stack in order:
        label = f'idle-{stack}-{rows}-{run}'
        print(f'BEGIN {label}', flush=True)
        sample_session(stack, rows, label, 65)
        print(f'END {label}', flush=True)
