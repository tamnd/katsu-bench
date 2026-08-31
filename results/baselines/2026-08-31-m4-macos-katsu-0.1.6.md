# Runtime benchmarks

Machine: Darwin 15.8, Apple M4, 10 cores, 24576 MiB.
Load average: 3.75 when the run started, 3.93 when it finished.

## Distance to the goal

The goal is ten times better than the best rival on every axis, not ten times better than the worst one. This table is regenerated from the same run as everything below it.

| Axis | katsu | Best rival | Standing | Left to find |
|---|---:|---|---|---|
| Cold start (ms) | 1.59 | `bun` 6.07 | 3.82x ahead | 2.6x |
| Baseline memory at idle (MiB) | not yet | `bun` 12.27 | cannot run this yet | all of it. /Users/apple/.local/bin/katsu exited after 402 ms instead of staying idle, so there was no idle to measure |
| Distribution size (MiB) | 1.74 | `bun` 60.61 | 34.77x ahead | goal reached |
| Compute: alloc, In process (ms) | not yet | `bun` 40.25 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: alloc, Wall clock (ms) | not yet | `bun` 48.44 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: alloc, Runtime overhead (ms) | not yet | `bun` 8.09 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: alloc, Peak memory (MiB) | not yet | `bun` 46.55 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: fib, In process (ms) | 875.16 | `bun` 34.28 | 25.53x behind | 255.3x |
| Compute: fib, Wall clock (ms) | 877.76 | `bun` 41.56 | 21.12x behind | 211.2x |
| Compute: fib, Runtime overhead (ms) | 2.46 | `bun` 7.25 | 2.94x ahead | 3.4x |
| Compute: fib, Peak memory (MiB) | 2.70 | `bun` 18.03 | 6.67x ahead | 1.5x |
| Compute: json, In process (ms) | not yet | `bun` 68.21 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: json, Wall clock (ms) | not yet | `bun` 75.57 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: json, Runtime overhead (ms) | not yet | `bun` 7.34 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: json, Peak memory (MiB) | not yet | `bun` 34.53 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: nbody, In process (ms) | not yet | `node` 91.41 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: nbody, Wall clock (ms) | not yet | `deno` 109.51 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: nbody, Runtime overhead (ms) | not yet | `bun` 7.79 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: nbody, Peak memory (MiB) | not yet | `bun` 21.14 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: sort, In process (ms) | not yet | `bun` 34.62 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: sort, Wall clock (ms) | not yet | `bun` 42.28 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: sort, Runtime overhead (ms) | not yet | `bun` 7.59 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: sort, Peak memory (MiB) | not yet | `bun` 31.30 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: strings, In process (ms) | not yet | `node` 23.39 | cannot run this yet | all of it. out of memory |
| Compute: strings, Wall clock (ms) | not yet | `bun` 38.26 | cannot run this yet | all of it. out of memory |
| Compute: strings, Runtime overhead (ms) | not yet | `bun` 13.84 | cannot run this yet | all of it. out of memory |
| Compute: strings, Peak memory (MiB) | not yet | `bun` 42.05 | cannot run this yet | all of it. out of memory |

**katsu ran 6 of the 27 axes in this report.** The 21 it did not run are marked as such above. Every win here is a win on an axis where katsu is doing less work than the runtime it beat, because most of a runtime is not built yet, so read them as a starting position rather than as a result. This paragraph goes away when the number that did not run reaches zero.

## Cold start (ms)

| Runtime | Version | Median | IQR | p25 | p75 | Min | Max | Runs |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| `katsu` | 0.1.6 | 1.59 | 0.12 | 1.55 | 1.67 | 1.47 | 3.84 | 25 |
| `node` | 26.7.0 | 27.46 | 2.43 | 26.13 | 28.57 | 24.77 | 34.53 | 25 |
| `bun` | 1.4.0 | 6.07 | 0.81 | 5.73 | 6.54 | 5.23 | 36.67 | 25 |
| `deno` | 2.9.6 | 12.67 | 1.01 | 11.95 | 12.96 | 11.40 | 108.37 | 25 |

`katsu` wins this axis at 1.59 ms, against `node` at 27.46, which is 17.3x.

## Baseline memory at idle (MiB)

| Runtime | Version | Median | IQR | p25 | p75 | Min | Max | Runs |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| `katsu` | 0.1.6 | not measured | | | | | | |
| `node` | 26.7.0 | 40.78 | 0.64 | 40.54 | 41.18 | 40.48 | 42.19 | 10 |
| `bun` | 1.4.0 | 12.27 | 0.08 | 12.23 | 12.31 | 12.16 | 12.58 | 10 |
| `deno` | 2.9.6 | 33.50 | 0.49 | 33.29 | 33.78 | 33.23 | 34.73 | 10 |

`katsu` could not be measured: /Users/apple/.local/bin/katsu exited after 402 ms instead of staying idle, so there was no idle to measure

`bun` wins this axis at 12.27 MiB, against `node` at 40.78, which is 3.3x.

## Distribution size (MiB)

| Runtime | Version | Median | IQR | p25 | p75 | Min | Max | Runs |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| `katsu` | 0.1.6 | 1.74 | 0.00 | 1.74 | 1.74 | 1.74 | 1.74 | 1 |
| `node` | 26.7.0 | 119.09 | 0.00 | 119.09 | 119.09 | 119.09 | 119.09 | 1 |
| `bun` | 1.4.0 | 60.61 | 0.00 | 60.61 | 60.61 | 60.61 | 60.61 | 1 |
| `deno` | 2.9.6 | 120.43 | 0.00 | 120.43 | 120.43 | 120.43 | 120.43 | 1 |

`katsu` wins this axis at 1.74 MiB, against `deno` at 120.43, which is 69.1x.

## Compute: alloc

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.6 | not measured | not measured | not measured | not measured |
| `node` | 26.7.0 | 50.77 | 80.58 | 27.54 | 80.11 |
| `bun` | 1.4.0 | 40.25 | 48.44 | 8.09 | 46.55 |
| `deno` | 2.9.6 | 52.70 | 68.04 | 15.56 | 70.39 |

`katsu` could not be measured: katsu could not run alloc: katsu: not implemented yet: compute/alloc.js:19:21: new is not supported yet. See https://github.com/tamnd/katsu/milestones

In process. `bun` wins this axis at 40.25 ms, against `deno` at 52.70, which is 1.3x.

Wall clock. `bun` wins this axis at 48.44 ms, against `node` at 80.58, which is 1.7x.

Runtime overhead. `bun` wins this axis at 8.09 ms, against `node` at 27.54, which is 3.4x.

Peak memory. `bun` wins this axis at 46.55 MiB, against `node` at 80.11, which is 1.7x.

## Compute: fib

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.6 | 875.16 | 877.76 | 2.46 | 2.70 |
| `node` | 26.7.0 | 53.38 | 80.63 | 27.89 | 46.34 |
| `bun` | 1.4.0 | 34.28 | 41.56 | 7.25 | 18.03 |
| `deno` | 2.9.6 | 56.54 | 72.02 | 15.39 | 36.06 |

In process. `bun` wins this axis at 34.28 ms, against `katsu` at 875.16, which is 25.5x.

Wall clock. `bun` wins this axis at 41.56 ms, against `katsu` at 877.76, which is 21.1x.

Runtime overhead. `katsu` wins this axis at 2.46 ms, against `node` at 27.89, which is 11.3x.

Peak memory. `katsu` wins this axis at 2.70 MiB, against `node` at 46.34, which is 17.1x.

## Compute: json

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.6 | not measured | not measured | not measured | not measured |
| `node` | 26.7.0 | 118.10 | 146.12 | 28.86 | 70.23 |
| `bun` | 1.4.0 | 68.21 | 75.57 | 7.34 | 34.53 |
| `deno` | 2.9.6 | 109.75 | 124.62 | 14.59 | 59.80 |

`katsu` could not be measured: katsu could not run json: katsu: not implemented yet: compute/json.js:18:11: an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones

In process. `bun` wins this axis at 68.21 ms, against `node` at 118.10, which is 1.7x.

Wall clock. `bun` wins this axis at 75.57 ms, against `node` at 146.12, which is 1.9x.

Runtime overhead. `bun` wins this axis at 7.34 ms, against `node` at 28.86, which is 3.9x.

Peak memory. `bun` wins this axis at 34.53 MiB, against `node` at 70.23, which is 2.0x.

## Compute: nbody

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.6 | not measured | not measured | not measured | not measured |
| `node` | 26.7.0 | 91.41 | 118.18 | 26.77 | 49.98 |
| `bun` | 1.4.0 | 109.77 | 117.47 | 7.79 | 21.14 |
| `deno` | 2.9.6 | 95.63 | 109.51 | 14.35 | 40.09 |

`katsu` could not be measured: katsu could not run nbody: katsu: not implemented yet: compute/nbody.js:13:10: an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones

In process. Too close to call between `node`, `deno`. Their interquartile ranges overlap, so this axis does not separate them and no winner is claimed.

Wall clock. Too close to call between `deno`, `node`, `bun`. Their interquartile ranges overlap, so this axis does not separate them and no winner is claimed.

Runtime overhead. `bun` wins this axis at 7.79 ms, against `node` at 26.77, which is 3.4x.

Peak memory. `bun` wins this axis at 21.14 MiB, against `node` at 49.98, which is 2.4x.

## Compute: sort

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.6 | not measured | not measured | not measured | not measured |
| `node` | 26.7.0 | 76.51 | 104.82 | 29.01 | 65.59 |
| `bun` | 1.4.0 | 34.62 | 42.28 | 7.59 | 31.30 |
| `deno` | 2.9.6 | 77.74 | 94.71 | 16.36 | 56.22 |

`katsu` could not be measured: katsu could not run sort: katsu: not implemented yet: compute/sort.js:23:19: new is not supported yet. See https://github.com/tamnd/katsu/milestones

In process. `bun` wins this axis at 34.62 ms, against `deno` at 77.74, which is 2.2x.

Wall clock. `bun` wins this axis at 42.28 ms, against `node` at 104.82, which is 2.5x.

Runtime overhead. `bun` wins this axis at 7.59 ms, against `node` at 29.01, which is 3.8x.

Peak memory. `bun` wins this axis at 31.30 MiB, against `node` at 65.59, which is 2.1x.

## Compute: strings

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.6 | not measured | not measured | not measured | not measured |
| `node` | 26.7.0 | 23.39 | 50.22 | 26.92 | 69.30 |
| `bun` | 1.4.0 | 24.96 | 38.26 | 13.84 | 42.05 |
| `deno` | 2.9.6 | 28.95 | 45.45 | 17.23 | 60.75 |

`katsu` could not be measured: katsu could not run strings: katsu: fatal: out of memory

In process. Too close to call between `node`, `bun`. Their interquartile ranges overlap, so this axis does not separate them and no winner is claimed.

Wall clock. Too close to call between `bun`, `deno`. Their interquartile ranges overlap, so this axis does not separate them and no winner is claimed.

Runtime overhead. `bun` wins this axis at 13.84 ms, against `node` at 26.92, which is 1.9x.

Peak memory. `bun` wins this axis at 42.05 MiB, against `node` at 69.30, which is 1.6x.
