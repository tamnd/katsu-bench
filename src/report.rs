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

/// What else the machine was doing while the numbers were being taken.
///
/// Sampled twice rather than once, because a run takes twenty minutes and the interesting
/// question is not what the load was at any single instant but whether the machine stayed the
/// same machine from the first measurement to the last. A run that starts quiet and ends busy
/// has measured the last runtime under conditions the first one never saw.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Load {
    /// One minute load average when the run started.
    pub start: f64,
    /// One minute load average when the run finished.
    pub end: f64,
}

impl Load {
    /// The one minute load average right now.
    pub fn one_minute() -> f64 {
        sysinfo::System::load_average().one
    }

    /// Whether this much load is enough to distort the numbers on a machine with this many cores.
    ///
    /// Half the cores is the line, and it is a judgement rather than a measured threshold. Below
    /// it there is a spare core for the runtime under test even at the worst moment, above it the
    /// runtime is competing for one, and the difference shows up as a longer wall clock for
    /// everybody rather than as a difference between runtimes.
    #[allow(clippy::cast_precision_loss)]
    fn heavy(self, cores: usize) -> bool {
        let line = cores as f64 / 2.0;
        self.start > line || self.end > line
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
    /// What this axis belongs with, when several axes describe one workload.
    ///
    /// The compute workloads produce four numbers each and all four are about the same run, so
    /// they are printed as four columns of one table rather than as four tables that a reader
    /// has to hold in their head at once. Anything with no group prints on its own.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// The column heading to use inside a group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
    /// The unit the numbers are in.
    pub unit: String,
    /// Whether smaller is better.
    pub lower_is_better: bool,
    /// One entry per runtime, in the order they were measured.
    pub runtimes: Vec<RuntimeResult>,
}

impl AxisResult {
    /// The column heading for this axis inside its group.
    fn column(&self) -> String {
        let metric = self.metric.as_deref().unwrap_or(&self.axis);
        format!("{metric} ({})", self.unit)
    }
}

/// A whole run.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RunReport {
    /// Where it was taken.
    pub machine: Machine,
    /// What else the machine was doing, absent in results taken before this was recorded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load: Option<Load>,
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
    if let Some(load) = report.load {
        let _ = writeln!(
            out,
            "Load average: {:.2} when the run started, {:.2} when it finished.",
            load.start, load.end
        );
        if load.heavy(machine.cores) {
            let _ = writeln!(out);
            let _ = writeln!(
                out,
                "**The machine was busy.** A one minute load average that high on {} cores means every runtime here was competing for a core with something else, so every absolute time in this report is inflated and none of them should be compared against a figure taken on a quiet machine. The ratios between runtimes survive better than the absolutes do, because the runtimes were interleaved and all of them paid the same tax, but they carry an error bar too. This paragraph appears when the load passes half the core count and it is the reason to move the published numbers onto dedicated hardware.",
                machine.cores
            );
        }
    }
    if machine.ci {
        let _ = writeln!(out);
        let _ = writeln!(
            out,
            "**Taken on a shared CI runner.** These numbers are good for spotting a trend and bad for comparing runtimes, because the machine is shared with whatever else the provider put on it. Published comparisons come from the dedicated hardware named in the README."
        );
    }

    write_scoreboard(&mut out, report);

    let mut rest = report.axes.as_slice();
    while let Some(first) = rest.first() {
        match &first.group {
            None => {
                write_single(&mut out, first);
                rest = &rest[1..];
            }
            Some(group) => {
                let end = rest
                    .iter()
                    .position(|axis| axis.group.as_deref() != Some(group.as_str()))
                    .unwrap_or(rest.len());
                write_group(&mut out, group, &rest[..end]);
                rest = &rest[end..];
            }
        }
    }

    out
}

/// The name of the runtime this repository exists to measure.
const OURS: &str = "katsu";

/// How far past the best rival the goal is.
const GOAL: f64 = 10.0;

/// The scoreboard against the goal, axis by axis.
///
/// This is the section the whole repository is for. Every axis gets one line saying where we
/// stand against the best rival on that axis and how much is left to find, including the axes
/// where we are behind and the axes where we cannot run the workload at all. An axis katsu
/// cannot run yet says exactly that, because the alternative is a scoreboard that gets shorter
/// every time we fail at something, which would climb towards the goal by forgetting.
fn write_scoreboard(out: &mut String, report: &RunReport) {
    let _ = writeln!(out);
    let _ = writeln!(out, "## Distance to the goal");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "The goal is ten times better than the best rival on every axis, not ten times better than the worst one. This table is regenerated from the same run as everything below it."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "| Axis | katsu | Best rival | Standing | Left to find |"
    );
    let _ = writeln!(out, "|---|---:|---|---|---|");

    for axis in &report.axes {
        let name = match (&axis.group, &axis.metric) {
            (Some(group), Some(metric)) => format!("{group}, {metric}"),
            _ => axis.axis.clone(),
        };

        let ours = axis.runtimes.iter().find(|r| r.runtime == OURS);
        let best_rival = axis
            .runtimes
            .iter()
            .filter(|r| r.runtime != OURS)
            .filter_map(|r| r.summary.map(|s| (r, s)))
            .min_by(|a, b| order(a.1.median, b.1.median, axis.lower_is_better));

        let Some((rival, rival_summary)) = best_rival else {
            let _ = writeln!(out, "| {name} ({}) | | no rival measured | | |", axis.unit);
            continue;
        };

        let rival_cell = format!("`{}` {:.2}", rival.runtime, rival_summary.median);

        match ours.and_then(|r| r.summary) {
            None => {
                let reason = ours
                    .and_then(|r| r.note.as_deref())
                    .map_or_else(|| "not installed".to_owned(), short_reason);
                let _ = writeln!(
                    out,
                    "| {name} ({}) | not yet | {rival_cell} | cannot run this yet | all of it. {reason} |",
                    axis.unit
                );
            }
            Some(mine) => {
                let ratio = if axis.lower_is_better {
                    rival_summary.median / mine.median
                } else {
                    mine.median / rival_summary.median
                };
                // A ratio against zero is infinite, and infinity in the standing column has
                // never once meant we were infinitely better at something. It means the
                // measurement did not measure anything, so it says that instead.
                if !ratio.is_finite() || ratio <= 0.0 {
                    let _ = writeln!(
                        out,
                        "| {name} ({}) | {:.2} | {rival_cell} | not comparable, one side measured zero | unknown |",
                        axis.unit, mine.median
                    );
                    continue;
                }
                let standing = if ratio >= 1.0 {
                    format!("{ratio:.2}x ahead")
                } else {
                    format!("{:.2}x behind", 1.0 / ratio)
                };
                let left = if ratio >= GOAL {
                    "goal reached".to_owned()
                } else {
                    format!("{:.1}x", GOAL / ratio)
                };
                let _ = writeln!(
                    out,
                    "| {name} ({}) | {:.2} | {rival_cell} | {standing} | {left} |",
                    axis.unit, mine.median
                );
            }
        }
    }

    write_scoreboard_caveat(out, report);
}

/// Say how much of the scoreboard katsu is not actually competing on.
///
/// Without this, a partly built runtime reads as winning. The axes it can win before it can run
/// a program are the ones that do not require running a program, and a reader looking at a row
/// saying `goal reached` deserves to be told in the same breath that most of the suite did not
/// run at all. This paragraph disappears on its own the day every workload runs.
fn write_scoreboard_caveat(out: &mut String, report: &RunReport) {
    let total = report.axes.len();
    let unrun = report
        .axes
        .iter()
        .filter(|axis| {
            axis.runtimes
                .iter()
                .find(|r| r.runtime == OURS)
                .is_none_or(|r| r.summary.is_none())
        })
        .count();

    if unrun == 0 {
        return;
    }

    let _ = writeln!(out);
    if unrun == total {
        let _ = writeln!(
            out,
            "**katsu ran none of the {total} axes in this report.** Everything above is a baseline for the rivals and nothing above is a result for us."
        );
    } else {
        let _ = writeln!(
            out,
            "**katsu ran {} of the {total} axes in this report.** The {unrun} it did not run are marked as such above. Every win here is a win on an axis where katsu is doing less work than the runtime it beat, because most of a runtime is not built yet, so read them as a starting position rather than as a result. This paragraph goes away when the number that did not run reaches zero.",
            total - unrun
        );
    }
}

/// The part of a failure worth putting in a table cell.
///
/// A runtime that cannot run a workload says so through several layers, each of which adds its
/// own prefix, so the message arrives as `katsu could not run fib: katsu: syntax error:
/// compute/fib.js:23:45: an object literal is not supported yet`. Everything before the last
/// colon is context the surrounding row already gives. What is left is the actual reason, and
/// the full message is still printed under the table it belongs to.
fn short_reason(note: &str) -> String {
    let line = note.lines().next().unwrap_or(note).trim();
    line.rsplit_once(": ")
        .map_or(line, |(_, tail)| tail)
        .to_owned()
}

/// Order two medians so that the better one sorts first.
fn order(left: f64, right: f64, lower_is_better: bool) -> std::cmp::Ordering {
    let ordering = left
        .partial_cmp(&right)
        .unwrap_or(std::cmp::Ordering::Equal);
    if lower_is_better {
        ordering
    } else {
        ordering.reverse()
    }
}

/// One axis on its own, with the full distribution rather than only the median.
fn write_single(out: &mut String, axis: &AxisResult) {
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

    write_notes(out, axis);

    if let Some(verdict) = verdict(axis) {
        let _ = writeln!(out);
        let _ = writeln!(out, "{verdict}");
    }
}

/// Several axes that describe one workload, as one table with a column each.
///
/// Medians only, because a table with four metrics and six distribution columns each is
/// forty numbers a row and nobody reads it. The full distributions are in the JSON.
fn write_group(out: &mut String, group: &str, axes: &[AxisResult]) {
    let Some(first) = axes.first() else {
        return;
    };

    let _ = writeln!(out);
    let _ = writeln!(out, "## {group}");
    let _ = writeln!(out);

    let _ = write!(out, "| Runtime | Version |");
    for axis in axes {
        let _ = write!(out, " {} |", axis.column());
    }
    let _ = writeln!(out);
    let _ = write!(out, "|---|---|");
    for _ in axes {
        let _ = write!(out, "---:|");
    }
    let _ = writeln!(out);

    for (index, entry) in first.runtimes.iter().enumerate() {
        let _ = write!(out, "| `{}` | {} |", entry.runtime, entry.version);
        for axis in axes {
            // Indexed rather than searched by name, because every axis in a group was filled in
            // by the same loop over the same runtimes and so holds them in the same order.
            match axis.runtimes.get(index).and_then(|r| r.summary) {
                Some(s) => {
                    let _ = write!(out, " {:.2} |", s.median);
                }
                None => {
                    let _ = write!(out, " not measured |");
                }
            }
        }
        let _ = writeln!(out);
    }

    write_notes(out, first);

    for axis in axes {
        if let Some(verdict) = verdict(axis) {
            let _ = writeln!(out);
            let _ = writeln!(
                out,
                "{}. {verdict}",
                axis.metric.as_deref().unwrap_or("This")
            );
        }
    }
}

/// Why any runtime in this axis could not be measured.
fn write_notes(out: &mut String, axis: &AxisResult) {
    for entry in &axis.runtimes {
        if let Some(note) = &entry.note {
            let _ = writeln!(out);
            let _ = writeln!(out, "`{}` could not be measured: {note}", entry.runtime);
        }
    }
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
    use super::{AxisResult, Load, Machine, RunReport, RuntimeResult, markdown, verdict};
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
            group: None,
            metric: None,
            unit: "ms".into(),
            lower_is_better: true,
            runtimes,
        }
    }

    fn grouped(name: &str, metric: &str, unit: &str, runtimes: Vec<RuntimeResult>) -> AxisResult {
        AxisResult {
            axis: format!("{name}, {metric}"),
            group: Some(format!("Compute: {name}")),
            metric: Some(metric.into()),
            unit: unit.into(),
            lower_is_better: true,
            runtimes,
        }
    }

    fn machine() -> Machine {
        Machine {
            os: "test".into(),
            os_version: "0".into(),
            cpu: "test".into(),
            cores: 1,
            memory_mib: 1,
            ci: false,
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
            machine: machine(),
            load: None,
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
        let mut on_ci = machine();
        on_ci.ci = true;
        let report = RunReport {
            machine: on_ci,
            load: None,
            axes: vec![],
        };
        assert!(markdown(&report).contains("shared CI runner"));
    }

    #[test]
    fn a_run_taken_on_a_busy_machine_says_so_and_a_quiet_one_does_not() {
        let mut quiet = machine();
        quiet.cores = 10;
        let mut busy = quiet.clone();
        busy.cores = 10;

        let report = RunReport {
            machine: quiet,
            load: Some(Load {
                start: 0.4,
                end: 1.2,
            }),
            axes: vec![],
        };
        let table = markdown(&report);
        assert!(
            table.contains("Load average: 0.40 when the run started, 1.20 when it finished."),
            "{table}"
        );
        assert!(!table.contains("machine was busy"), "{table}");

        let report = RunReport {
            machine: busy,
            // Quiet at the start and busy at the end, which is the case the warning exists for.
            // A run that only reports one load average would call this one clean and it is not.
            load: Some(Load {
                start: 0.4,
                end: 7.1,
            }),
            axes: vec![],
        };
        assert!(markdown(&report).contains("machine was busy"));
    }

    #[test]
    fn the_metrics_of_one_workload_are_one_table_with_a_column_each() {
        let report = RunReport {
            machine: machine(),
            load: None,
            axes: vec![
                grouped(
                    "fib",
                    "In process",
                    "ms",
                    vec![entry("node", &[60.0, 61.0]), entry("bun", &[45.0, 46.0])],
                ),
                grouped(
                    "fib",
                    "Peak memory",
                    "MiB",
                    vec![entry("node", &[52.0, 53.0]), entry("bun", &[70.0, 71.0])],
                ),
            ],
        };

        let table = markdown(&report);
        assert!(table.contains("## Compute: fib"), "{table}");
        assert!(table.contains("In process (ms)"), "{table}");
        assert!(table.contains("Peak memory (MiB)"), "{table}");
        assert_eq!(
            table.matches("## Compute: fib").count(),
            1,
            "the two metrics belong in one table, not two"
        );
        // One row per runtime, holding both metrics, so the row must carry both medians.
        assert!(
            table.contains("| `bun` | 1.0.0 | 45.50 | 70.50 |"),
            "{table}"
        );
    }

    #[test]
    fn the_scoreboard_says_plainly_when_we_cannot_run_the_workload_at_all() {
        let mut ours = entry("katsu", &[1.0]);
        ours.summary = None;
        ours.note = Some("katsu could not run fib: call is not implemented yet".into());

        let report = RunReport {
            machine: machine(),
            load: None,
            axes: vec![grouped(
                "fib",
                "In process",
                "ms",
                vec![ours, entry("node", &[60.0, 61.0])],
            )],
        };

        let table = markdown(&report);
        assert!(table.contains("Distance to the goal"), "{table}");
        assert!(table.contains("cannot run this yet"), "{table}");
        assert!(table.contains("call is not implemented yet"), "{table}");
    }

    #[test]
    fn a_failure_is_shortened_to_the_part_the_row_does_not_already_say() {
        assert_eq!(
            super::short_reason(
                "katsu could not run fib: katsu: syntax error: compute/fib.js:23:45: an object literal is not supported yet"
            ),
            "an object literal is not supported yet"
        );
        // Nothing to strip, so nothing is stripped rather than the whole message vanishing.
        assert_eq!(super::short_reason("not installed"), "not installed");
    }

    #[test]
    fn the_scoreboard_measures_us_against_the_best_rival_and_not_the_worst() {
        let report = RunReport {
            machine: machine(),
            load: None,
            axes: vec![axis(vec![
                entry("katsu", &[5.0, 5.0, 5.0]),
                entry("bun", &[10.0, 10.0, 10.0]),
                entry("node", &[100.0, 100.0, 100.0]),
            ])],
        };

        let table = markdown(&report);
        assert!(
            table.contains("`bun` 10.00"),
            "the best rival is bun: {table}"
        );
        assert!(table.contains("2.00x ahead"), "{table}");
        // Ten times the best rival is the goal, so being twice as fast leaves five times to
        // find, not twenty times as the comparison against node would have suggested.
        assert!(table.contains("| 5.0x |"), "{table}");
    }

    #[test]
    fn the_scoreboard_states_a_loss_as_a_loss() {
        let report = RunReport {
            machine: machine(),
            load: None,
            axes: vec![axis(vec![
                entry("katsu", &[40.0, 40.0, 40.0]),
                entry("node", &[10.0, 10.0, 10.0]),
            ])],
        };
        assert!(markdown(&report).contains("4.00x behind"));
    }
}
