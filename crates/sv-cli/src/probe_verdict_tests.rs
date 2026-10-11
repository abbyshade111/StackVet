//! The probe's decision on whether the address answered, and its exit code, on every platform (backlog 0235).

use super::*;

#[test]
fn an_address_nothing_came_back_from_is_not_reached_and_exits_not_assessed() {
    let out = Outcome::default();
    assert!(!reached(&out));
    assert_eq!(exit_code(reached(&out)), exit::NOT_ASSESSED);
}

#[test]
fn an_address_that_answered_is_reached_and_exits_clean() {
    let mut out = Outcome::default();
    out.verified.push(sv_check::Verified::new(
        "probe.security-headers",
        &[],
        "the home page".to_owned(),
    ));
    assert!(reached(&out));
    assert_eq!(exit_code(reached(&out)), exit::CLEAN);
}
