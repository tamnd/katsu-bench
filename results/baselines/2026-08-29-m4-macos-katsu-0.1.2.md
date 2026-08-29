# Runtime benchmarks

Machine: Darwin 15.8, Apple M4, 10 cores, 24576 MiB.

## Distance to the goal

The goal is ten times better than the best rival on every axis, not ten times better than the worst one. This table is regenerated from the same run as everything below it.

| Axis | katsu | Best rival | Standing | Left to find |
|---|---:|---|---|---|
| Cold start (ms) | 1.57 | `bun` 5.31 | 3.38x ahead | 3.0x |
| Baseline memory at idle (MiB) | not yet | `bun` 12.30 | cannot run this yet | all of it. /Users/apple/.local/bin/katsu exited after 404 ms instead of staying idle, so there was no idle to measure |
| Distribution size (MiB) | 1.70 | `bun` 60.61 | 35.74x ahead | goal reached |
| Compute: alloc, In process (ms) | not yet | `bun` 39.64 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: alloc, Wall clock (ms) | not yet | `bun` 47.28 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: alloc, Runtime overhead (ms) | not yet | `bun` 7.90 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: alloc, Peak memory (MiB) | not yet | `bun` 46.39 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: fib, In process (ms) | not yet | `bun` 38.12 | cannot run this yet | all of it. performance is not defined |
| Compute: fib, Wall clock (ms) | not yet | `bun` 45.88 | cannot run this yet | all of it. performance is not defined |
| Compute: fib, Runtime overhead (ms) | not yet | `bun` 7.64 | cannot run this yet | all of it. performance is not defined |
| Compute: fib, Peak memory (MiB) | not yet | `bun` 18.05 | cannot run this yet | all of it. performance is not defined |
| Compute: json, In process (ms) | not yet | `bun` 72.55 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: json, Wall clock (ms) | not yet | `bun` 79.84 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: json, Runtime overhead (ms) | not yet | `bun` 7.71 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: json, Peak memory (MiB) | not yet | `bun` 34.61 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: nbody, In process (ms) | not yet | `node` 95.24 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: nbody, Wall clock (ms) | not yet | `deno` 109.78 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: nbody, Runtime overhead (ms) | not yet | `bun` 7.52 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: nbody, Peak memory (MiB) | not yet | `bun` 21.12 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: sort, In process (ms) | not yet | `bun` 35.63 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: sort, Wall clock (ms) | not yet | `bun` 43.16 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: sort, Runtime overhead (ms) | not yet | `bun` 7.53 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: sort, Peak memory (MiB) | not yet | `bun` 31.34 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: strings, In process (ms) | not yet | `bun` 12.75 | cannot run this yet | all of it. performance is not defined |
| Compute: strings, Wall clock (ms) | not yet | `bun` 20.80 | cannot run this yet | all of it. performance is not defined |
| Compute: strings, Runtime overhead (ms) | not yet | `bun` 8.17 | cannot run this yet | all of it. performance is not defined |
| Compute: strings, Peak memory (MiB) | not yet | `bun` 41.78 | cannot run this yet | all of it. performance is not defined |

**katsu ran 2 of the 27 axes in this report.** The 25 it did not run are marked as such above. Every win here is a win on an axis where katsu is doing less work than the runtime it beat, because most of a runtime is not built yet, so read them as a starting position rather than as a result. This paragraph goes away when the number that did not run reaches zero.

## Cold start (ms)

| Runtime | Version | Median | IQR | p25 | p75 | Min | Max | Runs |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| `katsu` | 0.1.2 | 1.57 | 0.15 | 1.51 | 1.66 | 1.43 | 3.93 | 25 |
| `node` | 26.8.1 | 24.11 | 1.82 | 23.37 | 25.19 | 22.56 | 66.27 | 25 |
| `bun` | 1.4.0 | 5.31 | 0.50 | 5.16 | 5.67 | 4.94 | 28.78 | 25 |
| `deno` | 2.9.6 | 12.65 | 0.77 | 12.53 | 13.30 | 12.33 | 75.28 | 25 |

`katsu` wins this axis at 1.57 ms, against `node` at 24.11, which is 15.3x.

## Baseline memory at idle (MiB)

| Runtime | Version | Median | IQR | p25 | p75 | Min | Max | Runs |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| `katsu` | 0.1.2 | not measured | | | | | | |
| `node` | 26.8.1 | 47.55 | 0.04 | 47.54 | 47.57 | 47.41 | 48.38 | 10 |
| `bun` | 1.4.0 | 12.30 | 0.17 | 12.21 | 12.38 | 12.14 | 12.53 | 10 |
| `deno` | 2.9.6 | 33.73 | 0.51 | 33.55 | 34.06 | 33.23 | 34.80 | 10 |

`katsu` could not be measured: /Users/apple/.local/bin/katsu exited after 404 ms instead of staying idle, so there was no idle to measure

`bun` wins this axis at 12.30 MiB, against `node` at 47.55, which is 3.9x.

## Distribution size (MiB)

| Runtime | Version | Median | IQR | p25 | p75 | Min | Max | Runs |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| `katsu` | 0.1.2 | 1.70 | 0.00 | 1.70 | 1.70 | 1.70 | 1.70 | 1 |
| `node` | 26.8.1 | 139.05 | 0.00 | 139.05 | 139.05 | 139.05 | 139.05 | 1 |
| `bun` | 1.4.0 | 60.61 | 0.00 | 60.61 | 60.61 | 60.61 | 60.61 | 1 |
| `deno` | 2.9.6 | 120.43 | 0.00 | 120.43 | 120.43 | 120.43 | 120.43 | 1 |

`katsu` wins this axis at 1.70 MiB, against `node` at 139.05, which is 82.0x.

## Compute: alloc

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.2 | not measured | not measured | not measured | not measured |
| `node` | 26.8.1 | 50.09 | 77.05 | 26.84 | 88.31 |
| `bun` | 1.4.0 | 39.64 | 47.28 | 7.90 | 46.39 |
| `deno` | 2.9.6 | 49.63 | 63.95 | 14.47 | 70.36 |

`katsu` could not be measured: katsu could not run alloc: katsu: not implemented yet: compute/alloc.js:19:21: new is not supported yet. See https://github.com/tamnd/katsu/milestones

In process. `bun` wins this axis at 39.64 ms, against `node` at 50.09, which is 1.3x.

Wall clock. `bun` wins this axis at 47.28 ms, against `node` at 77.05, which is 1.6x.

Runtime overhead. `bun` wins this axis at 7.90 ms, against `node` at 26.84, which is 3.4x.

Peak memory. `bun` wins this axis at 46.39 MiB, against `node` at 88.31, which is 1.9x.

## Compute: fib

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.2 | not measured | not measured | not measured | not measured |
| `node` | 26.8.1 | 53.27 | 79.39 | 26.77 | 54.67 |
| `bun` | 1.4.0 | 38.12 | 45.88 | 7.64 | 18.05 |
| `deno` | 2.9.6 | 51.38 | 65.22 | 13.36 | 35.77 |

`katsu` could not be measured: katsu could not run fib: ReferenceError: performance is not defined

In process. `bun` wins this axis at 38.12 ms, against `node` at 53.27, which is 1.4x.

Wall clock. `bun` wins this axis at 45.88 ms, against `node` at 79.39, which is 1.7x.

Runtime overhead. `bun` wins this axis at 7.64 ms, against `node` at 26.77, which is 3.5x.

Peak memory. `bun` wins this axis at 18.05 MiB, against `node` at 54.67, which is 3.0x.

## Compute: json

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.2 | not measured | not measured | not measured | not measured |
| `node` | 26.8.1 | 114.98 | 142.86 | 26.95 | 78.02 |
| `bun` | 1.4.0 | 72.55 | 79.84 | 7.71 | 34.61 |
| `deno` | 2.9.6 | 115.34 | 130.12 | 14.53 | 60.02 |

`katsu` could not be measured: katsu could not run json: katsu: not implemented yet: compute/json.js:18:11: an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones

In process. `bun` wins this axis at 72.55 ms, against `deno` at 115.34, which is 1.6x.

Wall clock. `bun` wins this axis at 79.84 ms, against `node` at 142.86, which is 1.8x.

Runtime overhead. `bun` wins this axis at 7.71 ms, against `node` at 26.95, which is 3.5x.

Peak memory. `bun` wins this axis at 34.61 MiB, against `node` at 78.02, which is 2.3x.

## Compute: nbody

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.2 | not measured | not measured | not measured | not measured |
| `node` | 26.8.1 | 95.24 | 121.34 | 26.61 | 58.55 |
| `bun` | 1.4.0 | 113.56 | 121.12 | 7.52 | 21.12 |
| `deno` | 2.9.6 | 95.88 | 109.78 | 13.90 | 40.33 |

`katsu` could not be measured: katsu could not run nbody: katsu: not implemented yet: compute/nbody.js:13:10: an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones

In process. Too close to call between `node`, `deno`. Their interquartile ranges overlap, so this axis does not separate them and no winner is claimed.

Wall clock. `deno` wins this axis at 109.78 ms, against `node` at 121.34, which is 1.1x.

Runtime overhead. `bun` wins this axis at 7.52 ms, against `node` at 26.61, which is 3.5x.

Peak memory. `bun` wins this axis at 21.12 MiB, against `node` at 58.55, which is 2.8x.

## Compute: sort

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.2 | not measured | not measured | not measured | not measured |
| `node` | 26.8.1 | 77.97 | 107.24 | 28.40 | 74.38 |
| `bun` | 1.4.0 | 35.63 | 43.16 | 7.53 | 31.34 |
| `deno` | 2.9.6 | 75.69 | 91.69 | 15.52 | 56.62 |

`katsu` could not be measured: katsu could not run sort: katsu: not implemented yet: compute/sort.js:23:19: new is not supported yet. See https://github.com/tamnd/katsu/milestones

In process. `bun` wins this axis at 35.63 ms, against `node` at 77.97, which is 2.2x.

Wall clock. `bun` wins this axis at 43.16 ms, against `node` at 107.24, which is 2.5x.

Runtime overhead. `bun` wins this axis at 7.53 ms, against `node` at 28.40, which is 3.8x.

Peak memory. `bun` wins this axis at 31.34 MiB, against `node` at 74.38, which is 2.4x.

## Compute: strings

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.2 | not measured | not measured | not measured | not measured |
| `node` | 26.8.1 | 24.49 | 51.90 | 27.37 | 78.12 |
| `bun` | 1.4.0 | 12.75 | 20.80 | 8.17 | 41.78 |
| `deno` | 2.9.6 | 22.84 | 36.75 | 14.33 | 60.03 |

`katsu` could not be measured: katsu could not run strings: ReferenceError: performance is not defined

In process. `bun` wins this axis at 12.75 ms, against `node` at 24.49, which is 1.9x.

Wall clock. `bun` wins this axis at 20.80 ms, against `node` at 51.90, which is 2.5x.

Runtime overhead. `bun` wins this axis at 8.17 ms, against `node` at 27.37, which is 3.4x.

Peak memory. `bun` wins this axis at 41.78 MiB, against `node` at 78.12, which is 1.9x.
