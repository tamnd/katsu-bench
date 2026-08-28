//! Finding the runtimes on this machine and asking each what version it is.

use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};

/// One runtime under test.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Runtime {
    /// Short name, for example `node`.
    pub name: String,
    /// Resolved path to the executable.
    pub path: PathBuf,
    /// Whatever it printed for `--version`, cleaned up.
    pub version: String,
}

/// The runtimes we compare against, in the order the tables list them.
///
/// Node is first because it is the reference and the thing the claim is about. Bun is the
/// strongest competitor on startup and memory, which is the axis we are loudest about, so
/// leaving it out would be cowardice.
pub const CANDIDATES: &[&str] = &["katsu", "node", "bun", "deno"];

/// Resolve a program on `PATH`.
pub fn which(program: &str) -> Option<PathBuf> {
    if program.contains('/') {
        let path = Path::new(program);
        return path.is_file().then(|| path.to_path_buf());
    }
    std::env::split_paths(&std::env::var_os("PATH")?)
        .map(|dir| dir.join(program))
        .find(|candidate| candidate.is_file())
}

/// Resolve one runtime, or `None` if it is not installed here.
pub fn resolve(name: &str) -> Option<Runtime> {
    let path = which(name)?;
    let output = Command::new(&path).arg("--version").output().ok()?;
    let raw = String::from_utf8_lossy(&output.stdout);
    // Deno prints three lines and puts its build triple on the first one, Node prefixes a
    // `v`, Bun prints the bare number. Take the first line, drop the leading program name,
    // and keep the first token, so the column stays a version rather than a paragraph.
    let version = raw
        .lines()
        .next()
        .unwrap_or_default()
        .trim()
        .trim_start_matches(|c: char| c.is_alphabetic() || c.is_whitespace())
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .to_owned();

    Some(Runtime {
        name: name.to_owned(),
        path,
        version: if version.is_empty() {
            "unknown".into()
        } else {
            version
        },
    })
}

/// Every candidate runtime that is actually installed.
///
/// A runtime that is missing is left out of the table rather than reported as a zero or a
/// win by default. An absent competitor is not a beaten competitor.
pub fn discover(only: Option<&[String]>) -> Vec<Runtime> {
    let wanted: Vec<String> = match only {
        Some(list) if !list.is_empty() => list.to_vec(),
        _ => CANDIDATES.iter().map(|s| (*s).to_string()).collect(),
    };
    wanted.iter().filter_map(|name| resolve(name)).collect()
}

#[cfg(test)]
mod tests {
    use super::{CANDIDATES, discover, which};

    #[test]
    fn node_is_the_first_reference_after_ourselves() {
        assert_eq!(CANDIDATES[0], "katsu");
        assert_eq!(CANDIDATES[1], "node");
    }

    #[test]
    fn a_program_that_does_not_exist_resolves_to_nothing() {
        assert!(which("definitely-not-a-real-runtime-binary").is_none());
    }

    #[test]
    fn discovery_leaves_out_what_is_not_installed_rather_than_inventing_it() {
        let found = discover(Some(&["definitely-not-a-real-runtime-binary".to_string()]));
        assert!(found.is_empty());
    }
}
