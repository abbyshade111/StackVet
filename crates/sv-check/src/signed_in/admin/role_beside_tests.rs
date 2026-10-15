//! A role written into sign-up or the email change is found only when the admin page that then opens shows what the
//! admin is served there (backlog 0560, ADR-119): an "awaits review" page served with 200 is a 2xx too.

use super::super::fake_app::*;
use super::super::tests::with_signup;
use super::*;

/// Sign-up, an email change, and an admin page, with the admin made from `seed`.
fn users_with_admin() -> UsersSection {
    let mut u = with_signup();
    u.seed = Some("seed".into());
    u.admin = vec!["/admin".into()];
    u
}

fn held() -> Outcome {
    run_against(
        Flaws {
            role_request_held_on_200: true,
            ..Default::default()
        },
        &users_with_admin(),
    )
}

fn said<'a>(o: &'a Outcome, starts: &str) -> Option<&'a String> {
    o.not_assessed
        .iter()
        .find(|(ids, why)| ids == "V8.3.1, V15.3.3" && why.starts_with(starts))
        .map(|(_, why)| why)
}

#[test]
fn a_sign_up_held_for_review_on_a_200_page_is_neither_found_nor_credited() {
    let o = held();
    assert!(
        !o.findings.iter().any(|f| f.rule_id == ROLE_FIELD.rule_id),
        "{:?}",
        o.findings
    );
    assert!(
        !o.verified.iter().any(|v| v.check_id == ROLE_FIELD.rule_id),
        "{:?}",
        o.verified
    );
    let why = said(&o, "A role written into the sign-up form").expect("said not judged");
    assert!(
        why.contains("answered 200 at /admin") && why.contains("shows little of what the admin"),
        "{why}"
    );
}

#[test]
fn an_email_change_held_for_review_on_a_200_page_is_neither_found_nor_credited() {
    let o = held();
    assert!(
        !o.findings
            .iter()
            .any(|f| f.rule_id == EMAIL_ROLE_FIELD.rule_id),
        "{:?}",
        o.findings
    );
    assert!(
        !o.verified
            .iter()
            .any(|v| v.check_id == EMAIL_ROLE_FIELD.rule_id),
        "{:?}",
        o.verified
    );
    // The setup: the change was sent from an account refused the admin page.
    assert!(
        o.steps
            .iter()
            .any(|s| s.starts_with("an account refused 1 admin page sent its email change")),
        "{:?}",
        o.steps
    );
    let why = said(&o, "A role written into the email change").expect("said not judged");
    assert!(why.contains("answered 200 at /admin"), "{why}");
}

/// The fake app, keeping the id of every request sent to it.
struct Recording {
    app: FakeApp,
    ids: Vec<String>,
}

impl Http for Recording {
    fn send(&mut self, r: &ProbeRequest) -> Option<ProbeResponse> {
        self.ids.push(r.id.clone());
        self.app.send(r)
    }
    fn now(&mut self) -> u64 {
        self.app.now()
    }
    fn wait(&mut self, seconds: u64) {
        self.app.wait(seconds);
    }
}

#[test]
fn a_role_given_is_found_beside_the_admins_own_page() {
    // The control: the page the role opens is the admin's page, read by the admin earlier in the run.
    let acc = accounts();
    let mut app = FakeApp::new(Flaws {
        signup_trusts_role: true,
        email_change_trusts_role: true,
        ..Default::default()
    });
    for (account, admin) in [(&acc.a, false), (&acc.b, false)] {
        app.users
            .insert(account.user.clone(), (account.password.clone(), admin));
    }
    let admin = acc.admin.clone().unwrap();
    app.users.insert(admin.user, (admin.password, true));
    let mut http = Recording {
        app,
        ids: Vec::new(),
    };
    let o = run(
        &mut http,
        &users_with_admin(),
        &acc,
        true,
        &Default::default(),
    );
    for rule in [ROLE_FIELD.rule_id, EMAIL_ROLE_FIELD.rule_id] {
        assert!(
            o.findings.iter().any(|f| f.rule_id == rule),
            "{rule}: {:?}",
            o.not_assessed
        );
    }
    // The setup: the admin's own read happened before either role check asked the page.
    let at = |id: &str| http.ids.iter().position(|r| r == id);
    let admins = at("admin-admin-0").expect("the admin read the page");
    for id in ["role-admin-0-claimed", "email-role-admin-0-after-0"] {
        assert!(at(id).is_some_and(|i| i > admins), "{id}: {:?}", http.ids);
    }
}
