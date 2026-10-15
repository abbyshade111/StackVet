//! Backlog 0530: the pinning finding's title says what its text says. In the trial of 14 October 2026, every app's
//! `requirements.txt` pinned each line with `==`, the text said the lockfile was missing, and the title read "Python
//! does not pin the versions it installs". A title says a version is not pinned only when one moves.

use super::*;

fn title_in(name: &str, files: &[(&str, &str)]) -> String {
    let dir = std::env::temp_dir().join(format!("sv-pin-title-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    for (file, text) in files {
        let path = dir.join(file);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }
    let report = check_dir(&dir);
    std::fs::remove_dir_all(&dir).ok();
    report
        .findings
        .iter()
        .find(|f| f.rule_id == "config.versions-pinned")
        .unwrap_or_else(|| panic!("{name}: no pinning finding: {report:?}"))
        .title
        .clone()
}

#[test]
fn a_requirements_file_pinned_line_by_line_is_titled_for_its_missing_lockfile() {
    let title = title_in(
        "pinned",
        &[("requirements.txt", "flask==3.0.0\ngunicorn==22.0.0\n")],
    );
    assert_eq!(title, "No Python lockfile beside `requirements.txt`");
    assert!(!title.contains("does not pin"), "{title}");
}

#[test]
fn each_ecosystem_is_named_once_with_the_files_it_has_no_lockfile_beside() {
    let title = title_in(
        "two",
        &[
            ("requirements.txt", "flask==3.0.0\n"),
            (
                "worker/setup.cfg",
                "[options]\ninstall_requires =\n    requests>=2\n",
            ),
            ("web/package.json", "{\"name\":\"x\"}"),
        ],
    );
    assert!(title.starts_with("No "), "{title}");
    assert_eq!(title.matches("Python").count(), 1, "{title}");
    assert!(
        title.contains("`requirements.txt` or `worker/setup.cfg`"),
        "{title}"
    );
    assert!(
        title.contains(", and no npm lockfile beside `web/package.json`"),
        "{title}"
    );
}

#[test]
fn a_second_requirements_file_and_a_moving_version_keep_their_own_titles() {
    assert_eq!(
        title_in("dev", &[("requirements-dev.txt", "pytest>=8\n")]),
        "`requirements-dev.txt` does not pin and hash every package it lists"
    );
    let floating = title_in(
        "range",
        &[(
            "pom.xml",
            "<project><artifactId>x</artifactId><dependencies><dependency><groupId>org.x</groupId>\
             <artifactId>y</artifactId><version>[1.0,2.0)</version></dependency></dependencies></project>",
        )],
    );
    assert!(
        floating.ends_with("does not pin the versions it installs"),
        "{floating}"
    );
}
