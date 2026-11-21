//! `cv check` with the outside tools (backlog 0693): a tool that is not installed is said so and never counted as a
//! pass, and where hadolint, Checkov, and Trivy are installed, the app's own settings files and skip comments do not
//! hide their rules. Each of those was measured on 21 November 2026 to hide a rule when `cv` did nothing about it.
//!
//! On a computer without a tool, its part takes the honest-absence path and says so, as `sv`'s tests that start an
//! app do without Docker.

use std::path::{Path, PathBuf};
use std::process::Command;

fn folder(test: &str, files: &[(&str, &str)]) -> PathBuf {
    let root = std::env::temp_dir().join(format!("cv-tools-{test}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    for (path, text) in files {
        std::fs::write(root.join(path), text).unwrap();
    }
    root
}

fn cv(folder: &Path, path_env: Option<&str>) -> (i32, String, serde_json::Value) {
    let out_dir = folder.with_extension("report");
    let _ = std::fs::remove_dir_all(&out_dir);
    let mut command = Command::new(env!("CARGO_BIN_EXE_cv"));
    // From inside the app, as `cv check .` is run: Trivy reads `trivy.yaml` and `.trivyignore` from where it starts, so
    // started anywhere else this test would never meet the app's (it did not, until it was started here).
    command
        .current_dir(folder)
        .arg("check")
        .arg(folder)
        .arg("--out")
        .arg(&out_dir)
        .args(["--fail-on", "not-assessed"]);
    if let Some(path) = path_env {
        command.env("PATH", path);
    }
    let out = command.output().unwrap();
    let report = std::fs::read_to_string(out_dir.join("report.json")).unwrap_or_default();
    (
        out.status.code().unwrap(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        serde_json::from_str(&report).unwrap_or_default(),
    )
}

fn installed(program: &str) -> bool {
    Command::new(program)
        .arg("--version")
        .output()
        .is_ok_and(|o| o.status.success())
}

/// Every rule id a tool's findings carry in the report, the ones folded into `cv`'s own included.
fn reported(report: &serde_json::Value) -> Vec<String> {
    let mut ids: Vec<String> = report["outside_tools"]["findings"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|f| f["rule_id"].as_str().map(str::to_owned))
        .collect();
    for f in report["checked"]["findings"]
        .as_array()
        .into_iter()
        .flatten()
    {
        for by in f["also_reported_by"].as_array().into_iter().flatten() {
            ids.push(by.as_str().unwrap().to_owned());
        }
    }
    ids
}

fn tool<'r>(report: &'r serde_json::Value, id: &str) -> &'r serde_json::Value {
    report["outside_tools"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .find(|t| t["id"] == id)
        .unwrap_or_else(|| panic!("{id} is in the report: {report}"))
}

const DOCKERFILE: &str = "FROM node:latest\n# hadolint ignore=DL3008\nRUN apt-get update && apt-get install -y curl\nUSER root\nCMD node server.js\n";

#[test]
fn a_tool_that_is_not_installed_is_said_so_and_is_never_a_pass() {
    let app = folder(
        "absent",
        &[(
            "Dockerfile",
            "FROM node:24-alpine\nUSER node\nCMD [\"node\", \"s.js\"]\n",
        )],
    );
    // A PATH holding none of the three.
    let (code, said, report) = cv(&app, Some("/usr/bin:/bin"));
    for id in ["hadolint", "checkov", "trivy"] {
        let line = tool(&report, id);
        assert_eq!(line["status"], "not run", "{line}");
        assert!(
            line["said"].to_string().contains("brew install"),
            "it says how to install {id}: {line}"
        );
    }
    assert!(said.contains("not run"), "{said}");
    assert_eq!(
        code, 2,
        "a tool that did not run is something not assessed: {said}"
    );
}

#[test]
fn the_app_s_own_settings_and_skip_comments_do_not_hide_a_rule() {
    let app = folder(
        "settings",
        &[
            ("Dockerfile", DOCKERFILE),
            // Each measured on 21 November 2026 to hide the rule named, when cv did nothing about it.
            (".hadolint.yaml", "ignored:\n  - DL3007\n"),
            ("trivy.yaml", "misconfiguration:\n  scanners: []\n"),
            (".trivyignore", "DS-0026\n"),
        ],
    );
    let (_, said, report) = cv(&app, None);
    let ids = reported(&report);
    if installed("hadolint") {
        assert_eq!(tool(&report, "hadolint")["status"], "ran", "{said}");
        for rule in ["hadolint.DL3007", "hadolint.DL3008", "hadolint.DL3002"] {
            assert!(ids.iter().any(|i| i == rule), "{rule} hidden: {ids:?}");
        }
    } else {
        println!("hadolint is not installed here, so the hadolint half of this test was not run");
    }
    if installed("trivy") {
        assert_eq!(tool(&report, "trivy")["status"], "ran", "{said}");
        for rule in ["trivy.DS-0026", "trivy.DS-0001"] {
            assert!(ids.iter().any(|i| i == rule), "{rule} hidden: {ids:?}");
        }
    } else {
        println!("trivy is not installed here, so the Trivy half of this test was not run");
    }
}

#[test]
fn checkov_with_the_app_s_settings_file_present_ran_only_in_part() {
    let app = folder(
        "checkov",
        &[
            ("Dockerfile", DOCKERFILE),
            (".checkov.yaml", "skip-check:\n  - CKV_DOCKER_2\n"),
        ],
    );
    if !installed("checkov") {
        println!("Checkov is not installed here, so this test was not run");
        return;
    }
    let (code, said, report) = cv(&app, None);
    let line = tool(&report, "checkov");
    assert_eq!(line["status"], "ran in part", "{line}");
    assert!(
        line["said"].to_string().contains(".checkov.yaml"),
        "the file is named: {line}"
    );
    assert_eq!(code, 2, "{said}");
    // Without the file, the same run is whole.
    std::fs::remove_file(app.join(".checkov.yaml")).unwrap();
    let (_, _, report) = cv(&app, None);
    assert_eq!(tool(&report, "checkov")["status"], "ran", "{report}");
    assert!(
        reported(&report)
            .iter()
            .any(|i| i == "checkov.CKV_DOCKER_2")
    );
}

#[test]
fn a_root_container_every_tool_reports_is_one_finding_naming_them() {
    let app = folder("root", &[("Dockerfile", DOCKERFILE)]);
    let (_, said, report) = cv(&app, None);
    let roots: Vec<&serde_json::Value> = report["checked"]["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["rule_id"] == "cv.container-root")
        .collect();
    assert_eq!(roots.len(), 1, "{said}");
    let by = roots[0]["also_reported_by"].to_string();
    for (program, rule) in [
        ("hadolint", "hadolint.DL3002"),
        ("checkov", "checkov.CKV_DOCKER_8"),
        ("trivy", "trivy.DS-0002"),
    ] {
        if installed(program) {
            assert!(by.contains(rule), "{rule} not folded: {by}");
            let left: Vec<String> = reported(&report);
            assert_eq!(
                left.iter().filter(|i| *i == rule).count(),
                1,
                "{rule} is listed once: {left:?}"
            );
        }
    }
}
