# Runtime benchmarks

Machine: Darwin 15.8, Apple M4, 10 cores, 24576 MiB.

**Added by hand after the run, because the harness cannot see it and a reader needs it.** This machine was not quiet. Load average was 5.83 over the previous minute with one unrelated user process holding a core at 97 percent throughout, and every absolute number below is inflated by roughly a factor of three against the same machine quiet. The evidence is the cold start row measured against the 0.1.2 run on the same machine two hours earlier: katsu 1.57 to 4.31, node 24.11 to 70.50, bun 5.31 to 14.22, deno 12.65 to 24.00, which is every runtime moving the same way by a similar factor rather than any one of them changing. Ratios between runtimes in this report survive that and absolutes do not, so compare across a row and not against a different run. The katsu against bun cold start standing is 3.30x here against 3.38x in the quiet run, which is the same number.

## Distance to the goal

The goal is ten times better than the best rival on every axis, not ten times better than the worst one. This table is regenerated from the same run as everything below it.

| Axis | katsu | Best rival | Standing | Left to find |
|---|---:|---|---|---|
| Cold start (ms) | 4.31 | `bun` 14.22 | 3.30x ahead | 3.0x |
| Baseline memory at idle (MiB) | not yet | `bun` 12.56 | cannot run this yet | all of it. /Users/apple/.local/bin/katsu exited after 407 ms instead of staying idle, so there was no idle to measure |
| Distribution size (MiB) | 1.71 | `bun` 60.61 | 35.41x ahead | goal reached |
| Compute: alloc, In process (ms) | not yet | `bun` 92.21 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: alloc, Wall clock (ms) | not yet | `bun` 107.60 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: alloc, Runtime overhead (ms) | not yet | `bun` 15.56 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: alloc, Peak memory (MiB) | not yet | `bun` 46.59 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: fib, In process (ms) | 2329.34 | `bun` 92.31 | 25.23x behind | 252.3x |
| Compute: fib, Wall clock (ms) | 2334.36 | `bun` 109.09 | 21.40x behind | 214.0x |
| Compute: fib, Runtime overhead (ms) | 5.37 | `bun` 16.76 | 3.12x ahead | 3.2x |
| Compute: fib, Peak memory (MiB) | 2.70 | `bun` 18.25 | 6.75x ahead | 1.5x |
| Compute: json, In process (ms) | not yet | `bun` 164.10 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: json, Wall clock (ms) | not yet | `bun` 179.72 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: json, Runtime overhead (ms) | not yet | `bun` 16.24 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: json, Peak memory (MiB) | not yet | `bun` 34.94 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: nbody, In process (ms) | not yet | `node` 212.71 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: nbody, Wall clock (ms) | not yet | `deno` 260.12 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: nbody, Runtime overhead (ms) | not yet | `bun` 16.17 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: nbody, Peak memory (MiB) | not yet | `bun` 21.33 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: sort, In process (ms) | not yet | `bun` 75.40 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: sort, Wall clock (ms) | not yet | `bun` 89.50 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: sort, Runtime overhead (ms) | not yet | `bun` 13.76 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: sort, Peak memory (MiB) | not yet | `bun` 31.47 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: strings, In process (ms) | not yet | `bun` 29.50 | cannot run this yet | all of it. out of memory |
| Compute: strings, Wall clock (ms) | not yet | `bun` 45.66 | cannot run this yet | all of it. out of memory |
| Compute: strings, Runtime overhead (ms) | not yet | `bun` 16.87 | cannot run this yet | all of it. out of memory |
| Compute: strings, Peak memory (MiB) | not yet | `bun` 42.02 | cannot run this yet | all of it. out of memory |

**katsu ran 6 of the 27 axes in this report.** The 21 it did not run are marked as such above. Every win here is a win on an axis where katsu is doing less work than the runtime it beat, because most of a runtime is not built yet, so read them as a starting position rather than as a result. This paragraph goes away when the number that did not run reaches zero.

## Cold start (ms)

| Runtime | Version | Median | IQR | p25 | p75 | Min | Max | Runs |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| `katsu` | 0.1.3 | 4.31 | 0.85 | 3.98 | 4.83 | 3.05 | 14.71 | 25 |
| `node` | 26.8.1 | 70.50 | 12.31 | 64.57 | 76.88 | 59.26 | 161.10 | 25 |
| `bun` | 1.4.0 | 14.22 | 1.53 | 13.22 | 14.76 | 11.47 | 41.67 | 25 |
| `deno` | 2.9.6 | 24.00 | 1.74 | 23.17 | 24.91 | 21.59 | 121.49 | 25 |

`katsu` wins this axis at 4.31 ms, against `node` at 70.50, which is 16.4x.

## Baseline memory at idle (MiB)

| Runtime | Version | Median | IQR | p25 | p75 | Min | Max | Runs |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| `katsu` | 0.1.3 | not measured | | | | | | |
| `node` | 26.8.1 | 47.77 | 0.72 | 47.60 | 48.32 | 47.42 | 49.52 | 10 |
| `bun` | 1.4.0 | 12.56 | 0.16 | 12.47 | 12.63 | 12.30 | 12.77 | 10 |
| `deno` | 2.9.6 | 34.90 | 0.43 | 34.63 | 35.06 | 34.14 | 35.56 | 10 |

`katsu` could not be measured: /Users/apple/.local/bin/katsu exited after 407 ms instead of staying idle, so there was no idle to measure

`bun` wins this axis at 12.56 MiB, against `node` at 47.77, which is 3.8x.

## Distribution size (MiB)

| Runtime | Version | Median | IQR | p25 | p75 | Min | Max | Runs |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| `katsu` | 0.1.3 | 1.71 | 0.00 | 1.71 | 1.71 | 1.71 | 1.71 | 1 |
| `node` | 26.8.1 | 139.05 | 0.00 | 139.05 | 139.05 | 139.05 | 139.05 | 1 |
| `bun` | 1.4.0 | 60.61 | 0.00 | 60.61 | 60.61 | 60.61 | 60.61 | 1 |
| `deno` | 2.9.6 | 120.43 | 0.00 | 120.43 | 120.43 | 120.43 | 120.43 | 1 |

`katsu` wins this axis at 1.71 MiB, against `node` at 139.05, which is 81.2x.

## Compute: alloc

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.3 | not measured | not measured | not measured | not measured |
| `node` | 26.8.1 | 127.90 | 186.49 | 58.59 | 90.59 |
| `bun` | 1.4.0 | 92.21 | 107.60 | 15.56 | 46.59 |
| `deno` | 2.9.6 | 112.92 | 138.77 | 27.54 | 71.84 |

`katsu` could not be measured: katsu could not run alloc: katsu: not implemented yet: compute/alloc.js:19:21: new is not supported yet. See https://github.com/tamnd/katsu/milestones

In process. `bun` wins this axis at 92.21 ms, against `node` at 127.90, which is 1.4x.

Wall clock. `bun` wins this axis at 107.60 ms, against `node` at 186.49, which is 1.7x.

Runtime overhead. `bun` wins this axis at 15.56 ms, against `node` at 58.59, which is 3.8x.

Peak memory. `bun` wins this axis at 46.59 MiB, against `node` at 90.59, which is 1.9x.

## Compute: fib

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.3 | 2329.34 | 2334.36 | 5.37 | 2.70 |
| `node` | 26.8.1 | 128.86 | 187.37 | 58.22 | 56.08 |
| `bun` | 1.4.0 | 92.31 | 109.09 | 16.76 | 18.25 |
| `deno` | 2.9.6 | 110.55 | 137.31 | 26.86 | 36.84 |

In process. `bun` wins this axis at 92.31 ms, against `katsu` at 2329.34, which is 25.2x.

Wall clock. `bun` wins this axis at 109.09 ms, against `katsu` at 2334.36, which is 21.4x.

Runtime overhead. `katsu` wins this axis at 5.37 ms, against `node` at 58.22, which is 10.8x.

Peak memory. `katsu` wins this axis at 2.70 MiB, against `node` at 56.08, which is 20.7x.

## Compute: json

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.3 | not measured | not measured | not measured | not measured |
| `node` | 26.8.1 | 285.18 | 344.02 | 60.85 | 80.28 |
| `bun` | 1.4.0 | 164.10 | 179.72 | 16.24 | 34.94 |
| `deno` | 2.9.6 | 268.79 | 300.28 | 29.07 | 62.56 |

`katsu` could not be measured: katsu could not run json: katsu: not implemented yet: compute/json.js:18:11: an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones

In process. `bun` wins this axis at 164.10 ms, against `node` at 285.18, which is 1.7x.

Wall clock. `bun` wins this axis at 179.72 ms, against `node` at 344.02, which is 1.9x.

Runtime overhead. `bun` wins this axis at 16.24 ms, against `node` at 60.85, which is 3.7x.

Peak memory. `bun` wins this axis at 34.94 MiB, against `node` at 80.28, which is 2.3x.

## Compute: nbody

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.3 | not measured | not measured | not measured | not measured |
| `node` | 26.8.1 | 212.71 | 276.27 | 58.42 | 59.92 |
| `bun` | 1.4.0 | 254.19 | 270.48 | 16.17 | 21.33 |
| `deno` | 2.9.6 | 229.42 | 260.12 | 29.82 | 41.86 |

`katsu` could not be measured: katsu could not run nbody: katsu: not implemented yet: compute/nbody.js:13:10: an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones

In process. Too close to call between `node`, `deno`. Their interquartile ranges overlap, so this axis does not separate them and no winner is claimed.

Wall clock. Too close to call between `deno`, `node`, `bun`. Their interquartile ranges overlap, so this axis does not separate them and no winner is claimed.

Runtime overhead. `bun` wins this axis at 16.17 ms, against `node` at 58.42, which is 3.6x.

Peak memory. `bun` wins this axis at 21.33 MiB, against `node` at 59.92, which is 2.8x.

## Compute: sort

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.3 | not measured | not measured | not measured | not measured |
| `node` | 26.8.1 | 162.97 | 219.58 | 57.25 | 75.47 |
| `bun` | 1.4.0 | 75.40 | 89.50 | 13.76 | 31.47 |
| `deno` | 2.9.6 | 165.25 | 199.35 | 31.71 | 57.61 |

`katsu` could not be measured: katsu could not run sort: katsu: not implemented yet: compute/sort.js:23:19: new is not supported yet. See https://github.com/tamnd/katsu/milestones

In process. `bun` wins this axis at 75.40 ms, against `deno` at 165.25, which is 2.2x.

Wall clock. `bun` wins this axis at 89.50 ms, against `node` at 219.58, which is 2.5x.

Runtime overhead. `bun` wins this axis at 13.76 ms, against `node` at 57.25, which is 4.2x.

Peak memory. `bun` wins this axis at 31.47 MiB, against `node` at 75.47, which is 2.4x.

## Compute: strings

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.3 | not measured | not measured | not measured | not measured |
| `node` | 26.8.1 | 64.21 | 128.46 | 65.00 | 79.73 |
| `bun` | 1.4.0 | 29.50 | 45.66 | 16.87 | 42.02 |
| `deno` | 2.9.6 | 58.46 | 93.02 | 33.46 | 61.39 |

`katsu` could not be measured: katsu could not run strings: katsu: fatal: out of memory

In process. `bun` wins this axis at 29.50 ms, against `node` at 64.21, which is 2.2x.

Wall clock. `bun` wins this axis at 45.66 ms, against `node` at 128.46, which is 2.8x.

Runtime overhead. `bun` wins this axis at 16.87 ms, against `node` at 65.00, which is 3.9x.

Peak memory. `bun` wins this axis at 42.02 MiB, against `node` at 79.73, which is 1.9x.
