//! What the AI coding tool did that is worth a look, end to end (backlog 0630, ADR-131): a first report, made without
//! the hook record, keeps the commit it was made at and reads none of the record; a second, after a commit an AI
//! coding tool made and with the person's consent to the record, reads the commits since the first and the record's
//! commands and writes.

use serde_json::Value;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use sv_frameworks::paths::Canonical;

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

struct Place {
    scratch: PathBuf,
    app: PathBuf,
    home: PathBuf,
}

impl Place {
    fn new(name: &str) -> Place {
        let scratch =
            std::env::temp_dir().join(format!("sv-tool-behavior-{name}-{}", std::process::id()));
        std::fs::remove_dir_all(&scratch).ok();
        let app = scratch.join("app");
        copy(
            &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/flask-booking"),
            &app,
        );
        let home = scratch.join("home");
        std::fs::create_dir_all(&home).unwrap();
        Place {
            scratch,
            app: app.canonical().unwrap(),
            home,
        }
    }

    fn sv(&self, args: &[&str]) -> Output {
        self.run(Command::new(env!("CARGO_BIN_EXE_sv")).args(args), None)
    }

    fn git(&self, args: &[&str]) {
        let out = self.run(
            Command::new("git")
                .args([
                    "-c",
                    "user.email=t@t",
                    "-c",
                    "user.name=t",
                    "-c",
                    "commit.gpgsign=false",
                ])
                .args(args),
            None,
        );
        assert!(
            out.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }

    fn run(&self, command: &mut Command, stdin: Option<&str>) -> Output {
        let mut child = command
            .current_dir(&self.app)
            .env("HOME", &self.home)
            .env("XDG_STATE_HOME", self.home.join("state"))
            .env("XDG_CONFIG_HOME", self.home.join(".config"))
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("it runs");
        child
            .stdin
            .take()
            .unwrap()
            .write_all(stdin.unwrap_or("").as_bytes())
            .unwrap();
        child.wait_with_output().unwrap()
    }

    fn read(&self, name: &str) -> String {
        std::fs::read_to_string(self.app.join("stackvet-report").join(name)).unwrap()
    }
}

impl Drop for Place {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.scratch).ok();
    }
}

const HEADING: &str = "What the AI coding tool did that is worth a look";

#[test]
fn the_second_report_reads_the_tool_s_commit_since_the_first_and_the_record_only_when_asked() {
    let place = Place::new("two");
    place.git(&["init", "-q", "-b", "main"]);
    place.git(&["add", "-A"]);
    place.git(&["commit", "-qm", "Start the app"]);

    // The hook records a force-push and a write to the answers, as Claude Code would hand them to it.
    let printed = place.sv(&["connect", "claude", "--hook"]);
    let block: Value = serde_json::from_slice(&printed.stdout).unwrap();
    let hook = block["hooks"]["PostToolUse"][0]["hooks"][0]["command"]
        .as_str()
        .unwrap()
        .to_owned();
    let app = place.app.display().to_string();
    for input in [
        r#"{"hook_event_name":"PostToolUse","session_id":"s","tool_name":"Bash","tool_input":{"command":"git push --force origin main"}}"#.to_owned(),
        format!(r#"{{"hook_event_name":"PostToolUse","session_id":"s","tool_name":"Write","tool_input":{{"file_path":"{app}/stackvet.toml"}}}}"#),
    ] {
        let ran = place.run(Command::new("sh").args(["-c", &hook]), Some(&input));
        assert!(ran.status.success(), "{}", String::from_utf8_lossy(&ran.stderr));
    }
    let records = place.home.join("state/stackvet/hook-records");
    assert_eq!(
        std::fs::read_dir(&records)
            .expect("the record was kept")
            .count(),
        1
    );

    // The first report, without --hook-record: the commit is kept, and none of the record is read.
    let first = place.sv(&["report", "--no-tools"]);
    assert!(
        first.status.code().is_some_and(|c| c <= 2),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    let json: Value = serde_json::from_str(&place.read("report.json")).unwrap();
    let section = &json["tool_behavior"];
    assert_eq!(
        section["at_commit"].as_str().map(str::len),
        Some(40),
        "{section}"
    );
    assert!(
        !section.to_string().contains("force-push"),
        "the record was read without consent: {section}"
    );
    let information = place.read("information.md");
    assert!(
        information.contains(&format!("## {HEADING}")),
        "{information}"
    );
    assert!(
        information.contains("the hook record was not read for this run"),
        "{information}"
    );
    assert!(
        information.contains("this folder held no earlier report"),
        "{information}"
    );
    assert!(!place.read("security.md").contains(HEADING));

    // An AI coding tool's commit, then the second report, with the person's consent to the record.
    let views = place.app.join("app.py");
    let mut text = std::fs::read_to_string(&views).expect("the example has an app.py");
    text.push_str("\nDEBUG_TOKEN = None  # nosec\n");
    std::fs::write(&views, text).unwrap();
    place.git(&[
        "commit",
        "-qam",
        "Quiet a warning\n\nCo-Authored-By: Claude <noreply@anthropic.com>",
    ]);
    let second = place.sv(&["report", "--no-tools", "--hook-record"]);
    assert!(
        second.status.code().is_some_and(|c| c <= 2),
        "{}",
        String::from_utf8_lossy(&second.stderr)
    );
    let json: Value = serde_json::from_str(&place.read("report.json")).unwrap();
    let section = &json["tool_behavior"];
    assert_eq!(section["tool_commits"], 1, "{section}");
    assert_eq!(section["other_commits"], 0, "{section}");
    let read = section["commits_read"].as_str().unwrap_or_default();
    assert!(
        read.starts_with("the 1 commit since the last report"),
        "{read}"
    );
    let seen: Vec<(String, String)> = section["seen"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|s| {
            (
                s["kind"].as_str().unwrap().to_owned(),
                s["what"].as_str().unwrap().to_owned(),
            )
        })
        .collect();
    assert!(
        seen.iter()
            .any(|(k, w)| k == "guard-edited" && w.contains("(nosec) was added in app.py")),
        "{seen:?}"
    );
    assert!(
        seen.iter()
            .any(|(k, w)| k == "command" && w.starts_with("a force-push")),
        "{seen:?}"
    );
    assert!(
        seen.iter()
            .any(|(k, w)| k == "answers-changed" && w.contains("stackvet.toml")),
        "{seen:?}"
    );
    let information = place.read("information.md");
    assert!(information.contains("| A guard edited |"), "{information}");
    assert!(place.read("report.html").contains(HEADING));
}
