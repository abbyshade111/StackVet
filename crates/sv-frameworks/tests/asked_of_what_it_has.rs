//! Three level-3 requirements are asked only of an app that has the thing they ask about (backlog 0624, part 3):
//! V6.6.4 of one that sends push sign-in prompts, V12.3.5 of one built as several services, and V8.3.3 of one where a
//! request reaches it through an intermediary: another of its own services, a program with an API key, or an AI that
//! acts for a person. Until 20 October 2026 the first applied to every app with sign-in and the second to every app
//! that called out, sent email, or had AI; the third to every app.

use std::path::PathBuf;
use sv_frameworks::Frameworks;
use sv_frameworks::applicability::{ApplicabilityConfig, ConditionContext, bucket};
use sv_frameworks::{Condition, Source};

fn data() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data")
}

/// Every question the owner is asked answered `answer`, except the ones in `but`, answered the other way.
fn every(answer: bool, but: &[Condition]) -> ConditionContext {
    let mut ctx = ConditionContext::default();
    for c in Condition::ALL
        .iter()
        .filter(|c| c.source() == Source::Claim)
    {
        ctx.set(*c, if but.contains(c) { !answer } else { answer });
    }
    ctx
}

fn applies(ctx: &ConditionContext, id: &str) -> bool {
    let config = ApplicabilityConfig::load_v2(
        &data().join("knowledge"),
        &data().join("applicability-v2.json"),
    )
    .unwrap();
    let frameworks = Frameworks::load(&data().join("frameworks")).unwrap();
    // Level 3: all three are level-3 requirements, so at 1 or 2 none applies whatever the answers.
    let b = bucket(&frameworks, &config, ctx, 3);
    let applicable = b.applicable.iter().any(|a| a == id);
    let not = b.not_applicable.iter().any(|na| na.id == id);
    assert!(applicable != not, "{id} is in exactly one bucket");
    applicable
}

#[test]
fn each_is_asked_only_of_an_app_that_has_what_it_asks_about() {
    // The setup: every answer yes, so each applies, and a no below is what turns it off.
    let yes = every(true, &[]);
    for id in ["V6.6.4", "V8.3.3", "V12.3.5"] {
        assert!(applies(&yes, id), "{id} with every yes");
    }

    // Push sign-in prompts, and only those, place V6.6.4: sign-in alone does not.
    assert!(!applies(
        &every(true, &[Condition::OutOfBandAuth]),
        "V6.6.4"
    ));
    assert!(applies(
        &every(false, &[Condition::Auth, Condition::OutOfBandAuth]),
        "V6.6.4"
    ));

    // Traffic between the app's own services places V12.3.5; calling out, email, and AI do not.
    let one_service = every(true, &[Condition::MultipleServices]);
    assert!(!applies(&one_service, "V12.3.5"));
    assert!(applies(
        &every(false, &[Condition::MultipleServices]),
        "V12.3.5"
    ));

    // Any one intermediary places V8.3.3; with none, it does not apply, whatever else the app does.
    let intermediaries = [
        Condition::MultipleServices,
        Condition::PublicApi,
        Condition::AiActions,
    ];
    assert!(!applies(&every(true, &intermediaries), "V8.3.3"));
    for c in intermediaries {
        assert!(applies(&every(false, &[c]), "V8.3.3"), "{c:?} alone");
    }
}

#[test]
fn the_reasons_say_what_the_app_does_not_have() {
    let config = ApplicabilityConfig::load_v2(
        &data().join("knowledge"),
        &data().join("applicability-v2.json"),
    )
    .unwrap();
    let frameworks = Frameworks::load(&data().join("frameworks")).unwrap();
    let none = bucket(&frameworks, &config, &every(false, &[]), 3);
    let reason = |id: &str| {
        none.not_applicable
            .iter()
            .find(|na| na.id == id)
            .map(|na| na.reason.clone())
            .unwrap_or_else(|| panic!("{id} applies with every no"))
    };
    assert!(reason("V6.6.4").contains("push"), "{}", reason("V6.6.4"));
    assert!(
        reason("V12.3.5").contains("one service"),
        "{}",
        reason("V12.3.5")
    );
    let v833 = reason("V8.3.3");
    for word in ["one service", "API keys", "AI"] {
        assert!(v833.contains(word), "{v833}");
    }
}
