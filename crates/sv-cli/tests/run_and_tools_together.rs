//! One report holds both what running the app found and what the outside tools found (backlog 0377). In the second loop
//! trial each build was checked twice, with `--run` and again with `--tools`, and neither report had both: on one build
//! the `--tools` report had a high Bandit finding in the app's own code that the `--run` report never mentioned. Here
//! `--run` and `--tools` are given together, with a stand-in Bandit, and the one report has the run's part and the
//! tool's finding. No container is needed: an app with no `[stack.run]` cannot be started, and the report says so,
//! which is the run's part as much as a started app's would be.
#![cfg(unix)]

use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::process::Command;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-run-and-tools-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(dir.join("bin")).unwrap();
    dir
}

/// A stand-in Bandit whose one finding is a high one in `app.py`, as the trial's was, and in front of every other tool
/// `sv` knows, one that is not there to run (it fails when asked its version).
fn stand_ins(bin: &Path) {
    let sarif = json!({
        "version": "2.1.0",
        "runs": [{
            "tool": { "driver": { "name": "Bandit", "rules": [{ "id": "B613",
                "shortDescription": { "text": "trojansource" } }] } },
            "results": [{
                "ruleId": "B613",
                "level": "error",
                "message": { "text": "A Python source file contains bidirectional control characters." },
                "locations": [{ "physicalLocation": {
                    "artifactLocation": { "uri": "app.py" },
                    "region": { "startLine": 2 },
                } }],
            }],
        }],
    });
    sv_check::test_support::executable(
        &bin.join("bandit"),
        &format!(
            "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then echo 'bandit 1.9.4'; exit 0; fi\nout=''\nwhile [ $# -gt 0 ]; do if [ \"$1\" = \"--output\" ]; then out=\"$2\"; fi; shift; done\ncat > \"$out\" <<'SARIF'\n{sarif}\nSARIF\nexit 1\n"
        ),
    );
    let adapters: Value = serde_json::from_str(
        &std::fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/adapters.json"),
        )
        .unwrap(),
    )
    .unwrap();
    let mut commands: Vec<String> = adapters["adapters"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|a| {
            ["version", "run", "prepare"].map(|k| a[k]["command"].as_str().map(str::to_owned))
        })
        .flatten()
        .filter(|c| c != "bandit")
        .collect();
    commands.sort();
    commands.dedup();
    assert!(
        commands.iter().any(|c| c == "semgrep"),
        "the setup: the adapters were read: {commands:?}"
    );
    for command in &commands {
        sv_check::test_support::executable(&bin.join(command), "#!/bin/sh\nexit 127\n");
    }
}

#[test]
fn one_report_holds_the_run_and_the_outside_tools() {
    let root = scratch("both");
    stand_ins(&root.join("bin"));
    let app = root.join("app");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(app.join("app.py"), "def hello():\n    return 'hi'\n").unwrap();
    std::fs::write(
        app.join("stackvet.toml"),
        "manifest-version = 1\n[app]\nname = \"Both\"\n[stack]\nlanguages = [\"python\"]\n",
    )
    .unwrap();
    let out = root.join("report");
    let path = format!(
        "{}:{}",
        root.join("bin").display(),
        std::env::var("PATH").unwrap_or_default()
    );
    let done = Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(["report", app.to_str().unwrap(), "--run", "--tools", "--out"])
        .arg(&out)
        .env("PATH", &path)
        .output()
        .expect("sv runs");
    let said = String::from_utf8_lossy(&done.stderr).into_owned();
    let report: Value = serde_json::from_str(
        &std::fs::read_to_string(out.join("report.json"))
            .unwrap_or_else(|e| panic!("no report.json ({e}): {said}")),
    )
    .unwrap();
    std::fs::remove_dir_all(&root).ok();

    // The run's part: it was asked for, and says why the app could not be started.
    assert_eq!(
        report["run_status"]["state"], "could-not-start",
        "{}",
        report["run_status"]
    );
    // The outside tools' part: Bandit ran, and its finding in the app's own code is in the same report.
    let bandit = report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["rule_id"].as_str().is_some_and(|r| r.contains("B613")))
        .unwrap_or_else(|| panic!("Bandit's finding is not in the report: {said}"));
    assert_eq!(bandit["location"]["file"], "app.py", "{bandit}");
}
