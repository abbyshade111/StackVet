//! Backlog 0654, ADR-139: Rust code compiled only for tests, read one way for every check: by a `cfg` that holds only
//! in a test build, on the file itself or on the line that declares it, and never by one that can hold otherwise.

use super::*;
use crate::files::Listing;

#[test]
fn a_cfg_is_test_only_when_no_build_but_a_test_can_hold_it() {
    for attribute in [
        "#[cfg(test)]",
        "#[cfg( test )]",
        "#![cfg(test)]",
        "#[cfg(all(test, unix))]",
        "#[cfg(all(unix, test))]",
        "#[cfg(all(unix, all(test, feature = \"slow\")))]",
    ] {
        assert!(cfg_test_only(attribute), "{attribute}");
    }
    for attribute in [
        "#[cfg(unix)]",
        "#[cfg(any(test, unix))]",
        "#[cfg(not(test))]",
        "#[cfg(all(not(test), unix))]",
        "#[cfg(any(test, feature = \"test-support\"))]",
        "#[test]",
        "#[cfg_attr(test, derive(Debug))]",
    ] {
        assert!(!cfg_test_only(attribute), "{attribute}");
    }
}

#[test]
fn only_a_declaration_at_the_top_level_is_read() {
    let source = r#"
// #[cfg(test)] mod in_a_comment;
/* #[cfg(test)]
   mod in_a_block_comment; */
const BRACES: &str = "{ not a block";
const CHAR: char = '{';
fn takes<'a>(s: &'a str) -> &'a str { s }

pub mod app;

#[cfg(all(test, unix))]
mod stand_in_tests;

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;

mod inline {
    #[cfg(test)]
    mod inside;
}

#[cfg(any(test, feature = "x"))]
mod sometimes;
"#;
    let read: Vec<(Vec<String>, bool)> = declared_modules(source, "src/lib.rs")
        .into_iter()
        .map(|d| (d.files, d.test_only))
        .collect();
    let names: Vec<(&str, bool)> = read
        .iter()
        .map(|(files, test)| (files[0].as_str(), *test))
        .collect();
    assert_eq!(
        names,
        [
            ("src/app.rs", false),
            ("src/stand_in_tests.rs", true),
            ("src/lib_tests.rs", true),
            ("src/sometimes.rs", false),
        ]
    );
}

fn write(root: &std::path::Path, file: &str, text: &str) {
    let path = root.join(file);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}

#[test]
fn every_check_reads_rusts_test_modules_through_the_listing() {
    let root = std::env::temp_dir().join(format!("sv-rust-tests-{}", std::process::id()));
    std::fs::remove_dir_all(&root).ok();
    let body = "fn f() {}\n";
    write(
        &root,
        "src/adapters.rs",
        "pub fn run() {}\n\n#[cfg(all(test, unix))]\nmod fence_tests;\n\n#[cfg(any(test, unix))]\nmod sometimes;\n",
    );
    write(&root, "src/lib.rs", "pub mod adapters;\n");
    write(&root, "src/adapters/fence_tests.rs", body);
    write(&root, "src/adapters/sometimes.rs", body);
    write(&root, "src/whole.rs", "#![cfg(test)]\nfn t() {}\n");
    write(&root, "tests/cli.rs", body);
    let listing = Listing::of(&root);
    // The setup, checked before anything rests on it.
    assert!(
        listing
            .files
            .iter()
            .any(|f| f.relative == "src/adapters/fence_tests.rs")
    );
    let test: Vec<(&str, bool)> = [
        "src/adapters/fence_tests.rs",
        "src/whole.rs",
        "tests/cli.rs",
        "src/adapters/sometimes.rs",
        "src/adapters.rs",
        "src/lib.rs",
    ]
    .iter()
    .map(|f| (*f, listing.is_test_code(f)))
    .collect();
    assert_eq!(
        test,
        [
            ("src/adapters/fence_tests.rs", true),
            ("src/whole.rs", true),
            ("tests/cli.rs", true),
            ("src/adapters/sometimes.rs", false),
            ("src/adapters.rs", false),
            ("src/lib.rs", false),
        ]
    );
    // A half of the listing reads the same: the declaring file is read from the folder, not from the half.
    let (ours, theirs) = listing.split(&["src/adapters".to_owned()]);
    assert!(theirs.is_test_code("src/adapters/fence_tests.rs"));
    assert!(!ours.is_test_code("src/adapters.rs"));
    std::fs::remove_dir_all(&root).ok();
}
