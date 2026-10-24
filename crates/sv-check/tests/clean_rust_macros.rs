//! Backlog 0660: a clean result in Rust says the standard macros' arguments were read, and that a macro of the
//! app's own was not, so "nothing found" is no wider than what was looked at.

use std::path::PathBuf;
use sv_check::ast;

fn scope_in_rust(source: &str, rule_id: &str) -> String {
    let dir = sv_check::test_support::scratch("clean-rust-macros");
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(dir.join("src/main.rs"), source).unwrap();
    let rules = ast::AstRules::load(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/ast-rules.json"),
    )
    .expect("the rules load");
    let scan = ast::scan_dir(&rules, &dir);
    std::fs::remove_dir_all(&dir).ok();
    // The setup: the file was read, and the rule made a clean claim at all.
    assert_eq!(scan.parsed_by_language.get("rust"), Some(&1), "the setup");
    scan.verified
        .iter()
        .find(|v| v.check_id == rule_id)
        .unwrap_or_else(|| panic!("{rule_id} made no clean claim: {:?}", scan.verified))
        .scope
        .clone()
}

#[test]
fn a_clean_rust_result_says_the_standard_macros_were_read_and_the_apps_own_were_not() {
    let source = "fn main() {\n    println!(\"{}\", 2);\n}\n";
    // A rule that names its calls: the note joins the list of them.
    let scope = scope_in_rust(source, "ast.file-path-from-value");
    assert!(
        scope.contains(
            "; inside the standard macros' arguments too, such as `format!`, `println!`, and `vec!`, but not \
             inside a macro of the app's own or a library's)"
        ),
        "{scope}"
    );
    // A rule that says its Rust reach in words of its own still says it.
    let scope = scope_in_rust(source, "ast.weak-hash-function");
    assert!(
        scope.contains("1 rust file (") && scope.contains("not inside a macro of the app's own"),
        "{scope}"
    );
}
