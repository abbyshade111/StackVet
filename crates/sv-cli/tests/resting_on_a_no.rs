//! What compliance.md says when stackvet.toml answers `false` to a capability nobody checked (backlog 0600): the
//! requirements that do not apply on that word alone are counted beside the counts, with the answers named, and
//! each capability answer says who gave it.

use std::path::PathBuf;
use std::process::Command;

/// Runs `sv report` on a one-file app with this `[capabilities]` section, and returns compliance.md.
fn compliance_with(name: &str, capabilities: &str) -> String {
    let dir: PathBuf =
        std::env::temp_dir().join(format!("sv-on-a-no-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("app.py"), "def home():\n    return 'hi'\n").unwrap();
    std::fs::write(
        dir.join("stackvet.toml"),
        format!(
            "manifest-version = 1\n\n[app]\nname = \"Notes\"\naudience = \"just-me\"\n\
             deployment = \"local-only\"\n\n[stack]\nlanguages = [\"python\"]\n\n\
             [capabilities]\n{capabilities}"
        ),
    )
    .unwrap();
    let out_dir = dir.join("report");
    let out = Command::new(env!("CARGO_BIN_EXE_sv"))
        .arg("report")
        .arg(&dir)
        .arg("--out")
        .arg(&out_dir)
        .output()
        .expect("sv runs");
    let compliance = std::fs::read_to_string(out_dir.join("compliance.md")).unwrap_or_default();
    std::fs::remove_dir_all(&dir).ok();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(!compliance.is_empty(), "the report was not written");
    compliance
}

/// The number at the start of the line that counts them, if there is one.
fn counted(compliance: &str) -> Option<usize> {
    let line = compliance
        .lines()
        .find(|l| l.contains("not apply only because stackvet.toml says no to"))?;
    line.split(' ').next()?.parse().ok()
}

#[test]
fn the_requirements_resting_on_a_no_are_counted_and_the_answers_named() {
    // `deployment = "local-only"` is a no to `internet` in both; only one says no to `uploads`.
    let silent = compliance_with("silent", "");
    let no = compliance_with("no", "uploads = false\n");
    let before = counted(&silent).unwrap_or(0);
    let after = counted(&no).unwrap_or_else(|| panic!("no line counts them:\n{no}"));
    assert!(
        after > before,
        "`uploads = false` rules out requirements on the file's word: {before} then {after}\n{no}"
    );
    let line = no.lines().find(|l| l.contains("says no to")).unwrap();
    assert!(line.contains("`uploads`"), "{line}");
    assert!(line.contains("`sv` did not check"), "{line}");
    assert!(
        !silent
            .lines()
            .any(|l| l.contains("says no to") && l.contains("`uploads`")),
        "an unanswered question is not a no:\n{silent}"
    );

    // Each answer says who gave it.
    assert!(
        no.contains("| about | stackvet.toml | who said it | the code | what that means |"),
        "{no}"
    );
    let uploads = no
        .lines()
        .find(|l| l.starts_with("| uploads |"))
        .unwrap_or_else(|| panic!("no uploads line:\n{no}"));
    assert!(
        uploads.contains("| no | not recorded, so read as your AI coding tool's |"),
        "{uploads}"
    );
    let silent_uploads = silent
        .lines()
        .find(|l| l.starts_with("| uploads |"))
        .unwrap_or_else(|| panic!("no uploads line:\n{silent}"));
    assert!(
        silent_uploads.contains("| nothing | nobody |"),
        "{silent_uploads}"
    );
}
