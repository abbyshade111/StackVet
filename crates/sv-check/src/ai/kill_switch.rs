//! The kill switch an app's AI feature has, or has not, asked through the test model (moved out of `ai.rs` on 21
//! October 2026, backlog 0324, unchanged; still named `ai::kill_switch`).

use super::*;

/// Whether the kill switch halts the AI feature (C9.6.1), asked of a second copy of the app started
/// with the switch on. `started` is whether that copy came up; `markers` carries the control from
/// the first copy: a plain message reached the model there, with the switch off.
pub fn kill_switch(
    http: &mut dyn Http,
    section: &AiSection,
    ctx: &Context,
    markers: &LogMarkers,
    started: bool,
    out: &mut Outcome,
) {
    const ID: &str = "C9.6.1";
    let say = |why: String, out: &mut Outcome| out.not_assessed.push((ID.to_owned(), why));
    let Some(switch) = &section.kill_switch else {
        say(
            "Whether the AI feature can be switched off: name the setting that does it, as \
             `kill-switch = \"NAME=value\"` under `[stack.run.ai]` in stackvet.toml, and a copy \
             of the app is started with it on and asked."
                .to_owned(),
            out,
        );
        return;
    };
    if !markers.model_reached {
        say(
            "Whether the AI feature can be switched off: with the switch off it never reached the \
             test model, so its not reaching it with the switch on would show nothing."
                .to_owned(),
            out,
        );
        return;
    }
    if !started {
        say(
            format!(
                "Whether the AI feature can be switched off: a copy of the app started with \
                 {switch} did not come up, so there was nothing to ask."
            ),
            out,
        );
        return;
    }
    let mut session = Session::default();
    if section.signed_in {
        let Some((users, account)) = ctx.signed_in else {
            say(
                "Whether the AI feature can be switched off: `ai.signed-in` is set, and there is \
                 no `[stack.run.users]` to sign in with."
                    .to_owned(),
                out,
            );
            return;
        };
        // The copy has none of the first one's accounts unless `seed` gave them to it.
        if !ctx.seeded
            && let Some(signup) = &users.signup
        {
            sign_up(http, users, signup, "b-off", account);
        }
        match sign_in(http, users, "b-off", account, &mut out.steps) {
            Some(signed) => session = signed.session,
            None => {
                say(
                    "Whether the AI feature can be switched off: signing in to the copy started \
                     with the switch on got no answer."
                        .to_owned(),
                    out,
                );
                return;
            }
        }
    }
    let t = tag(200);
    let prompt = format!(
        "Hello, what can you help me with? {}",
        crate::stand_in::marker("PLAIN", &t)
    );
    let answer = send_filled(
        http,
        "ai-kill-switch",
        &with_prompt(section, &prompt),
        &mut session,
        &[],
    );
    let reached = seen(http, &t).unwrap_or_default().received;
    out.steps.push(format!(
        "started a copy of the app with {switch} and sent its AI feature a plain message ({}): {}",
        status(&answer),
        if reached {
            "it reached the test model"
        } else {
            "it did not reach the test model"
        }
    ));
    if reached {
        out.findings.push(on(vec!["ai-kill-switch".to_owned()], finding(
            &KILL_SWITCH,
            "The AI feature's kill switch does not stop it",
            Severity::Medium,
            format!(
                "With {switch} set, as stackvet.toml says turns the AI feature off, a message sent \
                 through {} still reached the model.",
                section.chat.path
            ),
        )));
    } else if answer
        .as_ref()
        .is_some_and(|r| (200..500).contains(&r.status))
    {
        out.verified.push(
            crate::Verified::new(
                KILL_SWITCH.rule_id,
                KILL_SWITCH.requirement_ids,
                format!(
                    "a copy of the app started with {switch} answered the AI feature ({}) without \
                 calling the model, where the same message reached it with the switch off; that \
                 the switch takes effect without a restart was not shown",
                    status(&answer)
                ),
            )
            // A copy started with the switch on, not one switched while it ran: the "immediately" half
            // of C9.6.1 was not shown (backlog 0304; ADR-053's in-part credit).
            .in_part(),
        );
    } else if answer.as_ref().is_some_and(|r| r.status >= 500) {
        // A server error cannot be told from a copy that crashed, so it is not a pass: the same status
        // rule as C2.1.3 (200 to 499 answers the request).
        say(
            format!(
                "Whether the AI feature can be switched off: with {switch} set, the copy answered with a \
                 server error ({}), which cannot be told from a copy that crashed. A feature that is off \
                 should answer with a status below 500.",
                status(&answer)
            ),
            out,
        );
    } else {
        say(
            format!(
                "Whether the AI feature can be switched off: with {switch} set, the app gave no \
                 answer at all, which is not the same as the feature being off."
            ),
            out,
        );
    }
}
