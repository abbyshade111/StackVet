//! Section V6.7 (cryptographic authenticators: the certificates that check their assertions, and their challenges)
//! applies to an app that checks passkeys, security keys, or client certificates itself, by the derived
//! `cryptographic-authenticator` and `mtls` conditions, either one; until 20 October 2026 it applied to every app with
//! sign-in (backlog 0624, part 3).

use std::path::PathBuf;
use sv_frameworks::Condition;
use sv_frameworks::Frameworks;
use sv_frameworks::applicability::{ApplicabilityConfig, ConditionContext, bucket};

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data")
}

#[test]
fn a_passkey_library_or_a_client_certificate_places_v6_7_and_neither_does_not() {
    let config = ApplicabilityConfig::load_v2(
        &data_dir().join("knowledge"),
        &data_dir().join("applicability-v2.json"),
    )
    .unwrap();
    let f = Frameworks::load(&data_dir().join("frameworks")).unwrap();
    let placed = |passkeys: bool, certificates: bool| {
        let mut ctx = ConditionContext::default();
        ctx.set(Condition::Auth, true);
        ctx.set(Condition::CryptographicAuthenticator, passkeys);
        ctx.set(Condition::Mtls, certificates);
        // Level 3: both of V6.7's requirements are level 3.
        let b = bucket(&f, &config, &ctx, 3);
        ["V6.7.1", "V6.7.2"].map(|id| {
            let yes = b.applicable.iter().any(|a| a == id);
            let no = b.not_applicable.iter().find(|na| na.id == id);
            assert!(yes != no.is_some(), "{id} is placed once");
            no.map(|na| na.reason.clone())
        })
    };
    // The setup: a sign-in with neither, so the section's own rules, not the chapter's, are what decide.
    let neither = placed(false, false);
    for reason in &neither {
        let reason = reason.as_deref().expect("V6.7 does not apply with neither");
        assert!(
            reason.contains("passkey") && reason.contains("client certificate"),
            "{reason}"
        );
    }
    assert_eq!(placed(true, false), [None, None], "a passkey library");
    assert_eq!(placed(false, true), [None, None], "a client certificate");
}
