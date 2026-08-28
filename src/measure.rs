//! Running one process and recording what it cost.
//!
//! Memory is measured from outside the process. A runtime's self reported heap size is not
//! what a container's memory limit counts, and the number that matters to somebody paying
//! for a container is the resident set the kernel sees.

use std::io::Read as _;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};

/// What one run of one process cost.
#[derive(Clone, Debug)]
pub struct Measurement {
    /// Wall clock from spawn to exit, in milliseconds.
    pub wall_ms: f64,
    /// Peak resident set size of the child, in bytes.
    pub peak_rss_bytes: u64,
    /// Whether the process exited zero.
    pub succeeded: bool,
    /// Everything the child wrote to standard output.
    pub stdout: String,
    /// Everything the child wrote to standard error, which is where a runtime puts the stack
    /// trace that explains why it could not run the workload.
    pub stderr: String,
}

impl Measurement {
    /// The first line of standard error, for putting a failure in a table cell.
    pub fn first_error_line(&self) -> String {
        self.stderr
            .lines()
            .map(str::trim)
            .find(|line| !line.is_empty())
            .unwrap_or("no output on standard error")
            .to_owned()
    }
}

/// Run a process to completion and measure it.
///
/// The child's peak resident set comes from `wait4`, which is the kernel's own accounting
/// rather than a sample taken from outside at whatever moment we happened to look. Polling
/// cannot see the peak of a process that lives for eight milliseconds.
///
/// Both output streams are drained, and they are drained on two threads. A single thread reading
/// one pipe to the end while the other fills up is a deadlock, and it is the kind that only shows
/// up once a runtime starts printing more than a pipe buffer's worth of deprecation warnings.
pub fn run_once(program: &Path, args: &[&str], cwd: &Path) -> Result<Measurement> {
    let started = Instant::now();

    let mut child = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("spawning {}", program.display()))?;

    #[allow(clippy::cast_possible_wrap)]
    let pid = child.id() as libc::pid_t;

    let mut out = child.stdout.take().context("stdout was piped")?;
    let mut err = child.stderr.take().context("stderr was piped")?;
    let draining_stderr = std::thread::spawn(move || {
        let mut text = String::new();
        let _ = err.read_to_string(&mut text);
        text
    });
    let mut stdout = String::new();
    let _ = out.read_to_string(&mut stdout);
    let stderr = draining_stderr.join().unwrap_or_default();

    let mut status: libc::c_int = 0;
    // SAFETY: `rusage` is a plain C struct of integers with no invalid bit patterns, so an
    // all zero value is a valid initial state. `wait4` overwrites it before we read it.
    let mut usage: libc::rusage = unsafe { std::mem::zeroed() };

    // SAFETY: `pid` names a child we just spawned and have not reaped. `status` and `usage`
    // are live, correctly typed, exclusively borrowed locals. We reap the child here rather
    // than through `Child::wait`, and `Child`'s Drop on Unix neither waits nor kills, so
    // there is no double reap. `child` is not waited on anywhere after this point.
    let reaped = unsafe { libc::wait4(pid, &raw mut status, 0, &raw mut usage) };
    let wall = started.elapsed();
    drop(child);

    if reaped < 0 {
        bail!(
            "wait4 on {} failed: {}",
            program.display(),
            std::io::Error::last_os_error()
        );
    }

    Ok(Measurement {
        wall_ms: duration_ms(wall),
        peak_rss_bytes: max_rss_bytes(&usage),
        succeeded: exited_zero(status),
        stdout,
        stderr,
    })
}

/// Start a process, let it settle, sample its resident set, then stop it.
///
/// This is the measurement for a program that does not exit, such as an idle server. The
/// settle time matters: sampling immediately after spawn measures startup rather than
/// idle, and the two are different numbers that get confused constantly.
pub fn run_until_idle(
    program: &Path,
    args: &[&str],
    cwd: &Path,
    settle: Duration,
) -> Result<Measurement> {
    let started = Instant::now();

    let child = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .stdin(Stdio::null())
        // Both streams discarded rather than piped. Nobody is reading them here, and a child
        // that fills an unread pipe blocks on the write, which would be recorded as the memory
        // of a process sitting idle when it is really the memory of a process stuck.
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .with_context(|| format!("spawning {}", program.display()))?;

    #[allow(clippy::cast_possible_wrap)]
    let pid = child.id() as libc::pid_t;

    std::thread::sleep(settle);

    // Whether it is still running, asked before the sample is taken.
    //
    // A process that has already exited still reaps successfully and still has a `ru_maxrss`,
    // so without this check a runtime that cannot run the workload at all is recorded as using
    // no memory while idle, which reads as the best result in the table. That is precisely
    // backwards, and it is the sort of flattering nonsense this repository exists not to print.
    let mut early: libc::c_int = 0;
    // SAFETY: `pid` is our own child and has not been reaped. `WNOHANG` makes this return
    // immediately with zero if the child is still running, and `early` is a live local.
    let exited = unsafe { libc::waitpid(pid, &raw mut early, libc::WNOHANG) };
    if exited == pid {
        drop(child);
        bail!(
            "{} exited after {:.0} ms instead of staying idle, so there was no idle to measure",
            program.display(),
            duration_ms(started.elapsed())
        );
    }

    let sampled = sample_rss(pid);

    // SAFETY: `pid` is our own child, still unreaped, so the pid cannot have been recycled.
    unsafe { libc::kill(pid, libc::SIGTERM) };

    let mut status: libc::c_int = 0;
    // SAFETY: as in `run_once`. Zeroed rusage is valid, and we own the reap.
    let mut usage: libc::rusage = unsafe { std::mem::zeroed() };
    // SAFETY: as in `run_once`.
    let reaped = unsafe { libc::wait4(pid, &raw mut status, 0, &raw mut usage) };
    drop(child);

    if reaped < 0 {
        bail!(
            "wait4 on {} failed: {}",
            program.display(),
            std::io::Error::last_os_error()
        );
    }

    // Prefer the sample taken while the process was genuinely idle. Peak resident set over
    // the whole lifetime includes the startup spike, and reporting that as an idle figure
    // would overstate every runtime including our own.
    let resident = sampled.unwrap_or_else(|| max_rss_bytes(&usage));

    Ok(Measurement {
        wall_ms: duration_ms(started.elapsed()),
        peak_rss_bytes: resident,
        succeeded: true,
        // Not collected here. This process is killed rather than allowed to finish, so whatever
        // it had written so far says nothing about whether it worked.
        stdout: String::new(),
        stderr: String::new(),
    })
}

/// The size of a runtime's executable plus every non system library it loads to start.
///
/// Measuring the executable alone gives a wrong answer often enough to be worthless. A
/// Homebrew `node` is a fifty kilobyte launcher in front of a `libnode` dylib that holds the
/// entire engine, so an executable only measurement reports it as three thousand times
/// smaller than Deno, which is not true in any sense a reader would mean it.
///
/// So this counts the executable and every shared library it actually loads at startup,
/// excluding the ones the operating system ships. Those are excluded because every runtime
/// on the machine shares them, including ours, and counting them would measure the OS
/// instead of the runtime.
///
/// It is still not the installed footprint. A Node install also carries headers, npm and a
/// large `lib` tree, and the container image is larger again. The README says so next to the
/// number rather than letting the number imply more than it measures.
pub fn distribution_size(program: &Path) -> Result<u64> {
    let resolved = std::fs::canonicalize(program)
        .with_context(|| format!("resolving {}", program.display()))?;

    let mut total = file_size(&resolved)?;
    let mut counted = vec![resolved.clone()];

    for library in loaded_libraries(&resolved)? {
        if is_system_library(&library) {
            continue;
        }
        let Ok(canonical) = std::fs::canonicalize(&library) else {
            continue;
        };
        if counted.contains(&canonical) {
            continue;
        }
        if let Ok(size) = file_size(&canonical) {
            total += size;
            counted.push(canonical);
        }
    }

    Ok(total)
}

fn file_size(path: &Path) -> Result<u64> {
    Ok(std::fs::metadata(path)
        .with_context(|| format!("stat {}", path.display()))?
        .len())
}

/// Whether a library is part of the operating system rather than part of the runtime.
fn is_system_library(path: &Path) -> bool {
    const SYSTEM_PREFIXES: &[&str] = &[
        "/usr/lib",
        "/System/",
        "/lib/",
        "/lib64/",
        "/usr/lib64",
        "/usr/libexec",
    ];
    let text = path.to_string_lossy();
    SYSTEM_PREFIXES
        .iter()
        .any(|prefix| text.starts_with(prefix))
}

/// The shared libraries a program loads, as absolute paths.
///
/// On macOS this asks the dynamic linker to report what it actually loaded, rather than
/// reading the load commands with `otool` and then trying to resolve `@rpath` ourselves.
/// The linker already knows the answer and it is the answer that is true at runtime.
#[cfg(target_os = "macos")]
fn loaded_libraries(program: &Path) -> Result<Vec<std::path::PathBuf>> {
    let output = Command::new(program)
        .arg("--version")
        .env("DYLD_PRINT_LIBRARIES", "1")
        .stdin(Stdio::null())
        .output()
        .with_context(|| format!("running {} to list its libraries", program.display()))?;

    Ok(String::from_utf8_lossy(&output.stderr)
        .lines()
        .filter_map(|line| line.split_once(" /").map(|(_, rest)| rest))
        .map(|rest| std::path::PathBuf::from(format!("/{rest}")))
        .collect())
}

/// The shared libraries a program loads, as absolute paths.
///
/// This one cannot fail, because a missing or unhappy `ldd` is a correct answer of "no
/// shared libraries found here" rather than an error. It keeps the `Result` anyway so that
/// the signature matches the macOS version, which can genuinely fail, and so that the caller
/// does not need to know which platform it is on.
#[allow(clippy::unnecessary_wraps)]
#[cfg(not(target_os = "macos"))]
fn loaded_libraries(program: &Path) -> Result<Vec<std::path::PathBuf>> {
    // A statically linked runtime makes `ldd` exit non zero, and that is a correct answer of
    // "no shared libraries" rather than a failure, so the status is not checked.
    let Ok(output) = Command::new("ldd").arg(program).output() else {
        return Ok(Vec::new());
    };

    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| line.split_once("=> ").map(|(_, rest)| rest))
        .filter_map(|rest| rest.split_once(" (").map(|(path, _)| path.trim()))
        .filter(|path| path.starts_with('/'))
        .map(std::path::PathBuf::from)
        .collect())
}

fn duration_ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

fn exited_zero(status: libc::c_int) -> bool {
    libc::WIFEXITED(status) && libc::WEXITSTATUS(status) == 0
}

/// `ru_maxrss` in bytes.
///
/// The unit is not portable. macOS reports bytes and Linux reports kilobytes, and getting
/// this wrong produces a memory comparison that is off by a factor of 1024, which is the
/// kind of error that makes a benchmark repository worthless.
fn max_rss_bytes(usage: &libc::rusage) -> u64 {
    let raw = u64::try_from(usage.ru_maxrss).unwrap_or(0);
    if cfg!(target_os = "macos") {
        raw
    } else {
        raw * 1024
    }
}

/// Read a running process's current resident set, or `None` if it cannot be read here.
#[cfg(target_os = "linux")]
fn sample_rss(pid: libc::pid_t) -> Option<u64> {
    let statm = std::fs::read_to_string(format!("/proc/{pid}/statm")).ok()?;
    let resident_pages: u64 = statm.split_whitespace().nth(1)?.parse().ok()?;
    // SAFETY: `sysconf` takes an integer name, returns a long, and touches no memory.
    let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
    (page_size > 0).then(|| resident_pages * u64::try_from(page_size).unwrap_or(4096))
}

/// Read a running process's current resident set, or `None` if it cannot be read here.
#[cfg(not(target_os = "linux"))]
fn sample_rss(pid: libc::pid_t) -> Option<u64> {
    let output = std::process::Command::new("ps")
        .args(["-o", "rss=", "-p", &pid.to_string()])
        .output()
        .ok()?;
    let kilobytes: u64 = String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse()
        .ok()?;
    Some(kilobytes * 1024)
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::time::Duration;

    use super::{distribution_size, exited_zero, is_system_library, run_once, run_until_idle};

    #[test]
    fn a_successful_exit_is_recognised() {
        assert!(exited_zero(0));
    }

    #[test]
    fn a_failing_exit_status_is_recognised() {
        // Exit code 1 with no signal is 0x0100 in wait status encoding.
        assert!(!exited_zero(0x0100));
    }

    #[test]
    fn running_true_costs_almost_nothing_and_succeeds() {
        let measurement =
            run_once(Path::new("/usr/bin/true"), &[], Path::new(".")).expect("true should run");
        assert!(measurement.succeeded);
        assert!(measurement.wall_ms >= 0.0);
        assert!(
            measurement.peak_rss_bytes > 0,
            "the kernel should report some resident set"
        );
    }

    #[test]
    fn running_false_is_recorded_as_a_failure_rather_than_ignored() {
        let measurement =
            run_once(Path::new("/usr/bin/false"), &[], Path::new(".")).expect("false should run");
        assert!(!measurement.succeeded);
    }

    #[test]
    fn libraries_the_operating_system_ships_are_not_charged_to_the_runtime() {
        assert!(is_system_library(Path::new("/usr/lib/libSystem.B.dylib")));
        assert!(is_system_library(Path::new(
            "/lib/x86_64-linux-gnu/libc.so.6"
        )));
        assert!(!is_system_library(Path::new(
            "/opt/homebrew/Cellar/node/26.7.0/lib/libnode.147.dylib"
        )));
    }

    #[test]
    fn a_launcher_in_front_of_a_dylib_is_not_reported_as_a_tiny_runtime() {
        let Some(node) = crate::runtime::which("node") else {
            return;
        };
        let executable_only = std::fs::metadata(std::fs::canonicalize(&node).expect("node path"))
            .expect("stat node")
            .len();
        let whole = distribution_size(&node).expect("node should be measurable");
        assert!(
            whole >= executable_only,
            "the distribution cannot be smaller than the executable inside it"
        );
        assert!(
            whole > 8 * 1024 * 1024,
            "a JavaScript engine does not fit in 8 MiB, so {whole} bytes means the linked \
             libraries were missed"
        );
    }

    #[test]
    fn a_process_that_died_early_is_an_error_and_not_a_runtime_that_uses_no_memory() {
        let error = run_until_idle(
            Path::new("/usr/bin/true"),
            &[],
            Path::new("."),
            Duration::from_millis(150),
        )
        .expect_err("a process that exits immediately has no idle to measure");
        assert!(
            format!("{error}").contains("instead of staying idle"),
            "{error}"
        );
    }

    #[test]
    fn a_process_that_does_not_exit_is_sampled_and_then_stopped() {
        let measurement = run_until_idle(
            Path::new("/bin/sleep"),
            &["30"],
            Path::new("."),
            Duration::from_millis(150),
        )
        .expect("sleep should run");
        assert!(measurement.succeeded);
        assert!(
            measurement.wall_ms >= 150.0,
            "it should have been left alone to settle"
        );
        assert!(
            measurement.wall_ms < 5000.0,
            "it should have been stopped, not waited out"
        );
    }
}
