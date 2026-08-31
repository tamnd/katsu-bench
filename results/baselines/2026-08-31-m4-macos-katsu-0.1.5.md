# Runtime benchmarks

Machine: Darwin 15.8, Apple M4, 10 cores, 24576 MiB.
Load average: 3.70 when the run started, 3.69 when it finished.

## Distance to the goal

The goal is ten times better than the best rival on every axis, not ten times better than the worst one. This table is regenerated from the same run as everything below it.

| Axis | katsu | Best rival | Standing | Left to find |
|---|---:|---|---|---|
| Cold start (ms) | 1.99 | `bun` 6.59 | 3.31x ahead | 3.0x |
| Baseline memory at idle (MiB) | not yet | `bun` 12.25 | cannot run this yet | all of it. /tmp/katsu-bin/katsu exited after 401 ms instead of staying idle, so there was no idle to measure |
| Distribution size (MiB) | 1.74 | `bun` 60.61 | 34.77x ahead | goal reached |
| Compute: alloc, In process (ms) | not yet | `bun` 38.63 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: alloc, Wall clock (ms) | not yet | `bun` 46.07 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: alloc, Runtime overhead (ms) | not yet | `bun` 7.54 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: alloc, Peak memory (MiB) | not yet | `bun` 46.34 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: fib, In process (ms) | 789.33 | `bun` 33.47 | 23.58x behind | 235.8x |
| Compute: fib, Wall clock (ms) | 791.76 | `bun` 40.48 | 19.56x behind | 195.6x |
| Compute: fib, Runtime overhead (ms) | 2.37 | `bun` 6.90 | 2.91x ahead | 3.4x |
| Compute: fib, Peak memory (MiB) | 2.72 | `bun` 18.05 | 6.64x ahead | 1.5x |
| Compute: json, In process (ms) | not yet | `bun` 68.83 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: json, Wall clock (ms) | not yet | `bun` 75.98 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: json, Runtime overhead (ms) | not yet | `bun` 7.36 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: json, Peak memory (MiB) | not yet | `bun` 34.55 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: nbody, In process (ms) | not yet | `node` 86.81 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: nbody, Wall clock (ms) | not yet | `deno` 100.58 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: nbody, Runtime overhead (ms) | not yet | `bun` 7.08 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: nbody, Peak memory (MiB) | not yet | `bun` 21.14 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: sort, In process (ms) | not yet | `bun` 34.43 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: sort, Wall clock (ms) | not yet | `bun` 41.90 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: sort, Runtime overhead (ms) | not yet | `bun` 7.49 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: sort, Peak memory (MiB) | not yet | `bun` 31.39 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: strings, In process (ms) | not yet | `bun` 12.16 | cannot run this yet | all of it. out of memory |
| Compute: strings, Wall clock (ms) | not yet | `bun` 19.61 | cannot run this yet | all of it. out of memory |
| Compute: strings, Runtime overhead (ms) | not yet | `bun` 7.39 | cannot run this yet | all of it. out of memory |
| Compute: strings, Peak memory (MiB) | not yet | `bun` 41.78 | cannot run this yet | all of it. out of memory |

**katsu ran 6 of the 27 axes in this report.** The 21 it did not run are marked as such above. Every win here is a win on an axis where katsu is doing less work than the runtime it beat, because most of a runtime is not built yet, so read them as a starting position rather than as a result. This paragraph goes away when the number that did not run reaches zero.

## Cold start (ms)

| Runtime | Version | Median | IQR | p25 | p75 | Min | Max | Runs |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| `katsu` | 0.1.5 | 1.99 | 0.26 | 1.93 | 2.18 | 1.77 | 3.36 | 25 |
| `node` | 26.7.0 | 27.95 | 1.74 | 27.15 | 28.89 | 25.63 | 34.04 | 25 |
| `bun` | 1.4.0 | 6.59 | 1.07 | 6.05 | 7.13 | 5.82 | 26.92 | 25 |
| `deno` | 2.9.6 | 13.31 | 1.08 | 12.67 | 13.76 | 12.04 | 71.81 | 25 |

`katsu` wins this axis at 1.99 ms, against `node` at 27.95, which is 14.0x.

## Baseline memory at idle (MiB)

| Runtime | Version | Median | IQR | p25 | p75 | Min | Max | Runs |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| `katsu` | 0.1.5 | not measured | | | | | | |
| `node` | 26.7.0 | 41.72 | 1.26 | 41.25 | 42.52 | 40.80 | 43.12 | 10 |
| `bun` | 1.4.0 | 12.25 | 0.17 | 12.20 | 12.37 | 12.14 | 12.48 | 10 |
| `deno` | 2.9.6 | 33.47 | 0.21 | 33.33 | 33.55 | 33.27 | 33.91 | 10 |

`katsu` could not be measured: /tmp/katsu-bin/katsu exited after 401 ms instead of staying idle, so there was no idle to measure

`bun` wins this axis at 12.25 MiB, against `node` at 41.72, which is 3.4x.

## Distribution size (MiB)

| Runtime | Version | Median | IQR | p25 | p75 | Min | Max | Runs |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| `katsu` | 0.1.5 | 1.74 | 0.00 | 1.74 | 1.74 | 1.74 | 1.74 | 1 |
| `node` | 26.7.0 | 119.09 | 0.00 | 119.09 | 119.09 | 119.09 | 119.09 | 1 |
| `bun` | 1.4.0 | 60.61 | 0.00 | 60.61 | 60.61 | 60.61 | 60.61 | 1 |
| `deno` | 2.9.6 | 120.43 | 0.00 | 120.43 | 120.43 | 120.43 | 120.43 | 1 |

`katsu` wins this axis at 1.74 MiB, against `deno` at 120.43, which is 69.1x.

## Compute: alloc

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.5 | not measured | not measured | not measured | not measured |
| `node` | 26.7.0 | 49.59 | 77.36 | 27.97 | 79.91 |
| `bun` | 1.4.0 | 38.63 | 46.07 | 7.54 | 46.34 |
| `deno` | 2.9.6 | 48.89 | 63.04 | 14.18 | 69.80 |

`katsu` could not be measured: katsu could not run alloc: katsu: not implemented yet: compute/alloc.js:19:21: new is not supported yet. See https://github.com/tamnd/katsu/milestones

In process. `bun` wins this axis at 38.63 ms, against `node` at 49.59, which is 1.3x.

Wall clock. `bun` wins this axis at 46.07 ms, against `node` at 77.36, which is 1.7x.

Runtime overhead. `bun` wins this axis at 7.54 ms, against `node` at 27.97, which is 3.7x.

Peak memory. `bun` wins this axis at 46.34 MiB, against `node` at 79.91, which is 1.7x.

## Compute: fib

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.5 | 789.33 | 791.76 | 2.37 | 2.72 |
| `node` | 26.7.0 | 49.27 | 76.76 | 27.17 | 46.36 |
| `bun` | 1.4.0 | 33.47 | 40.48 | 6.90 | 18.05 |
| `deno` | 2.9.6 | 49.99 | 64.59 | 14.16 | 35.62 |

In process. `bun` wins this axis at 33.47 ms, against `katsu` at 789.33, which is 23.6x.

Wall clock. `bun` wins this axis at 40.48 ms, against `katsu` at 791.76, which is 19.6x.

Runtime overhead. `katsu` wins this axis at 2.37 ms, against `node` at 27.17, which is 11.4x.

Peak memory. `katsu` wins this axis at 2.72 MiB, against `node` at 46.36, which is 17.1x.

## Compute: json

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.5 | not measured | not measured | not measured | not measured |
| `node` | 26.7.0 | 116.81 | 145.44 | 29.13 | 70.19 |
| `bun` | 1.4.0 | 68.83 | 75.98 | 7.36 | 34.55 |
| `deno` | 2.9.6 | 110.46 | 124.61 | 14.26 | 59.92 |

`katsu` could not be measured: katsu could not run json: katsu: not implemented yet: compute/json.js:18:11: an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones

In process. `bun` wins this axis at 68.83 ms, against `node` at 116.81, which is 1.7x.

Wall clock. `bun` wins this axis at 75.98 ms, against `node` at 145.44, which is 1.9x.

Runtime overhead. `bun` wins this axis at 7.36 ms, against `node` at 29.13, which is 4.0x.

Peak memory. `bun` wins this axis at 34.55 MiB, against `node` at 70.19, which is 2.0x.

## Compute: nbody

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.5 | not measured | not measured | not measured | not measured |
| `node` | 26.7.0 | 86.81 | 115.88 | 27.85 | 49.98 |
| `bun` | 1.4.0 | 105.26 | 112.49 | 7.08 | 21.14 |
| `deno` | 2.9.6 | 86.86 | 100.58 | 13.35 | 39.92 |

`katsu` could not be measured: katsu could not run nbody: katsu: not implemented yet: compute/nbody.js:13:10: an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones

In process. Too close to call between `node`, `deno`. Their interquartile ranges overlap, so this axis does not separate them and no winner is claimed.

Wall clock. `deno` wins this axis at 100.58 ms, against `node` at 115.88, which is 1.2x.

Runtime overhead. `bun` wins this axis at 7.08 ms, against `node` at 27.85, which is 3.9x.

Peak memory. `bun` wins this axis at 21.14 MiB, against `node` at 49.98, which is 2.4x.

## Compute: sort

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.5 | not measured | not measured | not measured | not measured |
| `node` | 26.7.0 | 71.44 | 100.83 | 29.02 | 65.64 |
| `bun` | 1.4.0 | 34.43 | 41.90 | 7.49 | 31.39 |
| `deno` | 2.9.6 | 70.04 | 84.81 | 14.97 | 56.03 |

`katsu` could not be measured: katsu could not run sort: katsu: not implemented yet: compute/sort.js:23:19: new is not supported yet. See https://github.com/tamnd/katsu/milestones

In process. `bun` wins this axis at 34.43 ms, against `node` at 71.44, which is 2.1x.

Wall clock. `bun` wins this axis at 41.90 ms, against `node` at 100.83, which is 2.4x.

Runtime overhead. `bun` wins this axis at 7.49 ms, against `node` at 29.02, which is 3.9x.

Peak memory. `bun` wins this axis at 31.39 MiB, against `node` at 65.64, which is 2.1x.

## Compute: strings

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.5 | not measured | not measured | not measured | not measured |
| `node` | 26.7.0 | 23.25 | 51.48 | 28.58 | 69.16 |
| `bun` | 1.4.0 | 12.16 | 19.61 | 7.39 | 41.78 |
| `deno` | 2.9.6 | 21.13 | 34.86 | 13.52 | 59.45 |

`katsu` could not be measured: katsu could not run strings: katsu: fatal: out of memory

In process. `bun` wins this axis at 12.16 ms, against `node` at 23.25, which is 1.9x.

Wall clock. `bun` wins this axis at 19.61 ms, against `node` at 51.48, which is 2.6x.

Runtime overhead. `bun` wins this axis at 7.39 ms, against `node` at 28.58, which is 3.9x.

Peak memory. `bun` wins this axis at 41.78 MiB, against `node` at 69.16, which is 1.7x.
