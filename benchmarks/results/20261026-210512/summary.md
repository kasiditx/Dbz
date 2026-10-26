# Summary of 20261026-210512

env: Apple M4 Pro · macOS 26.5.2 · rustc 1.99.1 (58cf8f9c3 2026-10-13) · git 7086980 (dirty) · Now drawing from 'Battery Power'

## Startup (warm, n=10; cold NOT MEASURED)

| metric | p50 ms | p95 ms | min | max |
|---|---|---|---|---|
| process spawn → READY (external) | 200.1 | 749.5 | 190.7 | 749.5 |
| main() → first ui() (internal) | 110.5 | 156.4 | 103.8 | 156.4 |
| main() → frame 6 ready (internal) | 194.4 | 324.4 | 185.5 | 324.4 |

raw external ready ms: [749.5, 200.1, 207.4, 200.0, 203.2, 196.2, 190.7, 209.0, 197.0, 206.6] 

## Frame metrics (all frames of all runs pooled; warmup phase excluded)

| scenario | rows | phase | frames | ui() cpu p50 | p95 | max | frame Δ p50 | p95 | max | max cells/frame | max rows drawn | distinct win sizes |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| nav | 100000 | nav | 2010 | 0.64 | 2.30 | 3.73 | 8.31 | 9.54 | 16.70 | 540 | 27 | 1 |
| nav | 1000000 | nav | 2010 | 0.66 | 2.28 | 3.53 | 8.30 | 9.45 | 15.52 | 540 | 27 | 1 |
| resize | 100000 | resize | 1210 | 0.25 | 0.69 | 2.19 | 5.27 | 1009.55 | 1033.13 | 540 | 27 | 242 |
| scroll | 100000 | slow | 1500 | 0.72 | 1.03 | 1.40 | 8.31 | 9.57 | 25.73 | 540 | 27 | 1 |
| scroll | 100000 | sweep | 1510 | 2.17 | 2.48 | 3.40 | 8.31 | 9.32 | 16.72 | 560 | 28 | 1 |
| scroll | 1000000 | slow | 1500 | 0.83 | 1.01 | 1.17 | 8.31 | 9.45 | 15.46 | 540 | 27 | 1 |
| scroll | 1000000 | sweep | 1510 | 2.11 | 2.43 | 3.28 | 8.31 | 9.32 | 15.50 | 560 | 28 | 1 |

## Process-level (per run; wait4 rusage)

| scenario | rows | runs | peak RSS MiB (min/med/max) | avg CPU % (cpu_time/wall) |
|---|---|---|---|---|
| nav | 100000 | 5 | 114 / 119 / 120 | 33 (min 30, max 35) |
| nav | 1000000 | 5 | 116 / 117 / 124 | 34 (min 33, max 34) |
| resize | 100000 | 5 | 115 / 116 / 116 | 14 (min 4, max 47) |
| scroll | 100000 | 5 | 114 / 117 / 118 | 36 (min 34, max 37) |
| scroll | 1000000 | 5 | 116 / 116 / 121 | 37 (min 34, max 37) |

## Idle (no repaint requested by app; ps samples every 1 s after 2 s settle; focused editor caret blink may repaint)

| rows | runs | secs | RSS MiB first→last (median run) | max RSS MiB | CPU % over window (cputime delta/wall) |
|---|---|---|---|---|---|
| 1000 | 1 | 60 | 114 → 111 | 114 | 0.74 (runs: [0.74]) |
| 100000 | 3 | 60 | 113 → 112 | 114 | 2.87 (runs: [1.4, 6.7, 0.5]) |
| 1000000 | 1 | 60 | 111 → 111 | 111 | 0.80 (runs: [0.8]) |
