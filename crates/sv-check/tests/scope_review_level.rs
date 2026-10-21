//! `[app] level` is confirmed with the two answers that set the level (ADR-130), and a confirmation sealed before the
//! field existed still holds: left out, it adds nothing to what the seal is made over.

use std::path::Path;
use sv_check::seal::scope_review_fields;
use sv_manifest::{Manifest, ScopeReview};

fn entry(level: Option<u8>) -> ScopeReview {
    ScopeReview {
        audience: "customers".to_owned(),
        categories: Some(vec!["contact".to_owned()]),
        level,
        by: "owner".to_owned(),
        on: "2026-10-20".to_owned(),
        seal: None,
    }
}

fn manifest(level: &str) -> Manifest {
    let text = format!(
        "manifest-version = 1\n[app]\nname = \"t\"\naudience = \"customers\"\n{level}[data]\ncategories = [\"contact\"]\n"
    );
    Manifest::parse(&text, Path::new("stackvet.toml")).expect("the manifest parses")
}

#[test]
fn left_out_the_level_seals_over_exactly_what_it_did_before() {
    // What `scope_review_fields` gave for this entry before `level` existed, written out.
    assert_eq!(
        scope_review_fields(&entry(None)),
        [
            "scope-review",
            "owner",
            "2026-10-20",
            "customers",
            "listed",
            "contact"
        ]
    );
}

#[test]
fn an_asked_level_is_sealed_after_the_audience() {
    assert_eq!(
        scope_review_fields(&entry(Some(3))),
        [
            "scope-review",
            "owner",
            "2026-10-20",
            "customers",
            "level=3",
            "listed",
            "contact"
        ]
    );
}

#[test]
fn a_confirmation_holds_only_for_the_level_it_confirmed() {
    // The setup: each confirmation holds for the file it was made from.
    assert!(entry(None).still_holds_for(&manifest("")));
    assert!(entry(Some(3)).still_holds_for(&manifest("level = 3\n")));
    // Adding the level, or taking it away, after confirming makes the answers unconfirmed again.
    assert!(!entry(None).still_holds_for(&manifest("level = 3\n")));
    assert!(!entry(Some(3)).still_holds_for(&manifest("")));
}
