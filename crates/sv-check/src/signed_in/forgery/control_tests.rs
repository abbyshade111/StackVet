//! A `creates` entry's refusal from another site counts only beside a control: the same request, sent
//! with the session and its token, accepted (backlog 0420; ADR-053, Later). Until then an entry the app
//! refused for any reason (a mistyped path, a form where it wants JSON, an empty field) read as refusing
//! the forgery, and beside the `owned` create it made the V3.5.1 credit whole.

use super::super::fake_app::{FakeApp, Flaws, run_seeded, users};
use super::*;

/// The users section with the second kind of record, and no changing request that is not forged.
fn with_comments() -> sv_manifest::UsersSection {
    let mut u = with_a_create_nothing_takes();
    u.creates[0].path = "/comments".into();
    u
}

/// The users section with one more create request, at a path the fake app does not have, so every
/// request there is refused, the app's own included; and no changing request that is not forged.
fn with_a_create_nothing_takes() -> sv_manifest::UsersSection {
    let mut u = users();
    u.creates = vec![sv_manifest::CreateTemplate {
        method: "POST".into(),
        path: "/nowhere".into(),
        form: [("text", "{marker}")]
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect(),
        json: Default::default(),
        read: None,
        id_field: None,
    }];
    u.admin_actions = Vec::new();
    u.change_password = None;
    u.change_email = None;
    u.delete_account = None;
    u.upload = None;
    u.once = None;
    if let Some(owned) = u.owned.as_mut() {
        owned.update = None;
        owned.delete = None;
    }
    u
}

#[test]
fn a_create_the_app_refuses_with_its_own_token_leaves_the_credit_in_part() {
    let (out, _) = run_seeded(
        FakeApp::new(Flaws::default()),
        &with_a_create_nothing_takes(),
    );
    let credit = out
        .verified
        .iter()
        .find(|v| v.check_id == FORGERY.rule_id)
        .expect("the owned create, refused beside its control, is still credited");
    assert!(credit.in_part, "{credit:?}");
    assert!(
        credit.scope.contains("a request that creates a record,"),
        "only the owned create is counted: {}",
        credit.scope
    );
    assert!(
        credit.scope.contains("not judged: /nowhere"),
        "{}",
        credit.scope
    );
    let why: Vec<&String> = out
        .not_assessed
        .iter()
        .filter(|(ids, _)| ids.contains("V3.5.1"))
        .map(|(_, why)| why)
        .collect();
    assert!(
        why.iter()
            .any(|w| w.contains("/nowhere") && w.contains("with the session and its token")),
        "{why:?}"
    );
}

#[test]
fn a_create_that_crashes_from_another_site_is_not_read_as_refusing_it() {
    // The check's guard (backlog 0280) already keeps a crash from reading as a refusal: the whole credit
    // is withheld, and the run says why. Held here beside the control, so the two cannot drift apart.
    let flaws = Flaws {
        comments_crash_foreign: true,
        ..Flaws::default()
    };
    let (out, _) = run_seeded(FakeApp::new(flaws), &with_comments());
    assert!(
        out.verified.iter().all(|v| v.check_id != FORGERY.rule_id),
        "credited beside a crash: {:?}",
        out.verified
    );
    assert!(
        out.not_assessed
            .iter()
            .any(|(ids, why)| ids.contains("V3.5.1")
                && why.contains("crashed or did not answer rather than refusing")
                && why.contains("forged-create-1 (500)")),
        "{:?}",
        out.not_assessed
    );
    // The control: without the crash, the same two requests are refused and the credit is whole.
    let (whole, _) = run_seeded(FakeApp::new(Flaws::default()), &with_comments());
    let credit = whole
        .verified
        .iter()
        .find(|v| v.check_id == FORGERY.rule_id)
        .expect("credited");
    assert!(!credit.in_part, "{credit:?}");
}
