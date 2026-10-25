//! The browser's storage after signing out of a sign-in through another service (V14.3.1; backlog 0629): one more
//! ordinary sign-in through the test provider, its cookies handed to the real browser, the private page opened, and
//! the app's own sign-out control clicked, as after the password sign-in (`browser::sign_out_check`). Credited apart
//! from the password sign-in's, and in part, as ADR-053's Later entry of the owner's choice of 20 October 2026 has
//! both stay. Run only when the run has a browser (`[stack.run.users] browser`).

use super::*;

/// The requests a finding here rests on: the page that starts the sign-in, and the provider's return to the app.
const EVIDENCE: [&str; 2] = ["oidc-start-browser", "oidc-back-browser"];

/// What every note here says first.
const THROUGH: &str = "after a sign-in through the test provider, ";

pub(crate) fn browser_storage(http: &mut dyn Http, section: &OidcSection, out: &mut Outcome) {
    let not_assessed = |out: &mut Outcome, why: &str| {
        out.not_assessed.push((
            "V14.3.1".to_owned(),
            format!("In a real browser, {THROUGH}{why}"),
        ));
    };
    let Some(logout) = &section.logout else {
        not_assessed(
            out,
            "whether signing out empties the browser's storage was not asked: `[stack.run.oidc]` has no `logout`, so \
             there is no sign-out control to look for.",
        );
        return;
    };
    if !set_mode(http, idp::NORMAL) {
        not_assessed(out, "the test provider did not take its instructions.");
        return;
    }
    let mut begun = match begin(http, section, "browser") {
        Ok(begun) => begun,
        Err(why) => {
            not_assessed(out, &format!("the sign-in could not be made ({why})."));
            return;
        }
    };
    let Some(back) = http.send(&get(EVIDENCE[1], &begun.back, &begun.session)) else {
        not_assessed(out, "the app did not answer the provider's return.");
        return;
    };
    begun.session.absorb(&back);
    if begun.session.cookies().is_empty() {
        not_assessed(
            out,
            "whether signing out empties the browser's storage was not asked: the sign-in gave no cookie to hand to \
             the browser.",
        );
        return;
    }
    out.steps.push(
        "signed in through the test provider once more, for the browser's storage after signing out".to_owned(),
    );
    // The browser is handed the session's cookies, so it needs no provider. Told to refuse every sign-in for the
    // browser's job, the provider cannot sign the person straight back in when the private page is asked for after
    // signing out, which a real browser, following its redirects, would take for a sign-out that did not end the
    // session.
    if !set_mode(http, idp::REFUSE) {
        not_assessed(out, "the test provider did not take its instructions.");
        return;
    }
    // The app's front page, opened with no cookies, for what it keeps for anyone: the private page would send the
    // browser to the provider, whose storage is not the app's.
    crate::browser::storage_after_sign_out(
        http,
        ("/", &section.private, &logout.path),
        &begun.session,
        THROUGH,
        out,
    );
    set_mode(http, idp::NORMAL);
}

#[cfg(test)]
mod tests;
