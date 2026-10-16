//! V6.8.3 (each SAML assertion used once) applies only to an app that takes SAML sign-ins, by the derived `saml`
//! condition, while the rest of V6.8 stays with the `oauth` claim (backlog 0500).

use std::path::PathBuf;
use sv_frameworks::Condition;
use sv_frameworks::Frameworks;
use sv_frameworks::applicability::{ApplicabilityConfig, ConditionContext, bucket};

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data")
}

#[test]
fn saml_places_v6_8_3_and_oauth_the_rest_of_v6_8() {
    let config = ApplicabilityConfig::load_v2(
        &data_dir().join("knowledge"),
        &data_dir().join("applicability-v2.json"),
    )
    .unwrap();
    let f = Frameworks::load(&data_dir().join("frameworks")).unwrap();
    let applies = |oauth: bool, saml: bool, id: &str| {
        let mut ctx = ConditionContext::default();
        ctx.set(Condition::Auth, true);
        ctx.set(Condition::Oauth, oauth);
        ctx.set(Condition::Saml, saml);
        let b = bucket(&f, &config, &ctx, 3);
        let yes = b.applicable.iter().any(|a| a == id);
        let no = b.not_applicable.iter().any(|na| na.id == id);
        assert!(yes != no, "{id} is placed once, oauth {oauth}, saml {saml}");
        yes
    };
    // Sign-in through another service, with no SAML library: V6.8 applies, V6.8.3 does not.
    assert!(
        applies(true, false, "V6.8.2"),
        "the setup: V6.8 applies with oauth"
    );
    assert!(!applies(true, false, "V6.8.3"));
    // A SAML library: V6.8.3 applies.
    assert!(applies(true, true, "V6.8.3"));
    // Its own accounts only: V6.8.2 does not apply, whatever the packages.
    assert!(!applies(false, false, "V6.8.2"));
}
