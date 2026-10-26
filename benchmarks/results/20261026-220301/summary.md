# Summary of 20261026-220301

env: Apple M4 Pro · macOS 26.5.2 · rustc 1.99.1 (58cf8f9c3 2026-10-13) · git 0d4972d (dirty) · Now drawing from 'Battery Power'

## Startup (warm, n=10; cold NOT MEASURED)

| metric | p50 ms | p95 ms | min | max |
|---|---|---|---|---|
| process spawn → READY (external) | 206.6 | 830.3 | 191.9 | 830.3 |
| main() → first ui() (internal) | 111.5 | 143.2 | 104.6 | 143.2 |
| main() → frame 6 ready (internal) | 201.3 | 253.2 | 186.7 | 253.2 |

raw external ready ms: [830.3, 224.7, 220.8, 191.9, 200.6, 206.6, 199.4, 219.5, 209.7, 203.5] 

## Frame metrics (all frames of all runs pooled; warmup phase excluded)

| scenario | rows | phase | frames | ui() cpu p50 | p95 | max | frame Δ p50 | p95 | max | max cells/frame | max rows drawn | distinct win sizes |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| nav | 100000 | nav | 2010 | 0.54 | 2.22 | 3.48 | 8.32 | 9.42 | 16.26 | 540 | 27 | 1 |
| nav | 1000000 | nav | 2010 | 0.66 | 2.31 | 3.48 | 8.31 | 9.50 | 16.80 | 540 | 27 | 1 |
| resize (programmatic InnerSize EVERY frame: STRESS, not representative; see docs/evidence/T-003-followup.md) | 100000 | resize | 1210 | 0.44 | 0.80 | 2.44 | 7.75 | 1011.95 | 1025.03 | 540 | 27 | 242 |
| scroll | 100000 | slow | 1500 | 0.62 | 0.94 | 1.13 | 8.32 | 9.18 | 16.55 | 540 | 27 | 1 |
| scroll | 100000 | sweep | 1510 | 1.97 | 2.35 | 3.10 | 8.33 | 9.26 | 16.76 | 560 | 28 | 1 |
| scroll | 1000000 | slow | 1500 | 0.84 | 0.99 | 1.13 | 8.32 | 9.42 | 16.81 | 540 | 27 | 1 |
| scroll | 1000000 | sweep | 1510 | 2.25 | 2.47 | 3.25 | 8.31 | 9.34 | 16.79 | 560 | 28 | 1 |

## Process-level (per run; wait4 rusage)

| scenario | rows | runs | peak RSS MiB (min/med/max) | avg CPU % (cpu_time/wall) |
|---|---|---|---|---|
| nav | 100000 | 5 | 116 / 117 / 118 | 29 (min 27, max 33) |
| nav | 1000000 | 5 | 117 / 117 / 122 | 34 (min 33, max 35) |
| resize | 100000 | 5 | 115 / 117 / 119 | 4 (min 4, max 4) |
| scroll | 100000 | 5 | 121 / 121 / 124 | 32 (min 30, max 32) |
| scroll | 1000000 | 5 | 116 / 121 / 124 | 37 (min 36, max 37) |

## Idle (no repaint requested by app; ps samples every 1 s after 2 s settle; focused editor caret blink may repaint)

| rows | runs | secs | RSS MiB first→last (median run) | max RSS MiB | CPU % over window (cputime delta/wall) |
|---|---|---|---|---|---|
| 1000 | 1 | 60 | 118 → 118 | 118 | 0.77 (runs: [0.77]) |
| 100000 | 3 | 60 | 114 → 115 | 124 | 0.76 (runs: [0.53, 0.97, 0.77]) |
| 1000000 | 1 | 60 | 116 → 117 | 117 | 0.62 (runs: [0.62]) |
