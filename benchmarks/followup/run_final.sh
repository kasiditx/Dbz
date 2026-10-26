#!/bin/bash
# Final-build validation chain for the T-003 follow-up (hands off the machine while it runs).
set -u
export PATH="/opt/homebrew/opt/rustup/bin:$PATH"
cd "$(dirname "$0")/../.."
SP=${INJECT_DIR:?set INJECT_DIR to the dir holding the compiled inject binary}
python3 benchmarks/run_t003.py > benchmarks/results/final_run.log 2>&1
cd benchmarks/followup
for rep in 1 2 3; do for rows in 100000 1000000; do python3 resize_mouse.py $SP/inject final_mouse_${rows}_r$rep $rows; done; done > raw/final_mouse.log 2>&1
for rep in 1 2 3; do for rows in 100000 1000000; do python3 latency.py $SP/inject final_${rows}_r$rep $rows; done; done > raw/final_latency.log 2>&1
python3 idle_outlier.py final_idle 3 60 > raw/final_idle.log 2>&1
python3 e2e_input.py $SP/inject > raw/final_e2e.log 2>&1
echo ALL_DONE > raw/final_done.flag
