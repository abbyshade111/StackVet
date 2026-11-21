//! V10.5.3: whether the app refuses a sign-in provider whose published details (its discovery document,
//! `/.well-known/openid-configuration`) name another provider as their issuer (backlog 0620, ADR-127, Later, 21
//! November 2026).
//!
//! An app reads those details once, usually as it starts, so changing them under the copy of the app the rest of the
//! run talks to shows nothing. This starts a second copy of the app, as the first was started, while the test
//! provider's details name another issuer from the beginning, and signs in to it. Every address in the details is
//! still the test provider's, and the ID tokens still name it: only the name the details give themselves is wrong.
//!
//! - **The copy read the details and signed the probes in:** a finding, low. The app took details whose issuer did
//!   not match the one it was given (`OIDC_ISSUER`).
//! - **It read them and refused, or did not start at all:** credited *in part* only once a third copy, started while
//!   the details are right, signs in. Without that, the refusal could be the copy not working.
//! - **It never read them:** not assessed. An app given the provider's addresses directly has no details to check.

use super::{IDS, begin, finish, set_mode, sign_in};
use crate::finding::Severity;
use crate::script::{Services, Target, With};
use crate::signed_in::{Outcome, Patient, Rule, Session, finding, get};
use crate::stand_in::oidc as idp;
use std::cell::Cell;
use sv_manifest::OidcSection;

pub(super) const WRONG_METADATA: Rule = Rule {
    rule_id: "probe.oidc-metadata-issuer-not-checked",
    requirement_ids: &["V10.5.3"],
    cwe: &["CWE-346"],
    impact: "The app trusts the sign-in provider's published details even when they say they come from a \
             different provider. Whoever can serve those details, from a look-alike address or by changing them \
             on the way, can pass their own server off as the provider the app trusts.",
    fix: "After fetching the provider's discovery document, refuse it unless its `issuer` is exactly the issuer \
          address the app was configured with. Most OpenID Connect libraries check this when they are given the \
          issuer and left to find the rest; do not turn the check off.",
};

/// What one copy of the app did, started while the provider was set to a mode.
struct Copy {
    /// Whether it started and answered.
    started: bool,
    /// Whether a sign-in to it opened the private page; `Err` with why, when the sign-in could not be begun.
    opened: Result<bool, String>,
}

/// How many times the provider served its details wrong since its mode was last set, or `None` when it did not say.
fn served_wrong(http: &mut dyn crate::signed_in::Http) -> Option<u64> {
    let answer = http.provider(&get(
        "oidc-metadata-reads",
        idp::METADATA,
        &Session::default(),
    ))?;
    if answer.status != 200 {
        return None;
    }
    serde_json::from_str::<serde_json::Value>(&answer.body)
        .ok()?
        .get("served_wrong")?
        .as_u64()
}

/// Starts the second copy, with the provider already set, and signs in to it once.
fn try_copy(
    services: &dyn Services,
    http: &mut dyn crate::signed_in::Http,
    section: &OidcSection,
    who: &str,
) -> Copy {
    if !services.start_again() {
        return Copy {
            started: false,
            opened: Err("it did not start".to_owned()),
        };
    }
    let opened = begin(http, section, who).map(|mut begun| {
        let back = begun.back.clone();
        finish(http, section, &mut begun.session, &back, who)
    });
    Copy {
        started: true,
        opened,
    }
}

/// Whether the ordinary sign-in through the test provider worked in the run's own copy of the app: every way
/// `oidc::run` can stop early says so under [`IDS`], and V10.5.3 is among them.
fn ordinary_sign_in_worked(out: &Outcome) -> bool {
    !out.not_assessed.iter().any(|(ids, _)| ids == IDS)
}

/// V10.5.3, asked of a second copy of the app, after everything else asked of the first. `out` is the sign-in
/// suite's own outcome; nothing is started when the ordinary sign-in did not work there, since that already says
/// V10.5.3 was not assessed.
pub(crate) fn wrong_metadata(
    services: &dyn Services,
    section: &OidcSection,
    spent: &Cell<u64>,
    out: &mut Outcome,
) {
    if !ordinary_sign_in_worked(out) {
        return;
    }
    services.starting(
        "a second copy of the app, with the sign-in provider's details naming another issuer",
    );
    let mut asked = Outcome::default();
    let mut http = services.http(
        Target::Again,
        With {
            provider: true,
            ..With::default()
        },
    );
    let mut patient = Patient::within(http.as_mut(), spent);
    ask(services, &mut patient, section, &mut asked);
    patient.settle(&mut asked);
    out.findings.extend(asked.findings);
    out.verified.extend(asked.verified);
    out.not_assessed.extend(asked.not_assessed);
    out.steps.extend(asked.steps);
}

fn ask(
    services: &dyn Services,
    http: &mut dyn crate::signed_in::Http,
    section: &OidcSection,
    out: &mut Outcome,
) {
    let not_assessed = |out: &mut Outcome, why: String| {
        out.not_assessed
            .push((WRONG_METADATA.requirement_ids.join(", "), why));
    };
    if !set_mode(http, idp::WRONG_METADATA_ISS) {
        not_assessed(
            out,
            "The test provider did not take its instructions to name another issuer in its published details, \
             so no second copy of the app was started."
                .to_owned(),
        );
        return;
    }
    let wrong = try_copy(services, http, section, "again-wrong");
    // Read before the mode is put back, which starts the count again.
    let reads = served_wrong(http);
    services.remove_again();
    let put_back = set_mode(http, idp::NORMAL);
    let did = match (&wrong.started, &wrong.opened) {
        (false, _) => "did not start".to_owned(),
        (true, Ok(true)) => format!("signed in: {} opened", section.private),
        (true, Ok(false)) => format!("did not sign in: {} stayed shut", section.private),
        (true, Err(why)) => format!("did not begin a sign-in ({why})"),
    };
    let read = match reads {
        Some(0) => "never asked for the details".to_owned(),
        Some(1) => "asked for the details once".to_owned(),
        Some(n) => format!("asked for the details {n} times"),
        None => "the provider did not say whether the details were read".to_owned(),
    };
    out.steps.push(format!(
        "started a second copy of the app while the test provider's published details named another issuer: \
         it {read}, and {did}"
    ));
    let Some(reads) = reads else {
        not_assessed(
            out,
            "The test provider did not say whether the second copy of the app read its published details, so \
             what that copy did cannot be put down to them."
                .to_owned(),
        );
        return;
    };
    if reads == 0 {
        not_assessed(
            out,
            format!(
                "A second copy of the app, started while the test provider's published details named another \
                 issuer, never asked for them ({did}). An app given the provider's addresses directly has no \
                 published details to check, and this run cannot tell that apart from one that reads them only \
                 later."
            ),
        );
        return;
    }
    if wrong.opened == Ok(true) {
        out.findings.push(finding(
            &WRONG_METADATA,
            "The app takes a sign-in provider's details that name another provider",
            Severity::Low,
            format!(
                "A second copy of the app was started while the test provider's published details \
                 (`/.well-known/openid-configuration`) named `{}` as their issuer, where the app was told \
                 the provider's own address. It read them and signed the probes in: {} opened.",
                idp::OTHER_ISSUER,
                section.private
            ),
        ));
        return;
    }
    // Refused, or never started: credited only when a copy started while the details are right signs in.
    if !put_back {
        not_assessed(
            out,
            "The test provider did not take its instructions to put its published details right, so the \
             second copy's refusal could not be checked against one that should sign in."
                .to_owned(),
        );
        return;
    }
    let control = if services.start_again() {
        sign_in(http, section, idp::NORMAL, "again-right").unwrap_or(false)
    } else {
        false
    };
    services.remove_again();
    out.steps.push(format!(
        "started the copy again with the published details right: {}",
        if control {
            format!("signed in, {} opened", section.private)
        } else {
            "it did not sign in".to_owned()
        }
    ));
    if control {
        out.verified.push(
            crate::Verified::new(
                WRONG_METADATA.rule_id,
                WRONG_METADATA.requirement_ids,
                format!(
                    "a copy of the app started while the test provider's published details named another \
                     issuer read them and {}, where a copy started with them right signed in: one wrong \
                     issuer, read as the app started",
                    if wrong.started {
                        "signed nobody in"
                    } else {
                        "did not start"
                    }
                ),
            )
            .in_part(),
        );
    } else {
        not_assessed(
            out,
            format!(
                "A second copy of the app, started while the test provider's published details named another \
                 issuer, {did}, but a copy started with the details right did not sign in either, so the \
                 refusal may not have been about the details."
            ),
        );
    }
}

#[cfg(test)]
#[path = "metadata_tests.rs"]
mod tests;
