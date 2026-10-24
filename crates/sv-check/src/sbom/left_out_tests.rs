//! Backlog 0656, ADR-140: the bill of materials leaves out the lockfiles and manifests in test code and in folders named
//! as not the app, which are fixtures' and samples' own, says which it left out, and reads the app's own as before.

use super::*;
use std::path::PathBuf;

fn folder(name: &str, files: &[(&str, &str)]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-sbom-left-out-{name}-{}", std::process::id()));
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

fn names(sbom: &Sbom) -> Vec<String> {
    sbom.components.iter().map(Component::purl).collect()
}

#[test]
fn a_fixtures_lockfile_is_left_out_and_said_and_the_apps_own_is_read() {
    let dir = folder(
        "fixtures",
        &[
            ("package.json", PACKAGE),
            ("package-lock.json", LOCK),
            // A fixture with a lockfile, one with none, and a Python sample with a range: each made the list "not
            // complete" on StackVet's own report of 23 October 2026.
            ("tests/fixtures/node-app/package.json", PACKAGE),
            (
                "tests/fixtures/node-app/package-lock.json",
                r#"{"name":"x","lockfileVersion":1,"dependencies":{"left-pad":{"version":"1.3.0"}}}"#,
            ),
            ("tests/fixtures/no-lock/package.json", PACKAGE),
            ("tests/fixtures/unpinned/requirements.txt", "flask>=2\n"),
        ],
    );
    let sbom = build_app_in(&sv_scan::files::Listing::of(&dir), &[]);
    std::fs::remove_dir_all(&dir).ok();
    assert_eq!(
        names(&sbom),
        ["pkg:npm/express@4.18.2"],
        "the app's own, and not the fixture's"
    );
    assert!(sbom.is_complete(), "unread: {:?}", sbom.unread);
    assert_eq!(sbom.left_out.len(), 3, "{:?}", sbom.left_out);
    let read = sbom.what_was_read();
    assert!(read.contains("3 lockfiles and manifests in test code or in folders named as not the app were left out"), "{read}");
    assert!(
        read.contains("`tests/fixtures/node-app/package-lock.json`"),
        "{read}"
    );
}

#[test]
fn a_folder_named_as_not_the_app_is_left_out_by_the_report() {
    let dir = folder(
        "not-the-app",
        &[
            ("package.json", PACKAGE),
            ("package-lock.json", LOCK),
            ("demo/shop/Gemfile", "gem 'rails'\n"),
            ("demo/notes/requirements.txt", "flask>=2\n"),
        ],
    );
    let listing = sv_scan::files::Listing::of(&dir);
    // `demo` is not a name test code goes by, so only the manifest's word leaves it out. Read whole, the samples make
    // the list incomplete; as the app, they are left out and named.
    let whole = build_in(&listing);
    let app = build_app_in(&listing, &["demo".to_owned()]);
    std::fs::remove_dir_all(&dir).ok();
    assert!(
        !whole.is_complete(),
        "the setup: the samples are read when nothing names them"
    );
    assert_eq!(names(&app), ["pkg:npm/express@4.18.2"]);
    assert!(app.is_complete(), "unread: {:?}", app.unread);
    assert_eq!(
        app.left_out,
        ["demo/notes/requirements.txt", "demo/shop/Gemfile"],
        "each sample's file is named"
    );
}

#[test]
fn an_app_whose_only_lockfile_is_a_tests_is_credited_nothing() {
    let dir = folder(
        "only-tests",
        &[
            ("app.py", "print(1)\n"),
            ("tests/fixtures/app/package.json", PACKAGE),
            ("tests/fixtures/app/package-lock.json", LOCK),
        ],
    );
    let sbom = build_app_in(&sv_scan::files::Listing::of(&dir), &[]);
    std::fs::remove_dir_all(&dir).ok();
    assert!(sbom.components.is_empty());
    assert!(
        completeness_verified(&sbom).is_none(),
        "a list left empty by leaving the tests out credits nothing"
    );
    assert!(!sbom.left_out.is_empty());
}

#[test]
fn the_whole_list_the_vulnerability_checks_read_keeps_a_fixtures_packages() {
    // The inventory leaves a fixture out; the known-vulnerability checks must not, or a sample's vulnerable package
    // would go unseen. A first build of 0656 left test code out of `build_in` itself, and `sv check`'s tests that a set
    // apart folder's vulnerability is still found went red.
    let dir = folder(
        "whole",
        &[
            ("package.json", PACKAGE),
            ("package-lock.json", LOCK),
            ("tests/fixtures/node-app/package.json", PACKAGE),
            (
                "tests/fixtures/node-app/package-lock.json",
                r#"{"name":"x","lockfileVersion":1,"dependencies":{"left-pad":{"version":"1.3.0"}}}"#,
            ),
        ],
    );
    let whole = build(&dir);
    let inventory = build_app_in(&sv_scan::files::Listing::of(&dir), &[]);
    std::fs::remove_dir_all(&dir).ok();
    assert_eq!(
        names(&whole),
        ["pkg:npm/express@4.18.2", "pkg:npm/left-pad@1.3.0"]
    );
    assert_eq!(names(&inventory), ["pkg:npm/express@4.18.2"]);
}
