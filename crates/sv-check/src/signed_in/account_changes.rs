//! The account changes V7.5.1 names beside the email address: the phone number, the two-factor
//! settings, and what recovers the account (ADR-105, backlog 0232 group G).
//!
//! Each is tried as the email change is (`passwords::change_email_check`): on an account made for it
//! through `signup`, never A or B, the change is asked for with a wrong password and then with the
//! right one. None of the three is something to sign in with, so where the email check signs in
//! with the new address, these look for the new value on the private page stackvet.toml names as
//! `shows`. A change refused with the wrong password counts only beside the same change taking with
//! the right one; one that never takes shows nothing.
//!
//! Each credit is in part: whether V7.5.1 is met whole is decided where the report is put together,
//! once the email change and each of these is refused, or confirmed not offered through `sv review`.

use super::passwords::account_works;
use super::*;

/// The rule a change of this kind is found and credited under.
pub(super) fn rule_for(kind: &str) -> &'static Rule {
    match kind {
        "phone" => &PHONE_CHANGE_WITHOUT_PASSWORD,
        "two-factor" => &TWO_FACTOR_CHANGE_WITHOUT_PASSWORD,
        _ => &RECOVERY_CHANGE_WITHOUT_PASSWORD,
    }
}

/// What a change of this kind changes, as the report says it.
fn what(kind: &str) -> &'static str {
    match kind {
        "phone" => "the phone number",
        "two-factor" => "the two-factor settings",
        _ => "what recovers the account",
    }
}

/// The value sent as `{new_value}`: a phone number for `phone` and `two-factor` (the number the
/// codes go to), and an email address for `recovery`. Each kind and each `turn` takes its own
/// stretch of `spare`, so no value can be mistaken on the page for another: the wrong password's
/// for the right one's, or one change's for another's, which all share one account.
pub(super) fn new_value(kind: &str, spare: &str, turn: usize, user: &str) -> String {
    if kind == "recovery" {
        return format!("back-{turn}.{user}");
    }
    let from = if kind == "phone" { 0 } else { 14 };
    let digits: String = spare
        .chars()
        .cycle()
        .skip(from + turn * 7)
        .take(7)
        .map(|c| char::from(b'0' + (c.to_digit(16).unwrap_or(0) % 10) as u8))
        .collect();
    format!("+1555{digits}")
}

/// Whether `body` shows `value`: as written, or for a phone number by its digits alone, since apps
/// write a number in their own format.
pub(super) fn shows_value(kind: &str, body: &str, value: &str) -> bool {
    if body.contains(value) {
        return true;
    }
    if kind == "recovery" {
        return false;
    }
    let digits = |s: &str| s.chars().filter(char::is_ascii_digit).collect::<String>();
    digits(body).contains(&digits(value))
}

pub(super) fn account_change_checks(
    http: &mut dyn Http,
    users: &UsersSection,
    accounts: &Accounts,
    confirm: Option<&str>,
    out: &mut Outcome,
) {
    const IDS: &str = "V7.5.1";
    let mut given = Vec::new();
    for (kind, key) in sv_manifest::ACCOUNT_CHANGES {
        match users.account_change(kind) {
            Some(change) => given.push((kind, key, change)),
            // Listed as not offered: the report says whether the owner confirmed it (ADR-105).
            None if users.not_offered.iter().any(|n| n == kind) => {}
            None => out.not_assessed.push((
                IDS.to_owned(),
                format!(
                    "Whether changing {} needs the password again: stackvet.toml sets no `{key}` \
                     under `[stack.run.users]`, and `not-offered` does not list `{kind}`.",
                    what(kind)
                ),
            )),
        }
    }
    if given.is_empty() {
        return;
    }
    let all_of = |why: &str, out: &mut Outcome| {
        for (kind, _, _) in &given {
            out.not_assessed.push((
                IDS.to_owned(),
                format!(
                    "Whether changing {} needs the password again: {why}",
                    what(kind)
                ),
            ));
        }
    };
    let Some(signup) = &users.signup else {
        all_of(
            "it is only ever done to an account made for it, and stackvet.toml sets no `signup` \
             to make one.",
            out,
        );
        return;
    };
    let Some(confirm) = confirm else {
        all_of(
            "telling needs a private page a signed-in user alone can open, and none was shown.",
            out,
        );
        return;
    };
    let spare = &accounts.spare;
    if spare.len() < 32 {
        return;
    }
    // One account and one session for all three: none of these changes how the account signs in,
    // so they need no sign-in each, and a run stays inside the sign-ins the spec states.
    let account = Account {
        user: format!("changes.{}", accounts.a.user),
        password: format!("Ac-{}-aZ9!", &spare[2..26]),
    };
    sign_up(http, users, signup, "changes", &account);
    if !account_works(http, users, "changes", &account, confirm, &mut out.steps) {
        all_of(
            &format!(
                "the account made for it, {}, could not sign in to begin with.",
                account.user
            ),
            out,
        );
        return;
    }
    let mut quiet = Vec::new();
    let Some(signed_in) = sign_in(http, users, "changes-session", &account, &mut quiet) else {
        all_of(
            &format!(
                "the account made for it, {}, could not sign in.",
                account.user
            ),
            out,
        );
        return;
    };
    let mut session = signed_in.session;
    let wrong = format!("Wr-{}-aZ9!", &spare[6..30]);
    for (kind, key, change) in given {
        one_change(
            http,
            users,
            &mut session,
            &account,
            &wrong,
            (kind, key, change),
            spare,
            out,
        );
    }
}

/// One change, asked for with `wrong` and then with the account's own password, in `session`.
#[allow(clippy::too_many_arguments)]
fn one_change(
    http: &mut dyn Http,
    users: &UsersSection,
    session: &mut Session,
    account: &Account,
    wrong: &str,
    (kind, key, change): (&str, &str, &sv_manifest::AccountChange),
    spare: &str,
    out: &mut Outcome,
) {
    const IDS: &str = "V7.5.1";
    let what = what(kind);
    let request = change.request();
    let mut pages = vec![change.shows.clone()];
    pages.extend(users.private.iter().cloned());

    // Asks for the change, then opens `shows`: the answer to the change, whether the page could be
    // opened, and whether it shows the value.
    let mut attempt = |http: &mut dyn Http, password: &str, value: &str, label: &str| {
        let values = Values {
            user: &account.user,
            password,
            new_value: value,
            ..Default::default()
        };
        let answer = send_template(
            http,
            &format!("change-{kind}-{label}"),
            &request,
            &values,
            session,
            &pages,
        )
        .0;
        let page = http.send(&get(
            &format!("shows-{kind}-{label}"),
            &change.shows,
            session,
        ));
        if let Some(page) = &page {
            session.absorb(page);
        }
        let shown = ok(&page)
            && page
                .as_ref()
                .is_some_and(|p| shows_value(kind, &p.body, value));
        (answer, ok(&page), shown)
    };
    let unread = |out: &mut Outcome| {
        out.not_assessed.push((
            IDS.to_owned(),
            format!(
                "Whether changing {what} needs the password again: {} (`shows` under `{key}`) \
                 could not be opened while signed in, so whether a change took cannot be seen.",
                change.shows
            ),
        ));
    };

    let value = new_value(kind, spare, 0, &account.user);
    let (answer, opened, shown) = attempt(http, wrong, &value, "wrong-password");
    out.steps.push(format!(
        "asked to change {what} giving a wrong password ({})",
        status(&answer)
    ));
    if shown {
        out.findings.push(finding_on(
            vec![
                format!("change-{kind}-wrong-password"),
                format!("shows-{kind}-wrong-password"),
            ],
            rule_for(kind),
            match kind {
                "phone" => "The phone number can be changed without the password",
                "two-factor" => "The two-factor settings can be changed without the password",
                _ => "What recovers the account can be changed without the password",
            },
            Severity::High,
            format!(
                "A request to {} giving a wrong password changed {what}: {} then showed the new \
                 value.",
                change.path, change.shows
            ),
        ));
        return;
    }
    if !opened {
        unread(out);
        return;
    }

    let value = new_value(kind, spare, 1, &account.user);
    let (answer, opened, shown) = attempt(http, &account.password, &value, "right-password");
    out.steps.push(format!(
        "asked to change {what} giving the right password ({})",
        status(&answer)
    ));
    if !opened {
        unread(out);
        return;
    }
    if !shown {
        out.not_assessed.push((
            IDS.to_owned(),
            format!(
                "A change of {what} through {} with the right password did not take: {} did not \
                 show the new value. That happens when the page writes it differently, shows it \
                 past the first 4,000 characters, or waits for it to be confirmed first; \
                 otherwise check `{key}` in stackvet.toml. With no change that works, a refused \
                 one shows nothing.",
                change.path, change.shows
            ),
        ));
        return;
    }
    let rule = rule_for(kind);
    out.verified.push(
        crate::Verified::new(
            rule.rule_id,
            rule.requirement_ids,
            format!(
                "a change of {what} through {} giving a wrong password, refused where the same \
                 change with the right one took",
                change.path
            ),
        )
        // One of what V7.5.1 names; whether it is met whole is decided with the report (ADR-105).
        .in_part(),
    );
}
