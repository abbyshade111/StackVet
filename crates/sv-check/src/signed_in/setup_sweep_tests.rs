//! Backlog 0559, the setup sweep (ADR-119, rule 1): every check that makes an account of its own through `signup`
//! asserts the account was made before it concludes anything. With A and B seeded and a `signup` that makes no account
//! (refused, or answered as if it worked), no finding is raised, and each such check says it was not assessed and why.
//!
//! Before the sweep, the same runs raised "Wrong passwords can be tried without limit" (backlog 0522), and the emailed
//! code's checks blamed the mail server for an email that never could have come.

use super::fake_app::{FakeApp, Flaws, accounts};

fn run(flaws: Flaws) -> super::Outcome {
    let mut app = FakeApp::new(flaws);
    let mut acc = accounts();
    acc.admin = None;
    acc.totp = None;
    for account in [&acc.a, &acc.b] {
        app.users
            .insert(account.user.clone(), (account.password.clone(), false));
    }
    let policy = sv_manifest::PolicySection {
        failed_sign_ins: Some(3),
        within_minutes: Some(15),
        ..Default::default()
    };
    super::run(&mut app, &super::tests::with_signup(), &acc, true, &policy)
}

/// Each check that makes an account of its own, by the requirements it names and what its line says.
const NOT_MADE: &[(&str, &str)] = &[
    ("V6.3.1", "the account was not made"),
    (
        "V6.5.1, V6.5.4, V6.6.2, V6.6.3",
        "could not sign in with its password",
    ),
    (
        "V6.2.2, V6.2.3, V7.4.3, V6.3.7",
        "could not sign in to begin with",
    ),
    ("V6.4.3, V6.3.8", "could not sign in to begin with"),
    ("V7.5.1", "could not sign in to begin with"),
    ("V7.4.2", "could not be signed in twice to begin with"),
    (
        "V6.2.1, V6.2.4, V6.2.5, V6.2.8, V6.2.9, V6.2.11, V6.2.12",
        "could not then sign in",
    ),
];

#[test]
fn no_check_concludes_anything_from_an_account_its_sign_up_did_not_make() {
    for (name, flaws) in [
        (
            "refused",
            Flaws {
                signup_closed: true,
                ..Default::default()
            },
        ),
        (
            "answered and made nothing",
            Flaws {
                signup_does_nothing: true,
                ..Default::default()
            },
        ),
    ] {
        let out = run(flaws);
        // The setup: A, seeded, signed in and opened a private page, so the run reached every check.
        assert!(
            out.steps
                .iter()
                .any(|s| s.starts_with("signed in as A and opened")),
            "{name}: the run did not get going: {:?}",
            out.steps
        );
        assert!(
            out.findings.is_empty(),
            "{name}: findings from accounts never made: {:?}",
            out.findings.iter().map(|f| &f.rule_id).collect::<Vec<_>>()
        );
        for (ids, says) in NOT_MADE {
            assert!(
                out.not_assessed
                    .iter()
                    .any(|(i, why)| i == ids && why.contains(says)),
                "{name}: {ids} does not say \"{says}\": {:?}",
                out.not_assessed
                    .iter()
                    .filter(|(i, _)| i == ids)
                    .collect::<Vec<_>>()
            );
        }
        // Nothing blames the mail server for an email an account that was never made could not get.
        assert!(
            !out.not_assessed
                .iter()
                .any(|(_, why)| why.contains("sent no email")),
            "{name}: {:?}",
            out.not_assessed
        );
    }
}
