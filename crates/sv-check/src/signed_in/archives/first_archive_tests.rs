//! The ordinary archive is the gate the archive checks rest on, so it is not taken as kept on "under 400" alone
//! when its answer is the upload form again (backlog 0560, ADR-119).

use super::super::fake_app::*;
use super::tests::stated;
use super::*;

#[test]
fn an_ordinary_archive_answered_with_the_form_again_is_not_taken_as_kept() {
    let o = run_against(
        Flaws {
            uploads_off_on_200: true,
            ..Default::default()
        },
        &stated(),
    );
    assert!(
        o.not_assessed.iter().any(|(id, why)| id == "V5.2.3"
            && why.contains("ordinary small zip")
            && why.contains("with the upload form again")),
        "{:?}",
        o.not_assessed
    );
    assert!(
        !o.findings
            .iter()
            .any(|f| f.rule_id == ARCHIVE_UNCHECKED.rule_id),
        "{:?}",
        o.findings
    );
    assert!(
        !o.verified
            .iter()
            .any(|v| v.check_id == ARCHIVE_UNCHECKED.rule_id),
        "{:?}",
        o.verified
    );
}
