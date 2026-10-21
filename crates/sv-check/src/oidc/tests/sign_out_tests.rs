//! Signing out of a sign-in through another service (V7.4.1; backlog 0232, part B, the owner's choice of 20 October
//! 2026): with `[stack.run.oidc] logout`, the session the ordinary sign-in opened is copied, signed out, and the copy
//! sent to the private page again. Still let in is a finding; refused is credited in part; with no `logout`, it is said.

use super::*;

fn with_logout() -> OidcSection {
    OidcSection {
        logout: Some(sv_manifest::RequestTemplate {
            method: "POST".into(),
            path: "/logout".into(),
            form: BTreeMap::new(),
            json: BTreeMap::new(),
        }),
        ..section()
    }
}

fn outcome(flaws: Flaws, section: &OidcSection) -> Outcome {
    let mut fake = Fake {
        flaws,
        mode: idp::NORMAL.into(),
        ..Default::default()
    };
    run(&mut fake, section)
}

#[test]
fn a_sign_out_that_ends_the_session_is_credited_in_part() {
    let o = outcome(Flaws::default(), &with_logout());
    let credit = o
        .verified
        .iter()
        .find(|v| v.check_id == crate::signed_in::LOGOUT.rule_id)
        .unwrap_or_else(|| panic!("{:#?}", o.steps));
    assert!(credit.in_part, "{credit:?}");
    assert!(
        credit.scope.contains("through the test provider"),
        "{}",
        credit.scope
    );
    assert!(
        o.findings
            .iter()
            .all(|f| f.rule_id != crate::signed_in::LOGOUT.rule_id)
    );
}

#[test]
fn a_sign_out_that_keeps_the_session_is_found_and_credits_nothing() {
    let o = outcome(
        Flaws {
            logout_keeps_session: true,
            ..Flaws::default()
        },
        &with_logout(),
    );
    let found = o
        .findings
        .iter()
        .find(|f| f.rule_id == crate::signed_in::LOGOUT.rule_id)
        .unwrap_or_else(|| panic!("{:#?}", o.steps));
    assert_eq!(found.requirement_ids, ["V7.4.1"]);
    assert!(
        found.description.contains("still opened /account"),
        "{}",
        found.description
    );
    assert!(
        !o.verified
            .iter()
            .any(|v| v.check_id == crate::signed_in::LOGOUT.rule_id)
    );
}

#[test]
fn no_logout_named_is_said_and_neither_found_nor_credited() {
    let o = outcome(Flaws::default(), &section());
    assert!(
        o.not_assessed
            .iter()
            .any(|(id, why)| id == "V7.4.1" && why.contains("has no `logout`")),
        "{:?}",
        o.not_assessed
    );
    assert!(
        !o.verified
            .iter()
            .any(|v| v.check_id == crate::signed_in::LOGOUT.rule_id)
    );
    assert!(
        o.findings
            .iter()
            .all(|f| f.rule_id != crate::signed_in::LOGOUT.rule_id)
    );
}
