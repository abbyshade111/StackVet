//! The served-file checks judge three files each, not one (backlog 0232, part 4; ADR-053, Later): `.php`, `.phtml`,
//! and `.php5` for V5.3.1, and `.html`, `.htm`, and `.xhtml` for V3.2.1. Whole only when every file was judged safe.

use super::super::fake_app::*;
use super::*;

fn with_upload() -> UsersSection {
    let mut u = users();
    u.upload = Some(UploadSection {
        path: "/upload".into(),
        field: "file".into(),
        form: [("csrf_token".to_owned(), "{csrf}".to_owned())].into(),
        serves_at: Some("/files/{name}".into()),
        max_bytes: Some(UPLOAD_LIMIT as u64),
        ..Default::default()
    });
    u
}

fn run(flaws: Flaws) -> Outcome {
    run_against(flaws, &with_upload())
}

fn credit<'a>(o: &'a Outcome, rule: &str) -> Option<&'a crate::Verified> {
    o.verified.iter().find(|v| v.check_id == rule)
}

#[test]
fn a_correct_app_is_credited_whole_for_each_set_naming_every_file() {
    let o = run(Flaws::default());
    for (rule, names) in [
        (
            UPLOAD_EXECUTED.rule_id,
            ["sv-probe.php", "sv-probe.phtml", "sv-probe.php5"],
        ),
        (
            UPLOAD_RENDERED.rule_id,
            ["sv-probe.html", "sv-probe.htm", "sv-probe.xhtml"],
        ),
    ] {
        let credit = credit(&o, rule).unwrap_or_else(|| panic!("{rule}: {:?}", o.not_assessed));
        assert!(!credit.in_part, "{credit:?}");
        for name in names {
            assert!(
                credit.scope.contains(name),
                "{name} missing: {}",
                credit.scope
            );
        }
    }
}

#[test]
fn one_file_of_the_set_run_by_the_server_is_found_naming_it() {
    let o = run(Flaws {
        runs_uploaded_php5: true,
        ..Default::default()
    });
    let found = o
        .findings
        .iter()
        .find(|f| f.rule_id == UPLOAD_EXECUTED.rule_id)
        .expect("the finding");
    assert_eq!(found.evidence, ["upload-code-2-fetch"]);
    assert!(
        found.description.contains("`.php5`"),
        "{}",
        found.description
    );
    assert!(
        credit(&o, UPLOAD_EXECUTED.rule_id).is_none(),
        "credited beside its own finding"
    );
}

#[test]
fn xhtml_served_as_its_own_type_is_a_page_a_browser_renders() {
    let o = run(Flaws {
        renders_uploaded_xhtml: true,
        ..Default::default()
    });
    let found = o
        .findings
        .iter()
        .find(|f| f.rule_id == UPLOAD_RENDERED.rule_id)
        .expect("the finding");
    assert_eq!(found.evidence, ["upload-page-file-2-fetch"]);
    assert!(
        found.description.contains("application/xhtml+xml"),
        "{}",
        found.description
    );
}

#[test]
fn a_file_of_the_set_refused_leaves_the_credit_in_part() {
    let o = run(Flaws {
        refuses_phtml: true,
        ..Default::default()
    });
    let credit = credit(&o, UPLOAD_EXECUTED.rule_id).expect("credited for the two it saw");
    assert!(credit.in_part, "{credit:?}");
    assert!(!credit.scope.contains(".phtml"), "{}", credit.scope);
    assert!(
        o.not_assessed
            .iter()
            .any(|(_, why)| why.contains("refused `sv-probe.phtml`")),
        "{:?}",
        o.not_assessed
    );
}
