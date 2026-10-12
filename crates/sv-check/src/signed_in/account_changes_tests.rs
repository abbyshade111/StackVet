//! The phone, two-factor, and recovery changes V7.5.1 names beside the email address (ADR-105,
//! backlog 0232 group G), against the scripted app.

use super::account_changes::{new_value, rule_for, shows_value};
use super::fake_app::*;
use super::tests::with_signup;
use super::*;

const KINDS: [&str; 3] = ["phone", "two-factor", "recovery"];

fn run_with_changes(flaws: Flaws) -> Outcome {
    run_against(flaws, &with_account_changes(with_signup()))
}

fn why_not_assessed(o: &Outcome) -> Vec<&str> {
    o.not_assessed
        .iter()
        .filter(|(ids, _)| ids == "V7.5.1")
        .map(|(_, why)| why.as_str())
        .collect()
}

#[test]
fn each_change_that_needs_the_password_is_credited_in_part() {
    let o = run_with_changes(Flaws::default());
    for kind in KINDS {
        let rule = rule_for(kind).rule_id;
        assert!(!rule_ids(&o).contains(&rule), "{kind}: {:#?}", o.findings);
        assert!(verified_ids(&o).contains(&rule), "{kind}: {:?}", o.steps);
        // One of what V7.5.1 names: whether it is met whole is the report's to say (ADR-105).
        assert!(credited_in_part(&o, rule), "{kind}");
    }
    // The control really ran: each change with the right password was asked for and took.
    for what in [
        "the phone number",
        "the two-factor settings",
        "what recovers the account",
    ] {
        assert!(
            o.steps
                .iter()
                .any(|s| s == &format!("asked to change {what} giving the right password (303)")),
            "{what}: {:?}",
            o.steps
        );
    }
    assert!(
        why_not_assessed(&o).is_empty(),
        "{:?}",
        why_not_assessed(&o)
    );
}

#[test]
fn each_change_without_the_password_is_found_on_its_own() {
    for (kind, flaws) in [
        (
            "phone",
            Flaws {
                phone_change_without_password: true,
                ..Default::default()
            },
        ),
        (
            "two-factor",
            Flaws {
                two_factor_change_without_password: true,
                ..Default::default()
            },
        ),
        (
            "recovery",
            Flaws {
                recovery_change_without_password: true,
                ..Default::default()
            },
        ),
    ] {
        let o = run_with_changes(flaws);
        let rule = rule_for(kind).rule_id;
        assert_eq!(rule_ids(&o), vec![rule], "{kind}: {:?}", o.steps);
        assert!(
            !verified_ids(&o).contains(&rule),
            "{kind}: found and credited"
        );
        let f = o.findings.iter().find(|f| f.rule_id == rule).unwrap();
        assert_eq!(f.requirement_ids, ["V7.5.1"]);
        assert!(
            f.description
                .ends_with("/account/details then showed the new value."),
            "{}",
            f.description
        );
        // The other two are still refused, and still credited.
        for other in KINDS.iter().filter(|k| **k != kind) {
            assert!(
                verified_ids(&o).contains(&rule_for(other).rule_id),
                "{other}"
            );
        }
    }
}

#[test]
fn a_change_that_never_takes_is_not_assessed() {
    // Refusing the wrong password means nothing if the right one changes nothing either.
    let o = run_with_changes(Flaws {
        account_change_does_nothing: true,
        ..Default::default()
    });
    for kind in KINDS {
        assert!(
            !verified_ids(&o).contains(&rule_for(kind).rule_id),
            "{kind}"
        );
    }
    let why = why_not_assessed(&o);
    assert_eq!(
        why.iter().filter(|w| w.contains("did not take")).count(),
        3,
        "{why:?}"
    );
}

#[test]
fn a_change_not_given_is_not_assessed_unless_listed_as_not_offered() {
    // The scripted app's own sign-up, with no phone, two-factor, or recovery change named.
    let o = run_against(Flaws::default(), &with_signup());
    let why = why_not_assessed(&o);
    for key in ["change-phone", "change-two-factor", "change-recovery"] {
        assert!(
            why.iter().any(|w| w.contains(&format!("sets no `{key}`"))),
            "{key}: {why:?}"
        );
    }
    // Listed as not offered: the report, not the check, says whether the owner confirmed it.
    let mut users = with_signup();
    users.not_offered = vec!["phone".into(), "recovery".into()];
    let o = run_against(Flaws::default(), &users);
    let why = why_not_assessed(&o);
    assert!(!why.iter().any(|w| w.contains("`change-phone`")), "{why:?}");
    assert!(
        !why.iter().any(|w| w.contains("`change-recovery`")),
        "{why:?}"
    );
    assert!(
        why.iter().any(|w| w.contains("`change-two-factor`")),
        "{why:?}"
    );
}

#[test]
fn a_page_that_cannot_be_opened_is_named_and_credits_nothing() {
    let mut users = with_account_changes(with_signup());
    if let Some(change) = users.change_phone.as_mut() {
        change.shows = "/no-such-page".into();
    }
    let o = run_against(Flaws::default(), &users);
    assert!(!verified_ids(&o).contains(&rule_for("phone").rule_id));
    let why = why_not_assessed(&o);
    assert!(
        why.iter()
            .any(|w| w.contains("/no-such-page (`shows` under `change-phone`) could not be opened")),
        "{why:?}"
    );
    // The other two are untouched.
    assert!(verified_ids(&o).contains(&rule_for("recovery").rule_id));
}

#[test]
fn a_phone_number_is_found_however_the_page_writes_it_and_the_two_turns_differ() {
    let spare = "0123456789abcdef0123456789abcdef";
    let (first, second) = (
        new_value("phone", spare, 0, "u@example.test"),
        new_value("phone", spare, 1, "u@example.test"),
    );
    assert_ne!(first, second);
    // The two-factor number never repeats the phone's, since one account holds both.
    for turn in [0, 1] {
        let two_factor = new_value("two-factor", spare, turn, "u@example.test");
        assert_ne!(two_factor, first);
        assert_ne!(two_factor, second);
    }
    assert!(first.starts_with("+1555") && first.len() == 12, "{first}");
    let digits: String = first.chars().filter(char::is_ascii_digit).collect();
    let written = format!(
        "<td>+{} ({}) {}-{}</td>",
        &digits[..1],
        &digits[1..4],
        &digits[4..7],
        &digits[7..]
    );
    assert!(shows_value("phone", &written, &first), "{written}");
    assert!(!shows_value("phone", &written, &second));
    // An address is matched as written, never by its digits.
    let address = new_value("recovery", spare, 0, "u1@example.test");
    assert_eq!(address, "back-0.u1@example.test");
    assert!(!shows_value("recovery", "back-1.u1@example.test", &address));
}
