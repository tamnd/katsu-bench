# katsu-bench

Benchmarks for [katsu](https://github.com/tamnd/katsu) against Node.js, Bun and Deno. Every axis is published win or lose, the machine is named, and the command is in the repository so anybody can rerun it.

**Status: the harness works, the runtime it exists to measure mostly does not yet.** katsu 0.1.5 ran 6 of the 27 axes in the last full run, up from 2. Everything else is Node against Bun against Deno with katsu marked as unable to run it and the reason printed. That is deliberate and it is the point of doing this now. A benchmark harness first proves it can measure runtimes whose relative performance is already public knowledge, because a harness that has only ever been pointed at your own project is a harness you cannot trust when it agrees with you. It also fixes the baseline before we have any incentive to move it.

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

## Where 0.1.2 stands

katsu 0.1.2 is the first release measured from the published tarball rather than from a local `cargo build --release`, which is the binary a user would actually download. Same machine, 25 runs after 3 discarded, in `results/baselines/2026-08-29-m4-macos-katsu-0.1.2.md`.

| Axis | katsu 0.1.2 | Best rival | Standing | Left to find |
|---|---:|---|---|---|
| Cold start (ms) | 1.57 | bun 5.31 | 3.38x ahead | 3.0x |
| Baseline memory at idle (MiB) | not yet | bun 12.30 | cannot run this yet | all of it |
| Distribution size (MiB) | 1.70 | bun 60.61 | 35.74x ahead | goal reached |

Read both moving rows as noise rather than as progress. Cold start went from 1.55 ms to 1.57 and the standing went from 3.21x to 3.38x, but the lead moved because Bun measured 5.31 ms this time against 4.97 last time, not because katsu got faster, and both differences are inside the run to run spread the IQR column already tells you about. The distribution grew from 1.66 MiB to 1.70, which is 40 KiB of binary bought by three milestones of language work, and it is the direction that number will keep going.

The interesting change in this run is in the failure reasons rather than in the numbers, because they are a list of exactly what is missing and they moved.

| Workload | Blocked on at 0.0.5 | Blocked on at 0.1.2 |
|---|---|---|
| fib | an object literal | `performance` is not defined |
| strings | a `for` loop | `performance` is not defined |
| json | an object literal | an array literal |
| nbody | an array literal | an array literal |
| alloc | `new` | `new` |
| sort | `new` | `new` |

Two of the six workloads have run out of language to be missing. `fib` and `strings` now parse, lower and start executing, and they get as far as the first line of the timing harness before stopping on `performance.now()`.

Read that carefully, because it is easy to read it as more than it is, and the first version of this section did. A failure reason names the first thing a workload hits, not the last. Adding `performance.now()` does not make either of these workloads run, it moves them to the next missing thing, and the honest question is how many more there are behind it. Counted by hand against the source of each workload:

| Workload | Still needs, in the order it would hit them |
|---|---|
| fib | `performance.now()`, `String()`, `JSON.stringify()` |
| strings | `performance.now()`, string `.length`, `.slice`, `.charCodeAt`, `.split`, `.join`, `Array.prototype.sort`, indexing, relational comparison on strings, `String()`, `JSON.stringify()` |

So `fib` is genuinely close and `strings` is not. The recursive part of `fib` already runs and already gives the right answer, which was checked directly rather than assumed: `fib(20)` under katsu 0.1.2 prints 6765, and the object literal the workload builds its result line from prints correctly too. Three names stand between that and the first compute number this repository ever publishes for katsu. `strings` needs most of `String.prototype` and a working `Array.prototype.sort`, which is prototype chain work, which is the same thing `alloc` and `sort` are waiting for.

The other three still need object model work as well: array literals for `json` and `nbody`, and constructors for `alloc` and `sort`.

This is the reason the failure reason is printed in full rather than collapsed to "unsupported". A reason that changes between runs is a progress report, and a reason that does not change is a milestone that has not landed yet. It is also the reason a failure reason is a starting point for an estimate rather than an estimate, and this section is now written to say so.

One thing found while checking that: `'abc'.length` evaluates to `undefined` under katsu 0.1.2 rather than to 3. It does not throw, which is worse than throwing, because a workload reading a length gets a wrong number rather than an error and everything downstream of it is quietly nonsense. Filed as [tamnd/katsu#58](https://github.com/tamnd/katsu/issues/58) rather than worked around here, because a benchmark harness that papers over a wrong answer in the thing it is measuring is worth nothing.

## The first compute number, and it is a loss

katsu 0.1.3 added `performance.now()`, `String()` and `JSON.stringify()`, which were the three names counted out in the table above, and `fib` runs. It is the first compute row this repository has ever published for katsu and it is a bad one. Measured from the published 0.1.3 tarball, same machine, 25 runs after 3 discarded, in `results/baselines/2026-08-29-m4-macos-katsu-0.1.3.md`.

| Axis | katsu 0.1.3 | Best rival | Standing | Left to find |
|---|---:|---|---|---|
| Cold start (ms) | 4.31 | bun 14.22 | 3.30x ahead | 3.0x |
| Baseline memory at idle (MiB) | not yet | bun 12.56 | cannot run this yet | all of it |
| Distribution size (MiB) | 1.71 | bun 60.61 | 35.41x ahead | goal reached |
| fib, in process (ms) | 2329.34 | bun 92.31 | 25.23x behind | 252.3x |
| fib, wall clock (ms) | 2334.36 | bun 109.09 | 21.40x behind | 214.0x |
| fib, runtime overhead (ms) | 5.37 | bun 16.76 | 3.12x ahead | 3.2x |
| fib, peak memory (MiB) | 2.70 | bun 18.25 | 6.75x ahead | 1.5x |

Read the absolute numbers in this run with a warning attached, and it is a different warning from the usual one. The machine was not quiet. An unrelated user process held a core at 97 percent for the whole run and the load average was 5.83, and every absolute here is inflated by roughly a factor of three against the same machine two hours earlier. The way to see that is the cold start row across the two runs: katsu went 1.57 to 4.31, node 24.11 to 70.50, bun 5.31 to 14.22, deno 12.65 to 24.00. Every runtime moved the same way by a similar factor, which is a machine changing and not four runtimes changing. So compare across a row here and not against a different run. The katsu against bun cold start standing is 3.30x in this run against 3.38x in the quiet one, which is the same number.

The compute loss survives that correction completely and is not a measurement artifact. A quiet head to head taken separately, five consecutive runs of each, put katsu at 1,055 ms against node's 63, bun's 45 and deno's 68, which is the same 23x behind bun that the loaded run reports. katsu loses `fib` by a factor of twenty odd and there is nothing subtle about why. `fib` is a call benchmark wearing an arithmetic benchmark's clothes, one comparison and one addition per call and nothing else, so it punishes inline caches, shape guards and a JIT, and katsu has none of the three. Two of them are M1 work and the third is M3.

The memory column is the interesting one and it is the half of the goal this project might reach first. katsu runs `fib` in 2.70 MiB of peak resident set against node's 56.08 and bun's 18.25, which is 20.7x less than node and 6.75x less than the best rival, so the 10x resource goal is within 1.5x on this workload. Attach the obvious caveat before believing it: katsu has no garbage collector, `fib` allocates almost nothing, and the workload that does allocate is the one that dies. That is the next row.

The failure reasons moved again, and one of them moved in a way worth calling out.

| Workload | Blocked on at 0.1.2 | Blocked on at 0.1.3 |
|---|---|---|
| fib | `performance` is not defined | runs |
| strings | `performance` is not defined | out of memory |
| json | an array literal | an array literal |
| nbody | an array literal | an array literal |
| alloc | `new` | `new` |
| sort | `new` | `new` |

`strings` stopped failing on a missing name and started failing on exhausted memory, which means it now gets past the timing harness and into the workload before dying. That is progress and it is also the clearest statement yet of [tamnd/katsu#60](https://github.com/tamnd/katsu/issues/60): katsu has no collector, the heap is a bump allocator over a 4 GiB cage, and a program that allocates in a loop fills it and stops. The three remaining reasons are the same three as last time, array literals and `new`, both of which are object model work.

The estimate in the previous section held up exactly. `fib` needed three names and got three names and now runs. `strings` was called out as not close, and it is not close: it needed most of `String.prototype`, a working sort and a collector, and it now demonstrates the collector part directly.

## Where 0.1.4 stands, and what the load average turned out to be worth

katsu 0.1.4 put the prototype in the shape, gave properties their attributes and bound `this` at call sites. None of that touches the call path, and `fib` is a call benchmark, so the honest expectation before running this was that nothing would move. Measured from the published 0.1.4 tarball, same machine, 25 runs after 3 discarded, in `results/baselines/2026-08-29-m4-macos-katsu-0.1.4.md`.

| Axis | katsu 0.1.4 | Best rival | Standing | Left to find |
|---|---:|---|---|---|
| Cold start (ms) | 2.76 | bun 8.80 | 3.19x ahead | 3.1x |
| Baseline memory at idle (MiB) | not yet | bun 12.28 | cannot run this yet | all of it |
| Distribution size (MiB) | 1.73 | bun 60.61 | 35.09x ahead | goal reached |
| fib, in process (ms) | 1305.82 | bun 56.89 | 22.95x behind | 229.5x |
| fib, wall clock (ms) | 1309.55 | bun 66.90 | 19.57x behind | 195.7x |
| fib, runtime overhead (ms) | 3.66 | bun 9.99 | 2.73x ahead | 3.7x |
| fib, peak memory (MiB) | 2.67 | bun 18.03 | 6.75x ahead | 1.5x |

Nothing moved, which is the expected result and worth publishing as one. The `fib` absolute went from 2329 ms to 1306 ms, which looks like the interpreter got nearly twice as fast and did not: bun's own `fib` went from 92 ms to 57 ms in the same pair of runs, which is the same proportion, and two runtimes cannot both have been rewritten between two afternoons. The machine was quieter, that is all. The standing against bun went 25.23x behind to 22.95x behind, and that residual is the subject of the next paragraph rather than a win.

This is the first run with the load average recorded, and the first thing it bought was an explanation for a discrepancy that had been sitting unexplained. The same katsu 0.1.4 tarball has now been measured against bun on this machine three times: at a load high enough to pin a core, 20.94x behind; at a load of 4.95 falling to 4.16, 22.95x behind; and by hand on a quiet machine, about 24x behind. The ratio moves with the load and it moves in katsu's favour as the machine gets busier, which is the opposite of flattering and is exactly why it is worth writing down. The likely reason is that katsu is one thread and bun is not: bun has collector and compiler threads that want cores of their own, so contention costs bun more than it costs a single threaded interpreter, and a loaded machine quietly hands katsu a result it has not earned. Treat that as the reading consistent with three runs rather than as a proven mechanism. Either way the correction goes the wrong way for us, so the number to plan against is the quiet one, and this is the argument for moving published runs onto dedicated hardware rather than onto whichever laptop is free.

The memory story is unchanged and is still the half of the goal that is closest. 2.67 MiB of peak resident set against bun's 18.03 on the same workload, with the same caveat as before: there is no collector yet, `fib` allocates almost nothing, and the workload that does allocate is the one that dies out of memory.

The failure reasons did not move at all this time. `new`, array literals and a collector are the three things standing between katsu and the other five compute workloads, and 0.1.4 was object model work underneath all three rather than any of them.

## Where 0.1.5 stands, and why the first inline cache does not show up here

katsu 0.1.5 is the first release with an inline cache in it. Every property read site now remembers the shape it last saw and the position the property was at, so a site that keeps seeing the same kind of object skips the walk up the shape chain entirely. In katsu's own microbenchmarks that took a hot property read from 12.1 ns to 9.5 ns, about fourteen percent off the line. None of that should be visible in this report, and the point of publishing the run is to say so out loud rather than to leave the release unmeasured. Measured from the published 0.1.5 tarball, same machine, 25 runs after 3 discarded, in `results/baselines/2026-08-31-m4-macos-katsu-0.1.5.md`.

| Axis | katsu 0.1.5 | Best rival | Standing | Left to find |
|---|---:|---|---|---|
| Cold start (ms) | 1.99 | bun 6.59 | 3.31x ahead | 3.0x |
| Baseline memory at idle (MiB) | not yet | bun 12.25 | cannot run this yet | all of it |
| Distribution size (MiB) | 1.74 | bun 60.61 | 34.77x ahead | goal reached |
| fib, in process (ms) | 789.33 | bun 33.47 | 23.58x behind | 235.8x |
| fib, wall clock (ms) | 791.76 | bun 40.48 | 19.56x behind | 195.6x |
| fib, runtime overhead (ms) | 2.37 | bun 6.90 | 2.91x ahead | 3.4x |
| fib, peak memory (MiB) | 2.72 | bun 18.05 | 6.64x ahead | 1.5x |

The only compute workload katsu can run is `fib`, and `fib` reads no properties. It is a function, a comparison, an addition and two recursive calls, so there is no access site in it for a cache to fill and no shape chain for a cache to skip. A change that makes property reads fourteen percent faster is worth exactly nothing on this workload and the table agrees with that prediction, which is the useful part: the harness did not invent a win where the design says there cannot be one.

What the `fib` row did do is demonstrate the noise floor twice in one run, in opposite directions. Against node the ratio went from 17.47x behind at 0.1.4 to 16.02x behind here, which looks like an eight percent gain. Against bun it went from 22.95x behind to 23.58x behind, which looks like a three percent loss. The same binary cannot have got faster and slower in the same afternoon, and neither figure is a result. The machine was quieter for this run, a load average of 3.70 against the 4.95 the 0.1.4 run started at, and the three runtimes did not absorb that quiet in the same proportion. Read both numbers as unchanged, and read the pair of them as a reminder of how much of a cross release comparison on a shared laptop is weather.

The memory row is the one worth pointing at. 2.72 MiB of peak resident set on `fib` against bun's 18.05 and node's 46.36, which is 6.64x better than the best rival and 17.1x better than node, and it is the closest published number to the half of the goal about resources. Be precise about what it measures. It is a fact about how little the interpreter allocates, not a fact about how well it cleans up, because there is nothing to clean up with. katsu has no garbage collector, the heap is a bump allocator over a cage, and the workload in this suite that does allocate in a loop is the one that dies out of memory. The honest version of this row is that katsu starts from a very good place on memory and has not yet paid for a collector, and the number to watch is what this row reads after [tamnd/katsu#60](https://github.com/tamnd/katsu/issues/60) lands.

Six of the 27 axes ran, the same six as last time, and the four failure reasons are unchanged: `new` blocks `alloc` and `sort`, an array literal blocks `json` and `nbody`, the missing collector blocks `strings`, and the missing event loop blocks idle memory because a process that exits after 401 ms has no idle to sample. Two releases in a row with the same blockers is not drift, it is what depth first work on the object model looks like from the outside, but it does mean this report keeps measuring the same one workload and the scoreboard cannot say much until that changes.

## Where 0.1.6 stands, and the one thing that did move

katsu 0.1.6 is the release where a function became an object. A function can carry properties now, so `Foo.prototype` and the statics on a constructor are ordinary properties, and `Object` is a function rather than a namespace object with the wrong type tag. Measured from the published 0.1.6 tarball, same machine, 25 runs after 3 discarded, in `results/baselines/2026-08-31-m4-macos-katsu-0.1.6.md`.

| Axis | katsu 0.1.6 | Best rival | Standing | Left to find |
|---|---:|---|---|---|
| Cold start (ms) | 1.59 | bun 6.07 | 3.82x ahead | 2.6x |
| Baseline memory at idle (MiB) | not yet | bun 12.27 | cannot run this yet | all of it |
| Distribution size (MiB) | 1.74 | bun 60.61 | 34.77x ahead | goal reached |
| fib, in process (ms) | 875.16 | bun 34.28 | 25.53x behind | 255.3x |
| fib, wall clock (ms) | 877.76 | bun 41.56 | 21.12x behind | 211.2x |
| fib, runtime overhead (ms) | 2.46 | bun 7.25 | 2.94x ahead | 3.4x |
| fib, peak memory (MiB) | 2.70 | bun 18.03 | 6.67x ahead | 1.5x |

The `fib` row got worse, from 23.58x behind bun to 25.53x, and this is the release where that number deserves a straight answer rather than the usual sentence about weather. Two things are true at once and both belong here.

The first is that the session was slower for everybody. Node went from 49.27 ms to 53.38, deno from 49.99 to 56.54, bun from 33.47 to 34.28. That is between two and thirteen percent depending on which rival you pick, and katsu moved eleven percent in the same direction, which puts it inside the band the machine moved rather than outside it. The load average was 3.75 for this run against 3.70 for the last one, so the coarse measure of business says the two sessions were the same and the runtimes disagree with it by up to eleven percent. That is the noise floor of a shared laptop and it is the reason this repository keeps saying the ratio column is the one to read.

The second is that there is a real mechanism this time, and it is small. A closure grew from sixteen bytes to twenty in this release, because a function now has a field pointing at the object its properties live in. `fib` makes no closures in its loop and hangs nothing off a function, so the only thing it can pay is the extra four bytes on the two functions it defines, and katsu's own microbenchmarks on a Linux box measured the call benchmarks inside noise across that change with one run of six showing three percent. So a couple of percent of the eleven is plausibly real and the rest is the machine. Nobody should read 25.53x as a regression of two points and nobody should read it as unchanged either.

A direct head to head between the two release binaries was attempted to settle it and could not: the machine's load average went past 26 while it ran, and `fib` under the same binary varied between 2.4 and 5.9 seconds inside one alternating sequence. That failure is worth publishing too, because it is the clearest argument yet for the item this repository has been carrying since the first baseline. These numbers need dedicated hardware, and a run this noisy is not something a careful reading can fix afterwards.

The memory row is unchanged at 2.70 MiB against bun's 18.03, which is the answer to the obvious worry about a release that made two heap objects bigger. `fib` allocates almost nothing either way. The row that will actually test the bigger closure is the one that allocates in a loop, and that row still does not run.

Six of the 27 axes ran, the same six for the third release running, and the four failure reasons are unchanged: `new` blocks `alloc` and `sort`, an array literal blocks `json` and `nbody`, the missing collector blocks `strings`, and the missing event loop blocks idle memory. The first of those is the one to watch, because everything `new` was waiting for landed in this release.

## The rules come before the numbers

These are the rules this repository holds itself to, written down before there was anything to report.

Every published figure names the machine, the operating system, the exact versions compared, the workload, and the number of runs. A number without those is not a result, it is an assertion.

Medians and interquartile ranges, never means. One slow run caused by a background process moves a mean and barely touches a median, and the mean is the number that ends up in a blog post.

A difference we cannot separate from noise is reported as no difference. The test is whether the interquartile ranges overlap, and when they do the table says "too close to call" rather than quietly reporting a five percent win. This is implemented in code rather than left to editorial discretion, in `verdict()` in `src/report.rs`.

Losses are published in the same table as wins, with the same prominence. There is no separate page for the axes we do badly on.

A competitor that could not be measured stays in the table with the reason. A missing row looks exactly like a beaten competitor to anybody skimming.

Every run records the machine's one minute load average when it started and when it finished, and a run taken above half the core count says on its face that the machine was busy. Two samples rather than one, because a run takes long enough that a machine which was quiet for the first runtime and busy for the last has not measured the two of them against each other at all. This exists because the loaded run in the 0.1.3 section below had to have its warning written by hand after the fact, and a warning that depends on somebody remembering is a warning that will eventually be missing.

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
