//! `.github/CODEOWNERS` names the owner for the files that judge the required checks, and only paths that exist
//! (backlog 0290; ADR-051, Later, 14 October 2026). A path that no longer exists owns nothing, so the review it was
//! meant to require would quietly stop being asked.

use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn every_path_codeowners_names_exists_and_the_judges_are_all_named() {
    let text =
        std::fs::read_to_string(root().join(".github/CODEOWNERS")).expect("CODEOWNERS is there");
    let rules: Vec<(&str, &str)> = text
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| {
            let mut parts = l.split_whitespace();
            (parts.next().unwrap(), parts.next().unwrap_or(""))
        })
        .collect();
    // The setup: the file was read and has rules, so an empty check below is not an empty file.
    assert!(!rules.is_empty(), "{text}");
    for (path, owner) in &rules {
        assert!(owner.starts_with('@'), "{path} has no owner: {owner:?}");
        let on_disk = root().join(path.trim_start_matches('/'));
        assert!(
            on_disk.exists(),
            "CODEOWNERS names {path}, which does not exist"
        );
    }
    for judge in [
        "/.github/workflows/",
        "/tools/ci_scope.py",
        "/tools/adr_check.py",
        "/tools/record_numbers.py",
        "/action.yml",
    ] {
        assert!(
            rules.iter().any(|(path, _)| *path == judge),
            "CODEOWNERS does not name {judge}"
        );
    }
}
