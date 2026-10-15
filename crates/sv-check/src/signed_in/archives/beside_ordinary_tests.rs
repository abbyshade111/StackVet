//! An archive past the stated limits is judged against the ordinary archive, never by "under 400" alone
//! (backlog 0560, ADR-119).

use super::super::fake_app::*;
use super::tests::stated;
use super::*;

#[test]
fn an_archive_refused_on_the_form_with_200_is_not_an_archive_accepted() {
    let o = run_against(
        Flaws {
            upload_refusals_on_200: true,
            ..Default::default()
        },
        &stated(),
    );
    assert!(
        o.steps
            .iter()
            .any(|s| s.starts_with("uploaded an ordinary small zip")),
        "{:?}",
        o.steps
    );
    assert!(
        !o.findings
            .iter()
            .any(|f| f.rule_id == ARCHIVE_UNCHECKED.rule_id),
        "{:?}",
        o.findings
    );
    assert!(
        o.not_assessed.iter().any(|(id, why)| id == "V5.2.3"
            && why.contains("was answered unlike the ordinary zip")
            && why.contains("with a form on it")),
        "{:?}",
        o.not_assessed
    );
}
