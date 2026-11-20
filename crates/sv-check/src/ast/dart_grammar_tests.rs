//! What tree-sitter-dart 0.3 renamed, held one witness at a time (backlog 0685). Version 0.3 gave
//! Dart's `+` a node of its own (`additive_operator`, in the field `operator`), renamed
//! `initialized_variable_definition` to `initialized_variable_declaration`, and renamed a map's
//! `pair` to `map_element`. Each witness below turns red when the code that reads one of those is
//! put back the way it was before 0.3, so none of them passes by way of another name in the source:
//! the older witness for `ast.insecure-random-for-code` in Dart also sits in a function named
//! `sendOtp`, so it kept passing when the variable's name was lost. Its own file, so its cases do
//! not land at the end of `tests.rs`.

use super::*;
use std::path::PathBuf;

fn rules() -> &'static AstRules {
    static RULES: OnceLock<AstRules> = OnceLock::new();
    RULES.get_or_init(|| {
        AstRules::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/ast-rules.json"))
            .expect("rules load")
    })
}

/// Whether `rule` reports `source`, read as Dart. A file the parser could not read, or a query
/// that did not compile, fails the test rather than reading as clean.
fn reported(rule: &str, source: &str) -> bool {
    let read = read_file(rules(), "dart", "lib/notes.dart", source);
    assert!(!read.parse_error, "did not parse: {source}");
    assert!(
        read.broken.is_empty(),
        "a query did not compile: {:?}",
        read.broken
    );
    read.findings.iter().any(|f| f.rule_id == rule)
}

#[test]
fn the_grammar_is_the_one_these_witnesses_were_written_for() {
    // The witnesses below are only worth something under the grammar that renamed these nodes: under
    // 0.2, the operator was not named and the old names were the right ones.
    let language: tree_sitter::Language = tree_sitter_dart::LANGUAGE.into();
    for kind in [
        "additive_operator",
        "initialized_variable_declaration",
        "map_element",
    ] {
        assert!(
            language.id_for_node_kind(kind, true) != 0,
            "tree-sitter-dart has no named node `{kind}`: the grammar changed again"
        );
    }
}

#[test]
fn two_pieces_of_fixed_text_joined_with_plus_are_fixed_text() {
    const RULE: &str = "ast.sql-built-by-hand";
    assert!(!reported(
        RULE,
        "Future f() => db.rawQuery('select * ' + 'from notes');"
    ));
    assert!(!reported(
        RULE,
        "Future f() => db.rawQuery('select * ' + 'from notes ' + 'order by id');"
    ));
    // The operator being skipped must not make a value that is not fixed text look fixed.
    assert!(reported(
        RULE,
        "Future f(String id) => db.rawQuery('select * from notes where id = ' + id);"
    ));
}

#[test]
fn a_variable_named_for_a_code_names_the_value() {
    const RULE: &str = "ast.insecure-random-for-code";
    // Nothing but the variable says this is a code: the function is called `handle`.
    assert!(reported(
        RULE,
        "void handle() { final otp = Random().nextInt(900000); mail(otp); }"
    ));
    assert!(!reported(
        RULE,
        "void handle() { final face = Random().nextInt(6); show(face); }"
    ));
}

#[test]
fn a_map_key_named_for_a_code_names_the_value() {
    const RULE: &str = "ast.insecure-random-for-code";
    // A map entry's key, `map_element` since 0.3.
    assert!(reported(
        RULE,
        "void handle() { mail({'otp': Random().nextInt(900000)}); }"
    ));
    assert!(!reported(
        RULE,
        "void handle() { show({'face': Random().nextInt(6)}); }"
    ));
}

#[test]
fn a_function_stored_under_a_map_key_is_named_by_the_key() {
    const RULE: &str = "ast.insecure-random-for-code";
    // The closure has no name of its own, so the function around the value is named by its key.
    assert!(reported(
        RULE,
        "final handlers = {'sendOtp': () { final n = Random().nextInt(900000); mail(n); }};"
    ));
    assert!(!reported(
        RULE,
        "final handlers = {'roll': () { final n = Random().nextInt(6); show(n); }};"
    ));
}
