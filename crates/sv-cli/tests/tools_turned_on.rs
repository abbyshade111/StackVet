//! CodeQL runs only when the owner turns it on, and off it is not named anywhere a run writes (backlog 0415;
//! ADR-115). Its license allows it only on open-source code, for academic research, or with a paid GitHub
//! Advanced Security license, so a report must not send the owner of a private app to install it.

use std::path::{Path, PathBuf};
use std::process::Command;

use sv_check::test_support::scratch;

/// A stand-in that will not start for every program `data/adapters.json` names, so the run is the same on a
/// computer with the tools as on one without, and quick. Each is reported as not run, which is all this needs.
fn stand_ins(bin: &Path) {
    let adapters: serde_json::Value = serde_json::from_str(
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
        .collect();
    commands.sort();
    commands.dedup();
    // The setup: CodeQL is among them, so its absence below is the setting's doing.
    assert!(commands.iter().any(|c| c == "codeql"), "{commands:?}");
    for command in &commands {
        sv_check::test_support::executable(
            &bin.join(command),
            "#!/bin/sh\necho 'kept out of this test' >&2\nexit 1\n",
        );
    }
}

/// `sv report` with the outside tools on, over a small Python app with `tools` as its `[tools]` lines. Every
/// file the run wrote and what it printed, as one text.
fn run(name: &str, tools: &str) -> String {
    let dir = scratch(name);
    let app = dir.join("app");
    let bin = dir.join("bin");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::create_dir_all(&bin).unwrap();
    std::fs::write(app.join("app.py"), "def hello():\n    return 'hi'\n").unwrap();
    std::fs::write(
        app.join("stackvet.toml"),
        format!(
            "manifest-version = 1\n[app]\nname = \"Notes\"\n[stack]\nlanguages = [\"python\"]\n{tools}"
        ),
    )
    .unwrap();
    stand_ins(&bin);
    let out = dir.join("report");
    let path = format!(
        "{}:{}",
        bin.display(),
        std::env::var("PATH").unwrap_or_default()
    );
    let ran = Command::new(env!("CARGO_BIN_EXE_sv"))
        .arg("report")
        .arg(&app)
        .args(["--tools", "--out"])
        .arg(&out)
        .env("PATH", path)
        .output()
        .expect("sv runs");
    let mut said = format!(
        "{}{}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    );
    let mut files = 0;
    for entry in std::fs::read_dir(&out).expect("a report folder") {
        let path = entry.unwrap().path();
        if let Ok(text) = std::fs::read_to_string(&path) {
            said.push_str(&text);
            files += 1;
        }
    }
    // The setup: a report was written and the tools were tried, so what is missing is missing.
    assert!(files >= 3, "{files} files in {}:\n{said}", out.display());
    assert!(
        said.contains("bandit"),
        "the outside tools were not tried:\n{said}"
    );
    std::fs::remove_dir_all(&dir).ok();
    said
}

#[test]
fn off_codeql_is_not_run_and_not_named_and_on_it_is() {
    for tools in ["", "[tools]\ncodeql = false\n"] {
        let said = run("off", tools);
        // Named as a tool: by its name, or by any of its adapters' ids. (A data file `sv` read, such as
        // `codeql-suites.json` in report.json's list of them, names no tool.)
        let names = |l: &&str| {
            l.contains("CodeQL")
                || ["codeql-python", "codeql-javascript", "codeql-ruby"]
                    .iter()
                    .any(|id| l.contains(id))
        };
        let named: Vec<&str> = said.lines().filter(names).collect();
        assert!(
            named.is_empty(),
            "with {tools:?} the run names CodeQL:\n{}",
            named.join("\n")
        );
    }
    // Turned on and missing: named as not run, with how to install it, as before.
    let said = run("on", "[tools]\ncodeql = true\n");
    assert!(said.contains("codeql-python"), "{said}");
}
