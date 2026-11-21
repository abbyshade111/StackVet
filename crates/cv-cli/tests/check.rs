//! `cv check` as a person runs it: what it prints, the `report.json` it writes, and its exit codes (ADR-029's, as
//! ConfigVet follows them).

use std::path::PathBuf;
use std::process::Command;

fn folder(test: &str, files: &[(&str, &str)]) -> PathBuf {
    let root = std::env::temp_dir().join(format!("cv-cli-{test}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    for (path, text) in files {
        let path = root.join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }
    root
}

fn cv(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_cv"))
        .args(args)
        .output()
        .unwrap();
    (
        out.status.code().unwrap(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

const APP: &[(&str, &str)] = &[
    ("package.json", r#"{"dependencies":{"express":"5.1.0"}}"#),
    (
        "Dockerfile",
        "FROM node:24-alpine\nCMD [\"node\", \"index.js\"]\n",
    ),
];

#[test]
fn a_finding_is_listed_written_and_exits_0_unless_asked() {
    let root = folder("finding", APP);
    let dir = root.to_str().unwrap();
    let (code, stdout, stderr) = cv(&["check", dir]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    assert!(stdout.contains("missing, default unsafe"), "{stdout}");
    assert!(stdout.contains("cv.express-node-env"), "{stdout}");
    let report: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(root.join("cv-report/report.json")).unwrap())
            .unwrap();
    assert_eq!(report["tool"], "cv");
    let findings = report["checked"]["findings"].as_array().unwrap();
    assert_eq!(
        findings.len(),
        2,
        "NODE_ENV and the root user: {findings:?}"
    );
    assert!(
        findings
            .iter()
            .all(|f| f["fingerprint"].is_null() || f["fingerprint"] == "")
    );

    // Asked to fail on findings, and run again over its own report folder, which it may write over.
    let (code, _, stderr) = cv(&["check", dir, "--fail-on", "attention"]);
    assert_eq!(code, 1, "{stderr}");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_folder_with_no_deployment_file_exits_2() {
    let root = folder("empty", &[("README.md", "hello\n")]);
    let out = root.join("elsewhere");
    let (code, stdout, _) = cv(&[
        "check",
        root.to_str().unwrap(),
        "--out",
        out.to_str().unwrap(),
    ]);
    assert_eq!(code, 2, "{stdout}");
    assert!(
        stdout.contains("no deployment setting was assessed"),
        "{stdout}"
    );
    assert!(out.join("report.json").is_file());
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_file_it_could_not_read_exits_2() {
    let root = folder("unread", &[("compose.yaml", "services: [\n")]);
    let (code, stdout, _) = cv(&["check", root.to_str().unwrap()]);
    assert_eq!(code, 2, "{stdout}");
    assert!(stdout.contains("Could not read compose.yaml"), "{stdout}");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn it_never_writes_into_a_folder_holding_someone_else_s_files() {
    let root = folder("refuse", &[APP[0], APP[1], ("out/notes.txt", "mine\n")]);
    let out = root.join("out");
    let (code, _, stderr) = cv(&[
        "check",
        root.to_str().unwrap(),
        "--out",
        out.to_str().unwrap(),
    ]);
    assert_eq!(code, 3, "{stderr}");
    assert!(stderr.contains("files cv did not write"), "{stderr}");
    assert!(!out.join("report.json").exists());
    assert_eq!(
        std::fs::read_to_string(out.join("notes.txt")).unwrap(),
        "mine\n"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_folder_that_is_not_there_or_an_unknown_option_exits_3() {
    assert_eq!(cv(&["check", "/no/such/folder/for/cv"]).0, 3);
    assert_eq!(cv(&["check", ".", "--token", "x"]).0, 3);
    assert_eq!(cv(&[]).0, 3);
    let (code, stdout, _) = cv(&["--version"]);
    assert_eq!(code, 0);
    assert!(stdout.starts_with("cv "));
}
