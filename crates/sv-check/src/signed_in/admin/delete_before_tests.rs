//! Another user's delete is judged only on a record its owner found just before it was sent (backlog 0560,
//! ADR-119): a change that sent the record for review leaves it not found whatever the delete does.

use super::super::fake_app::*;
use super::*;

fn with_writes() -> UsersSection {
    let mut u = users();
    let owned = u.owned.as_mut().expect("the fixture has an owned record");
    owned.update = Some(sv_manifest::RequestTemplate {
        method: "POST".into(),
        path: "/notes/{id}/edit".into(),
        form: [("text", "{marker}"), ("csrf_token", "{csrf}")]
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect(),
        json: Default::default(),
    });
    owned.delete = Some(sv_manifest::RequestTemplate {
        method: "POST".into(),
        path: "/notes/{id}/delete".into(),
        form: [("csrf_token", "{csrf}")]
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect(),
        json: Default::default(),
    });
    u
}

fn deleted_finding(o: &Outcome) -> bool {
    o.findings
        .iter()
        .any(|f| f.title == "One user can delete another user's records")
}

#[test]
fn a_record_gone_before_the_delete_is_not_blamed_on_it() {
    let o = run_against(
        Flaws {
            idor_update: true,
            edit_archives: true,
            ..Default::default()
        },
        &with_writes(),
    );
    assert!(
        !deleted_finding(&o),
        "a record the change had already sent away was read as deleted: {:?}",
        o.findings
    );
    assert!(
        o.not_assessed.iter().any(|(id, why)| id == "V8.2.2"
            && why.contains("was already not found just before the second user's delete was sent")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_delete_that_took_is_still_found() {
    let o = run_against(
        Flaws {
            idor_delete: true,
            ..Default::default()
        },
        &with_writes(),
    );
    assert!(deleted_finding(&o), "{:?}", o.not_assessed);
}
