//! Backlog 0653's last part (ADR-138, Later): Rust's safe forms for `ast.file-path-from-value`, as the other languages
//! have them. A path set only to fixed text, through names or a constant, and a join under a constant are spared; a
//! path any part of which the rule cannot tell is fixed is still reported.

use super::*;
use std::path::PathBuf;

const RULE: &str = "ast.file-path-from-value";

fn rules() -> &'static AstRules {
    static RULES: OnceLock<AstRules> = OnceLock::new();
    RULES.get_or_init(|| {
        AstRules::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/ast-rules.json"))
            .expect("rules load")
    })
}

/// Whether the rule reports `source`, read as Rust. A file the parser could not read fails the test rather than
/// reading as clean.
fn reported(source: &str) -> bool {
    let read = read_file(rules(), "rust", "src/lib.rs", source);
    assert!(!read.parse_error, "did not parse: {source}");
    assert!(
        read.broken.is_empty(),
        "a query did not compile: {:?}",
        read.broken
    );
    read.findings.iter().any(|f| f.rule_id == RULE)
}

/// The setup: the rule does read Rust's file calls, so a spared case is spared, not unread.
#[test]
fn a_path_from_a_parameter_is_reported() {
    assert!(reported(
        "fn f(name: &str) { let _ = std::fs::read_to_string(name); }\n"
    ));
}

#[test]
fn a_name_set_only_to_fixed_text_is_spared() {
    for source in [
        "fn f() { let p = \"config.toml\"; let _ = std::fs::read_to_string(p); }\n",
        "fn f() { let p: &str = \"config.toml\"; let _ = File::open(p); }\n",
        "fn f() { let base = \"data\"; let p = base; let _ = fs::read(p); }\n",
    ] {
        assert!(!reported(source), "{source}");
    }
}

#[test]
fn a_constant_and_a_join_under_one_are_spared() {
    for source in [
        "const ROOT: &str = \"/srv/app\";\nfn f() { let _ = fs::read(ROOT); }\n",
        "const ROOT: &str = \"/srv/app\";\nfn f() { let _ = File::open(Path::new(ROOT).join(\"data.json\")); }\n",
        "fn f() { let _ = fs::read(PathBuf::from(env!(\"CARGO_MANIFEST_DIR\")).join(\"fixtures\").join(\"a.txt\")); }\n",
        "const ROOT: &str = \"/srv/app\";\nfn f() { let p = Path::new(ROOT).join(\"data.json\"); let _ = fs::read(p); }\n",
    ] {
        assert!(!reported(source), "{source}");
    }
}

#[test]
fn a_path_any_part_of_which_is_not_fixed_is_still_reported() {
    for source in [
        "const ROOT: &str = \"/srv/app\";\nfn f(name: &str) { let _ = fs::read(Path::new(ROOT).join(name)); }\n",
        "fn f() { let p = std::env::args().nth(1).unwrap(); let _ = fs::read(p); }\n",
        "fn f(name: &str) { let mut p = \"a\".to_owned(); p = name.to_owned(); let _ = fs::read(p); }\n",
        "fn f(name: &str) { let p = Path::new(\"/srv\").join(name); let _ = File::open(p); }\n",
        // The caller's `p` at the call, whatever an inner block sets its own `p` to.
        "fn f(p: &str, c: bool) { if c { let p = \"x\"; let _ = p; } let _ = fs::read(p); }\n",
    ] {
        assert!(reported(source), "{source}");
    }
}
