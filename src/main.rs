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

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand, ValueEnum};

use report::{AxisResult, Machine, RunReport, RuntimeResult};
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
}

impl Axis {
    const fn title(self) -> &'static str {
        match self {
            Axis::Startup => "Cold start",
            Axis::Memory => "Baseline memory at idle",
            Axis::Size => "Distribution size",
        }
    }

    const fn unit(self) -> &'static str {
        match self {
            Axis::Startup => "ms",
            Axis::Memory | Axis::Size => "MiB",
        }
    }

    /// Whether a lower number is better on this axis.
    ///
    /// Written out per axis rather than hardcoded, because the compute axes that land next
    /// are scores where higher wins, and a hardcoded `true` would silently invert the
    /// verdict on the first one of those to be added.
    const fn lower_is_better(self) -> bool {
        match self {
            Axis::Startup | Axis::Memory | Axis::Size => true,
        }
    }
}

const ALL_AXES: &[Axis] = &[Axis::Startup, Axis::Memory, Axis::Size];

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

    let mut results = Vec::new();

    for &axis in axes {
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
            unit: axis.unit().to_owned(),
            lower_is_better: axis.lower_is_better(),
            runtimes: per_runtime,
        });
    }

    Ok(RunReport {
        machine: Machine::here(),
        axes: results,
    })
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
            let script = workloads.join("hello.js");
            let script = script.to_string_lossy().into_owned();

            // No warmup. The whole point of a cold start number is that nothing was warm,
            // and a run after three throwaway runs has a hot page cache and a hot CPU.
            let _ = warmup;

            let mut samples = Vec::with_capacity(runs);
            for _ in 0..runs {
                let measurement = measure::run_once(&r.path, &[&script], workloads)?;
                if !measurement.succeeded {
                    bail!("{} could not run {script}", r.name);
                }
                samples.push(measurement.wall_ms);
            }
            Ok(Summary::of(&samples))
        }
        Axis::Memory => {
            let script = workloads.join("idle.js");
            let script = script.to_string_lossy().into_owned();

            let mut samples = Vec::with_capacity(runs);
            for _ in 0..runs.min(10) {
                let measurement = measure::run_until_idle(
                    &r.path,
                    &[&script],
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
    }
}

#[allow(clippy::cast_precision_loss)]
fn bytes_to_mib(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0)
}
