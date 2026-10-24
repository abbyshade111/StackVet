//! Backlog 0649: StackVet's Rust rules on an app shaped like a real one, `examples/rust-notes`, rather than on one-line
//! snippets. Each line marked `// sv:fault <rule>` must be found by that rule in the app's own code, each marked
//! `// sv:safe <rule>` must not be, and each marked `// sv:test <rule>` must be found and marked as test code: an
//! inline `#[cfg(test)]` module, a `#[path]` module of its own, and one under `cfg(all(test, unix))`. The markers,
//! not line numbers, say where, so the example can be edited without breaking this.

use std::collections::BTreeSet;
use std::path::PathBuf;
use sv_check::ast::{self, AstRules};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Every marker in the example: (kind, rule id, file, line). A marker counts only after code on its line, so a comment
/// that describes the markers is not one.
fn markers() -> Vec<(String, String, String, usize)> {
    let src = root().join("examples/rust-notes");
    let mut out = Vec::new();
    for file in [
        "src/main.rs",
        "src/notes.rs",
        "src/notes_tests.rs",
        "src/notes/fence_tests.rs",
    ] {
        let text = std::fs::read_to_string(src.join(file)).expect("the example's file");
        for (i, line) in text.lines().enumerate() {
            let Some((code, marks)) = line.split_once("// sv:") else {
                continue;
            };
            // A comment line, a doc comment among them, holds no code to mark.
            if code.trim().is_empty() || code.trim_start().starts_with("//") {
                continue;
            }
            let words: Vec<&str> = marks.split_whitespace().collect();
            // `fault weak-cipher sv:safe unauthenticated-encryption`: kinds and rules in pairs.
            for pair in words.chunks(2) {
                if let [kind, rule] = pair {
                    out.push((
                        kind.trim_start_matches("sv:").to_owned(),
                        format!("ast.{rule}"),
                        file.to_owned(),
                        i + 1,
                    ));
                }
            }
        }
    }
    out
}

#[test]
fn each_rust_rule_finds_its_fault_and_leaves_its_safe_form_in_an_app_shaped_like_a_real_one() {
    let dir = root().join("examples/rust-notes");
    let rules = AstRules::load(&root().join("data/ast-rules.json")).expect("the code rules load");
    let mut scan = ast::scan_dir(&rules, &dir);
    sv_check::finding::mark_rust_test_code(&dir, &mut scan.findings);
    // The setup, checked first: every Rust file parsed, and the markers were read.
    assert!(scan.unparsed_files.is_empty(), "{:?}", scan.unparsed_files);
    let marks = markers();
    assert!(marks.len() >= 36, "the markers were read: {}", marks.len());

    let at = |rule: &str, file: &str, line: usize| {
        scan.findings
            .iter()
            .find(|f| f.rule_id == rule && f.location.file == file && f.location.line == line)
    };
    let mut wrong = Vec::new();
    for (kind, rule, file, line) in &marks {
        match (kind.as_str(), at(rule, file, *line)) {
            ("fault", Some(f)) if f.in_test_code() => {
                wrong.push(format!("{rule} at {file}:{line} was read as test code"))
            }
            ("fault", None) => wrong.push(format!("{rule} missed {file}:{line}")),
            ("safe", Some(_)) => {
                wrong.push(format!("{rule} reported the safe form at {file}:{line}"))
            }
            ("test", None) => wrong.push(format!("{rule} missed the test code at {file}:{line}")),
            // Known not to be read yet (as calls in macros were until backlog 0660): found, it is no longer missed,
            // and the marker should become `fault`.
            ("missed", Some(_)) => wrong.push(format!(
                "{rule} now finds {file}:{line}, held as missed: make its marker `sv:fault` and close the item that held it"
            )),
            ("test", Some(f)) if !f.in_test_code() => {
                wrong.push(format!("{rule} at {file}:{line} was not read as test code"))
            }
            _ => {}
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

#[test]
fn every_rule_that_reads_rust_has_a_fault_in_the_example() {
    let rules = AstRules::load(&root().join("data/ast-rules.json")).expect("the code rules load");
    let rust_rules: BTreeSet<&str> = rules
        .coverage()
        .into_iter()
        .filter(|(_, languages, _)| languages.contains(&"rust"))
        .map(|(id, ..)| id)
        .collect();
    let marks = markers();
    let faulted: BTreeSet<&str> = marks
        .iter()
        .filter(|(kind, ..)| kind == "fault")
        .map(|(_, rule, ..)| rule.as_str())
        .collect();
    let missing: Vec<&&str> = rust_rules
        .iter()
        .filter(|r| !faulted.contains(**r))
        .collect();
    assert!(
        missing.is_empty(),
        "rules that read Rust with no fault in the example: {missing:?}"
    );
    assert!(
        rust_rules.len() >= 18,
        "the rules that read Rust were read: {}",
        rust_rules.len()
    );
}
