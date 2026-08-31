# Runtime benchmarks

Machine: Darwin 15.8, Apple M4, 10 cores, 24576 MiB.
Load average: 3.18 when the run started, 3.12 when it finished.

## Distance to the goal

The goal is ten times better than the best rival on every axis, not ten times better than the worst one. This table is regenerated from the same run as everything below it.

| Axis | katsu | Best rival | Standing | Left to find |
|---|---:|---|---|---|
| Cold start (ms) | 1.68 | `bun` 5.67 | 3.37x ahead | 3.0x |
| Baseline memory at idle (MiB) | not yet | `bun` 12.29 | cannot run this yet | all of it. /Users/apple/.local/bin/katsu exited after 403 ms instead of staying idle, so there was no idle to measure |
| Distribution size (MiB) | 1.77 | `bun` 60.61 | 34.15x ahead | goal reached |
| Compute: alloc, In process (ms) | not yet | `bun` 40.15 | cannot run this yet | all of it. Array is not defined |
| Compute: alloc, Wall clock (ms) | not yet | `bun` 47.73 | cannot run this yet | all of it. Array is not defined |
| Compute: alloc, Runtime overhead (ms) | not yet | `bun` 7.64 | cannot run this yet | all of it. Array is not defined |
| Compute: alloc, Peak memory (MiB) | not yet | `bun` 46.36 | cannot run this yet | all of it. Array is not defined |
| Compute: fib, In process (ms) | 781.89 | `bun` 33.43 | 23.39x behind | 233.9x |
| Compute: fib, Wall clock (ms) | 784.40 | `bun` 40.36 | 19.44x behind | 194.4x |
| Compute: fib, Runtime overhead (ms) | 2.42 | `bun` 6.86 | 2.83x ahead | 3.5x |
| Compute: fib, Peak memory (MiB) | 2.73 | `bun` 18.02 | 6.59x ahead | 1.5x |
| Compute: json, In process (ms) | not yet | `bun` 69.48 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: json, Wall clock (ms) | not yet | `bun` 77.31 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: json, Runtime overhead (ms) | not yet | `bun` 7.63 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: json, Peak memory (MiB) | not yet | `bun` 34.61 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: nbody, In process (ms) | not yet | `node` 97.12 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: nbody, Wall clock (ms) | not yet | `deno` 113.80 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: nbody, Runtime overhead (ms) | not yet | `bun` 7.81 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: nbody, Peak memory (MiB) | not yet | `bun` 21.16 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: sort, In process (ms) | not yet | `bun` 38.22 | cannot run this yet | all of it. Array is not defined |
| Compute: sort, Wall clock (ms) | not yet | `bun` 46.12 | cannot run this yet | all of it. Array is not defined |
| Compute: sort, Runtime overhead (ms) | not yet | `bun` 7.78 | cannot run this yet | all of it. Array is not defined |
| Compute: sort, Peak memory (MiB) | not yet | `bun` 31.31 | cannot run this yet | all of it. Array is not defined |
| Compute: strings, In process (ms) | not yet | `bun` 13.02 | cannot run this yet | all of it. out of memory |
| Compute: strings, Wall clock (ms) | not yet | `bun` 21.06 | cannot run this yet | all of it. out of memory |
| Compute: strings, Runtime overhead (ms) | not yet | `bun` 7.93 | cannot run this yet | all of it. out of memory |
| Compute: strings, Peak memory (MiB) | not yet | `bun` 41.81 | cannot run this yet | all of it. out of memory |

**katsu ran 6 of the 27 axes in this report.** The 21 it did not run are marked as such above. Every win here is a win on an axis where katsu is doing less work than the runtime it beat, because most of a runtime is not built yet, so read them as a starting position rather than as a result. This paragraph goes away when the number that did not run reaches zero.

## Cold start (ms)

| Runtime | Version | Median | IQR | p25 | p75 | Min | Max | Runs |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| `katsu` | 0.1.8 | 1.68 | 0.11 | 1.66 | 1.77 | 1.56 | 2.60 | 25 |
| `node` | 26.7.0 | 25.89 | 2.03 | 24.93 | 26.96 | 24.15 | 28.87 | 25 |
| `bun` | 1.4.0 | 5.67 | 1.01 | 5.18 | 6.19 | 5.01 | 28.12 | 25 |
| `deno` | 2.9.6 | 11.56 | 0.38 | 11.35 | 11.73 | 11.02 | 89.90 | 25 |

`katsu` wins this axis at 1.68 ms, against `node` at 25.89, which is 15.4x.

## Baseline memory at idle (MiB)

| Runtime | Version | Median | IQR | p25 | p75 | Min | Max | Runs |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| `katsu` | 0.1.8 | not measured | | | | | | |
| `node` | 26.7.0 | 40.60 | 0.13 | 40.57 | 40.70 | 40.48 | 40.81 | 10 |
| `bun` | 1.4.0 | 12.29 | 0.03 | 12.28 | 12.31 | 12.20 | 12.56 | 10 |
| `deno` | 2.9.6 | 33.49 | 0.37 | 33.30 | 33.67 | 33.25 | 34.45 | 10 |

`katsu` could not be measured: /Users/apple/.local/bin/katsu exited after 403 ms instead of staying idle, so there was no idle to measure

`bun` wins this axis at 12.29 MiB, against `node` at 40.60, which is 3.3x.

## Distribution size (MiB)

| Runtime | Version | Median | IQR | p25 | p75 | Min | Max | Runs |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| `katsu` | 0.1.8 | 1.77 | 0.00 | 1.77 | 1.77 | 1.77 | 1.77 | 1 |
| `node` | 26.7.0 | 119.09 | 0.00 | 119.09 | 119.09 | 119.09 | 119.09 | 1 |
| `bun` | 1.4.0 | 60.61 | 0.00 | 60.61 | 60.61 | 60.61 | 60.61 | 1 |
| `deno` | 2.9.6 | 120.43 | 0.00 | 120.43 | 120.43 | 120.43 | 120.43 | 1 |

`katsu` wins this axis at 1.77 MiB, against `deno` at 120.43, which is 67.9x.

## Compute: alloc

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.8 | not measured | not measured | not measured | not measured |
| `node` | 26.7.0 | 49.34 | 76.39 | 26.68 | 79.98 |
| `bun` | 1.4.0 | 40.15 | 47.73 | 7.64 | 46.36 |
| `deno` | 2.9.6 | 48.99 | 63.28 | 14.30 | 69.92 |

`katsu` could not be measured: katsu could not run alloc: ReferenceError: Array is not defined

In process. `bun` wins this axis at 40.15 ms, against `node` at 49.34, which is 1.2x.

Wall clock. `bun` wins this axis at 47.73 ms, against `node` at 76.39, which is 1.6x.

Runtime overhead. `bun` wins this axis at 7.64 ms, against `node` at 26.68, which is 3.5x.

Peak memory. `bun` wins this axis at 46.36 MiB, against `node` at 79.98, which is 1.7x.

## Compute: fib

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.8 | 781.89 | 784.40 | 2.42 | 2.73 |
| `node` | 26.7.0 | 48.80 | 74.72 | 25.78 | 46.44 |
| `bun` | 1.4.0 | 33.43 | 40.36 | 6.86 | 18.02 |
| `deno` | 2.9.6 | 48.98 | 62.54 | 13.31 | 35.59 |

In process. `bun` wins this axis at 33.43 ms, against `katsu` at 781.89, which is 23.4x.

Wall clock. `bun` wins this axis at 40.36 ms, against `katsu` at 784.40, which is 19.4x.

Runtime overhead. `katsu` wins this axis at 2.42 ms, against `node` at 25.78, which is 10.6x.

Peak memory. `katsu` wins this axis at 2.73 MiB, against `node` at 46.44, which is 17.0x.

## Compute: json

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.8 | not measured | not measured | not measured | not measured |
| `node` | 26.7.0 | 116.71 | 146.10 | 29.32 | 70.20 |
| `bun` | 1.4.0 | 69.48 | 77.31 | 7.63 | 34.61 |
| `deno` | 2.9.6 | 113.77 | 128.24 | 14.83 | 60.14 |

`katsu` could not be measured: katsu could not run json: katsu: not implemented yet: compute/json.js:18:11: an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones

In process. `bun` wins this axis at 69.48 ms, against `node` at 116.71, which is 1.7x.

Wall clock. `bun` wins this axis at 77.31 ms, against `node` at 146.10, which is 1.9x.

Runtime overhead. `bun` wins this axis at 7.63 ms, against `node` at 29.32, which is 3.8x.

Peak memory. `bun` wins this axis at 34.61 MiB, against `node` at 70.20, which is 2.0x.

## Compute: nbody

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.8 | not measured | not measured | not measured | not measured |
| `node` | 26.7.0 | 97.12 | 125.34 | 28.85 | 49.86 |
| `bun` | 1.4.0 | 119.17 | 127.04 | 7.81 | 21.16 |
| `deno` | 2.9.6 | 99.78 | 113.80 | 14.60 | 40.45 |

`katsu` could not be measured: katsu could not run nbody: katsu: not implemented yet: compute/nbody.js:13:10: an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones

In process. Too close to call between `node`, `deno`. Their interquartile ranges overlap, so this axis does not separate them and no winner is claimed.

Wall clock. `deno` wins this axis at 113.80 ms, against `bun` at 127.04, which is 1.1x.

Runtime overhead. `bun` wins this axis at 7.81 ms, against `node` at 28.85, which is 3.7x.

Peak memory. `bun` wins this axis at 21.16 MiB, against `node` at 49.86, which is 2.4x.

## Compute: sort

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.8 | not measured | not measured | not measured | not measured |
| `node` | 26.7.0 | 83.33 | 115.82 | 32.09 | 65.97 |
| `bun` | 1.4.0 | 38.22 | 46.12 | 7.78 | 31.31 |
| `deno` | 2.9.6 | 80.04 | 95.69 | 16.19 | 56.62 |

`katsu` could not be measured: katsu could not run sort: ReferenceError: Array is not defined

In process. `bun` wins this axis at 38.22 ms, against `node` at 83.33, which is 2.2x.

Wall clock. `bun` wins this axis at 46.12 ms, against `node` at 115.82, which is 2.5x.

Runtime overhead. `bun` wins this axis at 7.78 ms, against `node` at 32.09, which is 4.1x.

Peak memory. `bun` wins this axis at 31.31 MiB, against `node` at 65.97, which is 2.1x.

## Compute: strings

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.8 | not measured | not measured | not measured | not measured |
| `node` | 26.7.0 | 26.58 | 57.87 | 30.99 | 69.45 |
| `bun` | 1.4.0 | 13.02 | 21.06 | 7.93 | 41.81 |
| `deno` | 2.9.6 | 24.48 | 39.21 | 14.83 | 59.81 |

`katsu` could not be measured: katsu could not run strings: katsu: fatal: out of memory

In process. `bun` wins this axis at 13.02 ms, against `node` at 26.58, which is 2.0x.

Wall clock. `bun` wins this axis at 21.06 ms, against `node` at 57.87, which is 2.7x.

Runtime overhead. `bun` wins this axis at 7.93 ms, against `node` at 30.99, which is 3.9x.

Peak memory. `bun` wins this axis at 41.81 MiB, against `node` at 69.45, which is 1.7x.
