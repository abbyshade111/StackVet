//! A finding from a rule written for some languages, in a file of another, decides nothing (backlog 0615; ADR-111,
//! Later). Semgrep's insecure-WebSocket rule is a text pattern that matches `ws://` in any file: in `sv`'s own
//! repository it made 27 findings, 23 of them in Rust, and the one outside test code was a doc comment, the only thing
//! behind V4.4.1 needing attention. At the owner's choice of 20 October 2026 the rule names the languages it was
//! written for, JavaScript and TypeScript, and a match in a file of another is listed apart.

use std::path::PathBuf;
use sv_check::adapters::{self, Adapters};
use sv_check::finding::Outranked;

const WEBSOCKET: &str =
    "javascript.lang.security.detect-insecure-websocket.detect-insecure-websocket";

fn adapters() -> Adapters {
    let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/adapters.json");
    Adapters::load(&data).expect("the adapter file loads")
}

/// The finding Semgrep's report of `rule` on line 1 of `file` makes.
fn found(adapters: &Adapters, rule: &str, file: &str) -> sv_check::Finding {
    let semgrep = adapters
        .all()
        .iter()
        .find(|a| a.id == "semgrep")
        .expect("semgrep is listed");
    let sarif = serde_json::json!({
        "version": "2.1.0",
        "runs": [{ "tool": { "driver": { "name": "Semgrep OSS", "rules": [] } }, "results": [{
            "ruleId": rule,
            "level": "warning",
            "message": { "text": "Insecure WebSocket detected" },
            "locations": [{ "physicalLocation": {
                "artifactLocation": { "uri": file },
                "region": { "startLine": 1 }
            }}]
        }]}]
    })
    .to_string();
    let mut findings = adapters::parse_sarif(semgrep, &sarif).expect("the report reads");
    assert_eq!(findings.len(), 1, "the result made a finding");
    findings.remove(0)
}

#[test]
fn ws_in_javascript_is_still_a_finding_and_in_a_rust_comment_it_decides_nothing() {
    let adapters = adapters();
    let mut findings = vec![
        found(&adapters, WEBSOCKET, "static/chat.js"),
        found(&adapters, WEBSOCKET, "web/socket.ts"),
        found(&adapters, WEBSOCKET, "crates/sv-check/src/ast.rs"),
        found(&adapters, WEBSOCKET, "tools/probe.py"),
        found(&adapters, WEBSOCKET, "docs/notes.md"),
    ];
    // The setup: the rule is mapped, to V12.3.1 and against V4.4.1, or nothing below is about it.
    assert!(
        findings[0].requirement_ids.iter().any(|r| r == "V12.3.1"),
        "{:?}",
        findings[0].requirement_ids
    );
    assert!(
        findings
            .iter()
            .all(|f| f.outranked.is_none() && f.withholds_credit())
    );
    adapters::set_apart_other_languages(&adapters, &mut findings);
    // In the languages it was written for, it counts as before.
    for f in &findings[..2] {
        assert_eq!(f.outranked, None, "{}", f.location.file);
        assert!(f.withholds_credit(), "{}", f.location.file);
    }
    // Anywhere else it is listed apart, says which languages it was written for, and withholds nothing.
    for f in &findings[2..] {
        assert_eq!(
            f.outranked,
            Some(Outranked::OtherLanguage {
                languages: vec!["javascript".into(), "typescript".into()]
            }),
            "{}",
            f.location.file
        );
        assert!(!f.withholds_credit(), "{}", f.location.file);
    }
}

#[test]
fn a_rule_that_reads_files_of_its_own_or_any_file_is_never_set_apart_by_language() {
    let adapters = adapters();
    // A rule of any language (`*`), and one with files of its own (`targets`): neither is judged by the file's
    // extension. Found among the map's own rules, so the test follows the data.
    let semgrep = adapters.all().iter().find(|a| a.id == "semgrep").unwrap();
    let any = semgrep
        .rules
        .iter()
        .find(|(_, r)| r.languages.iter().any(|l| l == "*"))
        .map(|(id, _)| id.clone());
    let own = semgrep
        .rules
        .iter()
        .find(|(_, r)| !r.targets.is_empty())
        .map(|(id, _)| id.clone());
    let (any, own) = (
        any.expect("a rule of any language"),
        own.expect("a rule with files of its own"),
    );
    let mut findings = vec![
        found(&adapters, &any, "notes.unknown"),
        found(&adapters, &own, "app/views/a.erb"),
    ];
    adapters::set_apart_other_languages(&adapters, &mut findings);
    assert!(
        findings.iter().all(|f| f.outranked.is_none()),
        "{:?}",
        findings.iter().map(|f| &f.outranked).collect::<Vec<_>>()
    );
}
