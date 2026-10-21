//! V15.4 and C5.1 at level 3 (ADR-130). Their `never` rules said "level 3 only, above the level this app targets",
//! which became false once an app can ask for level 3; the overlay replaced them. Below level 3 they are above the
//! app's level, never "not applicable"; at level 3, V15.4 applies to every app and C5.1 by what the app's AI does.

use std::path::PathBuf;
use sv_frameworks::Frameworks;
use sv_frameworks::applicability::{ApplicabilityConfig, Buckets, ConditionContext, bucket};
use sv_frameworks::{Condition, Source};

fn data() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data")
}

fn buckets(level: u8, yes: &[Condition]) -> Buckets {
    let config = ApplicabilityConfig::load_v2(
        &data().join("knowledge"),
        &data().join("applicability-v2.json"),
    )
    .unwrap();
    let frameworks = Frameworks::load(&data().join("frameworks")).unwrap();
    let mut ctx = ConditionContext::default();
    for c in Condition::ALL
        .iter()
        .filter(|c| c.source() == Source::Claim)
    {
        ctx.set(*c, yes.contains(c));
    }
    ctx.set(Condition::Ai, true);
    bucket(&frameworks, &config, &ctx, level)
}

fn applies(b: &Buckets, id: &str) -> bool {
    b.applicable.iter().any(|a| a == id)
}

#[test]
fn below_level_three_they_are_above_the_app_s_level_and_not_excluded() {
    let b = buckets(2, &[Condition::SelfHostedModel, Condition::MultiAgent]);
    for id in ["V15.4.1", "V15.4.4", "C5.1.1", "C5.1.2"] {
        assert!(b.out_of_level.iter().any(|o| o == id), "{id} at level 2");
        assert!(
            !b.not_applicable.iter().any(|na| na.id == id),
            "{id} is not excluded at level 2"
        );
    }
}

#[test]
fn at_level_three_v15_4_applies_and_c5_1_follows_what_the_ai_does() {
    let none = buckets(3, &[]);
    for id in ["V15.4.1", "V15.4.2", "V15.4.3", "V15.4.4"] {
        assert!(applies(&none, id), "{id} at level 3");
    }
    // The setup: with neither a model of its own nor several agents, both are excluded, with a reason.
    for id in ["C5.1.1", "C5.1.2"] {
        let reason = none
            .not_applicable
            .iter()
            .find(|na| na.id == id)
            .map(|na| na.reason.clone())
            .unwrap_or_else(|| panic!("{id} applies with no model and one assistant"));
        assert!(!reason.contains("level 3 only"), "{id}: {reason}");
    }
    for c in [Condition::SelfHostedModel, Condition::Training] {
        let b = buckets(3, &[c]);
        assert!(applies(&b, "C5.1.1"), "{c:?}");
        assert!(!applies(&b, "C5.1.2"), "{c:?} is no second agent");
    }
    let agents = buckets(3, &[Condition::MultiAgent]);
    assert!(applies(&agents, "C5.1.2"));
    assert!(!applies(&agents, "C5.1.1"));
}
