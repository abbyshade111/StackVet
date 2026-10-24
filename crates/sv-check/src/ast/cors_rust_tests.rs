//! Backlog 0661: a CORS chain in Rust that lets every site in with credentials is one finding, at the chain's own
//! line, however many calls wrap it or follow it.

use super::*;
use std::path::PathBuf;

#[test]
fn a_chain_wrapped_in_the_routers_layer_is_one_finding_at_its_own_line() {
    let rules = AstRules::load(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/ast-rules.json"),
    )
    .expect("rules load");
    let source = "fn app() -> Router {\n    Router::new()\n        .route(\"/\", get(home))\n        .layer(\n            \
                  CorsLayer::new()\n                .allow_origin(AllowOrigin::mirror_request())\n                \
                  .allow_credentials(true)\n                .max_age(Duration::from_secs(600)),\n        )\n}\n";
    let read = read_file(&rules, "rust", "src/main.rs", source);
    assert!(!read.parse_error, "the setup: the file parses");
    let lines: Vec<usize> = read
        .findings
        .iter()
        .filter(|f| f.rule_id == "ast.cors-any-origin-with-credentials")
        .map(|f| f.location.line)
        .collect();
    assert_eq!(lines, [5], "one finding, where CorsLayer::new() starts");
}
