//! Backlog 0661, ADR-145: the dynamic-code rule reads Rust's embedded interpreters, so a Rust-only app with none of
//! their calls is credited V1.3.2 in part, saying what was looked for; one that runs a value through rhai is not.

use std::path::PathBuf;
use sv_check::ast;

const RULE: &str = "ast.dynamic-code-execution";

fn scan_rust(source: &str) -> ast::AstScan {
    let dir = sv_check::test_support::scratch("clean-rust-dynamic-code");
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(dir.join("src/main.rs"), source).unwrap();
    let rules = ast::AstRules::load(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/ast-rules.json"),
    )
    .expect("the rules load");
    let scan = ast::scan_dir(&rules, &dir);
    std::fs::remove_dir_all(&dir).ok();
    // The setup: the one Rust file was read.
    assert_eq!(scan.parsed_by_language.get("rust"), Some(&1), "the setup");
    scan
}

#[test]
fn a_rust_only_app_with_no_interpreter_call_is_credited_v1_3_2_in_part() {
    let scan = scan_rust("fn main() {\n    println!(\"{}\", 2 + 2);\n}\n");
    let claim = scan
        .verified
        .iter()
        .find(|v| v.check_id == RULE)
        .unwrap_or_else(|| panic!("{RULE} made no clean claim: {:?}", scan.verified));
    assert_eq!(claim.requirement_ids, ["V1.3.2"]);
    assert!(claim.in_part, "the credit is in part, as in every language");
    assert!(
        claim.scope.contains("1 rust file") && claim.scope.contains("rhai"),
        "it says what it looked for in Rust: {}",
        claim.scope
    );
}

#[test]
fn a_rust_app_that_runs_a_value_through_rhai_is_not_credited_and_is_told() {
    let scan = scan_rust(
        "fn calc(engine: &Engine, formula: &str) -> i64 {\n    engine.eval::<i64>(formula).unwrap()\n}\n",
    );
    assert!(
        scan.verified.iter().all(|v| v.check_id != RULE),
        "{:?}",
        scan.verified
    );
    assert!(
        scan.findings
            .iter()
            .any(|f| f.rule_id == RULE && f.location.line == 2),
        "{:?}",
        scan.findings
    );
}
