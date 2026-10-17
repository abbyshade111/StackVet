//! The simple-request check sent for every JSON create request stackvet.toml names, not one (backlog 0232, part 3;
//! ADR-053, Later). Whole only when two or more refused all three forms a browser sends without a preflight, none was
//! left unjudged, and the manifest names no other changing request, since those are not sent.

use super::super::fake_app::{FakeApp, Flaws, run_seeded, users};
use super::*;

fn json_create(path: &str) -> sv_manifest::CreateTemplate {
    sv_manifest::CreateTemplate {
        method: "POST".into(),
        path: path.into(),
        form: Default::default(),
        json: [("text".to_owned(), "{marker}".to_owned())].into(),
        read: None,
        id_field: None,
        page: false,
    }
}

/// The users section with the owned record and a second kind of record, both behind the JSON API, and no changing
/// request that is not sent.
fn two_json_creates() -> sv_manifest::UsersSection {
    let mut u = users();
    u.owned = Some(sv_manifest::OwnedSection {
        create: sv_manifest::RequestTemplate {
            method: "POST".into(),
            path: "/api/notes".into(),
            form: Default::default(),
            json: [("text".to_owned(), "{marker}".to_owned())].into(),
        },
        read: Some("/api/notes/{id}".into()),
        id_field: None,
        list: None,
        update: None,
        delete: None,
    });
    u.creates = vec![json_create("/api/comments")];
    u.admin_actions = Vec::new();
    u.change_password = None;
    u.change_email = None;
    u.delete_account = None;
    u.upload = Vec::new();
    u.once = None;
    u
}

fn credit(out: &Outcome) -> Option<&crate::Verified> {
    out.verified
        .iter()
        .find(|v| v.check_id == SIMPLE_REQUEST.rule_id)
}

/// The setup worked: the owned record was made through the API, and the second request's control was taken, or
/// nothing below means anything.
fn assert_both_tried(out: &Outcome) {
    assert!(
        out.steps
            .iter()
            .any(|s| s.contains("created a record at /api/notes/")),
        "{:?}",
        out.steps
    );
    for path in ["/api/notes", "/api/comments"] {
        assert!(
            out.steps.iter().any(|s| s.contains(&format!(
                "the create request at {path} as another site could send it"
            ))),
            "{path} not sent: {:?}",
            out.steps
        );
    }
}

#[test]
fn every_json_create_refused_in_all_three_forms_with_nothing_else_named_is_whole() {
    let (out, _) = run_seeded(FakeApp::new(Flaws::default()), &two_json_creates());
    assert_both_tried(&out);
    let credit = credit(&out).expect("credited");
    assert!(!credit.in_part, "{credit:?}");
    assert!(
        credit.scope.contains("2 JSON requests that create records"),
        "{}",
        credit.scope
    );
    assert!(credit.scope.contains("/api/comments"), "{}", credit.scope);
}

#[test]
fn a_second_create_that_takes_a_form_is_found_naming_it() {
    let flaws = Flaws {
        api_comments_take_forms: true,
        ..Flaws::default()
    };
    let (out, _) = run_seeded(FakeApp::new(flaws), &two_json_creates());
    assert_both_tried(&out);
    let found = out
        .findings
        .iter()
        .find(|f| f.rule_id == SIMPLE_REQUEST.rule_id)
        .expect("the finding");
    assert_eq!(
        found.evidence,
        ["simple-request-1-a-form", "simple-request-1-multipart"]
    );
    assert!(
        found
            .description
            .contains("/api/comments when its fields came as a form and as multipart"),
        "{}",
        found.description
    );
    assert!(
        !found.description.contains("/api/notes"),
        "{}",
        found.description
    );
    assert!(credit(&out).is_none(), "credited beside its own finding");
}

#[test]
fn a_changing_request_not_sent_keeps_the_credit_in_part_and_is_named() {
    let mut u = two_json_creates();
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
fn a_create_whose_control_is_not_taken_is_not_judged_and_keeps_the_credit_in_part() {
    let mut u = two_json_creates();
    u.creates = vec![json_create("/api/nowhere")];
    let (out, _) = run_seeded(FakeApp::new(Flaws::default()), &u);
    let credit = credit(&out).expect("credited for the one judged");
    assert!(credit.in_part, "{credit:?}");
    assert!(
        credit.scope.contains("not judged: /api/nowhere"),
        "{}",
        credit.scope
    );
    assert!(
        out.not_assessed.iter().any(|(ids, why)| ids == "V3.5.2"
            && why.contains("/api/nowhere")
            && why.contains("was not accepted")),
        "{:?}",
        out.not_assessed
    );
}

#[test]
fn one_json_create_named_is_one_sample() {
    let mut u = two_json_creates();
    u.creates = Vec::new();
    let (out, _) = run_seeded(FakeApp::new(Flaws::default()), &u);
    let credit = credit(&out).expect("credited");
    assert!(credit.in_part, "{credit:?}");
}

#[test]
fn a_form_create_beside_a_json_one_is_left_to_the_cross_site_check() {
    // `/comments` takes a form: no preflight guards it, so it is not sent here, and does not count as a second sample.
    let mut u = two_json_creates();
    u.creates = vec![sv_manifest::CreateTemplate {
        form: [("text".to_owned(), "{marker}".to_owned())].into(),
        json: Default::default(),
        ..json_create("/comments")
    }];
    let (out, _) = run_seeded(FakeApp::new(Flaws::default()), &u);
    assert!(
        !out.steps
            .iter()
            .any(|s| s.contains("the create request at /comments as another site")),
        "{:?}",
        out.steps
    );
    assert!(credit(&out).expect("credited").in_part);
}

#[test]
fn two_refused_and_one_not_judged_is_still_in_part() {
    // Two judged requests would be whole; the third, whose control was not taken, keeps it in part on its own.
    let mut u = two_json_creates();
    u.creates.push(json_create("/api/nowhere"));
    let (out, _) = run_seeded(FakeApp::new(Flaws::default()), &u);
    assert_both_tried(&out);
    let credit = credit(&out).expect("credited");
    assert!(
        credit.scope.contains("2 JSON requests that create records"),
        "{}",
        credit.scope
    );
    assert!(credit.in_part, "{credit:?}");
}
