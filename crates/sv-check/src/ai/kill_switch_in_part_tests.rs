//! The kill switch's credit is in part (backlog 0304): C9.6.1 asks for a switch that halts the model
//! "immediately", and the probe shows only that a copy started with the switch on does not call it,
//! not that the switch takes effect without a restart. Its own scope says so, and so must its tier.

use super::tests::ask_switched;
use super::*;

#[test]
fn a_switch_that_stops_the_model_is_credited_in_part() {
    let o = ask_switched(Default::default(), Default::default(), true);
    let credit = o
        .verified
        .iter()
        .find(|v| v.check_id == KILL_SWITCH.rule_id)
        .expect("the setup: the switch stopped the model and was credited");
    assert!(
        credit.scope.contains("without a restart was not shown"),
        "{}",
        credit.scope
    );
    assert!(
        credit.in_part,
        "C9.6.1 reads checked in full: {}",
        credit.scope
    );
}
