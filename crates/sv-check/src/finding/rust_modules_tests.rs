//! Backlog 0644, ADR-136: a Rust module that is a file of its own is test code when the file declaring it puts it under
//! `#[cfg(test)]`, by each path Rust reads a module from, and a module declared inside such a file is too.

use super::{Finding, Severity, mark_rust_test_code};
use std::path::Path;

fn finding_in(file: &str) -> Finding {
    Finding {
        evidence: Vec::new(),
        also_reported_by: Vec::new(),
        fingerprint: format!("fp-{file}"),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        outranked: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
        rule_id: "ast.file-path-from-value".into(),
        title: "A file is opened at a path built from a value".into(),
        severity: Severity::Medium,
        confidence: crate::Confidence::Low,
        location: crate::Location {
            file: file.into(),
            line: 2,
        },
        secret: None,
        requirement_ids: Vec::new(),
        cwe: Vec::new(),
        description: String::new(),
        impact: String::new(),
        fix: String::new(),
    }
}

fn write(dir: &Path, file: &str, text: &str) {
    let path = dir.join(file);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}

/// Which of `files` are marked as test code, in an app laid out as each test writes it.
fn marked(dir: &Path, files: &[&str]) -> Vec<bool> {
    let mut findings: Vec<Finding> = files.iter().map(|f| finding_in(f)).collect();
    mark_rust_test_code(dir, &mut findings);
    findings.iter().map(|f| f.marked_test_code).collect()
}

fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-rust-modules-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn each_way_rust_finds_a_test_module_file_is_read() {
    let dir = scratch("forms");
    let body = "fn helper() {\n    std::fs::read(p);\n}\n";
    // `lib.rs` keeps its modules beside it; `ast.rs`, in `ast/`.
    write(
        &dir,
        "src/lib.rs",
        "pub mod ast;\npub mod app;\n\n#[cfg(test)]\nmod fake_app;\n\n#[cfg(test)]\n#[path = \"lib_tests.rs\"]\nmod tests;\n",
    );
    write(&dir, "src/fake_app.rs", body);
    write(&dir, "src/lib_tests.rs", body);
    write(&dir, "src/app.rs", body);
    write(
        &dir,
        "src/ast.rs",
        "pub fn scan() {}\n\n#[cfg(test)]\n// The tests of the scan.\nmod tests;\n\n#[cfg(test)]\nmod fixtures;\n",
    );
    write(
        &dir,
        "src/ast/tests.rs",
        "mod helpers;\nfn t() {\n    std::fs::read(p);\n}\n",
    );
    write(&dir, "src/ast/fixtures/mod.rs", body);
    // Declared inside a test module, so it is built only for tests too.
    write(&dir, "src/ast/tests/helpers.rs", body);
    let files = [
        "src/fake_app.rs",
        "src/lib_tests.rs",
        "src/ast/tests.rs",
        "src/ast/fixtures/mod.rs",
        "src/ast/tests/helpers.rs",
        "src/app.rs",
        "src/lib.rs",
    ];
    assert_eq!(
        marked(&dir, &files),
        [true, true, true, true, true, false, false],
        "{files:?}"
    );
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_module_without_cfg_test_or_named_by_nothing_is_the_apps_own() {
    let dir = scratch("own");
    let body = "fn helper() {\n    std::fs::read(p);\n}\n";
    write(
        &dir,
        "src/main.rs",
        "#[cfg(feature = \"extra\")]\nmod extra;\n\n#[path = \"tests_like_name.rs\"]\nmod tools;\n\n#[cfg(test)]\nmod tests {\n    fn t() {}\n}\n",
    );
    write(&dir, "src/extra.rs", body);
    write(&dir, "src/tests_like_name.rs", body);
    write(&dir, "src/orphan.rs", body);
    assert_eq!(
        marked(
            &dir,
            &["src/extra.rs", "src/tests_like_name.rs", "src/orphan.rs"]
        ),
        [false, false, false]
    );
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_loop_of_path_declarations_ends() {
    let dir = scratch("loop");
    write(
        &dir,
        "src/a.rs",
        "#[path = \"b.rs\"]\nmod b;\nfn f() {\n    std::fs::read(p);\n}\n",
    );
    write(
        &dir,
        "src/b.rs",
        "#[path = \"a.rs\"]\nmod a;\nfn f() {\n    std::fs::read(p);\n}\n",
    );
    assert_eq!(marked(&dir, &["src/a.rs", "src/b.rs"]), [false, false]);
    std::fs::remove_dir_all(&dir).ok();
}
