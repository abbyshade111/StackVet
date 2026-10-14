//! Every question only the person can answer has a plain sentence beside the standard's words, and it
//! is a question of its own (backlog 0427). The standard's words, `asks`, stay the citation, held by
//! the catalogs' own tests; `plain` is what the person is asked.

use std::path::PathBuf;
use sv_check::design::Questions;
use sv_check::human::{HumanChecks, STANDARDS_WORDS, checklist};
use sv_check::notes::Catalog;

fn data(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../data")
        .join(name)
}

/// Each question's id, its plain sentence, and the standard's words.
fn questions() -> Vec<(String, Option<String>, String)> {
    let notes = Catalog::load(&data("security-notes.json")).expect("the notes catalog loads");
    let design =
        Questions::load(&data("design-questions.json")).expect("the design questions load");
    let mut all: Vec<(String, Option<String>, String)> = notes
        .sections
        .iter()
        .map(|s| (s.id.clone(), s.plain.clone(), s.asks.clone()))
        .collect();
    all.extend(
        design
            .questions
            .iter()
            .map(|q| (q.id.clone(), q.plain.clone(), q.asks.clone())),
    );
    all
}

#[test]
fn every_question_has_a_plain_sentence_of_its_own() {
    let all = questions();
    // The setup: both catalogs were read, so a missing sentence below is a missing sentence.
    assert!(all.len() >= 30, "{}", all.len());
    for (id, plain, asks) in &all {
        let plain = plain
            .as_deref()
            .unwrap_or_else(|| panic!("{id} has no plain sentence"));
        let plain = plain.trim();
        assert!(
            plain.ends_with('?') || plain.ends_with(".)"),
            "{id}: {plain}"
        );
        assert!(plain.contains('?'), "{id} does not ask: {plain}");
        assert_ne!(
            plain,
            asks.trim(),
            "{id}: the plain sentence is the standard's words"
        );
        // Plain means without the standard's terms of art.
        for jargon in [
            "trusted service layer",
            "trusted backend",
            "credential stuffing",
            "malicious account lockout",
            "untrusted consumer",
            "logical consistency",
        ] {
            assert!(
                !plain.to_lowercase().contains(jargon),
                "{id} says {jargon:?}: {plain}"
            );
        }
    }
}

#[test]
fn the_checklist_asks_in_plain_words_first_and_keeps_the_standards_words() {
    let notes = Catalog::load(&data("security-notes.json")).unwrap();
    let design = Questions::load(&data("design-questions.json")).unwrap();
    let human = HumanChecks::load(&data("human-checks.json")).unwrap();
    let wanted = ["V8.3.1", "V6.1.1", "V2.3.1"]
        .map(str::to_owned)
        .into_iter()
        .collect();
    let items = checklist(&notes, &design, &human, &wanted);
    // The setup: one of each route is in the list.
    assert_eq!(items.len(), 3, "{items:?}");
    for item in &items {
        let asked = item.asked();
        match &item.plain {
            Some(plain) => {
                assert!(asked.starts_with(plain.as_str()), "{asked}");
                assert!(
                    asked.ends_with(&format!("{STANDARDS_WORDS} {}", item.how)),
                    "{asked}"
                );
            }
            // A check to make by hand is already an instruction in plain words.
            None => {
                assert_eq!(item.id, "V2.3.1");
                assert_eq!(asked, item.how);
            }
        }
    }
}
