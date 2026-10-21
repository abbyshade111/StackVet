//! The how-to-verify catalog, held to the requirements it claims to explain.
//!
//! The same guards as the other two catalogs, plus the one this catalog needs and they do not: a
//! requirement must not appear here *and* in the security notes or the design questions. Two
//! catalogs would give different advice about the same thing, and the reader has no way to tell
//! which is meant — nor which one a later editor updated.

use std::collections::BTreeSet;
use std::path::PathBuf;
use sv_check::design::Questions;
use sv_check::human::{HumanChecks, checklist};
use sv_check::notes::Catalog;
use sv_check::suite::shares_no_words;
use sv_frameworks::Frameworks;

fn data(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../data")
        .join(name)
}

fn human() -> HumanChecks {
    HumanChecks::load(&data("human-checks.json")).expect("the human checks load")
}

fn notes() -> Catalog {
    Catalog::load(&data("security-notes.json")).expect("the notes catalog loads")
}

fn design() -> Questions {
    Questions::load(&data("design-questions.json")).expect("the design questions load")
}

fn frameworks() -> Frameworks {
    Frameworks::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/frameworks"))
        .expect("the frameworks load")
}

#[test]
fn every_check_names_a_requirement_that_exists() {
    let f = frameworks();
    let unknown: Vec<String> = human()
        .checks
        .iter()
        .map(|c| c.id.clone())
        .filter(|id| !f.requirements.contains_key(id))
        .collect();
    assert!(unknown.is_empty(), "no such requirement: {unknown:?}");
}

#[test]
fn no_requirement_is_explained_by_two_catalogs() {
    // The guard this catalog needs and the others do not. Two sets of advice about one requirement
    // is worse than one: the reader cannot tell which is meant, and a later editor updates whichever
    // they found first.
    let asked: BTreeSet<String> = notes()
        .sections
        .iter()
        .map(|s| s.id.clone())
        .chain(design().questions.iter().map(|q| q.id.clone()))
        .collect();
    let twice: Vec<String> = human()
        .checks
        .iter()
        .map(|c| c.id.clone())
        .filter(|id| asked.contains(id))
        .collect();
    assert!(
        twice.is_empty(),
        "these are already asked about in the security notes or the design questions, so they must \
         not be here too: {twice:?}"
    );
}

#[test]
fn every_check_shares_words_with_the_requirement_it_explains() {
    let f = frameworks();
    let mut adrift = Vec::new();
    for c in &human().checks {
        let Some(r) = f.requirements.get(&c.id) else {
            continue;
        };
        if shares_no_words(&format!("{} {}", c.title, c.how), &r.description) {
            adrift.push(format!("{}: {}", c.id, c.title));
        }
    }
    assert!(adrift.is_empty(), "these have drifted: {adrift:#?}");
}

#[test]
fn every_check_tells_somebody_what_to_actually_do() {
    // A line that restates the requirement is not an instruction. Each of these has to name an
    // action — open, try, list, compare — or it is the requirement's own wording with extra steps.
    let verbs = [
        "open",
        "try",
        "list",
        "compare",
        "take",
        "sign in",
        "run",
        "do ",
        "go ",
        "search",
        "check",
        "start",
        "write down",
        "look",
    ];
    let mut lazy = Vec::new();
    for c in &human().checks {
        let how = c.how.to_lowercase();
        if !verbs.iter().any(|v| how.contains(v)) {
            lazy.push(format!("{}: {}", c.id, c.how));
        }
    }
    assert!(
        lazy.is_empty(),
        "these restate the requirement rather than saying what to do: {lazy:#?}"
    );
}

#[test]
fn no_check_is_listed_twice() {
    let mut seen = BTreeSet::new();
    let repeated: Vec<String> = human()
        .checks
        .iter()
        .filter(|c| !seen.insert(c.id.clone()))
        .map(|c| c.id.clone())
        .collect();
    assert!(repeated.is_empty(), "listed twice: {repeated:?}");
}

/// The catalogs took levels 1 and 2 only until an app could be held to level 3 (ADR-130); above its level, an app is not
/// asked about a requirement at all, so an entry there would be answered and counted against nothing. What keeps that
/// so now is applicability: the checklist is built from the requirements that apply, which leave out every one above
/// the app's level. This builds it at level 2 and at level 3 from the same answers, and holds both.
#[test]
fn a_level_three_entry_reaches_only_an_app_held_to_level_three() {
    use sv_frameworks::applicability::{ApplicabilityConfig, ConditionContext, bucket};
    use sv_frameworks::{Condition, Source};
    let f = frameworks();
    let config = ApplicabilityConfig::load_v2(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/knowledge"),
        &data("applicability-v2.json"),
    )
    .unwrap();
    let mut ctx = ConditionContext::default();
    for c in Condition::ALL
        .iter()
        .filter(|c| c.source() == Source::Claim)
    {
        ctx.set(*c, true);
    }
    let (notes, design, human) = (notes(), design(), human());
    let listed = |level: u8| -> Vec<(String, u8)> {
        let wanted: BTreeSet<String> = bucket(&f, &config, &ctx, level)
            .applicable
            .into_iter()
            .collect();
        checklist(&notes, &design, &human, &wanted)
            .into_iter()
            .map(|item| {
                let level = f.requirements.get(&item.id).map_or(0, |r| r.level);
                (item.id, level)
            })
            .collect()
    };
    let at_two = listed(2);
    let at_three = listed(3);
    // The setup: every catalog has a level-3 entry, so the level-2 list leaving them out is the level's doing.
    for id in ["V2.3.5", "V3.7.5", "V13.1.4"] {
        assert!(
            at_three.iter().any(|(i, _)| i == id),
            "{id} is not on the level-3 checklist"
        );
    }
    let above: Vec<&String> = at_two
        .iter()
        .filter(|(_, l)| *l > 2)
        .map(|(i, _)| i)
        .collect();
    assert!(above.is_empty(), "on a level-2 app's checklist: {above:?}");
    assert!(at_three.len() > at_two.len());
}

#[test]
fn every_check_is_written_for_somebody_who_is_not_a_programmer() {
    let jargon = ["canonicaliz", "idempoten", "nonce", "deserializ", "mitigat"];
    let mut unreadable = Vec::new();
    for c in &human().checks {
        let text = format!("{} {}", c.title, c.how).to_lowercase();
        for word in jargon {
            if text.contains(word) {
                unreadable.push(format!("{}: {word}", c.id));
            }
        }
    }
    assert!(unreadable.is_empty(), "{unreadable:#?}");
}

/// Every checklist row says where to go and look, not only what the answer should be.
///
/// The `human-checks.json` entries are instructions already. The others come from the security
/// notes and the design questions, which say what to write down or decide — the right thing for
/// those files, and no help at all to somebody who does not know where their session timeouts are
/// configured. So each of those carries a `howToFindOut` line too.
///
/// The owner asked for the level 2 ones. Writing them left the level 1 entries as the only rows on
/// the checklist with nothing but a question, which is backwards: level 1 is where somebody starts.
/// So the rule is every catalog entry that could reach the checklist, and this refuses one without.
#[test]
fn every_question_on_the_checklist_says_where_to_look() {
    let f = frameworks();
    let notes = notes();
    let design = design();
    let human: BTreeSet<String> = human().checks.iter().map(|c| c.id.clone()).collect();

    let mut silent = Vec::new();
    for (id, has) in notes
        .sections
        .iter()
        .map(|s| (s.id.clone(), s.how_to_find_out.is_some()))
        .chain(
            design
                .questions
                .iter()
                .map(|q| (q.id.clone(), q.how_to_find_out.is_some())),
        )
    {
        if human.contains(&id) || has {
            continue;
        }
        // Only the ones that can reach the checklist: an ASVS requirement, at any level since level 3 is an opt-in
        // (ADR-130).
        if f.requirements.contains_key(&id) && id.starts_with('V') {
            silent.push(id);
        }
    }
    assert!(
        silent.is_empty(),
        "these appear on the checklist with no `howToFindOut`, so they say what the answer should \
         be and not where to find it: {silent:?}"
    );
}

#[test]
fn a_where_to_look_line_says_where_rather_than_repeating_the_question() {
    // A line that restates the question is worse than none: it takes up a row and teaches the
    // reader that the second sentence is never worth reading.
    let places = [
        "look",
        "find",
        "open",
        "list",
        "start from",
        "go ",
        "search",
        "check",
        ".env",
        "configuration",
        "code",
        "dependency",
        "time ",
        "take ",
        "sign in",
    ];
    let mut lazy = Vec::new();
    for (id, how) in notes()
        .sections
        .iter()
        .filter_map(|s| s.how_to_find_out.clone().map(|h| (s.id.clone(), h)))
        .chain(
            design()
                .questions
                .iter()
                .filter_map(|q| q.how_to_find_out.clone().map(|h| (q.id.clone(), h))),
        )
    {
        let lower = how.to_lowercase();
        if !places.iter().any(|p| lower.contains(p)) {
            lazy.push(format!("{id}: {how}"));
        }
    }
    assert!(lazy.is_empty(), "these do not say where to look: {lazy:#?}");
}

/// Every requirement no check may ever settle is explained by some catalog.
///
/// The other direction from everything above. `manualOnly` in `applicability.json` lists the
/// requirements a tool can never settle, so a person is the only way any of them gets an answer, and
/// a person needs to be told how. Ten were in no catalog when this was found (26 September 2026),
/// nine of them AISVS: the framework grew and the catalogs did not follow. This names the next one as
/// it is added. Level 3 joined on 21 October 2026, once an app could be held to it (ADR-130).
#[test]
fn every_requirement_only_a_person_can_settle_is_explained_somewhere() {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/knowledge/applicability.json");
    let applicability: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    let manual: Vec<String> = applicability["manualOnly"]
        .as_array()
        .expect("applicability.json has a manualOnly list")
        .iter()
        .map(|v| v.as_str().unwrap().to_owned())
        .collect();
    assert!(manual.len() > 20, "the list did not load: {manual:?}");
    let explained: BTreeSet<String> = human()
        .checks
        .iter()
        .map(|c| c.id.clone())
        .chain(notes().sections.iter().map(|s| s.id.clone()))
        .chain(design().questions.iter().map(|q| q.id.clone()))
        .collect();
    let f = frameworks();
    let unexplained: Vec<String> = manual
        .iter()
        .filter(|id| f.requirements.contains_key(*id))
        .filter(|id| !explained.contains(*id))
        .cloned()
        .collect();
    assert!(
        unexplained.is_empty(),
        "only a person can settle these, and no catalog tells them how: {unexplained:?}. Add a \
         line to human-checks.json, security-notes.json, or design-questions.json."
    );
}

#[test]
fn the_live_sites_ciphers_are_left_to_a_scanner_the_owner_runs() {
    // The owner's decision, 6 October 2026: V12.1.2 takes dozens of handshakes, `sv probe` keeps
    // to four, so the report says how to check it with a tool made for it rather than staying silent.
    let checks = human();
    let ciphers = checks
        .checks
        .iter()
        .find(|c| c.id == "V12.1.2")
        .expect("V12.1.2 has an instruction");
    assert!(ciphers.how.contains("testssl.sh"), "{}", ciphers.how);
    assert!(
        ciphers.how.contains("`sv` does not check this"),
        "{}",
        ciphers.how
    );
    // The control: V12.1.1, which `sv probe` does ask about, is not left to a scanner here.
    assert!(!checks.checks.iter().any(|c| c.id == "V12.1.1"));
}

#[test]
fn the_requirements_the_gap_analysis_named_have_instructions() {
    // Gap analysis 6.4 (7 October 2026): requirements no check settles, and nobody was told how to
    // check by hand. Each now has a line in one of the three catalogs, and losing one is caught here.
    let named = [
        "V2.2.1", "V1.3.3", "V1.3.5", "V1.3.8", "V6.5.2", "V6.5.3", "V8.4.1", "V11.6.1", "V13.3.2",
        "V16.3.4", "C2.1.1", "C2.1.2", "C2.1.4", "C2.1.5", "C2.2.1", "C2.2.2", "C7.4.1", "C7.4.2",
        "C9.1.1", "C9.2.1", "C9.3.1", "C10.1.1", "C10.2.2", "C10.2.3", "C10.3.1", "C10.3.2",
    ];
    let explained: BTreeSet<String> = human()
        .checks
        .iter()
        .map(|c| c.id.clone())
        .chain(notes().sections.iter().map(|s| s.id.clone()))
        .chain(design().questions.iter().map(|q| q.id.clone()))
        .collect();
    let missing: Vec<&str> = named
        .iter()
        .copied()
        .filter(|id| !explained.contains(*id))
        .collect();
    assert!(missing.is_empty(), "no instruction for {missing:?}");
}
