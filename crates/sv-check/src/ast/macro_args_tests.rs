//! Backlog 0660: the calls inside Rust's standard macros are read by the rules, at the line they are on; the macro
//! itself is never read as a call, and a macro not in the list stays unread.

use super::*;
use std::path::PathBuf;

fn found(source: &str) -> Vec<(String, usize)> {
    static RULES: std::sync::OnceLock<AstRules> = std::sync::OnceLock::new();
    let rules = RULES.get_or_init(|| {
        AstRules::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/ast-rules.json"))
            .expect("rules load")
    });
    scan_file(rules, "rust", "src/lib.rs", source)
        .into_iter()
        .map(|f| (f.rule_id, f.location.line))
        .collect()
}

const WEAK: &str = "ast.weak-hash-function";

#[test]
fn the_copy_keeps_every_line_and_length_and_rewrites_only_the_standard_macros() {
    let source = "fn f(b: &[u8], n: usize) {\n    println!(\"{:x}\", md5::compute(b));\n    \
                  let v = vec![0u8; n];\n    my_macro!(md5::compute(b));\n    std::eprintln!(\"{}\", n);\n}\n";
    let copy = macro_args::rewrite(source).expect("the file has standard macros");
    assert_eq!(copy.source.len(), source.len(), "nothing moves");
    assert_eq!(copy.source.lines().count(), source.lines().count());
    assert!(
        copy.source.contains("println (\"{:x}\", md5::compute(b))"),
        "{}",
        copy.source
    );
    assert!(
        copy.source.contains("std::eprintln (\"{}\", n)"),
        "a macro named by its path: {}",
        copy.source
    );
    assert!(
        copy.source.contains("vec![0u8; n]"),
        "a value and a count is not a list of expressions"
    );
    assert!(
        copy.source.contains("my_macro!(md5::compute(b))"),
        "the app's own macro is not read"
    );
    assert!(
        macro_args::rewrite("fn f() { let x = 1; }\n").is_none(),
        "no macro, no copy"
    );
}

#[test]
fn a_call_inside_a_standard_macro_is_found_at_its_line_and_one_in_the_apps_own_macro_is_not() {
    assert_eq!(
        found("fn f(b: &[u8]) -> String {\n    format!(\"{:x}\", md5::compute(b))\n}\n"),
        [(WEAK.to_owned(), 2)]
    );
    // Over several lines, the line is the call's, not the macro's.
    assert_eq!(
        found(
            "fn f(b: &[u8]) {\n    println!(\n        \"{:x}\",\n        md5::compute(b),\n    );\n}\n"
        ),
        [(WEAK.to_owned(), 4)]
    );
    // Nested: each round reads the next macro in.
    assert_eq!(
        found("fn f(b: &[u8]) {\n    assert_eq!(format!(\"{:x}\", md5::compute(b)), \"\");\n}\n"),
        [(WEAK.to_owned(), 2)]
    );
    assert_eq!(
        found("fn f(b: &[u8]) {\n    my_macro!(md5::compute(b));\n}\n"),
        []
    );
}

#[test]
fn the_macro_itself_is_never_read_as_a_call() {
    // `std::write!` written as `std::write (out, ...)` is shaped like `std::fs::write(path, ...)` with `out` as the
    // path. On its own its head is outside any arguments; inside another macro's it is not, and is still left out.
    for source in [
        "fn f(out: &mut String, path: &str) {\n    let _ = std::write!(out, \"{path}\");\n}\n",
        "fn f(out: &mut String, path: &str) {\n    assert!(std::write!(out, \"{path}\").is_ok());\n}\n",
    ] {
        assert_eq!(found(source), [], "{source}");
    }
}

#[test]
fn a_finding_outside_the_macro_on_its_line_is_kept_once() {
    let source =
        "fn f(b: &[u8]) {\n    let d = md5::compute(b); println!(\"{:x}\", md5::compute(b));\n}\n";
    assert_eq!(found(source), [(WEAK.to_owned(), 2)]);
}

#[test]
fn a_rewritten_macro_inside_another_is_not_held_and_what_is_inside_it_is() {
    // No rule today reads a rewritten macro's head as a call (the file rules want `fs::`, not `std::write`), so the
    // guard is held here directly: in the copy, the inner macro's call is not kept, and the call inside it is.
    let source = "fn f(out: &mut String, b: &[u8]) {\n    assert!(std::write!(out, \"{:x}\", md5::compute(b)).is_ok());\n}\n";
    let copy = macro_args::rewrite(source).expect("standard macros");
    let mut parser = Parser::new();
    parser.set_language(&grammar("rust").unwrap()).unwrap();
    let tree = parser.parse(&copy.source, None).unwrap();
    let mut calls = Vec::new();
    let mut stack = vec![tree.root_node()];
    while let Some(node) = stack.pop() {
        if node.kind() == "call_expression" {
            let callee = node.child_by_field_name("function").unwrap();
            calls.push((
                callee.utf8_text(copy.source.as_bytes()).unwrap().to_owned(),
                copy.holds(node),
            ));
        }
        let mut walk = node.walk();
        stack.extend(node.children(&mut walk));
    }
    calls.sort();
    assert_eq!(
        calls,
        [
            ("assert".to_owned(), false),
            ("md5::compute".to_owned(), true),
            ("std::write".to_owned(), false),
            // A call on the inner macro's result is code written in `assert!`'s arguments.
            (
                "std::write (out, \"{:x}\", md5::compute(b)).is_ok".to_owned(),
                true
            ),
        ],
        "{}",
        copy.source
    );
}
