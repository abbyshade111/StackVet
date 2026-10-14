//! The short version's count of requirements only a run that did not happen could reach counts an
//! outside tool's reach only for an app in a language that tool reads (backlog 0471): V15.3.5 and
//! V15.3.6 are credited by CodeQL for JavaScript alone, and a Python app was told "outside tools"
//! could have reached them.

use std::collections::BTreeSet;
use std::path::PathBuf;
use sv_check::Verified;
use sv_frameworks::Frameworks;
use sv_frameworks::applicability::Buckets;
use sv_report::{Inputs, Reach, RunStatus, build};

fn frameworks() -> Frameworks {
    Frameworks::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/frameworks"))
        .expect("the OWASP data loads")
}

fn inputs<'a>(
    frameworks: &'a Frameworks,
    buckets: &'a Buckets,
    verified: &'a [Verified],
) -> Inputs<'a> {
    Inputs {
        set_aside: Vec::new(),
        test_information: Vec::new(),
        reviews_not_counted: Vec::new(),
        app_name: "Test",
        on_the_internet: false,
        ai_tool: Default::default(),
        target_level: 3,
        generated: None,
        made_by: Default::default(),
        run_note: None,
        run_steps: Vec::new(),
        test_output: None,
        run_status: None,
        coding_rules_cited: Default::default(),
        frameworks,
        buckets,
        claims: &[],
        findings: Vec::new(),
        verified,
        gaps: vec![],
        manual_only: Default::default(),
        named_in_tests: Default::default(),
        not_for_tests: Default::default(),
        human: None,
        threats: None,
    }
}

fn langs(names: &[&str]) -> BTreeSet<String> {
    names.iter().map(|s| (*s).to_owned()).collect()
}

#[test]
fn a_tool_of_another_language_is_not_counted_as_a_reach() {
    let f = frameworks();
    let buckets = Buckets {
        applicable: vec!["V15.3.5".into(), "V3.3.1".into()],
        ..Default::default()
    };
    let reach = Reach {
        requirements: [
            ("V15.3.5".to_owned(), vec!["tools".to_owned()]),
            ("V3.3.1".to_owned(), vec!["tools".to_owned()]),
        ]
        .into_iter()
        .collect(),
        tools_languages: [
            (
                "V15.3.5".to_owned(),
                vec!["javascript".to_owned(), "typescript".to_owned()],
            ),
            ("V3.3.1".to_owned(), vec!["*".to_owned()]),
        ]
        .into_iter()
        .collect(),
    };
    let mut i = inputs(&f, &buckets, &[]);
    i.run_status = Some(RunStatus::NotAsked {
        why: "not asked".to_owned(),
    });
    let report = build(i);
    let count = |app: &[&str]| {
        sv_report::not_run_this_time(&report, &reach, false, &langs(app))
            .unwrap()
            .only_they_reach
    };
    // A Python app: only the tool of any language could have reached anything.
    assert_eq!(count(&["python"]), 1, "V3.3.1 alone");
    // A TypeScript app: both.
    assert_eq!(count(&["typescript"]), 2);
}

#[test]
fn the_generated_file_records_the_languages_of_the_javascript_only_reach() {
    // The setup the count rests on: coverage.py wrote what CodeQL for JavaScript reads.
    let text = std::fs::read_to_string(sv_frameworks::data::file("reach.json")).unwrap();
    let reach = sv_report::read_reach(&text).unwrap();
    let v15_3_5 = reach
        .tools_languages
        .get("V15.3.5")
        .expect("V15.3.5 has a tool reach");
    assert!(
        !v15_3_5.iter().any(|l| l == "python" || l == "*"),
        "{v15_3_5:?}"
    );
    assert!(v15_3_5.iter().any(|l| l == "javascript"), "{v15_3_5:?}");
}
