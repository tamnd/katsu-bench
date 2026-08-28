//! Turning measurements into the table that gets published.

use std::fmt::Write as _;

use serde::{Deserialize, Serialize};

use crate::stats::Summary;

/// What machine the numbers were taken on.
///
/// Recorded automatically, because a benchmark result with no machine attached is not a
/// result. Cloud runners are too noisy for anything but relative trends and the report
/// says so on its face rather than in a footnote.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Machine {
    /// Operating system name.
    pub os: String,
    /// Operating system version.
    pub os_version: String,
    /// CPU model string.
    pub cpu: String,
    /// Physical core count as the OS reports it.
    pub cores: usize,
    /// Total system memory in mebibytes.
    pub memory_mib: u64,
    /// Whether this looks like a shared CI runner rather than dedicated hardware.
    pub ci: bool,
}

impl Machine {
    /// Describe the machine this is running on.
    pub fn here() -> Machine {
        let mut system = sysinfo::System::new();
        system.refresh_memory();
        system.refresh_cpu_all();

        let cpu = system
            .cpus()
            .first()
            .map_or_else(|| "unknown".to_string(), |c| c.brand().trim().to_owned());

        Machine {
            os: sysinfo::System::name().unwrap_or_else(|| "unknown".into()),
            os_version: sysinfo::System::os_version().unwrap_or_else(|| "unknown".into()),
            cpu: if cpu.is_empty() {
                "unknown".into()
            } else {
                cpu
            },
            cores: system.cpus().len(),
            memory_mib: system.total_memory() / (1024 * 1024),
            ci: std::env::var_os("CI").is_some(),
        }
    }
}

/// One runtime's result on one axis.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeResult {
    /// Runtime name.
    pub runtime: String,
    /// Runtime version.
    pub version: String,
    /// The measurement, absent if it could not be taken.
    pub summary: Option<Summary>,
    /// Why it could not be taken, when it could not.
    pub note: Option<String>,
}

/// Every runtime's result on one axis.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AxisResult {
    /// Human readable axis name.
    pub axis: String,
    /// The unit the numbers are in.
    pub unit: String,
    /// Whether smaller is better.
    pub lower_is_better: bool,
    /// One entry per runtime, in the order they were measured.
    pub runtimes: Vec<RuntimeResult>,
}

/// A whole run.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RunReport {
    /// Where it was taken.
    pub machine: Machine,
    /// What was measured.
    pub axes: Vec<AxisResult>,
}

/// Render the published tables.
pub fn markdown(report: &RunReport) -> String {
    let mut out = String::new();
    let machine = &report.machine;

    let _ = writeln!(out, "# Runtime benchmarks");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "Machine: {} {}, {}, {} cores, {} MiB.",
        machine.os, machine.os_version, machine.cpu, machine.cores, machine.memory_mib
    );
    if machine.ci {
        let _ = writeln!(out);
        let _ = writeln!(
            out,
            "**Taken on a shared CI runner.** These numbers are good for spotting a trend and bad for comparing runtimes, because the machine is shared with whatever else the provider put on it. Published comparisons come from the dedicated hardware named in the README."
        );
    }

    for axis in &report.axes {
        let _ = writeln!(out);
        let _ = writeln!(out, "## {} ({})", axis.axis, axis.unit);
        let _ = writeln!(out);
        let _ = writeln!(
            out,
            "| Runtime | Version | Median | IQR | p25 | p75 | Min | Max | Runs |"
        );
        let _ = writeln!(out, "|---|---|---:|---:|---:|---:|---:|---:|---:|");

        for entry in &axis.runtimes {
            match &entry.summary {
                Some(s) => {
                    let _ = writeln!(
                        out,
                        "| `{}` | {} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {} |",
                        entry.runtime,
                        entry.version,
                        s.median,
                        s.iqr(),
                        s.p25,
                        s.p75,
                        s.min,
                        s.max,
                        s.runs
                    );
                }
                None => {
                    let _ = writeln!(
                        out,
                        "| `{}` | {} | not measured | | | | | | |",
                        entry.runtime, entry.version
                    );
                }
            }
        }

        for entry in &axis.runtimes {
            if let Some(note) = &entry.note {
                let _ = writeln!(out);
                let _ = writeln!(out, "`{}` could not be measured: {note}", entry.runtime);
            }
        }

        if let Some(verdict) = verdict(axis) {
            let _ = writeln!(out);
            let _ = writeln!(out, "{verdict}");
        }
    }

    out
}

/// State plainly who won this axis, or say that the axis did not separate them.
///
/// This is the function that keeps the repository honest. It is written so that it is
/// awkward to produce a favourable sentence about katsu without the data supporting it,
/// and so that a result too close to call says so instead of being rounded into a win.
fn verdict(axis: &AxisResult) -> Option<String> {
    let measured: Vec<(&RuntimeResult, Summary)> = axis
        .runtimes
        .iter()
        .filter_map(|r| r.summary.map(|s| (r, s)))
        .collect();

    if measured.len() < 2 {
        return None;
    }

    let best = measured.iter().min_by(|a, b| {
        let ordering =
            a.1.median
                .partial_cmp(&b.1.median)
                .unwrap_or(std::cmp::Ordering::Equal);
        if axis.lower_is_better {
            ordering
        } else {
            ordering.reverse()
        }
    })?;

    let contenders: Vec<&(&RuntimeResult, Summary)> = measured
        .iter()
        .filter(|entry| entry.0.runtime != best.0.runtime && !entry.1.separable_from(best.1))
        .collect();

    if !contenders.is_empty() {
        let names: Vec<String> = std::iter::once(format!("`{}`", best.0.runtime))
            .chain(contenders.iter().map(|c| format!("`{}`", c.0.runtime)))
            .collect();
        return Some(format!(
            "Too close to call between {}. Their interquartile ranges overlap, so this axis does not separate them and no winner is claimed.",
            names.join(", ")
        ));
    }

    let worst = measured.iter().max_by(|a, b| {
        let ordering =
            a.1.median
                .partial_cmp(&b.1.median)
                .unwrap_or(std::cmp::Ordering::Equal);
        if axis.lower_is_better {
            ordering
        } else {
            ordering.reverse()
        }
    })?;

    let ratio = if axis.lower_is_better {
        worst.1.median / best.1.median
    } else {
        best.1.median / worst.1.median
    };

    Some(format!(
        "`{}` wins this axis at {:.2} {}, against `{}` at {:.2}, which is {ratio:.1}x.",
        best.0.runtime, best.1.median, axis.unit, worst.0.runtime, worst.1.median
    ))
}

#[cfg(test)]
mod tests {
    use super::{AxisResult, Machine, RunReport, RuntimeResult, markdown, verdict};
    use crate::stats::Summary;

    fn entry(name: &str, samples: &[f64]) -> RuntimeResult {
        RuntimeResult {
            runtime: name.into(),
            version: "1.0.0".into(),
            summary: Some(Summary::of(samples)),
            note: None,
        }
    }

    fn axis(runtimes: Vec<RuntimeResult>) -> AxisResult {
        AxisResult {
            axis: "Cold start".into(),
            unit: "ms".into(),
            lower_is_better: true,
            runtimes,
        }
    }

    #[test]
    fn a_clear_win_is_stated_with_the_ratio() {
        let a = axis(vec![
            entry("katsu", &[2.0, 2.1, 2.2, 2.3, 2.4]),
            entry("node", &[40.0, 41.0, 42.0, 43.0, 44.0]),
        ]);
        let verdict = verdict(&a).expect("there should be a verdict");
        assert!(verdict.contains("`katsu` wins"), "{verdict}");
        assert!(verdict.contains("19.1x"), "{verdict}");
    }

    #[test]
    fn a_loss_is_stated_just_as_plainly_as_a_win() {
        let a = axis(vec![
            entry("katsu", &[40.0, 41.0, 42.0, 43.0, 44.0]),
            entry("bun", &[2.0, 2.1, 2.2, 2.3, 2.4]),
        ]);
        let verdict = verdict(&a).expect("there should be a verdict");
        assert!(verdict.contains("`bun` wins"), "{verdict}");
        assert!(verdict.contains("against `katsu`"), "{verdict}");
    }

    #[test]
    fn an_overlapping_result_is_not_rounded_into_a_win() {
        let a = axis(vec![
            entry("katsu", &[10.0, 11.0, 12.0, 13.0, 14.0]),
            entry("node", &[11.0, 12.0, 13.0, 14.0, 15.0]),
        ]);
        let verdict = verdict(&a).expect("there should be a verdict");
        assert!(verdict.contains("Too close to call"), "{verdict}");
        assert!(!verdict.contains("wins"), "{verdict}");
    }

    #[test]
    fn a_runtime_that_could_not_be_measured_stays_in_the_table() {
        let mut failed = entry("deno", &[1.0]);
        failed.summary = None;
        failed.note = Some("not installed".into());

        let report = RunReport {
            machine: Machine {
                os: "test".into(),
                os_version: "0".into(),
                cpu: "test".into(),
                cores: 1,
                memory_mib: 1,
                ci: false,
            },
            axes: vec![axis(vec![entry("katsu", &[1.0, 1.1, 1.2]), failed])],
        };

        let table = markdown(&report);
        assert!(
            table.contains("`deno`"),
            "an unmeasurable runtime must still appear"
        );
        assert!(table.contains("not measured"), "{table}");
        assert!(
            table.contains("could not be measured: not installed"),
            "{table}"
        );
    }

    #[test]
    fn a_ci_run_labels_itself_as_untrustworthy_for_comparison() {
        let report = RunReport {
            machine: Machine {
                os: "test".into(),
                os_version: "0".into(),
                cpu: "test".into(),
                cores: 1,
                memory_mib: 1,
                ci: true,
            },
            axes: vec![],
        };
        assert!(markdown(&report).contains("shared CI runner"));
    }
}
