//! A report on somebody else's project says whose it is on every page, counts risks to weigh rather than things to fix,
//! never reads nothing found as safe, and hands the reader none of the owner's work (backlog 0670, part 2; ADR-154).

use super::*;
use crate::Counts;
use crate::bluf::tests::{a_finding, report};

const SHA: &str = "0123456789abcdef0123456789abcdef01234567";

pub(super) fn fetched(answers: Answers) -> Report {
    let mut r = report(Counts {
        applicable: 40,
        not_verified: 30,
        not_assessed: 12,
        stated: 2,
        ..Counts::default()
    });
    r.fetched = Some(FetchedProject {
        address: "https://github.com/owner/project".to_owned(),
        commit: Some(SHA.to_owned()),
        answers,
    });
    r
}

/// Every page of the report, as it is written.
pub(super) fn pages(r: &Report) -> Vec<(&'static str, String)> {
    vec![
        ("report.html", crate::html::page(r)),
        ("compliance.md", crate::markdown::compliance(r)),
        ("security.md", crate::markdown::security(r)),
        ("low-findings.md", crate::markdown::low_findings(r)),
        ("fix-now.md", crate::pages::fix_now(r)),
        ("your-decisions.md", crate::pages::decisions(r)),
        ("not-examined.md", crate::pages::not_examined(r)),
        ("information.md", crate::pages::information(r)),
        ("before-going-live.md", crate::checklist::render(r)),
    ]
}

#[test]
fn every_page_names_the_project_its_commit_and_how_to_read_it() {
    let r = fetched(Answers::Nobody);
    for (name, text) in pages(&r) {
        for said in [
            "https://github.com/owner/project",
            SHA,
            "somebody else's project",
            "Nothing found is not the same as safe",
        ] {
            let said = if name.ends_with(".html") {
                crate::html::escape(said)
            } else {
                said.to_owned()
            };
            assert!(text.contains(&said), "{name} does not say {said:?}");
        }
        assert!(text.contains("nobody has answered its questions"), "{name}");
    }
    let own = fetched(Answers::Author);
    for (name, text) in pages(&own) {
        assert!(text.contains("its author"), "{name}");
        assert!(!text.contains("carried no stackvet.toml"), "{name}");
    }
    // An app of the person's own says none of it.
    let mine = report(Counts::default());
    for (name, text) in pages(&mine) {
        assert!(!text.contains("somebody else"), "{name}");
    }
}

#[test]
fn the_short_version_counts_risks_and_never_reads_as_safe() {
    let mut r = fetched(Answers::Nobody);
    let clean = crate::bluf::headline(&r);
    assert!(
        clean.starts_with(
            "Nothing here found a problem in this project. That does not make it safe to use"
        ),
        "{clean}"
    );
    assert!(clean.contains("30 of the 40 requirements"), "{clean}");
    assert!(
        clean.contains("12 more could not be placed without the author's answers"),
        "{clean}"
    );
    r.findings = vec![a_finding("ast.eval"), a_finding("ast.exec")];
    let found = crate::bluf::headline(&r);
    assert!(
        found.starts_with("2 risks to weigh were found in the project's own code"),
        "{found}"
    );
    for line in [&clean, &found] {
        for never in ["to fix", "is safe", "passed", "sound"] {
            assert!(!line.contains(never), "{never:?} in {line}");
        }
    }
}

#[test]
fn the_next_steps_are_the_reader_s_not_the_owner_s() {
    let mut r = fetched(Answers::Nobody);
    r.findings = vec![a_finding("ast.eval")];
    let steps = crate::bluf::next_steps(&r);
    let said: Vec<String> = steps
        .iter()
        .map(|s| format!("{} ({})", s.what, s.where_to_look))
        .collect();
    assert!(said[0].starts_with("Weigh the 1 risk found"), "{said:?}");
    assert!(
        said.iter().any(|s| s.contains("None of them passed")),
        "{said:?}"
    );
    assert!(said.iter().any(|s| s.contains("never started")), "{said:?}");
    for owners in [
        "Fix the",
        "Answer the",
        "write tests",
        "sv review",
        "Record the answers",
        "--run",
    ] {
        assert!(
            !said.iter().any(|s| s.contains(owners)),
            "{owners:?} is the owner's work: {said:?}"
        );
    }
    // The same report of the person's own app keeps the owner's steps.
    r.fetched = None;
    assert!(crate::bluf::next_steps(&r)[0].what.starts_with("Fix the"));
}

#[test]
fn a_project_that_was_started_says_so_above_everything() {
    let mut r = fetched(Answers::Author);
    let quiet = banner(&r).unwrap();
    assert!(
        quiet[0].starts_with("This is somebody else's project"),
        "{quiet:?}"
    );
    assert!(quiet[0].contains("without running any of it"), "{quiet:?}");
    r.run_status = Some(crate::RunStatus::Started {
        image: "python:3.12-slim".to_owned(),
        asked: 3,
        answered: 3,
        signed_in: false,
        tests: "not-declared".to_owned(),
    });
    let ran = banner(&r).unwrap();
    assert!(ran[0].starts_with("Code nobody vetted was run"), "{ran:?}");
    assert!(ran[1].contains("then by running it"), "{ran:?}");
    for (name, text) in pages(&r) {
        let said = if name.ends_with(".html") {
            crate::html::escape("Code nobody vetted was run")
        } else {
            "Code nobody vetted was run".to_owned()
        };
        assert!(text.contains(&said), "{name}");
    }
    assert!(
        !crate::bluf::next_steps(&r)
            .iter()
            .any(|s| s.what.contains("never started")),
        "a project that was started is not said never to have been"
    );
}
