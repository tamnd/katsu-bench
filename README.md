# katsu-bench

Benchmarks for [katsu](https://github.com/tamnd/katsu) against Node.js, Bun and Deno. Every axis is published win or lose, the machine is named, and the command is in the repository so anybody can rerun it.

**Status: the harness works, the runtime it exists to measure mostly does not yet.** katsu ran 2 of the 27 axes in the last full run. Everything else is Node against Bun against Deno with katsu marked as unable to run it and the reason printed. That is deliberate and it is the point of doing this now. A benchmark harness first proves it can measure runtimes whose relative performance is already public knowledge, because a harness that has only ever been pointed at your own project is a harness you cannot trust when it agrees with you. It also fixes the baseline before we have any incentive to move it.

The report opens with a scoreboard called `Distance to the goal`, one row per axis, saying where katsu stands against the best rival on that axis and how much is left to find. It measures against the best rival rather than against Node, because ten times better than the slowest competitor is not the claim. Axes katsu cannot run say so, and the paragraph under the table says how many of those there are, so the scoreboard cannot improve by quietly dropping the axes we fail.

## The baseline, taken before we could compete on it

Apple M4, macOS 15.8, 10 cores. Node 26.8.1, Bun 1.4.0, Deno 2.9.6, all the latest release on 28 August 2026. 25 runs per measurement after 3 discarded, medians.

| | Node | Bun | Deno |
|---|---:|---:|---:|
| Cold start (ms) | 26.0 | 5.2 | 11.9 |
| Idle memory (MiB) | 48.2 | 12.3 | 34.0 |
| Distribution size (MiB) | 139.1 | 60.6 | 120.4 |
| Runtime overhead per process (ms) | 25.8 | 7.1 | 13.5 |

The compute workloads, in process time in milliseconds, then peak resident memory in mebibytes.

| Workload | Node | Bun | Deno | Node MiB | Bun MiB | Deno MiB |
|---|---:|---:|---:|---:|---:|---:|
| alloc | 49.2 | 37.9 | 48.5 | 88.4 | 46.3 | 69.9 |
| fib | 48.8 | 33.7 | 49.9 | 54.6 | 18.1 | 36.0 |
| json | 110.1 | 68.1 | 111.6 | 77.6 | 34.5 | 60.2 |
| nbody | 87.3 | 105.1 | 86.8 | 58.7 | 21.1 | 40.1 |
| sort | 70.3 | 33.7 | 67.9 | 74.4 | 31.3 | 56.0 |
| strings | 22.3 | 11.4 | 21.3 | 77.9 | 41.7 | 59.5 |

Node and Deno are within noise of each other on five of the six, which is what you would expect from two runtimes sharing an engine. Bun wins five of six on time and all six on memory, and loses `nbody` by twenty percent, which is the workload that is nothing but floating point arithmetic and property access on fixed shape objects. The whole table, with interquartile ranges, every axis and every failure reason, is in `results/baselines/2026-08-28-m4-macos.md`. That file is committed and every later baseline will be too, because a baseline that only exists on the machine that took it is not a baseline. The raw JSON with every individual sample stays out of the repository, because it is machine output that nobody reads in a diff and it would grow the history faster than the code does.

Bun is the rival to beat on every axis here, and the goal is ten times better than that, not ten times better than Node.

## The first axis katsu can actually run

katsu 0.0.5 landed `console.log` and a `katsu run` that does what it says, which is the smallest thing a runtime can do that this harness can time. Cold start is now a real row rather than a failure reason. Same machine, same day, 25 runs after 3 discarded, in `results/baselines/2026-08-29-m4-macos.md`.

| Runtime | Version | Median | IQR | Min | Max |
|---|---|---:|---:|---:|---:|
| katsu | 0.0.5 | 1.55 ms | 0.09 | 1.42 | 3.31 |
| bun | 1.4.0 | 4.97 ms | 0.82 | 4.69 | 23.70 |
| deno | 2.9.6 | 12.49 ms | 0.73 | 11.46 | 67.49 |
| node | 26.8.1 | 24.63 ms | 1.98 | 22.87 | 62.22 |

15.9x faster than Node and 3.2x faster than Bun, which is the rival that matters on this axis. The scoreboard reads that as 3.21x ahead with 3.1x left to find, because the goal is ten times better than the best rival and Bun is the best rival.

Look at the maximum column before reading too much into any of it. Every runtime here has a worst case several times its own median, Node and Deno by a factor of nearly three and six, because a first run on a cold page cache is a different measurement from a run on a warm one and 25 runs is enough to catch one or two of those. That is why the published figure is the median with an interquartile range next to it rather than a mean, and it is the reason the IQR column exists at all.

Read it with the obvious caveat attached, and the caveat is large. **katsu is ahead here because it does less, not because it is better.** There is no module system, no `process`, no event loop, no filesystem and no standard library beyond `console.log`, and Node is carrying every one of those inside its 23.83 ms. This number is a starting position rather than a result, and the interesting question is not whether we win today but how much of the lead survives the next eight milestones landing on top of it. That is precisely why the row is recorded now, while the answer is still unknown to us.

The idle memory axis is still a failure reason, and for a good reason: a process with no event loop cannot idle. katsu exits after 410 ms rather than staying up, so there is no idle to sample. That row starts working in M2 and not before.

## The rules come before the numbers

These are the rules this repository holds itself to, written down before there was anything to report.

Every published figure names the machine, the operating system, the exact versions compared, the workload, and the number of runs. A number without those is not a result, it is an assertion.

Medians and interquartile ranges, never means. One slow run caused by a background process moves a mean and barely touches a median, and the mean is the number that ends up in a blog post.

A difference we cannot separate from noise is reported as no difference. The test is whether the interquartile ranges overlap, and when they do the table says "too close to call" rather than quietly reporting a five percent win. This is implemented in code rather than left to editorial discretion, in `verdict()` in `src/report.rs`.

Losses are published in the same table as wins, with the same prominence. There is no separate page for the axes we do badly on.

A competitor that could not be measured stays in the table with the reason. A missing row looks exactly like a beaten competitor to anybody skimming.

Results taken on a shared CI runner are labelled as such on their face. Cloud runners are noisy enough that they are good for spotting a regression trend over time and bad for comparing runtimes, and the report says so at the top rather than in a footnote nobody reads.

## The finding that shapes all of this

The most useful result in the 2026 runtime comparison literature is also the most inconvenient one for everybody selling a runtime. Marketing benchmarks report Bun at around 52,000 requests per second against Node's 14,000, a gap of nearly 270 percent. When the same comparison tests an actual application with a database and business logic, all three runtimes land at roughly 12,000 requests per second, essentially identical, because routing, validation, database round trips and application logic dominate and the engine becomes noise.

We take that seriously rather than working around it. It is why katsu's headline claim is about cold start, memory and deployment size rather than about requests per second, and it is why this repository will publish the application shaped throughput benchmark that shows the convergence even though that benchmark makes our own project look ordinary.

## What is measured today

`Cold start` is wall clock from process spawn to exit on a hello world, taken with no warmup at all. Warming up a cold start measurement measures the opposite of what it claims to: after three throwaway runs the page cache is hot, the CPU has clocked up, and you are reporting a warm start.

`Baseline memory at idle` is the resident set of a process that has finished starting and is sitting in the event loop, sampled from outside the process after a settle period. It is not the runtime's self reported heap. A container's memory limit counts resident set, not what the garbage collector thinks it owns.

`Distribution size` is the executable plus every non system shared library it loads at startup. Measuring the executable alone is wrong often enough to be useless: a Homebrew `node` is a fifty kilobyte launcher in front of a `libnode` dylib holding the whole engine, and an executable only measurement reports it as three thousand times smaller than Deno. Libraries the operating system ships are excluded because every runtime shares them.

`Compute` is six workloads in `workloads/compute`, and each one reports four numbers about the same single run of the same single process.

## The four numbers a compute workload reports

`In process` is what the workload timed itself, with `performance.now()` called either side of the work and nothing else in between. This is the number that is about the engine.

`Wall clock` is spawn to exit, measured from outside with the kernel's clock. This is the number that is about the user, because it includes finding the executable, mapping it, building the heap, parsing the script, compiling it, running it and tearing everything down.

`Runtime overhead` is wall clock minus in process time. It is the most useful number in the table for anybody deploying to a serverless platform, and it is why the four are taken from one run rather than from four: an in process time from Tuesday subtracted from a wall clock from Wednesday describes nothing. On this machine it is a flat cost per process of about 7 ms for Bun, 13 ms for Deno and 26 ms for Node, and it does not change with the workload, which is what tells you it is the runtime and not the program.

`Peak memory` is the peak resident set of the whole run, taken from `wait4` rather than by polling, because polling from outside cannot see the peak of a process that lives for forty milliseconds.

Every workload also prints a checksum, and every runtime has to produce the same one. A runtime that disagrees is reported as wrong rather than as fast. This is not a formality: the first version of the allocation workload only kept one object in a thousand, and both V8 and JavaScriptCore noticed the other nine hundred and ninety nine never escaped, replaced them with locals and deleted the allocation entirely, which showed up as three million allocations in ten milliseconds. Writing every object into a live array fixed it. A benchmark suite without a checksum would have published that number.

## What is measured next

JetStream 3.0, as the primary compute benchmark, once katsu can run it. It is the suite that rewards a real JIT and punishes a fast interpreter pretending to be one.

Application shaped HTTP throughput with a database behind it, which is the benchmark that produces the 12,000 requests per second convergence described above.

Time to first request for a realistic server, which is the number that actually decides a serverless bill.

## What is deliberately not measured

Speedometer 3.1, because it is a browser benchmark and there is no DOM here. Quoting it would be dishonest in a way that is easy to get away with.

Microbenchmarks of individual operations. They measure whether we happened to optimise the thing being measured, and the answer is usually yes, which is why everybody publishes them. The six compute workloads are deliberately at the other end of that scale: each one runs for tens of milliseconds and touches calls, allocation, property access, strings and native code together, because that is the smallest unit where the answer is not simply a function of what we chose to look at.

Anything about correctness. That is [tamnd/katsu-compat](https://github.com/tamnd/katsu-compat). Mixing correctness and performance into one score is how both stop meaning anything.

## Using it

```
cargo build --release
./target/release/katsu-bench list
./target/release/katsu-bench run --runs 25 --markdown results/latest.md
./target/release/katsu-bench run --axis compute --runtime node --runtime bun
```

Stable Rust 1.98 or newer, plus whichever runtimes you want in the table. Anything not installed is left out rather than reported as a zero. Runtimes are found on `PATH`, so put the katsu you want measured in front of it.

Run output goes to `results/`, which is ignored except for the markdown in `results/baselines/`. A run worth keeping gets its table copied into that directory under a name that says the date and the machine, and the ones already there stay exactly as they were taken. Rewriting an old baseline after the runtime got faster is how a project convinces itself it was always winning.

macOS and Linux today. The measurement code goes through `wait4` and `rusage` for the peak resident set, which is the kernel's own accounting rather than a sample we took at a moment of our choosing, and getting the equivalent on Windows means `GetProcessMemoryInfo` and a job object. That is worth doing properly rather than approximately, so it is its own piece of work rather than a hurried `#[cfg]`.

## The claim we are trying to earn

katsu targets 10x faster cold start and 10x less memory than Node, and 10x smaller deployment size. Those are the axes where a 10x is physically available, because Node's cold start is dominated by snapshot deserialisation and module resolution rather than by anything fundamental.

On compute throughput we do not expect 10x and will not claim it. A mature JIT against another mature JIT is a matter of tens of percent, and this repository exists partly so that when we are within noise of Node on JetStream, that is what the table says.

## License

MIT or Apache-2.0, at your option.
