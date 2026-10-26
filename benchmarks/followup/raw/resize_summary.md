## Programmatic / AX-driven resize (raw/ab_run.log)

| variant | case | runs | runs with >=1 stall | total stalls | total stalled ms |
|---|---|---|---|---|---|
| base | external_ax_100k | 4 | 2 | 3 | 2288 |
| base | external_ax_1m | 4 | 3 | 11 | 11150 |
| base | prog_every1_100k | 1 | 1 | 2 | 2015 |
| nofix | external_ax_100k | 4 | 1 | 1 | 749 |
| nofix | external_ax_1m | 4 | 0 | 0 | 0 |
| nofix | prog_every1_100k | 1 | 0 | 0 | 0 |

## Real mouse live-resize (CGEvent corner drags; raw/mouse_ab.log)

| variant | rows | runs | frames | stalls >100ms | worst frame interval ms |
|---|---|---|---|---|---|
| base | 100000 | 4 | 12568 | 0 | 20 |
| base | 1000000 | 4 | 12519 | 0 | 29 |
| nofix | 100000 | 4 | 12065 | 0 | 28 |
| nofix | 1000000 | 4 | 12088 | 0 | 26 |
