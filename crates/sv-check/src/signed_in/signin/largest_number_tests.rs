//! The cap on wrong-password attempts holds for every number stackvet.toml can give. `allowed + 2`
//! wrapped for the largest a u32 holds in a release build, where overflow is not checked, and the
//! check then sent one attempt and read past the end of its answers (backlog 0465).

use super::super::fake_app::Flaws;
use super::super::tests::run_with;

fn policy(failed: u32) -> sv_manifest::PolicySection {
    sv_manifest::PolicySection {
        failed_sign_ins: Some(failed),
        within_minutes: Some(15),
        ..Default::default()
    }
}

/// Whether V6.3.1 was refused for being above the cap, naming the number given.
fn refused_above_the_cap(failed: u32) -> bool {
    run_with(Flaws::default(), &policy(failed))
        .not_assessed
        .iter()
        .any(|(ids, why)| {
            ids.contains("V6.3.1")
                && why.contains(&format!("failed-sign-ins is {failed}."))
                && why.contains("at most 26")
        })
}

#[test]
fn the_largest_number_is_refused_above_the_cap_and_not_wrapped() {
    assert!(refused_above_the_cap(u32::MAX));
    assert!(refused_above_the_cap(u32::MAX - 1));
}

#[test]
fn the_cap_is_the_number_allowed_plus_two() {
    // 24 allowed is 26 attempts, the most the check makes; 25 would be 27.
    assert!(!refused_above_the_cap(24));
    assert!(refused_above_the_cap(25));
}
