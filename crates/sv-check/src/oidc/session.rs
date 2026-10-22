//! The session a sign-in through another service leaves (backlog 0232, part B): the cookie's attributes (V3.3.2,
//! V3.3.4) and a new session at sign-in (V7.2.4), checked as they are for the form's sign-in, on one more ordinary
//! sign-in through the test provider. The form's sign-in alone stood for "each way of signing in" before.

use super::*;

/// The requests a finding here rests on: the page that starts the sign-in, and the provider's return to the app.
const EVIDENCE: [&str; 2] = ["oidc-start-session", "oidc-back-session"];

pub(super) fn provider_session(http: &mut dyn Http, section: &OidcSection, out: &mut Outcome) {
    let ids = "V3.3.2, V3.3.4, V7.2.4";
    if !set_mode(http, idp::NORMAL) {
        out.not_assessed.push((
            ids.to_owned(),
            "In a sign-in through another service: the test provider did not take its instructions.".to_owned(),
        ));
        return;
    }
    let mut begun = match begin(http, section, "session") {
        Ok(begun) => begun,
        Err(why) => {
            out.not_assessed.push((
                ids.to_owned(),
                format!("In a sign-in through another service: it could not be made ({why})."),
            ));
            return;
        }
    };
    let before_login = begun.session.cookies().to_vec();
    let Some(back) = http.send(&get(EVIDENCE[1], &begun.back, &begun.session)) else {
        out.not_assessed.push((
            ids.to_owned(),
            "In a sign-in through another service: the app did not answer the provider's return."
                .to_owned(),
        ));
        return;
    };
    begun.session.absorb(&back);
    let works = ok(&http.send(&get(
        "oidc-private-session",
        &section.private,
        &begun.session,
    )));
    out.steps.push(format!(
        "signed in through the test provider once more, for its session: {} {}",
        section.private,
        if works { "opened" } else { "did not open" }
    ));
    crate::signed_in::provider_session_checks(
        http,
        begun.session,
        before_login,
        &back,
        works,
        (&section.private, EVIDENCE),
        out,
    );
}

#[cfg(test)]
#[path = "session_tests.rs"]
mod tests;
