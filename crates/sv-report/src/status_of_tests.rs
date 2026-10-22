use super::{Status, status_of};
use sv_check::Tier;

const ALL: [Tier; 6] = [
    Tier::Checked,
    Tier::AppTested,
    Tier::Documented,
    Tier::ByHand,
    Tier::Attested,
    Tier::Stated,
];

fn status_of_tier(tier: Tier) -> Status {
    match tier {
        Tier::Checked => Status::Checked,
        Tier::AppTested => Status::AppTested,
        Tier::Documented => Status::Documented,
        Tier::ByHand => Status::ByHand,
        Tier::Attested => Status::Attested,
        Tier::Stated => Status::Stated,
    }
}

#[test]
fn each_tier_alone_is_its_own_status_and_nothing_is_not_verified() {
    assert_eq!(status_of(false, false, &[]), Status::NotVerified);
    for tier in ALL {
        assert_eq!(
            status_of(false, false, &[(tier, false)]),
            status_of_tier(tier),
            "{tier:?}"
        );
    }
}

#[test]
fn the_strongest_tier_there_decides_whatever_else_is_beside_it() {
    // The order is the order of the enum: a check of sv's own, the app's tests, a written
    // answer, a check by hand, the owner's yes, the tool's yes.
    for (n, strongest) in ALL.iter().enumerate() {
        let credits: Vec<(Tier, bool)> = ALL[n..].iter().map(|t| (*t, false)).collect();
        assert_eq!(
            status_of(false, false, &credits),
            status_of_tier(*strongest),
            "{credits:?}"
        );
        let reversed: Vec<(Tier, bool)> = credits.iter().rev().copied().collect();
        assert_eq!(
            status_of(false, false, &reversed),
            status_of_tier(*strongest)
        );
    }
}

#[test]
fn a_finding_beats_every_credit() {
    let every: Vec<(Tier, bool)> = ALL.iter().map(|t| (*t, false)).collect();
    assert_eq!(status_of(true, false, &every), Status::NeedsAttention);
    assert_eq!(status_of(true, false, &[]), Status::NeedsAttention);
}

#[test]
fn checks_that_each_tried_only_part_are_checked_in_part_and_one_whole_check_is_checked() {
    assert_eq!(
        status_of(
            false,
            false,
            &[(Tier::Checked, true), (Tier::Checked, true)]
        ),
        Status::CheckedInPart
    );
    assert_eq!(
        status_of(
            false,
            false,
            &[(Tier::Checked, true), (Tier::Checked, false)]
        ),
        Status::Checked
    );
    // In part is only ever said of a check of sv's own; on a person's word it means nothing.
    assert_eq!(
        status_of(false, false, &[(Tier::Attested, true)]),
        Status::Attested
    );
}

#[test]
fn a_false_alarm_set_aside_stops_a_check_and_a_test_but_not_a_persons_word() {
    // The rule saw something there; a person's word that it was wrong does not show the
    // protection is in place, so no check's clean run makes it checked (ADR-023).
    assert_eq!(
        status_of(false, true, &[(Tier::Checked, false)]),
        Status::NotVerified
    );
    assert_eq!(
        status_of(false, true, &[(Tier::AppTested, false)]),
        Status::NotVerified
    );
    assert_eq!(
        status_of(
            false,
            true,
            &[(Tier::Checked, false), (Tier::Documented, false)]
        ),
        Status::Documented
    );
    for tier in [Tier::Documented, Tier::ByHand, Tier::Attested, Tier::Stated] {
        assert_eq!(
            status_of(false, true, &[(tier, false)]),
            status_of_tier(tier)
        );
    }
}
