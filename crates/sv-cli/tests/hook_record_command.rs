//! The hook record end to end (backlog 0566, ADR-122): `sv connect claude --hook` prints the hook, the shell runs the
//! command it prints as Claude Code would, and `sv report` reads the record only when the person passes --hook-record.

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
        let scratch = std::env::temp_dir().join(format!("sv-hook-{name}-{}", std::process::id()));
        std::fs::remove_dir_all(&scratch).ok();
        // A space and a quote in the app's path: the printed command must still name it as one word.
        let app = scratch.join("my app's folder");
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

    fn sv(&self, args: &[&str], stdin: Option<&str>) -> Output {
        self.run(Command::new(env!("CARGO_BIN_EXE_sv")).args(args), stdin)
    }

    fn run(&self, command: &mut Command, stdin: Option<&str>) -> Output {
        let mut child = command
            .current_dir(&self.app)
            .env("HOME", &self.home)
            .env("XDG_STATE_HOME", self.home.join("state"))
            .env("XDG_CONFIG_HOME", self.home.join(".config"))
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
}

impl Drop for Place {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.scratch).ok();
    }
}

#[test]
fn the_hook_is_printed_for_claude_code_alone_and_never_written() {
    let place = Place::new("print");
    let printed = place.sv(&["connect", "claude", "--hook"], None);
    assert!(
        printed.status.success(),
        "{}",
        String::from_utf8_lossy(&printed.stderr)
    );
    let block: Value = serde_json::from_slice(&printed.stdout).expect("the block is JSON");
    for event in ["PostToolUse", "PostToolUseFailure", "Stop"] {
        let command = block["hooks"][event][0]["hooks"][0]["command"]
            .as_str()
            .unwrap_or_default();
        assert!(command.contains(" hook-record --app '"), "{event}: {block}");
    }
    assert_eq!(
        block["hooks"]["PostToolUse"][0]["matcher"],
        "^(Bash|PowerShell|Edit|Write)$"
    );
    assert!(String::from_utf8_lossy(&printed.stderr).contains(".claude/settings.local.json"));
    assert!(
        !place.app.join(".claude").exists(),
        "sv writes nothing into the tool's settings"
    );

    for args in [
        &["connect", "vscode", "--hook"][..],
        &["connect", "--hook"][..],
        &["connect", "claude", "--hook", "--docker", "/usr/bin/docker"][..],
    ] {
        let refused = place.sv(args, None);
        assert!(!refused.status.success(), "{args:?}");
        assert!(refused.stdout.is_empty(), "{args:?}");
    }
}

#[cfg(unix)]
#[test]
fn the_printed_hook_records_and_a_report_reads_it_only_when_asked() {
    let place = Place::new("record");
    let printed = place.sv(&["connect", "claude", "--hook"], None);
    let block: Value = serde_json::from_slice(&printed.stdout).unwrap();
    let hook = block["hooks"]["PostToolUse"][0]["hooks"][0]["command"]
        .as_str()
        .unwrap()
        .to_owned();
    let app = place.app.display().to_string();
    for input in [
        format!(r#"{{"hook_event_name":"PostToolUse","session_id":"s","tool_name":"Write","tool_input":{{"file_path":"{app}/app.py"}}}}"#),
        r#"{"hook_event_name":"PostToolUse","session_id":"s","tool_name":"Bash","tool_input":{"command":"pytest -q"}}"#.to_owned(),
        r#"{"hook_event_name":"Stop","session_id":"s"}"#.to_owned(),
    ] {
        // Run as Claude Code runs a command hook: the printed string, through the shell.
        let ran = place.run(Command::new("sh").args(["-c", &hook]), Some(&input));
        assert!(ran.status.success(), "{}", String::from_utf8_lossy(&ran.stderr));
    }
    let records = place.home.join("state/stackvet/hook-records");
    let kept: Vec<_> = std::fs::read_dir(&records)
        .expect("the record is outside the app")
        .flatten()
        .collect();
    assert_eq!(kept.len(), 1);
    assert_eq!(
        std::fs::read_to_string(kept[0].path())
            .unwrap()
            .lines()
            .count(),
        3
    );

    // Without --hook-record nothing reads it.
    let plain = place.sv(&["report", "--no-tools"], None);
    assert!(
        plain.status.code().is_some_and(|c| c <= 2),
        "{}",
        String::from_utf8_lossy(&plain.stderr)
    );
    let read =
        |name: &str| std::fs::read_to_string(place.app.join("stackvet-report").join(name)).unwrap();
    let json: Value = serde_json::from_str(&read("report.json")).unwrap();
    assert!(json.get("hook_record").is_none(), "{}", json["hook_record"]);
    assert!(!read("compliance.md").contains("What the hook saw"));

    let asked = place.sv(&["report", "--no-tools", "--hook-record"], None);
    assert!(
        asked.status.code().is_some_and(|c| c <= 2),
        "{}",
        String::from_utf8_lossy(&asked.stderr)
    );
    let json: Value = serde_json::from_str(&read("report.json")).unwrap();
    assert_eq!(
        json["hook_record"]["commands"], 1,
        "{}",
        json["hook_record"]
    );
    assert_eq!(json["hook_record"]["writes"], 1);
    assert_eq!(json["hook_record"]["tip"]["lines"], 3);
    let compliance = read("compliance.md");
    assert!(
        compliance.contains("It saw 1 command, 1 file write, and 1 turn ended"),
        "{compliance}"
    );
    assert!(read("report.html").contains("What the hook saw your AI coding tool do"));
}
