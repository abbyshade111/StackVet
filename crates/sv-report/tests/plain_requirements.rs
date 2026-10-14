//! Backlog 0372: the plain-language lines in `data/plain-requirements.json`, held to the requirements they paraphrase.
//!
//! A plain line drifts from what it paraphrases, and a drifted line is worse than none: the person reads it, and the
//! requirement asks something else. So, as `crates/sv-check/tests/notes.rs` holds the questions:
//! 1. every id is a requirement that exists;
//! 2. every line shares words with its requirement's own text, the weak overlap the citations use;
//! 3. every ASVS level-1 requirement has a line, so the first set is not missing one by accident.
//!
//! None of this proves a line is faithful: each was written from the requirement's text and read against it.

use std::path::PathBuf;
use sv_check::suite::shares_no_words;
use sv_frameworks::Frameworks;

fn data() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data")
}

fn plain() -> sv_report::PlainRequirements {
    sv_frameworks::data::read_json(&data().join("plain-requirements.json"))
        .expect("the plain lines load")
}

fn frameworks() -> Frameworks {
    Frameworks::load(&data().join("frameworks")).expect("the frameworks load")
}

#[test]
fn every_line_names_a_requirement_that_exists_and_shares_words_with_it() {
    let frameworks = frameworks();
    let plain = plain();
    assert!(!plain.plain.is_empty(), "the setup: no lines");
    let mut wrong = Vec::new();
    for (id, line) in &plain.plain {
        match frameworks.requirements.get(id) {
            None => wrong.push(format!("{id}: no such requirement")),
            Some(r) if shares_no_words(line, &r.description) => wrong.push(format!(
                "{id}: shares no words with its requirement: {line}"
            )),
            Some(_) if line.trim().is_empty() || line.len() > 300 => {
                wrong.push(format!("{id}: empty, or too long to be a plain line"))
            }
            Some(_) => {}
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

#[test]
fn every_asvs_level_one_requirement_has_a_line() {
    let frameworks = frameworks();
    let plain = plain();
    let missing: Vec<&String> = frameworks
        .requirements
        .values()
        .filter(|r| r.id.starts_with('V') && r.level == 1)
        .map(|r| &r.id)
        .filter(|id| !plain.plain.contains_key(*id))
        .collect();
    assert!(missing.is_empty(), "no plain line for {missing:?}");
}
