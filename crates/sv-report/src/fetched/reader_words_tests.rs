//! Under the banner, no page of a report on a fetched project speaks to it as the reader's own app (backlog 0676): the
//! level does not say stackvet.toml answered what it never did, or ask the reader to confirm it; no finding is told to
//! be fixed or reviewed; and the build history says nothing of a build the reader had no part in.

use super::tests::{fetched, pages};
use super::*;
use crate::bluf::tests::a_finding;

/// A fetched project as the first one was: no stackvet.toml of its own, so the level rests on the defaults, with a
/// finding and no record of its build.
fn as_first_fetched(answers: Answers) -> Report {
    let mut r = fetched(answers);
    r.target_level = 2;
    r.level_why = Some(crate::LevelWhy {
        because: sv_manifest::Manifest::default().level_because(),
        level_two_more: 0,
        hints: Vec::new(),
        confirmed: None,
    });
    r.findings = vec![a_finding("ast.eval")];
    r.build_loop = Some(crate::BuildLoop::default());
    r
}

/// The words of a report on the reader's own app that are not the reader's to act on, or not true, of somebody
/// else's project.
const THE_OWNER_S: [&str; 6] = [
    "stackvet.toml says customers use it",
    "confirm them with `sv review`",
    "Fix it in the code and check again",
    "confirm it with `sv review` in your own terminal",
    "while this app was built",
    "used while the app was built",
];

#[test]
fn no_page_speaks_to_a_fetched_project_as_the_reader_s_own() {
    for answers in [Answers::Nobody, Answers::Author, Answers::Changed] {
        let r = as_first_fetched(answers);
        // The setup: each passage has something to say, so leaving it out is the change and not an empty report.
        assert!(r.level_why.is_some() && !r.findings.is_empty() && r.build_loop.is_some());
        for (name, text) in pages(&r) {
            let text = text.replace("&#39;", "'").replace("&#x27;", "'");
            for owners in THE_OWNER_S {
                assert!(
                    !text.contains(owners),
                    "{answers:?}, {name} says {owners:?}"
                );
            }
        }
    }
    // The same report of the person's own app keeps the owner's words: the passages are there to leave out.
    let mut mine = as_first_fetched(Answers::Nobody);
    mine.fetched = None;
    let all: String = pages(&mine).into_iter().map(|(_, t)| t).collect();
    for owners in [
        "confirm them with `sv review`",
        "Fix it in the code and check again",
        "while this app was built",
    ] {
        assert!(
            all.contains(owners),
            "the owner's report does not say {owners:?}"
        );
    }
}

#[test]
fn the_level_says_whose_answers_it_rests_on() {
    let nobody = crate::bluf::held_to(&as_first_fetched(Answers::Nobody));
    assert!(
        nobody.contains("nobody has said who uses this project"),
        "{nobody}"
    );
    assert!(nobody.contains("not a judgment of the project"), "{nobody}");
    let author = crate::bluf::held_to(&as_first_fetched(Answers::Author));
    assert!(
        author.contains("its author's word, which sv did not check"),
        "{author}"
    );
    let changed = crate::bluf::held_to(&as_first_fetched(Answers::Changed));
    assert!(
        changed.contains("changed since the project was fetched"),
        "{changed}"
    );
}

#[test]
fn a_finding_is_a_risk_to_weigh() {
    let r = as_first_fetched(Answers::Nobody);
    let step = crate::next_step(&r, &r.findings[0]).expect("a finding has a next step");
    assert!(
        step.starts_with("A risk to weigh, not yours to fix"),
        "{step}"
    );
    assert!(!step.contains("sv review"), "{step}");
}

#[test]
fn a_record_the_project_brought_is_said_to_be_its_author_s() {
    let mut r = as_first_fetched(Answers::Author);
    assert_eq!(crate::build_loop_line(&r), None);
    r.build_loop = Some(crate::BuildLoop {
        calls: 3,
        checks: 2,
        ..crate::BuildLoop::default()
    });
    let line = crate::build_loop_line(&r).expect("a record with calls in it is said");
    assert!(
        line.starts_with("The project came with a record of how it was built"),
        "{line}"
    );
    let (lead, _) = crate::build_loop_lines(&r).expect("and in two parts");
    assert!(lead.starts_with("The project came with a record"), "{lead}");
}
