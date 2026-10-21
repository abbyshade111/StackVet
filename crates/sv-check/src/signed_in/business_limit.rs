//! A business limit the documentation states, held on the server (V2.3.2; backlog 0489, the owner's choice of 20
//! October 2026).
//!
//! `[policy] limit` names one: what it is called, `at-most`, and the request that does one more. The first user
//! signs in and sends it `at-most` times, each with a marker of its own so none is refused for repeating a value,
//! then once more. The first `at-most` are the control (ADR-119): each answered as the first was shows the request
//! reaches what the limit guards; one refused before then is a limit lower than stated, a record the account already
//! held, or something else, and nothing is judged. Then the extra one:
//!
//! - **answered as the first was:** the limit is not held, a finding;
//! - **refused with a 4xx that is not a limit's own answer:** the limit seen held, *checked in part*, for the one limit
//!   named;
//! - **429, or 503 with `Retry-After`:** a rate limiter, not the business limit (ADR-021), so not assessed;
//! - **anything else** (a crash, an answer unlike the first): not assessed, with what was compared.
//!
//! It runs after the burst check, which waits out its minute, and as the first user, whose records the burst did not
//! make.

use super::*;

/// The requirement this is about.
const IDS: &str = "V2.3.2";

/// The most `at-most` tried: a stated limit above it is not sent.
pub(super) const MOST_TRIED: u32 = 50;

pub(super) fn business_limit_check(
    http: &mut dyn Http,
    users: &UsersSection,
    account: &Account,
    policy: &sv_manifest::PolicySection,
    out: &mut Outcome,
) {
    let Some(limit) = &policy.limit else {
        out.not_assessed.push((
            IDS.to_owned(),
            "Whether a business limit the documentation states is held: name one as `limit` under `[policy]` in \
             stackvet.toml (what it is called, `at-most`, and the request that does one), and this will try one \
             more than it."
                .to_owned(),
        ));
        return;
    };
    let name = &limit.name;
    if limit.at_most == 0 || limit.at_most > MOST_TRIED {
        out.not_assessed.push((
            IDS.to_owned(),
            format!(
                "Whether \"{name}\" is held: `[policy] limit` says at most {}; this check sends between 2 and {} \
                 requests, so it cannot hold the app to that number.",
                limit.at_most,
                MOST_TRIED + 1
            ),
        ));
        return;
    }
    let template = limit.request();
    let Some(mut session) =
        sign_in(http, users, "limit", account, &mut out.steps).map(|s| s.session)
    else {
        out.not_assessed.push((
            IDS.to_owned(),
            format!("Whether \"{name}\" is held: the first user could not sign in."),
        ));
        return;
    };
    let session = &mut session;
    let values = with_token(
        http,
        "limit-token",
        &template,
        &Values::default(),
        session,
        &users.private,
    );
    let sent = limit.at_most + 1;
    let markers: Vec<String> = (1..=sent)
        .map(|i| format!("sv-probe-limit-7c1d-{i}"))
        .collect();
    let answers: Vec<Option<ProbeResponse>> = markers
        .iter()
        .enumerate()
        .map(|(i, marker)| {
            let v = Values {
                marker,
                ..values.clone()
            };
            http.send(&request(
                &format!("limit-{}", i + 1),
                &template,
                &v,
                session,
            ))
        })
        .collect();
    let id = |n: u32| format!("limit-{n}");
    let not_assessed = |out: &mut Outcome, why: String| {
        out.not_assessed
            .push((IDS.to_owned(), format!("Whether \"{name}\" is held: {why}")));
    };
    // The first is the control's control: it has to be accepted for any answer after it to mean anything.
    let Some(first) = answers
        .first()
        .and_then(Option::as_ref)
        .filter(|_| answers.first().is_some_and(accepted))
    else {
        let first = answers.first().cloned().flatten();
        not_assessed(
            out,
            if super::answer_of(first.as_ref()).is_limited() {
                format!(
                    "the first {} was answered {}, a rate limiter's answer, not the limit's.",
                    template.path,
                    status(&first)
                )
            } else {
                format!(
                    "the first {} was answered {}, before any was made: the account may already hold as many as \
                     the limit allows (other checks make records as this user), or the request does not reach what the \
                     limit guards.",
                    template.path,
                    status(&first)
                )
            },
        );
        return;
    };
    // The rest of the first `at-most`, each answered as the first was.
    for (i, answer) in answers
        .iter()
        .enumerate()
        .take(limit.at_most as usize)
        .skip(1)
    {
        let n = i as u32 + 1;
        match super::control::beside(first, answer) {
            super::control::Beside::Like => {}
            _ if super::answer_of(answer.as_ref()).is_limited() => {
                not_assessed(
                    out,
                    format!(
                        "number {n} of {} was answered {}, a rate limiter's answer, before the limit was reached.",
                        limit.at_most,
                        status(answer)
                    ),
                );
                return;
            }
            super::control::Beside::Refused => {
                not_assessed(
                    out,
                    format!(
                        "number {n} was refused ({}), before the {} stackvet.toml states: the account may already \
                         hold some, or the limit is lower than stated.",
                        status(answer),
                        limit.at_most
                    ),
                );
                return;
            }
            super::control::Beside::Unclear(why) => {
                not_assessed(out, format!("number {n} of {}: {why}.", limit.at_most));
                return;
            }
        }
    }
    let extra = answers.last().cloned().flatten();
    match super::control::beside(first, &extra) {
        super::control::Beside::Like => {
            out.steps.push(format!(
                "\"{name}\": {} of {sent} went through, the last past the {} stated",
                sent, limit.at_most
            ));
            out.findings.push(finding_on(
                vec![id(1), id(sent)],
                &BUSINESS_LIMIT,
                "A stated business limit is not held",
                Severity::Medium,
                format!(
                    "stackvet.toml says \"{name}\" is at most {}. The first test user sent {} {} {sent} times, each \
                     with a value of its own, and the last, one past the limit, was answered as the first was ({}).",
                    limit.at_most,
                    template.method,
                    template.path,
                    status(&extra)
                ),
            ));
        }
        _ if super::answer_of(extra.as_ref()).is_limited() => not_assessed(
            out,
            format!(
                "the one past the limit was answered {}, a rate limiter's answer, not the limit's.",
                status(&extra)
            ),
        ),
        super::control::Beside::Refused => {
            out.steps.push(format!(
                "\"{name}\": {} went through, and the next was refused ({})",
                limit.at_most,
                status(&extra)
            ));
            out.verified.push(
                crate::Verified::new(
                    BUSINESS_LIMIT.rule_id,
                    BUSINESS_LIMIT.requirement_ids,
                    format!(
                        "\"{name}\", at most {} as stackvet.toml states: {} {} went through {} times and was refused \
                         ({}) the next; the one limit named",
                        limit.at_most,
                        template.method,
                        template.path,
                        limit.at_most,
                        status(&extra)
                    ),
                )
                .with_evidence(vec![id(1), id(sent)])
                .in_part(),
            );
        }
        super::control::Beside::Unclear(why) => {
            not_assessed(out, format!("the one past the limit: {why}."));
        }
    }
    crate::verified::unless_credited(BUSINESS_LIMIT.rule_id, &out.verified);
}
