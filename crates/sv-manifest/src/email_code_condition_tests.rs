//! V6.6.2's condition, `out-of-band-or-email-code`: yes for an `email-code` entry under `[stack.run.users]`
//! whatever `out-of-band-auth` says, and otherwise that answer as given, so V6.6.1 stays phone, SMS, and
//! push only (backlog 0468; ADR-015, Later, 14 October 2026).

use super::*;

const EMAIL_CODE: &str = "email-code = { request = { path = \"/login/code\", form = { email = \
                          \"{user}\" } }, use = { path = \"/login/verify\", form = { code = \
                          \"{code}\" } } }\n";

/// A manifest with `out-of-band-auth` as given (absent for `None`), and an `email-code` entry or not.
fn manifest(out_of_band: Option<bool>, email_code: bool) -> Manifest {
    let mut text = String::from("manifest-version = 1\n[app]\nname = \"R\"\n[capabilities]\n");
    if let Some(v) = out_of_band {
        text.push_str(&format!("out-of-band-auth = {v}\n"));
    }
    text.push_str(
        "[stack]\nlanguages = [\"python\"]\n[stack.run]\nstart = \"python app.py\"\n\
         [stack.run.users]\nseed = \"s\"\nlogin = { path = \"/login\" }\nprivate = [\"/a\"]\n",
    );
    if email_code {
        text.push_str(EMAIL_CODE);
    }
    let m = Manifest::parse(&text, Path::new("stackvet.toml")).expect("manifest parses");
    // The setup: the entry is read, or not, as asked, so what follows is about the condition.
    assert_eq!(
        m.stack.run.users.as_ref().map(|u| u.email_code.is_some()),
        Some(email_code)
    );
    assert_eq!(m.capabilities.out_of_band_auth, out_of_band);
    m
}

/// The two conditions' answers, and the claim line for V6.6.2's, with what the code was found to hold.
fn answers(
    m: &Manifest,
    provider_in_code: Option<bool>,
) -> (Option<bool>, Option<bool>, ResolvedClaim) {
    let (ctx, resolved) = resolve(m, &|c| {
        (c == Condition::OutOfBandAuth)
            .then_some(provider_in_code)
            .flatten()
    });
    let line = resolved
        .into_iter()
        .find(|r| r.condition == Condition::OutOfBandOrEmailCode)
        .expect("V6.6.2's condition is among the claims");
    (
        ctx.get(Condition::OutOfBandAuth),
        ctx.get(Condition::OutOfBandOrEmailCode),
        line,
    )
}

#[test]
fn an_email_code_entry_makes_v6_6_2_apply_and_leaves_v6_6_1_to_the_owners_answer() {
    for out_of_band in [None, Some(false)] {
        let (v6_6_1, v6_6_2, _) = answers(&manifest(out_of_band, true), None);
        assert_eq!(v6_6_2, Some(true), "with out-of-band-auth {out_of_band:?}");
        assert_eq!(v6_6_1, out_of_band, "V6.6.1 is phone, SMS, and push only");
    }
}

#[test]
fn without_an_email_code_entry_the_owners_answer_stands_and_its_no_is_labeled() {
    // Silence stays silence: not a no.
    let (_, v6_6_2, _) = answers(&manifest(None, false), None);
    assert_eq!(v6_6_2, None);
    // The owner's no, with nothing in the code to say either way, is taken on their word and says so.
    let (_, v6_6_2, line) = answers(&manifest(Some(false), false), None);
    assert_eq!(v6_6_2, Some(false));
    assert_eq!(line.state, ClaimState::Unverifiable, "{line:?}");
    // And their yes is a yes.
    let (_, v6_6_2, _) = answers(&manifest(Some(true), false), None);
    assert_eq!(v6_6_2, Some(true));
}

#[test]
fn a_phone_or_sms_provider_in_the_code_makes_both_apply_over_the_owners_no() {
    let (v6_6_1, v6_6_2, line) = answers(&manifest(Some(false), false), Some(true));
    assert_eq!(v6_6_1, Some(true));
    assert_eq!(v6_6_2, Some(true));
    assert_eq!(line.state, ClaimState::Contradicted, "{line:?}");
}
