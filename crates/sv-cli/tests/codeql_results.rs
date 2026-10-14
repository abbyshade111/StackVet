//! `sv report --codeql-results FILE` through the real `sv` (ADR-113): a CodeQL results file from the
//! owner's CI is credited when it is of the commit checked out, and named in the report, crediting
//! nothing, when it is of another.

use std::path::Path;
use std::process::Command;

fn copy(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() {
            copy(&path, &to.join(entry.file_name()));
        } else {
            std::fs::copy(&path, to.join(entry.file_name())).unwrap();
        }
    }
}

/// A clean CodeQL run of the Python rules `sv` maps, of `commit`.
fn sarif(commit: &str) -> String {
    let adapters: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/adapters.json"),
        )
        .unwrap(),
    )
    .unwrap();
    let rules: Vec<serde_json::Value> = adapters["adapters"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["id"] == "codeql-python")
        .expect("the setup: codeql-python is an adapter")["rules"]
        .as_object()
        .unwrap()
        .keys()
        .map(|id| serde_json::json!({ "id": id }))
        .collect();
    serde_json::json!({"version": "2.1.0", "runs": [{
        "tool": {"driver": {"name": "CodeQL", "rules": rules}},
        "versionControlProvenance": [{"revisionId": commit}],
        "results": [],
    }]})
    .to_string()
}

#[test]
fn a_file_of_the_commit_is_credited_and_one_of_another_is_named_and_credits_nothing() {
    let root = std::env::temp_dir().join(format!("sv-codeql-results-{}", std::process::id()));
    std::fs::remove_dir_all(&root).ok();
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();
    let app = root.join("app");
    copy(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/flask-booking"),
        &app,
    );
    std::fs::remove_dir_all(app.join(sv_scan::ecosystems::DEFAULT_REPORT_DIR)).ok();
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .arg("-C")
            .arg(&app)
            .args([
                "-c",
                "user.name=t",
                "-c",
                "user.email=t@example.test",
                "-c",
                "commit.gpgsign=false",
            ])
            .args(args)
            .output()
            .unwrap();
        assert!(out.status.success(), "the setup: git {args:?}");
        String::from_utf8_lossy(&out.stdout).trim().to_owned()
    };
    git(&["init", "-q"]);
    git(&["add", "-A"]);
    git(&["commit", "-q", "-m", "the app"]);
    let commit = git(&["rev-parse", "HEAD"]);
    assert_eq!(commit.len(), 40, "the setup: {commit}");

    let report = |file: &str, text: &str| {
        let results = root.join(file);
        std::fs::write(&results, text).unwrap();
        let out_dir = root.join(format!("out-{file}"));
        let made = Command::new(env!("CARGO_BIN_EXE_sv"))
            .args(["report", "--no-tools", "--codeql-results"])
            .arg(&results)
            .arg("--out")
            .arg(&out_dir)
            .arg(&app)
            .env("HOME", &home)
            .env_remove("XDG_CONFIG_HOME")
            .env_remove("XDG_DATA_HOME")
            .output()
            .unwrap();
        assert!(
            made.status.code() != Some(3),
            "sv failed: {}",
            String::from_utf8_lossy(&made.stderr)
        );
        std::fs::read_to_string(out_dir.join("report.json")).expect("the report was written")
    };

    let ours = report("ours.sarif", &sarif(&commit));
    assert!(
        ours.contains("from your own CI"),
        "the credit is not in the report"
    );
    assert!(!ours.contains("what your CI's CodeQL results would have said"));

    let theirs = report(
        "theirs.sarif",
        &sarif("fedcba9876543210fedcba9876543210fedcba98"),
    );
    assert!(
        !theirs.contains("from your own CI"),
        "a file of another commit was credited"
    );
    assert!(theirs.contains("what your CI's CodeQL results would have said"));
    assert!(
        theirs.contains("fedcba987654"),
        "the other commit is not named"
    );
    std::fs::remove_dir_all(&root).ok();
}
