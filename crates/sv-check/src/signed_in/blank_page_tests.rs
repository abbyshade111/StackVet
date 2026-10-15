//! Backlog 0598 (ADR-021): a page the app began and failed to send, an answer shorter than its `Content-Length`, reaches
//! the checks as no answer, and none of them reports what the blank page lacks. In the clean Cursor + Flash run of
//! 15 October 2026 the private page came blank and the report found "no sign-out link" on it.

use super::fake_app::Flaws;
use super::tests::run_with;

/// What the private-page checks find about the page itself: its headers, its caching, its sign-out link, and whether a
/// stranger can open it.
const ABOUT_THE_PAGE: &[&str] = &[
    "probe.private-page-headers",
    "probe.private-page-shared-cache",
    "probe.private-page-cached",
    "probe.no-sign-out-link",
    "probe.private-page-anonymous",
];

fn judged(out: &super::Outcome) -> Vec<&str> {
    out.findings
        .iter()
        .map(|f| f.rule_id.as_str())
        .chain(out.verified.iter().map(|v| v.check_id.as_str()))
        .filter(|id| ABOUT_THE_PAGE.contains(id))
        .collect()
}

#[test]
fn a_blank_private_page_is_judged_for_nothing_it_lacks() {
    // The setup: the same run with the page whole judges it, so what is missing below is the blank page's doing.
    let whole = run_with(Flaws::default(), &Default::default());
    assert!(
        !judged(&whole).is_empty(),
        "the page was never judged even whole"
    );

    let blank = run_with(
        Flaws {
            account_page_blank: true,
            ..Flaws::default()
        },
        &Default::default(),
    );
    let found: Vec<&str> = blank
        .findings
        .iter()
        .map(|f| f.rule_id.as_str())
        .filter(|id| ABOUT_THE_PAGE.contains(id))
        .collect();
    assert!(
        found.is_empty(),
        "a blank page was found lacking: {found:?}"
    );
    // And the run says it could not go on, rather than nothing.
    assert!(
        !blank.not_assessed.is_empty(),
        "nothing was said of what could not be judged"
    );
}
