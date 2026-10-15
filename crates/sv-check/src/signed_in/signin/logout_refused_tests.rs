//! A sign-out answered with a page that changed nothing the browser holds is not judged (backlog 0560,
//! ADR-119): a sign-out refused on a 200 page ("The CSRF token is missing.") is under 400 too, and the session
//! working afterwards would be blamed on the app.

use super::super::fake_app::*;
use super::*;

#[test]
fn a_sign_out_refused_on_a_200_page_is_not_a_session_that_outlived_it() {
    let o = run_against(
        Flaws {
            logout_refused_on_200: true,
            ..Default::default()
        },
        &users(),
    );
    assert!(
        o.steps.iter().any(|s| s == "A signed out (200)"),
        "{:#?}",
        o.steps
    );
    assert!(
        !rule_ids(&o).contains(&LOGOUT.rule_id),
        "a refused sign-out was read as one that ended nothing: {:?}",
        o.findings
    );
    assert!(!verified_ids(&o).contains(&LOGOUT.rule_id));
    assert!(
        o.not_assessed.iter().any(|(id, why)| id == "V7.4.1"
            && why
                .contains("answered 200 with a page, and the browser's own session still opened")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_sign_out_that_redirects_away_and_keeps_the_session_is_still_found() {
    let o = run_against(
        Flaws {
            logout_keeps_session: true,
            ..Default::default()
        },
        &users(),
    );
    assert!(
        rule_ids(&o).contains(&LOGOUT.rule_id),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_sign_out_refused_with_a_redirect_back_to_the_page_is_not_judged_either() {
    let o = run_against(
        Flaws {
            logout_refused_back: true,
            ..Default::default()
        },
        &users(),
    );
    assert!(!rule_ids(&o).contains(&LOGOUT.rule_id), "{:?}", o.findings);
    assert!(
        o.not_assessed.iter().any(|(id, why)| id == "V7.4.1"
            && why.contains("answered 303 with a redirect back to a private page")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_sign_out_that_sends_the_browser_away_and_changes_nothing_is_still_found() {
    // Neither a page nor a redirect back: the sign-out was answered as done, so the session working
    // afterwards is the app's, even though the browser's own cookie was not taken away.
    let o = run_against(
        Flaws {
            logout_does_nothing: true,
            ..Default::default()
        },
        &users(),
    );
    assert!(
        rule_ids(&o).contains(&LOGOUT.rule_id),
        "{:?}",
        o.not_assessed
    );
}
