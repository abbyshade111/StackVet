//! What `sv check` says it could not read, before what it found (backlog 0324): a file that is not text, a link not
//! followed, a file in a language the rules read that was not opened, and SQL the code rules do not read. Only the SQL
//! line was held by a test of `sv check` before `cmd_check` handed this to a function of its own.

use std::path::{Path, PathBuf};
use std::process::Command;

fn copy(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = to.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), target).unwrap();
        }
    }
}

/// The example app with a file that is not text, a link, and a SQL file beside its code.
fn odd_app(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-check-not-read-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    copy(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/flask-booking"),
        &dir,
    );
    std::fs::write(dir.join("broken.py"), b"\xff\xfe\x00bad = 1\n").unwrap();
    std::fs::write(dir.join("schema.sql"), "SELECT * FROM users;\n").unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink("/etc/hosts", dir.join("hosts-link")).unwrap();
    dir
}

#[test]
fn what_could_not_be_read_is_said_before_the_findings() {
    let app = odd_app("said");
    let out = Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(["check"])
        .arg(&app)
        .env("HOME", app.join("no-home"))
        .output()
        .unwrap();
    std::fs::remove_dir_all(&app).ok();
    let said = String::from_utf8_lossy(&out.stdout);
    // The setup: sv read the app and finished as a check does.
    assert!(
        out.status.code().is_some_and(|c| c < 3),
        "{said}\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        said.contains("Read ") && said.contains("files looking for credentials"),
        "{said}"
    );
    assert!(
        said.contains(
            "1 file was not read, so nothing is claimed about it:\n  broken.py — not a text file"
        ),
        "{said}"
    );
    assert!(
        said.contains("Not read — a file in a language the rules read was not opened"),
        "{said}"
    );
    assert!(
        said.contains("Not read by the rules that read code: `schema.sql` (1 SQL file)."),
        "{said}"
    );
    #[cfg(unix)]
    assert!(
        said.contains(
            "1 symbolic link was not followed, so whatever it points at was not read:\n  hosts-link"
        ),
        "{said}"
    );
    // Before what was found, never after it.
    let not_read = said.find("was not read, so nothing is claimed").unwrap();
    for later in ["Not assessed", "finding"] {
        if let Some(at) = said.find(later) {
            assert!(
                not_read < at,
                "`{later}` came before what was not read: {said}"
            );
        }
    }
}
