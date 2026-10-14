//! V6.6.2 applies to emailed sign-in codes as well as phone, SMS, and push ones, and V6.6.1 stays phone
//! and SMS only: each is placed by its own condition (backlog 0468; ADR-015, Later, 14 October 2026).

use std::path::PathBuf;
use sv_frameworks::Frameworks;
use sv_frameworks::applicability::{ApplicabilityConfig, ConditionContext, bucket};
use sv_frameworks::{Condition, Source};

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data")
}

#[test]
fn emailed_codes_place_v6_6_2_and_not_v6_6_1() {
    let config = ApplicabilityConfig::load_v2(
        &data_dir().join("knowledge"),
        &data_dir().join("applicability-v2.json"),
    )
    .unwrap();
    let f = Frameworks::load(&data_dir().join("frameworks")).unwrap();
    // Every question the owner is asked answered no, then an app whose only codes are emailed.
    let mut ctx = ConditionContext::default();
    for c in Condition::ALL
        .iter()
        .filter(|c| c.source() == Source::Claim)
    {
        ctx.set(*c, false);
    }
    // The setup: with every no, both are excluded, so the yes below is what moves V6.6.2.
    let none = bucket(&f, &config, &ctx, 2);
    for id in ["V6.6.1", "V6.6.2"] {
        assert!(
            none.not_applicable.iter().any(|na| na.id == id),
            "{id} with every no"
        );
    }
    ctx.set(Condition::OutOfBandOrEmailCode, true);
    let emailed = bucket(&f, &config, &ctx, 2);
    assert!(emailed.applicable.iter().any(|id| id == "V6.6.2"));
    assert!(
        emailed.not_applicable.iter().any(|na| na.id == "V6.6.1"),
        "V6.6.1 is about phone and SMS codes only"
    );
    // The reason V6.6.2 gives when it does not apply names email among the ways a code is sent.
    let reason = none
        .not_applicable
        .iter()
        .find(|na| na.id == "V6.6.2")
        .map(|na| na.reason.clone())
        .unwrap();
    assert!(reason.contains("or email"), "{reason}");
}
