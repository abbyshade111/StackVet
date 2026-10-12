//! The cross-site check sent for every create request stackvet.toml names, not one (backlog 0232, part 3; ADR-053,
//! Later). Whole only when two or more were refused and the manifest names no other changing request, since those are
//! not forged.

use super::super::fake_app::{FakeApp, Flaws, run_seeded, users};
use super::*;

fn comments() -> sv_manifest::CreateTemplate {
    sv_manifest::CreateTemplate {
        method: "POST".into(),
        path: "/comments".into(),
        form: [("text", "{marker}")]
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect(),
        json: Default::default(),
        read: None,
        id_field: None,
    }
}

/// The users section with a second kind of record, and no changing request that is not forged.
fn creates_only() -> sv_manifest::UsersSection {
    let mut u = users();
    u.creates = vec![comments()];
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

fn credit(out: &Outcome) -> Option<&crate::Verified> {
    out.verified.iter().find(|v| v.check_id == FORGERY.rule_id)
}

#[test]
fn every_create_refused_from_another_site_with_nothing_else_named_is_whole() {
    let (out, _) = run_seeded(FakeApp::new(Flaws::default()), &creates_only());
    let credit = credit(&out).expect("credited");
    assert!(!credit.in_part, "{credit:?}");
    assert!(
        credit.scope.contains("2 requests that create records"),
        "{}",
        credit.scope
    );
    assert!(credit.scope.contains("/comments"), "{}", credit.scope);
    assert!(
        !credit.scope.contains("not sent this way"),
        "{}",
        credit.scope
    );
}

#[test]
fn a_second_create_taken_from_another_site_is_found_naming_it() {
    let flaws = Flaws {
        comments_take_foreign: true,
        ..Flaws::default()
    };
    let (out, _) = run_seeded(FakeApp::new(flaws), &creates_only());
    let found = out
        .findings
        .iter()
        .find(|f| f.rule_id == FORGERY.rule_id)
        .expect("the finding");
    assert_eq!(found.evidence, ["forged-create-1"]);
    assert!(
        found.description.contains("/comments"),
        "{}",
        found.description
    );
    assert!(credit(&out).is_none(), "credited beside its own finding");
}

#[test]
fn a_changing_request_not_forged_keeps_the_credit_in_part_and_is_named() {
    let mut u = creates_only();
    u.change_password = users().change_password;
    let (out, _) = run_seeded(FakeApp::new(Flaws::default()), &u);
    let credit = credit(&out).expect("credited");
    assert!(credit.in_part, "{credit:?}");
    assert!(
        credit
            .scope
            .contains("not sent this way: the password change"),
        "{}",
        credit.scope
    );
}

#[test]
fn one_create_named_is_one_sample() {
    let mut u = creates_only();
    u.creates = Vec::new();
    let (out, _) = run_seeded(FakeApp::new(Flaws::default()), &u);
    let credit = credit(&out).expect("credited");
    assert!(credit.in_part, "{credit:?}");
}
