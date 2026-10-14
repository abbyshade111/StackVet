//! A CodeQL results file from the owner's CI is read only for the commit checked out, with nothing
//! changed since (ADR-113): each way a file can fail that is said, and credits nothing.

use super::*;
use std::path::PathBuf;

fn adapters() -> Adapters {
    Adapters::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/adapters.json"))
        .expect("the adapters load")
}

const COMMIT: &str = "0123456789abcdef0123456789abcdef01234567";
const OTHER: &str = "fedcba9876543210fedcba9876543210fedcba98";

/// A CodeQL run of the JavaScript rules `sv` maps, with `results`, of `commit` when given.
fn sarif(commit: Option<&str>, results: serde_json::Value) -> String {
    let ids: Vec<String> = adapters()
        .all()
        .iter()
        .find(|a| a.id == "codeql-javascript")
        .expect("the setup: codeql-javascript is an adapter")
        .rules
        .keys()
        .cloned()
        .collect();
    let mut run = serde_json::json!({
        "tool": {"driver": {"name": "CodeQL", "rules": ids.iter().map(|id| serde_json::json!({"id": id})).collect::<Vec<_>>()}},
        "results": results,
    });
    if let Some(commit) = commit {
        run["versionControlProvenance"] = serde_json::json!([{"repositoryUri": "https://example.test/app", "revisionId": commit}]);
    }
    serde_json::json!({"version": "2.1.0", "runs": [run]}).to_string()
}

fn read_it(text: &str, app: &AppCommit) -> CiResults {
    read(
        &adapters(),
        text,
        "codeql.sarif",
        app,
        Path::new(""),
        &["javascript".to_owned()],
    )
}

fn one_result() -> serde_json::Value {
    let id = adapters()
        .all()
        .iter()
        .find(|a| a.id == "codeql-javascript")
        .unwrap()
        .rules
        .keys()
        .next()
        .unwrap()
        .clone();
    serde_json::json!([{
        "ruleId": id,
        "level": "error",
        "message": {"text": "a value from the request reaches it"},
        "locations": [{"physicalLocation": {"artifactLocation": {"uri": "src/app.js"}, "region": {"startLine": 3}}}],
    }])
}

#[test]
fn a_clean_run_of_the_commit_checked_out_is_credited() {
    let out = read_it(
        &sarif(Some(COMMIT), serde_json::json!([])),
        &AppCommit::Clean(COMMIT.into()),
    );
    assert!(out.not_used.is_empty(), "{:?}", out.not_used);
    assert!(out.findings.is_empty());
    assert_eq!(out.verified.len(), 1, "{:?}", out.verified);
    let credit = &out.verified[0];
    assert_eq!(credit.check_id, "adapter.codeql-javascript");
    assert!(!credit.requirement_ids.is_empty());
    assert!(
        credit.scope.contains("from your own CI") && credit.scope.contains("0123456789ab"),
        "{}",
        credit.scope
    );
}

#[test]
fn a_run_of_the_commit_with_a_result_is_a_finding_and_no_credit() {
    let out = read_it(
        &sarif(Some(COMMIT), one_result()),
        &AppCommit::Clean(COMMIT.into()),
    );
    assert_eq!(out.findings.len(), 1, "{:?}", out.not_used);
    assert!(out.verified.is_empty());
}

#[test]
fn any_other_file_is_said_and_read_for_nothing() {
    let cases = [
        (
            sarif(Some(OTHER), one_result()),
            AppCommit::Clean(COMMIT.into()),
            "fedcba987654",
        ),
        (
            sarif(None, serde_json::json!([])),
            AppCommit::Clean(COMMIT.into()),
            "names no commit",
        ),
        (
            sarif(Some(COMMIT), serde_json::json!([])),
            AppCommit::Changed(COMMIT.into()),
            "have changed since",
        ),
        (
            sarif(Some(COMMIT), serde_json::json!([])),
            AppCommit::Unknown,
            "not a git checkout",
        ),
    ];
    for (text, app, says) in cases {
        let out = read_it(&text, &app);
        assert!(out.findings.is_empty(), "{says}: {:?}", out.findings);
        assert!(out.verified.is_empty(), "{says}: {:?}", out.verified);
        assert_eq!(out.not_used.len(), 1, "{says}");
        assert_eq!(out.not_used[0].0, Unused::OtherCode, "{says}");
        assert!(
            out.not_used[0].1.contains(says),
            "{says}: {}",
            out.not_used[0].1
        );
        assert!(
            out.not_used[0].1.contains("codeql.sarif"),
            "{}",
            out.not_used[0].1
        );
    }
}

#[test]
fn a_language_without_a_map_and_another_tool_are_said_and_read_for_nothing() {
    let go = serde_json::json!({"version": "2.1.0", "runs": [{
        "tool": {"driver": {"name": "CodeQL", "rules": [{"id": "go/sql-injection"}]}},
        "versionControlProvenance": [{"revisionId": COMMIT}],
        "results": [],
    }]})
    .to_string();
    let out = read_it(&go, &AppCommit::Clean(COMMIT.into()));
    assert!(out.verified.is_empty() && out.findings.is_empty());
    assert!(
        out.not_used[0].1.contains("go/sql-injection"),
        "{:?}",
        out.not_used
    );
    assert_eq!(out.not_used[0].0, Unused::NoMap);

    let other = serde_json::json!({"version": "2.1.0", "runs": [{
        "tool": {"driver": {"name": "Semgrep"}}, "results": [],
    }]})
    .to_string();
    let out = read_it(&other, &AppCommit::Clean(COMMIT.into()));
    assert!(out.verified.is_empty());
    assert!(
        out.not_used[0].1.contains("not of CodeQL"),
        "{:?}",
        out.not_used
    );
    assert_eq!(out.not_used[0].0, Unused::NoMap);

    let out = read_it("not json", &AppCommit::Clean(COMMIT.into()));
    assert!(
        out.not_used[0].1.contains("could not be read"),
        "{:?}",
        out.not_used
    );
    assert_eq!(out.not_used[0].0, Unused::Unreadable);
}

#[test]
fn the_apps_commit_is_read_from_git_and_a_change_is_seen() {
    let dir = std::env::temp_dir().join(format!("sv-ci-codeql-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    let git = |args: &[&str]| {
        let ok = std::process::Command::new("git")
            .arg("-C")
            .arg(&dir)
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
            .map(|o| o.status.success())
            .unwrap_or(false);
        assert!(ok, "the setup: git {args:?}");
    };
    assert_eq!(
        AppCommit::of(&dir),
        AppCommit::Unknown,
        "not yet a checkout"
    );
    git(&["init", "-q"]);
    std::fs::write(dir.join("app.js"), "console.log(1)\n").unwrap();
    git(&["add", "app.js"]);
    git(&["commit", "-q", "-m", "first"]);
    let AppCommit::Clean(commit) = AppCommit::of(&dir) else {
        panic!("a fresh commit is clean: {:?}", AppCommit::of(&dir));
    };
    assert_eq!(commit.len(), 40);
    // A file git does not track changes nothing; one it tracks does.
    std::fs::write(dir.join("notes.txt"), "x").unwrap();
    assert_eq!(AppCommit::of(&dir), AppCommit::Clean(commit.clone()));
    std::fs::write(dir.join("app.js"), "console.log(2)\n").unwrap();
    assert_eq!(AppCommit::of(&dir), AppCommit::Changed(commit));
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn reading_the_apps_commit_runs_no_program_the_repository_names() {
    // ADR-032: `git status` and `git diff` hash a file whose time changed through the repository's
    // clean filter, a program `.git/config` names. The app's commit is read without either.
    let dir = std::env::temp_dir().join(format!("sv-ci-codeql-filter-{}", std::process::id()));
    let marker = dir.with_extension("ran");
    std::fs::remove_dir_all(&dir).ok();
    std::fs::remove_file(&marker).ok();
    std::fs::create_dir_all(&dir).unwrap();
    let git = |args: &[&str]| {
        let ok = std::process::Command::new("git")
            .arg("-C")
            .arg(&dir)
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
            .map(|o| o.status.success())
            .unwrap_or(false);
        assert!(ok, "the setup: git {args:?}");
    };
    git(&["init", "-q"]);
    std::fs::write(dir.join("app.js"), "console.log(1)\n").unwrap();
    git(&["add", "app.js"]);
    git(&["commit", "-q", "-m", "first"]);
    std::fs::write(dir.join(".gitattributes"), "* filter=planted\n").unwrap();
    git(&[
        "config",
        "filter.planted.clean",
        &format!("touch '{}'; cat", marker.display()),
    ]);
    // The same content, a new time: the case in which git's own commands run the filter.
    std::thread::sleep(std::time::Duration::from_millis(1100));
    std::fs::write(dir.join("app.js"), "console.log(1)\n").unwrap();
    // The setup: git's own status does run it here, so its not running below means something.
    let _ = std::process::Command::new("git")
        .arg("-C")
        .arg(&dir)
        .args(["status", "--porcelain"])
        .output();
    assert!(
        marker.exists(),
        "the setup: git status ran the planted filter"
    );
    std::fs::remove_file(&marker).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(1100));
    std::fs::write(dir.join("app.js"), "console.log(1)\n").unwrap();

    let commit = AppCommit::of(&dir);
    assert!(
        !marker.exists(),
        "reading the app's commit ran the repository's filter"
    );
    assert!(matches!(commit, AppCommit::Clean(_)), "{commit:?}");
    std::fs::remove_dir_all(&dir).ok();
}
