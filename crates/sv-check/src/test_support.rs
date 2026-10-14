//! What the tests of this crate, and of the crates built on it, share. Not part of `sv`: no code
//! outside a test calls it (backlog 0501).

use std::path::{Path, PathBuf};
#[cfg(unix)]
use std::process::{Command, Stdio};

/// Writes `text` to `path` as a program, and makes it one that can be run, from a child process
/// rather than this one.
///
/// Written here, the file is open for writing for a moment, and any other test that starts a
/// program in that moment hands the open file to its child until the child's own program begins.
/// Running the script then fails with "text file busy", which `presence` reads as a tool that is
/// not installed. CI met it on 9 October 2026 (`a_tool_that_does_not_say_its_version_is_broken_
/// not_waited_for` read "Missing"); eight threads writing and running a script 150 times each met
/// it 45 times in 1,200 written here, and never written by a child.
#[cfg(unix)]
pub fn executable(path: &Path, text: &str) {
    use std::io::Write;
    let mut child = Command::new("sh")
        .args(["-c", "cat > \"$1\" && chmod 755 \"$1\"", "sh"])
        .arg(path)
        .stdin(Stdio::piped())
        .spawn()
        .expect("sh writes the script");
    child
        .stdin
        .take()
        .expect("its input")
        .write_all(text.as_bytes())
        .expect("the script is handed over");
    assert!(
        child.wait().expect("sh finishes").success(),
        "the script was not written: {}",
        path.display()
    );
}

/// Elsewhere a file runs by what it is named and what runs it, not by a permission, and the race is
/// a Unix one (a child keeps what its parent had open until its own program begins), so the file is
/// written here, as the tests did before this helper.
#[cfg(not(unix))]
pub fn executable(path: &Path, text: &str) {
    std::fs::write(path, text).expect("the script is written");
}

/// A scratch folder for one test, empty and made: the system's temporary folder, then `sv-`, the calling file's name,
/// `name`, this run's process id, and a number counted up on every call, so two tests, two runs, and two calls never
/// share one (backlog 0020, which made the names unique per run; backlog 0462, which wrote it once instead of in each
/// test file). A folder of that name left by an earlier run is removed first. Nothing removes it afterward: a failing
/// test leaves its files to be read.
#[track_caller]
pub fn scratch(name: &str) -> PathBuf {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let file = Path::new(std::panic::Location::caller().file())
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    let n = NEXT.fetch_add(1, Ordering::Relaxed);
    let label = if name.is_empty() {
        file
    } else {
        format!("{file}-{name}")
    };
    let dir = std::env::temp_dir().join(format!("sv-{label}-{}-{n}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).expect("a scratch folder can be made");
    dir
}

/// The status `compliance.md` gives requirement `id`: the second column of its row. Six of `sv-cli`'s tests each had
/// a copy until backlog 0462. A report with no row for `id` fails the test, naming it.
pub fn status_of<'a>(compliance: &'a str, id: &str) -> &'a str {
    let row = compliance
        .lines()
        .find(|line| line.starts_with(&format!("| {id} |")))
        .unwrap_or_else(|| panic!("no row for {id} in the report:\n{compliance}"));
    row.split('|').nth(2).unwrap_or("").trim()
}

#[cfg(test)]
#[path = "test_support/scratch_tests.rs"]
mod scratch_tests;

#[cfg(test)]
mod tests;
