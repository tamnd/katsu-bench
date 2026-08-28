# katsu-bench

Benchmarks for [katsu](https://github.com/tamnd/katsu) against Node.js, Bun and Deno. Every axis is published win or lose, the machine is named, and the command is in the repository so anybody can rerun it.

**Status: the harness works, the runtime it exists to measure does not yet.** katsu is pre M0 and cannot execute JavaScript, so the numbers below are Node against Bun against Deno with katsu absent from the table. That is deliberate. A benchmark harness first proves it can measure runtimes whose relative performance is already public knowledge, because a harness that has only ever been pointed at your own project is a harness you cannot trust when it agrees with you.

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

## What is measured next

JetStream 3.0, as the primary compute benchmark, once katsu can run it. It is the suite that rewards a real JIT and punishes a fast interpreter pretending to be one.

Application shaped HTTP throughput with a database behind it, which is the benchmark that produces the 12,000 requests per second convergence described above.

Time to first request for a realistic server, which is the number that actually decides a serverless bill.

## What is deliberately not measured

Speedometer 3.1, because it is a browser benchmark and there is no DOM here. Quoting it would be dishonest in a way that is easy to get away with.

Microbenchmarks of individual operations. They measure whether we happened to optimise the thing being measured, and the answer is usually yes, which is why everybody publishes them.

Anything about correctness. That is [tamnd/katsu-compat](https://github.com/tamnd/katsu-compat). Mixing correctness and performance into one score is how both stop meaning anything.

## Using it

```
cargo build --release
./target/release/katsu-bench list
./target/release/katsu-bench run --runs 30 --markdown results/latest.md
./target/release/katsu-bench run --axis startup --runtime node --runtime bun
```

Stable Rust 1.98 or newer, plus whichever runtimes you want in the table. Anything not installed is left out rather than reported as a zero.

## The claim we are trying to earn

katsu targets 10x faster cold start and 10x less memory than Node, and 10x smaller deployment size. Those are the axes where a 10x is physically available, because Node's cold start is dominated by snapshot deserialisation and module resolution rather than by anything fundamental.

On compute throughput we do not expect 10x and will not claim it. A mature JIT against another mature JIT is a matter of tens of percent, and this repository exists partly so that when we are within noise of Node on JetStream, that is what the table says.

## License

MIT or Apache-2.0, at your option.
