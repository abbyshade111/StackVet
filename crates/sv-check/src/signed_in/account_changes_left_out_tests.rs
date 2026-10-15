//! Sweep 3 (backlog 0561, ADR-119), inventory item 3: the phone, two-factor, and recovery changes are also sent with
//! the password left out. An app that checks the password only when it is sent refuses a wrong one and takes the
//! change without one.

use super::account_changes::rule_for;
use super::fake_app::*;
use super::tests::with_signup;
use super::*;

const KINDS: [&str; 3] = ["phone", "two-factor", "recovery"];

fn run_with_changes(flaws: Flaws) -> Outcome {
    run_against(flaws, &with_account_changes(with_signup()))
}

#[test]
fn each_change_taken_with_the_password_left_out_is_found() {
    let o = run_with_changes(Flaws {
        account_change_password_only_if_given: true,
        ..Default::default()
    });
    for (kind, what) in KINDS.iter().zip([
        "the phone number",
        "the two-factor settings",
        "what recovers the account",
    ]) {
        // The setup: the wrong password was refused, so only the left-out change can have taken.
        let step = format!("asked to change {what} giving a wrong password (403)");
        assert!(o.steps.contains(&step), "{step}: {:#?}", o.steps);
        let rule = rule_for(kind).rule_id;
        let found = o
            .findings
            .iter()
            .find(|f| f.rule_id == rule)
            .unwrap_or_else(|| panic!("{kind}: {:#?}", o.steps));
        assert!(
            found.title.ends_with("with the password left out"),
            "{kind}: {}",
            found.title
        );
        assert!(!verified_ids(&o).contains(&rule), "{kind}");
    }
}

#[test]
fn each_change_refused_both_ways_is_credited_and_says_so() {
    let o = run_with_changes(Flaws::default());
    for kind in KINDS {
        let rule = rule_for(kind).rule_id;
        let credit = o
            .verified
            .iter()
            .find(|v| v.check_id == rule)
            .unwrap_or_else(|| panic!("{kind}: {:#?}", o.steps));
        assert!(
            credit.scope.contains("and one leaving it out"),
            "{kind}: {}",
            credit.scope
        );
    }
}
