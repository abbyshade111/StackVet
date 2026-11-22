//! Every requirement ConfigVet's live rules cite is one the frameworks hold (ADR-157, Later, 21 November 2026: each
//! cited after reading its text). Here rather than in `src/`, since it reads the repository's `data/` folder where the
//! source is, which code under `src/` must never do (`crates/sv-cli/tests/moved.rs`).

use cv_live::judge::RULES;

#[test]
fn every_citation_names_a_requirement_that_exists() {
    let mut known = String::new();
    for f in ["asvs-5.0.0.json", "aisvs-1.0-appendix-c.json"] {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../data/frameworks")
            .join(f);
        known.push_str(&std::fs::read_to_string(path).unwrap());
    }
    assert!(
        known.contains("\"AC.8.3\""),
        "the setup: the frameworks were read"
    );
    assert!(RULES.len() >= 9, "the setup: the rules were read");
    for rule in RULES {
        for id in rule.requirement_ids {
            assert!(
                known.contains(&format!("\"{id}\"")),
                "{} cites {id}, which is not in the frameworks",
                rule.id
            );
        }
    }
}
