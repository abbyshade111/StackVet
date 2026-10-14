//! The outside tools run with every `sv report` unless `--no-tools` is given (ADR-111, backlog 0394). Each test takes
//! away `SV_NO_TOOLS_FOR_TEST`, which `.cargo/config.toml` sets for every other test, and gives `sv` a PATH with no
//! tool on it, so "tried" shows as each tool named as not installed, and nothing reaches semgrep.dev.

use std::path::{Path, PathBuf};
use std::process::Command;

const SV: &str = env!("CARGO_BIN_EXE_sv");

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-tools-default-{tag}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(dir.join("bin")).unwrap();
    dir
}

/// `sv report` on the Flask example with `extra`, nothing on the PATH, and the test switch taken away.
fn report(dir: &Path, extra: &[&str]) -> (Option<i32>, String, String) {
    let app = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/flask-booking");
    let out = Command::new(SV)
        .arg("report")
        .arg(&app)
        .arg("--out")
        .arg(dir.join("report"))
        .args(extra)
        .env("PATH", dir.join("bin"))
        .env_remove("SV_NO_TOOLS_FOR_TEST")
        .output()
        .unwrap();
    (
        out.status.code(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn the_switch_is_set_for_the_tests() {
    // The setup the rest of the tests rest on: without it, a computer with Semgrep installed would run it in every
    // test that writes a report.
    assert!(
        std::env::var_os("SV_NO_TOOLS_FOR_TEST").is_some(),
        ".cargo/config.toml no longer sets SV_NO_TOOLS_FOR_TEST"
    );
}

#[test]
fn a_plain_report_tries_every_tool_for_the_apps_languages() {
    let dir = scratch("plain");
    let (code, said, err) = report(&dir, &[]);
    let compliance = std::fs::read_to_string(dir.join("report/compliance.md")).unwrap_or_default();
    std::fs::remove_dir_all(&dir).ok();
    assert_eq!(
        code,
        Some(0),
        "a tool not installed is not a failure by default: {said}{err}"
    );
    for tool in ["bandit", "semgrep"] {
        assert!(
            said.contains(&format!("{tool} did not run")),
            "{tool}: {said}"
        );
    }
    // CodeQL runs only when stackvet.toml turns it on, and the example does not (ADR-115).
    assert!(!said.contains("codeql"), "{said}");
    assert!(!compliance.contains("--no-tools was given"), "{compliance}");
    // Nothing on screen says the tools were asked for: they were not, and still ran.
    assert!(
        said.contains("Not everything this run tried could run"),
        "{said}"
    );
    assert!(
        err.contains("Running the outside tools, unless --no-tools"),
        "{err}"
    );
    for asked in ["Asked for", "when asked with --tools"] {
        assert!(
            !said.contains(asked) && !err.contains(asked),
            "{asked}: {said}{err}"
        );
    }
}

#[test]
fn no_tools_runs_none_and_the_report_says_why() {
    let dir = scratch("none");
    let (code, said, err) = report(&dir, &["--no-tools"]);
    let compliance = std::fs::read_to_string(dir.join("report/compliance.md")).unwrap_or_default();
    std::fs::remove_dir_all(&dir).ok();
    assert_eq!(code, Some(0), "{said}{err}");
    assert!(!said.contains("did not run"), "{said}");
    assert!(compliance.contains("--no-tools was given"), "{compliance}");
}

#[test]
fn tools_is_still_accepted_and_both_together_are_refused() {
    let dir = scratch("both");
    let (code, said, _) = report(&dir, &["--tools"]);
    assert_eq!(code, Some(0), "{said}");
    assert!(said.contains("bandit did not run"), "{said}");
    let (code, _, err) = report(&dir, &["--tools", "--no-tools"]);
    std::fs::remove_dir_all(&dir).ok();
    assert_eq!(code, Some(3), "{err}");
    assert!(err.contains("say opposite things"), "{err}");
}
