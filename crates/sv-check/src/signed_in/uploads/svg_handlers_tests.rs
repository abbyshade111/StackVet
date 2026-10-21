//! Backlog 0232, part D: the uploaded SVG carries four ways of running code, so a cleaner that strips the tags and keeps
//! the attributes is found. `<script>` and `<foreignObject>` removed, with an `onload` handler and a `javascript:` link
//! kept, is still an image that runs code when it is opened.

use super::super::fake_app::*;
use super::tests::with_upload;
use super::*;

fn run(flaws: Flaws) -> Outcome {
    run_against(flaws, &with_upload(Some("/files/{name}"), None))
}

#[test]
fn a_cleaner_that_strips_tags_and_keeps_handlers_is_found_and_credits_nothing() {
    let o = run(Flaws {
        svg_handlers_kept: true,
        ..Flaws::default()
    });
    let found = o
        .findings
        .iter()
        .find(|f| f.rule_id == UPLOAD_SVG_SCRIPT.rule_id)
        .unwrap_or_else(|| panic!("{:#?}", o.steps));
    // What it kept, named; what it removed, not.
    for kept in ["`onload` handler", "`javascript:` link"] {
        assert!(found.description.contains(kept), "{}", found.description);
    }
    for removed in ["`<script>`", "`<foreignObject>`"] {
        assert!(
            !found.description.contains(removed),
            "{}",
            found.description
        );
    }
    assert!(
        !o.verified
            .iter()
            .any(|v| v.check_id == UPLOAD_SVG_SCRIPT.rule_id)
    );
}

#[test]
fn a_cleaner_that_removes_all_four_is_credited_in_part() {
    // The setup: the same run with the default cleaner, which removes every one of the four.
    let o = run(Flaws::default());
    assert!(
        o.findings
            .iter()
            .all(|f| f.rule_id != UPLOAD_SVG_SCRIPT.rule_id),
        "{:#?}",
        o.findings
    );
    let credit = o
        .verified
        .iter()
        .find(|v| v.check_id == UPLOAD_SVG_SCRIPT.rule_id)
        .unwrap_or_else(|| panic!("{:#?}", o.steps));
    assert!(
        credit.in_part && credit.scope.contains("all four removed"),
        "{credit:?}"
    );
}
