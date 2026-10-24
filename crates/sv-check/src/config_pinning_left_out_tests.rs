//! Backlog 0657, ADR-140 (Later): the pinning check asks the inventory's question, whether the versions the app installs
//! are fixed, so it leaves out what the inventory leaves out: a test's or a sample's own lockfiles and manifests.

use super::*;

fn folder(name: &str, files: &[(&str, &str)]) -> std::path::PathBuf {
    let dir =
        std::env::temp_dir().join(format!("sv-pinning-left-out-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    for (file, text) in files {
        let path = dir.join(file);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }
    dir
}

const PACKAGE: &str = r#"{"name":"app"}"#;
const LOCK: &str =
    r#"{"name":"app","lockfileVersion":1,"dependencies":{"express":{"version":"4.18.2"}}}"#;

fn pinning_found(report: &ConfigReport) -> bool {
    report
        .findings
        .iter()
        .any(|f| f.rule_id == "config.versions-pinned")
}

fn pinning_passed(report: &ConfigReport) -> bool {
    report
        .passed
        .iter()
        .any(|p| p.check_id == "config.versions-pinned")
}

#[test]
fn a_fixtures_unpinned_file_is_not_the_apps_and_the_apps_own_lockfile_passes() {
    // As StackVet's own `crates/sv-run/tests/fixtures/install-app/requirements.txt`: no lockfile, no real app behind it.
    let dir = folder(
        "fixture",
        &[
            ("package.json", PACKAGE),
            ("package-lock.json", LOCK),
            (
                "tests/fixtures/install-app/requirements.txt",
                "six==1.16.0\n",
            ),
        ],
    );
    let report = check_dir(&dir);
    std::fs::remove_dir_all(&dir).ok();
    assert!(!pinning_found(&report), "{:?}", report.findings);
    assert!(pinning_passed(&report));
}

#[test]
fn the_apps_own_unpinned_file_is_still_a_finding() {
    let dir = folder(
        "own",
        &[
            ("requirements.txt", "flask>=2\n"),
            ("tests/fixtures/app/package.json", PACKAGE),
            ("tests/fixtures/app/package-lock.json", LOCK),
        ],
    );
    let report = check_dir(&dir);
    std::fs::remove_dir_all(&dir).ok();
    assert!(pinning_found(&report), "{:?}", report.findings);
}

#[test]
fn an_app_whose_only_manifests_are_tests_is_told_so_and_credited_nothing() {
    let dir = folder(
        "only-tests",
        &[
            ("app.py", "print(1)\n"),
            ("tests/fixtures/app/requirements.txt", "flask>=2\n"),
        ],
    );
    let report = check_dir(&dir);
    std::fs::remove_dir_all(&dir).ok();
    assert!(!pinning_found(&report) && !pinning_passed(&report));
    let said: Vec<&String> = report
        .not_assessed
        .iter()
        .filter(|(id, _)| id == "config.versions-pinned")
        .map(|(_, why)| why)
        .collect();
    assert!(
        said.iter().any(
            |w| w.contains("Every package manifest found is in test code")
                && w.contains("`tests/fixtures/app/requirements.txt`")
        ),
        "{said:?}"
    );
}
