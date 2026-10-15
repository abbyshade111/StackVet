//! A private page served to somebody not signed in is read beside the same page served to the signed-in user
//! (backlog 0560, ADR-119): a sign-in form served with 200 at the private address is a 2xx too.

use super::fake_app::*;
use super::*;

#[test]
fn a_sign_in_form_served_with_200_is_not_the_page_opening() {
    let o = run_against(
        Flaws {
            private_signin_on_200: true,
            ..Default::default()
        },
        &users(),
    );
    assert!(
        !rule_ids(&o).contains(&PRIVATE_PAGE.rule_id),
        "a sign-in form on a 200 was read as the private page opening: {:?}",
        o.findings
    );
    // Not credited either: a page answered 200 to somebody not signed in is not shown refused.
    assert!(!verified_ids(&o).contains(&PRIVATE_PAGE.rule_id));
    assert!(
        o.not_assessed.iter().any(|(_, why)| why.contains(
            "/account answered 200 with a page that shows little of what the signed-in user"
        )),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn the_page_itself_served_without_signing_in_is_still_found() {
    let o = run_against(
        Flaws {
            private_open: true,
            ..Default::default()
        },
        &users(),
    );
    assert!(
        rule_ids(&o).contains(&PRIVATE_PAGE.rule_id),
        "{:?}",
        o.not_assessed
    );
}

/// The fake app, silent to every sign-in request: no answer at all, as from an app that went away.
struct SilentSignIn(FakeApp);

impl Http for SilentSignIn {
    fn send(&mut self, r: &ProbeRequest) -> Option<ProbeResponse> {
        if r.path == "/login" && r.method == "POST" {
            return None;
        }
        self.0.send(r)
    }
}

#[test]
fn a_page_open_to_everybody_is_found_even_when_signing_in_gets_no_answer() {
    // No signed-in page to read it beside: a 2xx is a finding, as before.
    let acc = accounts();
    let mut app = FakeApp::new(Flaws {
        private_open: true,
        ..Default::default()
    });
    app.users
        .insert(acc.a.user.clone(), (acc.a.password.clone(), false));
    let mut silent = SilentSignIn(app);
    let o = run(&mut silent, &users(), &acc, true, &Default::default());
    assert!(
        o.not_assessed
            .iter()
            .any(|(_, why)| why.contains("Signing in as the first test user got no answer")),
        "signing in was meant to get no answer: {:?}",
        o.not_assessed
    );
    assert!(
        rule_ids(&o).contains(&PRIVATE_PAGE.rule_id),
        "{:?}",
        o.findings
    );
}
