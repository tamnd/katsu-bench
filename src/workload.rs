//! The compute workloads and the line each one prints when it is done.
//!
//! A workload measures itself. It calls `performance.now()` either side of the work and prints
//! one line saying how long the work took and what it computed, and the harness reads that line
//! back. Doing it this way rather than timing the process from outside is the whole reason the
//! compute axis exists, because the outside number for a program that runs for eighty
//! milliseconds is mostly the runtime starting up, and the two costs need to be separable.
//!
//! The checksum is not decoration either. Every runtime is asked to produce the same one, and a
//! runtime that disagrees is reported as wrong rather than as fast. That catches the two failure
//! modes a benchmark suite dies of: a workload whose result depends on floating point details or
//! on iteration order, and an optimiser that deleted the work because nothing observed it.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

/// The prefix a workload prints in front of its result line.
///
/// A prefix rather than the whole of stdout, because a runtime is allowed to print warnings and
/// deprecation notices on its way through and we should not care that it did.
const MARKER: &str = "katsu-bench ";

/// One compute workload.
#[derive(Clone, Debug)]
pub struct Workload {
    /// Short name, taken from the file name.
    pub name: String,
    /// Path to the script.
    pub path: PathBuf,
}

/// What a workload reported about itself.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reported {
    /// The name the workload calls itself, checked against the file name.
    pub workload: String,
    /// Milliseconds spent inside the work, by the runtime's own clock.
    pub compute_ms: f64,
    /// The value the work computed, as a string so that a float and an integer compare the same
    /// way and neither of them goes through a second float parse to be checked.
    pub checksum: String,
}

/// Every compute workload in a directory, in a stable order.
///
/// Sorted by name, because the order axes appear in the published table should not depend on
/// what order the filesystem happened to hand the entries back in.
pub fn compute_workloads(dir: &Path) -> Result<Vec<Workload>> {
    if !dir.is_dir() {
        bail!("no compute workloads at {}", dir.display());
    }

    let mut found = Vec::new();
    for entry in std::fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let path = entry?.path();
        if path.extension().is_none_or(|ext| ext != "js") {
            continue;
        }
        let Some(name) = path.file_stem().map(|s| s.to_string_lossy().into_owned()) else {
            continue;
        };
        found.push(Workload { name, path });
    }

    found.sort_by(|a, b| a.name.cmp(&b.name));
    if found.is_empty() {
        bail!("no .js workloads in {}", dir.display());
    }
    Ok(found)
}

/// Read a workload's result line out of whatever it printed.
///
/// Takes the last marked line rather than the first, so that a runtime which prints a banner
/// containing the marker cannot shadow the real result.
pub fn parse_report(stdout: &str) -> Result<Reported> {
    let line = stdout
        .lines()
        .filter_map(|line| line.trim().strip_prefix(MARKER))
        .next_back()
        .context("the workload printed no result line, so it did not reach the end")?;

    serde_json::from_str(line).with_context(|| format!("the result line is not valid JSON: {line}"))
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{compute_workloads, parse_report};

    #[test]
    fn a_result_line_is_read_back_out_of_ordinary_output() {
        let report = parse_report(
            "some warning from the runtime\n\
             katsu-bench {\"workload\":\"fib\",\"compute_ms\":12.5,\"checksum\":\"9227465\"}\n",
        )
        .expect("the line should parse");
        assert_eq!(report.workload, "fib");
        assert!((report.compute_ms - 12.5).abs() < 1e-9);
        assert_eq!(report.checksum, "9227465");
    }

    #[test]
    fn the_last_marked_line_wins_so_a_banner_cannot_shadow_the_result() {
        let report = parse_report(
            "katsu-bench {\"workload\":\"banner\",\"compute_ms\":0.0,\"checksum\":\"0\"}\n\
             katsu-bench {\"workload\":\"fib\",\"compute_ms\":12.5,\"checksum\":\"9227465\"}\n",
        )
        .expect("the line should parse");
        assert_eq!(report.workload, "fib");
    }

    #[test]
    fn a_workload_that_died_before_printing_is_an_error_and_not_a_zero() {
        let error = parse_report("TypeError: undefined is not a function\n")
            .expect_err("there is no result here");
        assert!(format!("{error}").contains("no result line"));
    }

    #[test]
    fn every_shipped_workload_is_found_and_they_come_back_in_a_stable_order() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("workloads")
            .join("compute");
        let found = compute_workloads(&dir).expect("the workloads ship with the repository");
        let names: Vec<&str> = found.iter().map(|w| w.name.as_str()).collect();
        assert_eq!(names, ["alloc", "fib", "json", "nbody", "sort", "strings"]);
    }

    #[test]
    fn a_missing_directory_is_an_error_rather_than_an_empty_suite() {
        assert!(compute_workloads(Path::new("/definitely/not/here")).is_err());
    }
}
