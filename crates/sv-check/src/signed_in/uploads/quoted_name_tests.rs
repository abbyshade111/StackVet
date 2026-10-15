//! Backlog 0571 (from sweep 3, 0561): a name kept whole in `Content-Disposition` may only have been quoted, and a `"`
//! in the name then ends the quoted string. A second file, named with a `"` before the `;`, settles it.

use super::super::fake_app::*;
use super::tests::with_upload;
use super::*;

fn run(flaws: Flaws) -> Outcome {
    run_against(
        flaws,
        &with_upload(Some("/files/{name}"), Some(UPLOAD_LIMIT as u64)),
    )
}

fn credit(o: &Outcome) -> Option<&crate::Verified> {
    o.verified
        .iter()
        .find(|v| v.check_id == DOWNLOAD_NAME_INJECTED.rule_id)
}

#[test]
fn a_name_quoted_without_escaping_its_quote_is_found() {
    let o = run(Flaws {
        download_name_quoted_unescaped: true,
        ..Default::default()
    });
    // The setup: the name with a `;` alone came back whole, so only the quote can have broken it.
    assert!(
        o.steps.iter().any(|s| s
            == "fetched a file named `sv-probe;svinjected=1.gif` back: its name was served intact"),
        "{:#?}",
        o.steps
    );
    let found = o
        .findings
        .iter()
        .find(|f| f.rule_id == DOWNLOAD_NAME_INJECTED.rule_id)
        .unwrap_or_else(|| panic!("{:#?}", o.steps));
    assert!(
        found.evidence.iter().any(|e| e == "download-quoted"),
        "{found:?}"
    );
    assert!(credit(&o).is_none(), "{:?}", credit(&o));
}

#[test]
fn a_name_quoted_and_escaped_is_credited_for_both_names() {
    let o = run(Flaws {
        download_name_quoted_uncleaned: true,
        ..Default::default()
    });
    assert!(
        !o.findings
            .iter()
            .any(|f| f.rule_id == DOWNLOAD_NAME_INJECTED.rule_id),
        "{:?}",
        o.findings
    );
    let credit = credit(&o).unwrap_or_else(|| panic!("{:#?}", o.steps));
    assert!(credit.scope.contains("and one named"), "{}", credit.scope);
}

#[test]
fn a_cleaned_name_is_credited_for_both_names() {
    let o = run(Flaws::default());
    let credit = credit(&o).unwrap_or_else(|| panic!("{:#?}", o.steps));
    assert!(credit.scope.contains("and one named"), "{}", credit.scope);
}
