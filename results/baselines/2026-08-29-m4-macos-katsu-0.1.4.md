# Runtime benchmarks

Machine: Darwin 15.8, Apple M4, 10 cores, 24576 MiB.
Load average: 4.95 when the run started, 4.16 when it finished.

## Distance to the goal

The goal is ten times better than the best rival on every axis, not ten times better than the worst one. This table is regenerated from the same run as everything below it.

| Axis | katsu | Best rival | Standing | Left to find |
|---|---:|---|---|---|
| Cold start (ms) | 2.76 | `bun` 8.80 | 3.19x ahead | 3.1x |
| Baseline memory at idle (MiB) | not yet | `bun` 12.28 | cannot run this yet | all of it. /Users/apple/.katsu-releases/0.1.4/katsu-v0.1.4-aarch64-apple-darwin/katsu exited after 408 ms instead of staying idle, so there was no idle to measure |
| Distribution size (MiB) | 1.73 | `bun` 60.61 | 35.09x ahead | goal reached |
| Compute: alloc, In process (ms) | not yet | `bun` 79.48 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: alloc, Wall clock (ms) | not yet | `deno` 119.94 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: alloc, Runtime overhead (ms) | not yet | `deno` 26.60 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: alloc, Peak memory (MiB) | not yet | `bun` 46.77 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: fib, In process (ms) | 1305.82 | `bun` 56.89 | 22.95x behind | 229.5x |
| Compute: fib, Wall clock (ms) | 1309.55 | `bun` 66.90 | 19.57x behind | 195.7x |
| Compute: fib, Runtime overhead (ms) | 3.66 | `bun` 9.99 | 2.73x ahead | 3.7x |
| Compute: fib, Peak memory (MiB) | 2.67 | `bun` 18.03 | 6.75x ahead | 1.5x |
| Compute: json, In process (ms) | not yet | `bun` 105.02 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: json, Wall clock (ms) | not yet | `bun` 114.56 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: json, Runtime overhead (ms) | not yet | `bun` 10.25 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: json, Peak memory (MiB) | not yet | `bun` 34.70 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: nbody, In process (ms) | not yet | `node` 147.19 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: nbody, Wall clock (ms) | not yet | `node` 187.40 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: nbody, Runtime overhead (ms) | not yet | `bun` 13.22 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: nbody, Peak memory (MiB) | not yet | `bun` 21.36 | cannot run this yet | all of it. an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: sort, In process (ms) | not yet | `bun` 52.00 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: sort, Wall clock (ms) | not yet | `bun` 61.87 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: sort, Runtime overhead (ms) | not yet | `bun` 9.73 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: sort, Peak memory (MiB) | not yet | `bun` 31.31 | cannot run this yet | all of it. new is not supported yet. See https://github.com/tamnd/katsu/milestones |
| Compute: strings, In process (ms) | not yet | `bun` 21.74 | cannot run this yet | all of it. out of memory |
| Compute: strings, Wall clock (ms) | not yet | `bun` 33.68 | cannot run this yet | all of it. out of memory |
| Compute: strings, Runtime overhead (ms) | not yet | `bun` 12.08 | cannot run this yet | all of it. out of memory |
| Compute: strings, Peak memory (MiB) | not yet | `bun` 41.95 | cannot run this yet | all of it. out of memory |

**katsu ran 6 of the 27 axes in this report.** The 21 it did not run are marked as such above. Every win here is a win on an axis where katsu is doing less work than the runtime it beat, because most of a runtime is not built yet, so read them as a starting position rather than as a result. This paragraph goes away when the number that did not run reaches zero.

## Cold start (ms)

| Runtime | Version | Median | IQR | p25 | p75 | Min | Max | Runs |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| `katsu` | 0.1.4 | 2.76 | 0.22 | 2.66 | 2.88 | 2.45 | 5.28 | 25 |
| `node` | 26.7.0 | 35.99 | 2.70 | 34.60 | 37.30 | 32.69 | 47.42 | 25 |
| `bun` | 1.4.0 | 8.80 | 1.49 | 8.35 | 9.84 | 7.49 | 30.89 | 25 |
| `deno` | 2.9.6 | 15.19 | 1.51 | 14.85 | 16.36 | 14.53 | 81.09 | 25 |

`katsu` wins this axis at 2.76 ms, against `node` at 35.99, which is 13.0x.

## Baseline memory at idle (MiB)

| Runtime | Version | Median | IQR | p25 | p75 | Min | Max | Runs |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| `katsu` | 0.1.4 | not measured | | | | | | |
| `node` | 26.7.0 | 41.23 | 0.91 | 40.82 | 41.73 | 40.59 | 42.33 | 10 |
| `bun` | 1.4.0 | 12.28 | 0.21 | 12.25 | 12.47 | 12.20 | 12.59 | 10 |
| `deno` | 2.9.6 | 33.96 | 0.51 | 33.79 | 34.29 | 33.25 | 34.75 | 10 |

`katsu` could not be measured: /Users/apple/.katsu-releases/0.1.4/katsu-v0.1.4-aarch64-apple-darwin/katsu exited after 408 ms instead of staying idle, so there was no idle to measure

`bun` wins this axis at 12.28 MiB, against `node` at 41.23, which is 3.4x.

## Distribution size (MiB)

| Runtime | Version | Median | IQR | p25 | p75 | Min | Max | Runs |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| `katsu` | 0.1.4 | 1.73 | 0.00 | 1.73 | 1.73 | 1.73 | 1.73 | 1 |
| `node` | 26.7.0 | 119.09 | 0.00 | 119.09 | 119.09 | 119.09 | 119.09 | 1 |
| `bun` | 1.4.0 | 60.61 | 0.00 | 60.61 | 60.61 | 60.61 | 60.61 | 1 |
| `deno` | 2.9.6 | 120.43 | 0.00 | 120.43 | 120.43 | 120.43 | 120.43 | 1 |

`katsu` wins this axis at 1.73 MiB, against `deno` at 120.43, which is 69.7x.

## Compute: alloc

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.4 | not measured | not measured | not measured | not measured |
| `node` | 26.7.0 | 85.27 | 130.21 | 43.61 | 80.27 |
| `bun` | 1.4.0 | 79.48 | 125.46 | 42.45 | 46.77 |
| `deno` | 2.9.6 | 91.78 | 119.94 | 26.60 | 71.17 |

`katsu` could not be measured: katsu could not run alloc: katsu: not implemented yet: compute/alloc.js:19:21: new is not supported yet. See https://github.com/tamnd/katsu/milestones

In process. `bun` wins this axis at 79.48 ms, against `deno` at 91.78, which is 1.2x.

Wall clock. Too close to call between `deno`, `node`, `bun`. Their interquartile ranges overlap, so this axis does not separate them and no winner is claimed.

Runtime overhead. `deno` wins this axis at 26.60 ms, against `node` at 43.61, which is 1.6x.

Peak memory. `bun` wins this axis at 46.77 MiB, against `node` at 80.27, which is 1.7x.

## Compute: fib

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.4 | 1305.82 | 1309.55 | 3.66 | 2.67 |
| `node` | 26.7.0 | 74.76 | 113.86 | 38.73 | 46.48 |
| `bun` | 1.4.0 | 56.89 | 66.90 | 9.99 | 18.03 |
| `deno` | 2.9.6 | 74.68 | 92.33 | 17.75 | 35.84 |

In process. `bun` wins this axis at 56.89 ms, against `katsu` at 1305.82, which is 23.0x.

Wall clock. `bun` wins this axis at 66.90 ms, against `katsu` at 1309.55, which is 19.6x.

Runtime overhead. `katsu` wins this axis at 3.66 ms, against `node` at 38.73, which is 10.6x.

Peak memory. `katsu` wins this axis at 2.67 MiB, against `node` at 46.48, which is 17.4x.

## Compute: json

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.4 | not measured | not measured | not measured | not measured |
| `node` | 26.7.0 | 195.59 | 240.75 | 45.55 | 70.42 |
| `bun` | 1.4.0 | 105.02 | 114.56 | 10.25 | 34.70 |
| `deno` | 2.9.6 | 186.84 | 207.45 | 20.81 | 60.69 |

`katsu` could not be measured: katsu could not run json: katsu: not implemented yet: compute/json.js:18:11: an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones

In process. `bun` wins this axis at 105.02 ms, against `node` at 195.59, which is 1.9x.

Wall clock. `bun` wins this axis at 114.56 ms, against `node` at 240.75, which is 2.1x.

Runtime overhead. `bun` wins this axis at 10.25 ms, against `node` at 45.55, which is 4.4x.

Peak memory. `bun` wins this axis at 34.70 MiB, against `node` at 70.42, which is 2.0x.

## Compute: nbody

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.4 | not measured | not measured | not measured | not measured |
| `node` | 26.7.0 | 147.19 | 187.40 | 42.00 | 50.34 |
| `bun` | 1.4.0 | 191.60 | 229.97 | 13.22 | 21.36 |
| `deno` | 2.9.6 | 186.19 | 231.78 | 33.21 | 41.59 |

`katsu` could not be measured: katsu could not run nbody: katsu: not implemented yet: compute/nbody.js:13:10: an array literal is not supported yet. See https://github.com/tamnd/katsu/milestones

In process. `node` wins this axis at 147.19 ms, against `bun` at 191.60, which is 1.3x.

Wall clock. Too close to call between `node`, `bun`. Their interquartile ranges overlap, so this axis does not separate them and no winner is claimed.

Runtime overhead. Too close to call between `bun`, `node`, `deno`. Their interquartile ranges overlap, so this axis does not separate them and no winner is claimed.

Peak memory. `bun` wins this axis at 21.36 MiB, against `node` at 50.34, which is 2.4x.

## Compute: sort

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.4 | not measured | not measured | not measured | not measured |
| `node` | 26.7.0 | 123.03 | 168.48 | 45.45 | 66.05 |
| `bun` | 1.4.0 | 52.00 | 61.87 | 9.73 | 31.31 |
| `deno` | 2.9.6 | 116.43 | 137.38 | 21.31 | 56.88 |

`katsu` could not be measured: katsu could not run sort: katsu: not implemented yet: compute/sort.js:23:19: new is not supported yet. See https://github.com/tamnd/katsu/milestones

In process. `bun` wins this axis at 52.00 ms, against `node` at 123.03, which is 2.4x.

Wall clock. `bun` wins this axis at 61.87 ms, against `node` at 168.48, which is 2.7x.

Runtime overhead. `bun` wins this axis at 9.73 ms, against `node` at 45.45, which is 4.7x.

Peak memory. `bun` wins this axis at 31.31 MiB, against `node` at 66.05, which is 2.1x.

## Compute: strings

| Runtime | Version | In process (ms) | Wall clock (ms) | Runtime overhead (ms) | Peak memory (MiB) |
|---|---|---:|---:|---:|---:|
| `katsu` | 0.1.4 | not measured | not measured | not measured | not measured |
| `node` | 26.7.0 | 34.29 | 72.64 | 38.31 | 69.05 |
| `bun` | 1.4.0 | 21.74 | 33.68 | 12.08 | 41.95 |
| `deno` | 2.9.6 | 35.89 | 55.88 | 19.95 | 60.36 |

`katsu` could not be measured: katsu could not run strings: katsu: fatal: out of memory

In process. `bun` wins this axis at 21.74 ms, against `deno` at 35.89, which is 1.7x.

Wall clock. `bun` wins this axis at 33.68 ms, against `node` at 72.64, which is 2.2x.

Runtime overhead. `bun` wins this axis at 12.08 ms, against `node` at 38.31, which is 3.2x.

Peak memory. `bun` wins this axis at 41.95 MiB, against `node` at 69.05, which is 1.6x.
