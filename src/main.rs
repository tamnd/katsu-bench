//! Measures JavaScript runtimes against each other and publishes the losses next to the wins.
//!
//! The rules come before the numbers, and they are in `README.md`. The short version is that
//! every published figure names the machine, the operating system, the exact versions
//! compared, the workload and the run count, and that a difference we cannot separate from
//! noise is reported as no difference rather than as a small win.

mod measure;
mod report;
mod runtime;
mod stats;
mod workload;

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand, ValueEnum};

use report::{AxisResult, Load, Machine, RunReport, RuntimeResult};
use runtime::Runtime;
use stats::Summary;

/// Benchmarks for JavaScript runtimes.
#[derive(Debug, Parser)]
#[command(name = "katsu-bench", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Task,
}

#[derive(Debug, Subcommand)]
enum Task {
    /// List the runtimes found on this machine.
    List,
    /// Run one or more axes and write the results.
    Run {
        /// Which axes to run. Defaults to every axis that is implemented.
        #[arg(long, value_enum)]
        axis: Vec<Axis>,
        /// Restrict to these runtimes. Defaults to every one that is installed.
        #[arg(long)]
        runtime: Vec<String>,
        /// How many times to run each measurement.
        #[arg(long, default_value_t = 30)]
        runs: usize,
        /// Runs taken before measurement starts, discarded. Zero for startup axes, because
        /// warming up a cold start measurement measures the opposite of what it claims to.
        #[arg(long, default_value_t = 3)]
        warmup: usize,
        /// Where to write the machine readable results.
        #[arg(long, default_value = "results/latest.json")]
        out: PathBuf,
        /// Where to write the published table.
        #[arg(long)]
        markdown: Option<PathBuf>,
    },
}

/// The axes from the design document, and which of them are implemented.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
enum Axis {
    /// Wall clock from process spawn to exit, on an empty file and on a hello world.
    Startup,
    /// Resident set at a genuine idle, sampled from outside the process.
    Memory,
    /// Size of the runtime executable on disk.
    Size,
    /// The compute workloads, each reporting four numbers about the same run.
    Compute,
}

impl Axis {
    const fn title(self) -> &'static str {
        match self {
            Axis::Startup => "Cold start",
            Axis::Memory => "Baseline memory at idle",
            Axis::Size => "Distribution size",
            Axis::Compute => "Compute",
        }
    }

    const fn unit(self) -> &'static str {
        match self {
            Axis::Startup | Axis::Compute => "ms",
            Axis::Memory | Axis::Size => "MiB",
        }
    }

    /// Whether a lower number is better on this axis.
    ///
    /// Written out per axis rather than hardcoded, because a score where higher wins is a
    /// perfectly reasonable axis to add later and a hardcoded `true` would silently invert the
    /// verdict on the first one of those.
    const fn lower_is_better(self) -> bool {
        match self {
            Axis::Startup | Axis::Memory | Axis::Size | Axis::Compute => true,
        }
    }
}

const ALL_AXES: &[Axis] = &[Axis::Startup, Axis::Memory, Axis::Size, Axis::Compute];

/// The four numbers one compute run produces.
///
/// They come from one run of one process, which is the point. Wall clock and in process time
/// taken from separate runs could not be subtracted from each other, and the subtraction is the
/// interesting part: it is how much of a short program's latency is the runtime existing rather
/// than the program running.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Metric {
    /// What the workload timed with `performance.now()` around its own work.
    InProcess,
    /// Spawn to exit, measured from outside.
    Wall,
    /// Wall clock minus in process time.
    Overhead,
    /// Peak resident set over the whole run.
    Peak,
}

impl Metric {
    const fn title(self) -> &'static str {
        match self {
            Metric::InProcess => "In process",
            Metric::Wall => "Wall clock",
            Metric::Overhead => "Runtime overhead",
            Metric::Peak => "Peak memory",
        }
    }

    const fn unit(self) -> &'static str {
        match self {
            Metric::InProcess | Metric::Wall | Metric::Overhead => "ms",
            Metric::Peak => "MiB",
        }
    }
}

const COMPUTE_METRICS: &[Metric] = &[
    Metric::InProcess,
    Metric::Wall,
    Metric::Overhead,
    Metric::Peak,
];

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("katsu-bench: {error:#}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    match Cli::parse().command {
        Task::List => {
            let found = runtime::discover(None);
            if found.is_empty() {
                bail!("none of {:?} are on PATH", runtime::CANDIDATES);
            }
            for r in &found {
                println!("{:<8} {:<12} {}", r.name, r.version, r.path.display());
            }
            Ok(())
        }
        Task::Run {
            axis,
            runtime: only,
            runs,
            warmup,
            out,
            markdown,
        } => {
            let axes = if axis.is_empty() {
                ALL_AXES.to_vec()
            } else {
                axis
            };
            let runtimes = runtime::discover(Some(&only));
            if runtimes.is_empty() {
                bail!(
                    "no runtimes found. Install at least one of {:?}.",
                    runtime::CANDIDATES
                );
            }
            let report = measure_all(&axes, &runtimes, runs, warmup)?;

            if let Some(parent) = out.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&out, serde_json::to_vec_pretty(&report)?)?;

            let table = report::markdown(&report);
            if let Some(path) = &markdown {
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(path, &table)?;
            }
            print!("{table}");
            Ok(())
        }
    }
}

fn measure_all(
    axes: &[Axis],
    runtimes: &[Runtime],
    runs: usize,
    warmup: usize,
) -> Result<RunReport> {
    let workloads = Path::new(env!("CARGO_MANIFEST_DIR")).join("workloads");
    if !workloads.is_dir() {
        bail!("no workloads directory at {}", workloads.display());
    }

    let started_at = Load::one_minute();
    let mut results = Vec::new();

    for &axis in axes {
        if axis == Axis::Compute {
            results.extend(measure_compute(runtimes, &workloads, runs, warmup)?);
            continue;
        }

        let mut per_runtime = Vec::new();
        for r in runtimes {
            eprintln!("{} against {} {}", axis.title(), r.name, r.version);
            match measure_axis(axis, r, &workloads, runs, warmup) {
                Ok(summary) => per_runtime.push(RuntimeResult {
                    runtime: r.name.clone(),
                    version: r.version.clone(),
                    summary: Some(summary),
                    note: None,
                }),
                Err(error) => per_runtime.push(RuntimeResult {
                    runtime: r.name.clone(),
                    version: r.version.clone(),
                    summary: None,
                    // A runtime that could not be measured is reported as not measured. It
                    // is never dropped from the table, because a competitor that quietly
                    // disappears looks exactly like a competitor that lost.
                    note: Some(format!("{error:#}")),
                }),
            }
        }
        results.push(AxisResult {
            axis: axis.title().to_owned(),
            group: None,
            metric: None,
            unit: axis.unit().to_owned(),
            lower_is_better: axis.lower_is_better(),
            runtimes: per_runtime,
        });
    }

    Ok(RunReport {
        machine: Machine::here(),
        load: Some(Load {
            start: started_at,
            end: Load::one_minute(),
        }),
        axes: results,
    })
}

/// Every compute workload against every runtime, four axes to a workload.
///
/// The runs happen once and are turned into four axes afterwards rather than being run four
/// times, because the four numbers only mean anything together. Subtracting an in process time
/// taken on Tuesday from a wall clock taken on Wednesday would produce an overhead figure that
/// describes neither.
fn measure_compute(
    runtimes: &[Runtime],
    workloads: &Path,
    runs: usize,
    warmup: usize,
) -> Result<Vec<AxisResult>> {
    let scripts = workload::compute_workloads(&workloads.join("compute"))?;
    let mut results = Vec::new();

    for script in &scripts {
        // One entry per runtime per metric, filled in together so that every axis in the group
        // holds its runtimes in the same order and the table can index rather than search.
        let mut per_metric: Vec<Vec<RuntimeResult>> =
            COMPUTE_METRICS.iter().map(|_| Vec::new()).collect();
        // The first checksum anybody produced, which every later runtime has to match.
        let mut agreed: Option<(String, String)> = None;

        for r in runtimes {
            eprintln!("{} against {} {}", script.name, r.name, r.version);
            let outcome = run_compute(r, script, workloads, runs, warmup, &mut agreed);
            for (index, metric) in COMPUTE_METRICS.iter().enumerate() {
                let (summary, note) = match &outcome {
                    Ok(samples) => (Some(Summary::of(samples.of(*metric))), None),
                    Err(error) => (None, Some(format!("{error:#}"))),
                };
                per_metric[index].push(RuntimeResult {
                    runtime: r.name.clone(),
                    version: r.version.clone(),
                    summary,
                    note,
                });
            }
        }

        for (index, metric) in COMPUTE_METRICS.iter().enumerate() {
            results.push(AxisResult {
                axis: format!("{}, {}", script.name, metric.title().to_lowercase()),
                group: Some(format!("Compute: {}", script.name)),
                metric: Some(metric.title().to_owned()),
                unit: metric.unit().to_owned(),
                lower_is_better: true,
                runtimes: std::mem::take(&mut per_metric[index]),
            });
        }
    }

    Ok(results)
}

/// The four sample sets one runtime produced on one workload.
#[derive(Debug, Default)]
struct ComputeSamples {
    in_process: Vec<f64>,
    wall: Vec<f64>,
    overhead: Vec<f64>,
    peak: Vec<f64>,
}

impl ComputeSamples {
    fn of(&self, metric: Metric) -> &[f64] {
        match metric {
            Metric::InProcess => &self.in_process,
            Metric::Wall => &self.wall,
            Metric::Overhead => &self.overhead,
            Metric::Peak => &self.peak,
        }
    }
}

/// Run one workload against one runtime, `runs` times, checking it agreed with everyone else.
fn run_compute(
    r: &Runtime,
    script: &workload::Workload,
    cwd: &Path,
    runs: usize,
    warmup: usize,
    agreed: &mut Option<(String, String)>,
) -> Result<ComputeSamples> {
    // Relative to the working directory the child gets, so that a runtime's error message names
    // `compute/fib.js` rather than an absolute path that is longer than the message.
    let path = script
        .path
        .strip_prefix(cwd)
        .unwrap_or(&script.path)
        .to_string_lossy()
        .into_owned();
    let mut samples = ComputeSamples::default();

    for index in 0..(warmup + runs) {
        let measurement = measure::run_once(&r.path, &r.args_for(&path), cwd)?;
        if !measurement.succeeded {
            bail!(
                "{} could not run {}: {}",
                r.name,
                script.name,
                measurement.first_error_line()
            );
        }

        let reported = workload::parse_report(&measurement.stdout)
            .with_context(|| format!("{} on {}", r.name, script.name))?;

        // A runtime that computes a different answer is wrong, and a wrong answer arrived at
        // quickly is not a win. This is where a suite catches an optimiser that deleted the
        // work, and it is checked on every run rather than once, because a runtime that only
        // disagrees after its optimising tier kicks in is exactly the interesting case.
        match agreed {
            Some((owner, expected)) if expected != &reported.checksum => {
                bail!(
                    "{} computed {} on {}, but {owner} computed {expected}",
                    r.name,
                    reported.checksum,
                    script.name
                );
            }
            Some(_) => {}
            None => *agreed = Some((r.name.clone(), reported.checksum.clone())),
        }

        if index < warmup {
            continue;
        }

        samples.in_process.push(reported.compute_ms);
        samples.wall.push(measurement.wall_ms);
        // Clamped at zero. The two clocks are different clocks and on a very short workload the
        // difference can come out slightly negative, which is a measurement artefact and not a
        // runtime that finished before it started.
        samples
            .overhead
            .push((measurement.wall_ms - reported.compute_ms).max(0.0));
        samples.peak.push(bytes_to_mib(measurement.peak_rss_bytes));
    }

    Ok(samples)
}

fn measure_axis(
    axis: Axis,
    r: &Runtime,
    workloads: &Path,
    runs: usize,
    warmup: usize,
) -> Result<Summary> {
    match axis {
        Axis::Startup => {
            // Named relative to the working directory the child gets, for the same reason the
            // compute workloads are.
            let script = "hello.js".to_owned();

            // No warmup. The whole point of a cold start number is that nothing was warm,
            // and a run after three throwaway runs has a hot page cache and a hot CPU.
            let _ = warmup;

            let mut samples = Vec::with_capacity(runs);
            for _ in 0..runs {
                let measurement = measure::run_once(&r.path, &r.args_for(&script), workloads)?;
                if !measurement.succeeded {
                    bail!("{} could not run {script}", r.name);
                }
                samples.push(measurement.wall_ms);
            }
            Ok(Summary::of(&samples))
        }
        Axis::Memory => {
            let script = "idle.js".to_owned();

            let mut samples = Vec::with_capacity(runs);
            for _ in 0..runs.min(10) {
                let measurement = measure::run_until_idle(
                    &r.path,
                    &r.args_for(&script),
                    workloads,
                    Duration::from_millis(400),
                )?;
                samples.push(bytes_to_mib(measurement.peak_rss_bytes));
            }
            Ok(Summary::of(&samples))
        }
        Axis::Size => {
            let bytes = measure::distribution_size(&r.path)
                .with_context(|| format!("measuring {}", r.name))?;
            // One deterministic reading. Reporting it as thirty identical samples would
            // dress a `stat` call up as a measurement.
            Ok(Summary::of(&[bytes_to_mib(bytes)]))
        }
        // Handled by `measure_compute`, because one workload produces four axes and this
        // function's signature only has room for one.
        Axis::Compute => unreachable!("the compute axis is measured by measure_compute"),
    }
}

#[allow(clippy::cast_precision_loss)]
fn bytes_to_mib(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0)
}
