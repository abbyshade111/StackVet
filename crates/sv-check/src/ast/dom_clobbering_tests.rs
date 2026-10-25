//! `ast.dom-clobbering-fallback` and `ast.document-global-stored` (backlog 0625, part 12, V3.2.3): browser code that
//! reads a property of `window` or `document` the browser does not define, with a fallback, and code that stores a
//! value of its own on `document`. Each only ever a finding, at low confidence: careful code can still do either.

use super::*;
use std::path::PathBuf;

const FALLBACK: &str = "ast.dom-clobbering-fallback";
const STORED: &str = "ast.document-global-stored";

fn rules() -> &'static AstRules {
    static RULES: OnceLock<AstRules> = OnceLock::new();
    RULES.get_or_init(|| {
        AstRules::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/ast-rules.json"))
            .expect("rules load")
    })
}

/// The rules among `FALLBACK` and `STORED` that report `source`, read as JavaScript and as TypeScript, which must
/// agree. A file the parser could not read fails the test rather than reading as clean.
fn reported(source: &str) -> Vec<&'static str> {
    let mut seen = Vec::new();
    for (language, file) in [("javascript", "app.js"), ("typescript", "app.ts")] {
        let read = read_file(rules(), language, file, source);
        assert!(!read.parse_error, "{file} did not parse: {source}");
        assert!(
            read.broken.is_empty(),
            "a query did not compile: {:?}",
            read.broken
        );
        let mut these: Vec<&'static str> = [FALLBACK, STORED]
            .into_iter()
            .filter(|rule| read.findings.iter().any(|f| f.rule_id == *rule))
            .collect();
        these.sort_unstable();
        seen.push(these);
    }
    assert_eq!(
        seen[0], seen[1],
        "JavaScript and TypeScript disagree on: {source}"
    );
    seen.remove(0)
}

#[test]
fn a_made_up_global_read_with_a_fallback_is_found() {
    for source in [
        "const config = window.config || {};",
        "window.APP_SETTINGS_URL = window.settingsUrl || '/settings.json';",
        "const user = document.currentUser ?? null;",
        "if (window.isAdmin || false) { showAdmin(); }",
    ] {
        assert_eq!(reported(source), [FALLBACK], "{source}");
    }
}

#[test]
fn the_browsers_own_properties_read_with_a_fallback_are_left_alone() {
    for source in [
        "const store = window.localStorage || memoryStore;",
        "const raf = window.requestAnimationFrame || window.setTimeout;",
        "const Ctx = window.AudioContext || window.webkitAudioContext;",
        "const P = window.Promise || Polyfill;",
        "const where = document.referrer || 'direct';",
        "const t = document.title || 'Untitled';",
        // Not `window` or `document`: the app's own object.
        "const c = settings.config || {};",
    ] {
        assert!(
            reported(source).is_empty(),
            "{source}: {:?}",
            reported(source)
        );
    }
}

#[test]
fn a_value_stored_on_document_is_found() {
    for source in [
        "document.currentUser = user;",
        "document.appState = { ready: true };",
    ] {
        assert_eq!(reported(source), [STORED], "{source}");
    }
}

#[test]
fn what_a_page_may_set_on_document_is_left_alone() {
    for source in [
        "document.title = 'Inbox (3)';",
        "document.cookie = 'theme=dark; Secure';",
        "document.onkeydown = handleKeys;",
        "document.body.className = 'dark';",
        // `window`, not `document`: not this rule's.
        "window.currentUser = user;",
    ] {
        assert!(
            reported(source).is_empty(),
            "{source}: {:?}",
            reported(source)
        );
    }
}

#[test]
fn both_cite_v3_2_3_only_and_credit_nothing() {
    for rule in rules()
        .rules()
        .filter(|r| r.id == FALLBACK || r.id == STORED)
    {
        assert_eq!(rule.requirement_ids, ["V3.2.3"], "{}", rule.id);
        assert!(rule.findings_only, "{}", rule.id);
    }
    assert_eq!(
        rules()
            .rules()
            .filter(|r| r.id == FALLBACK || r.id == STORED)
            .count(),
        2
    );
}
