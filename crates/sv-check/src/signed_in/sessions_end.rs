//! Ending sessions (backlog 0492, ADR-125): a user ends their other sessions, giving the password again (V7.5.2), and
//! an admin ends a user's (V7.4.5), each tried from the manifest's `end-other-sessions` and `admin-end-sessions`.
//!
//! Both end the first user's sessions, so they run near the end of the run, after every check that uses A's session.
//! Each signs A in afresh, and each judges a session by whether it still opens the first private page.

use super::*;

/// Whether `session` still opens the private page `confirm`, asked as `id`.
fn opens(http: &mut dyn Http, id: &str, confirm: &str, session: &Session) -> bool {
    ok(&http.send(&get(id, confirm, session)))
}

/// A signed in afresh, as `who`, with the session shown to open `confirm`; `None`, said, when it does not.
fn fresh_session(
    http: &mut dyn Http,
    users: &UsersSection,
    who: &str,
    account: &Account,
    confirm: &str,
    out: &mut Outcome,
) -> Option<Session> {
    let signed_in = sign_in(http, users, who, account, &mut out.steps)?;
    opens(http, &format!("private-{who}"), confirm, &signed_in.session).then_some(signed_in.session)
}

/// V7.5.2 and V7.1.2's observation: A signs in twice, ends the other session from the first giving a wrong password
/// and then the right one, and the second session is asked again after each.
pub(super) fn end_other_sessions_check(
    http: &mut dyn Http,
    users: &UsersSection,
    accounts: &Accounts,
    confirm: &str,
    out: &mut Outcome,
) {
    let Some(end) = &users.end_other_sessions else {
        return;
    };
    const IDS: &str = "V7.5.2";
    let a = &accounts.a;
    let (Some(first), Some(second)) = (
        fresh_session(http, users, "end-1", a, confirm, out),
        fresh_session(http, users, "end-2", a, confirm, out),
    ) else {
        out.not_assessed.push((
            IDS.to_owned(),
            "Ending other sessions was not tried: the first test user, signed in afresh twice, did not open the \
             private page each time."
                .to_owned(),
        ));
        return;
    };
    // V7.1.2 asks the documentation how many sessions are allowed; this is only what the app did.
    out.steps.push(
        "signed in as A twice: both sessions stayed valid, which is a design choice, not a finding (V7.1.2 asks \
         that it be documented)"
            .to_owned(),
    );
    let asks_password = end
        .form
        .values()
        .chain(end.json.values())
        .any(|v| v.contains("{password}"));
    let send = |http: &mut dyn Http, id: &str, password: &str| {
        let values = Values {
            user: &a.user,
            password,
            ..Default::default()
        };
        let mut session = first.clone();
        send_template(http, id, end, &values, &mut session, &[]).0
    };

    // A wrong password first: a request that takes it ends the sessions without the password.
    if asks_password {
        let wrong = format!("Wrong-{}-aZ9!", &accounts.spare[..12]);
        let answer = send(http, "end-sessions-wrong", &wrong);
        out.steps.push(format!(
            "asked to end the other sessions giving a wrong password ({})",
            status(&answer)
        ));
        if !opens(http, "private-end-2-after-wrong", confirm, &second) {
            out.findings.push(finding_on(
                vec![
                    "end-sessions-wrong".to_owned(),
                    "private-end-2-after-wrong".to_owned(),
                ],
                &SESSIONS_END_WITHOUT_PASSWORD,
                "Other sessions can be ended without the password",
                Severity::Medium,
                format!(
                    "A request to {} giving a wrong password ended the first test user's other session: it no longer \
                     opened {confirm}.",
                    end.path
                ),
            ));
            return;
        }
    }

    let answer = send(http, "end-sessions", &a.password);
    out.steps.push(format!(
        "asked to end the other sessions{} ({})",
        if asks_password {
            " giving the right password"
        } else {
            ""
        },
        status(&answer)
    ));
    // A refusal or a crash is not the sessions ended or kept: the request did not do anything to judge (ADR-021).
    if !answer_of(answer.as_ref())
        .answered()
        .is_some_and(|r| r.status < 400)
    {
        out.not_assessed.push((
            IDS.to_owned(),
            format!(
                "The request to {} to end the other sessions{} was answered {}, so nothing ended to judge. Check \
                 `end-other-sessions` in stackvet.toml: a field the app needs may be missing.",
                end.path,
                if asks_password {
                    ", giving the right password,"
                } else {
                    ""
                },
                status(&answer)
            ),
        ));
        return;
    }
    let still_opens = opens(http, "private-end-2-after", confirm, &second);
    if still_opens {
        out.findings.push(finding_on(
            vec!["end-sessions".to_owned(), "private-end-2-after".to_owned()],
            &OTHER_SESSIONS_NOT_ENDED,
            "Ending other sessions leaves them working",
            Severity::Medium,
            format!(
                "After a request to {} ({}), the first test user's other session still opened {confirm}.",
                end.path,
                status(&answer)
            ),
        ));
        return;
    }
    if !opens(http, "private-end-1-after", confirm, &first) {
        // The session that asked was ended too: a sign-out of every session, not of the others. Nothing here says the
        // others were ended on purpose.
        out.not_assessed.push((
            IDS.to_owned(),
            format!(
                "A request to {} ended the session that sent it as well as the other one, so it reads as a sign-out \
                 rather than ending the other sessions.",
                end.path
            ),
        ));
        return;
    }
    if !asks_password {
        out.findings.push(finding_on(
            vec!["end-sessions".to_owned(), "private-end-2-after".to_owned()],
            &SESSIONS_END_WITHOUT_PASSWORD,
            "Other sessions are ended without the password being asked",
            Severity::Low,
            format!(
                "A request to {} ended the first test user's other session, and `end-other-sessions` in \
                 stackvet.toml sends no `{{password}}`: the password is not asked again. If the app does ask, add the \
                 field.",
                end.path
            ),
        ));
        return;
    }
    out.verified.push(
        crate::Verified::new(
            SESSIONS_END_WITHOUT_PASSWORD.rule_id,
            SESSIONS_END_WITHOUT_PASSWORD.requirement_ids,
            format!(
                "ending the other sessions through {} refused with a wrong password, and with the right one the other \
                 session no longer opened {confirm} while the one that asked still did (one other session; viewing \
                 the sessions not tried)",
                end.path
            ),
        )
        .in_part(),
    );
}

/// V7.4.5: the admin ends A's sessions, and A's session is asked again.
pub(super) fn admin_end_sessions_check(
    http: &mut dyn Http,
    users: &UsersSection,
    accounts: &Accounts,
    confirm: &str,
    out: &mut Outcome,
) {
    let Some(end) = &users.admin_end_sessions else {
        return;
    };
    const IDS: &str = "V7.4.5";
    let Some(admin) = &accounts.admin else {
        out.not_assessed.push((
            IDS.to_owned(),
            "`admin-end-sessions` is set, but the run has no admin account (`seed` and `admin` make one), so nobody \
             could send it."
                .to_owned(),
        ));
        return;
    };
    let Some(target) = fresh_session(http, users, "end-target", &accounts.a, confirm, out) else {
        out.not_assessed.push((
            IDS.to_owned(),
            "An admin ending a user's sessions was not tried: the first test user, signed in afresh, did not open the \
             private page."
                .to_owned(),
        ));
        return;
    };
    let Some(signed_in) = sign_in(http, users, "admin-end", admin, &mut out.steps) else {
        out.not_assessed.push((
            IDS.to_owned(),
            "An admin ending a user's sessions was not tried: signing in as the admin got no answer.".to_owned(),
        ));
        return;
    };
    let values = Values {
        user: &accounts.a.user,
        ..Default::default()
    };
    let mut session = signed_in.session.clone();
    let answer = send_template(http, "admin-end-sessions", end, &values, &mut session, &[]).0;
    out.steps.push(format!(
        "asked, as the admin, to end the first test user's sessions ({})",
        status(&answer)
    ));
    if !answer_of(answer.as_ref())
        .answered()
        .is_some_and(|r| r.status < 400)
    {
        out.not_assessed.push((
            IDS.to_owned(),
            format!(
                "The admin's request to {} was answered {}, so nothing ended to judge. Check `admin-end-sessions` in \
                 stackvet.toml.",
                end.path,
                status(&answer)
            ),
        ));
        return;
    }
    if opens(http, "private-end-target-after", confirm, &target) {
        out.findings.push(finding_on(
            vec![
                "admin-end-sessions".to_owned(),
                "private-end-target-after".to_owned(),
            ],
            &ADMIN_END_SESSIONS_IGNORED,
            "An admin's request to end a user's sessions leaves them working",
            Severity::Medium,
            format!(
                "After the admin's request to {} ({}), the first test user's session still opened {confirm}.",
                end.path,
                status(&answer)
            ),
        ));
        return;
    }
    out.verified.push(
        crate::Verified::new(
            ADMIN_END_SESSIONS_IGNORED.rule_id,
            ADMIN_END_SESSIONS_IGNORED.requirement_ids,
            format!(
                "the admin's request to {} ended the first test user's session: it no longer opened {confirm} (one \
                 user's one session; every user's not tried)",
                end.path
            ),
        )
        .in_part(),
    );
}
