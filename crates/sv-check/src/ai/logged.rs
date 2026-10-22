//! What the app's own output recorded about the AI feature (moved out of `ai.rs` on 21 October 2026, backlog 0324,
//! unchanged; still named `ai::logged`, `ai::LogMarkers`, and the rest).

use super::*;

// ------------------------------------------------------------------------------------------------
// What the app's own output recorded about the AI feature (C12.1.3, C12.2.1)
//
// Read after everything else, like the log check in `logs.rs`, and for the same reason credited or
// faulted only on a line that was found: an app that logs to a file or a service writes nothing to
// its output and is not logging any less for it.

/// What the log is searched for, from what the AI checks did.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LogMarkers {
    /// The call the plain message made: the model it asked for, and the token counts the test
    /// model reported, which nothing else in the run could have produced together.
    pub call: Option<Call>,
    /// The tag carried by the textbook injection.
    pub injection: Option<String>,
    /// Whether a plain message reached the model: the control for the kill switch, asked of a
    /// second copy of the app.
    pub model_reached: bool,
    /// The signed-in test user the AI feature was asked as, when it was asked signed in.
    pub who: Option<String>,
    /// The tag of the message the test model failed on, when the failure reached it: its error
    /// carries `SVERR` and this tag, which nothing else in the run writes (V16.3.4, ADR-071).
    pub failure: Option<String>,
    /// The tag the MCP control question's call to `sv_lookup` carried as its argument, when the call
    /// reached the test MCP server (C12.4.2, ADR-075).
    pub tool_call: Option<String>,
    /// Whether the textbook injection reached the model, when a plain message straight after it did,
    /// so a refusal of it was of it (C11.4.2, ADR-079).
    pub injection_reached: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Call {
    pub model: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
}

/// V16.3.4 (ADR-071): only ever credited, and in part, so its impact and fix say what the credit
/// stands for.
pub(super) const FAILURE_LOGGED: Rule = Rule {
    rule_id: "probe.ai-service-failure-logged",
    requirement_ids: &["V16.3.4"],
    cwe: &["CWE-778"],
    impact: "An outside service failing, which nothing in the app expected, leaves no trace to find \
             the cause by, or to see that it keeps happening.",
    fix: "Write every unexpected error to the app's log, with the error's own message and what the \
          app was doing, and never the secrets or personal data in the request.",
};

/// V16.3.4, from the app's output after the run: the failed message's marker found in it credits,
/// in part; not found says nothing either way, since an app that logs elsewhere writes nothing
/// there. Asked whenever the AI checks ran, before anything else here can return.
fn failure_logged(markers: &LogMarkers, log: &str, out: &mut Outcome) {
    if markers.call.is_none() && markers.injection.is_none() && markers.failure.is_none() {
        return;
    }
    let why = match &markers.failure {
        None => Some(
            "Whether the app logs an unexpected error: the message the test model was to fail on \
             did not reach it, so there was no failure to look for."
                .to_owned(),
        ),
        Some(tag) if log.contains(&format!("SVERR{tag}")) => {
            out.verified.push(
                crate::Verified::new(
                    FAILURE_LOGGED.rule_id,
                    FAILURE_LOGGED.requirement_ids,
                    "the AI service failing one message on purpose: the app's output carried the \
                     service's own error for it; one kind of failure of one service, not every \
                     unexpected error or security control failure"
                        .to_owned(),
                )
                .in_part(),
            );
            // The line the failure was found on, kept beside the report so a person can check it (0229, part 3).
            if let Some(line) = log.lines().find(|l| l.contains(&format!("SVERR{tag}"))) {
                out.log_lines.push(crate::logs::KeptLine {
                    read_for: "the AI service failing one message on purpose (V16.5.2, V16.5.3)"
                        .to_owned(),
                    line: line.to_owned(),
                });
            }
            None
        }
        Some(_) => Some(
            "Whether the app logs an unexpected error: the AI service failed one message on \
             purpose, and its error was not in what the app wrote to its output. That is not a \
             finding: an app that logs to a file or a service writes nothing there."
                .to_owned(),
        ),
    };
    if let Some(why) = why {
        out.not_assessed.push(("V16.3.4".to_owned(), why));
    }
    crate::verified::unless_credited(FAILURE_LOGGED.rule_id, &out.verified);
}

pub(super) const CALL_LOG: Rule = Rule {
    rule_id: "probe.ai-call-log-incomplete",
    requirement_ids: &["C12.1.3"],
    cwe: &["CWE-778"],
    impact: "A record of each model call that leaves out which model, how many tokens, which \
             service, or what kind of call cannot answer the questions it is kept for: what a bill \
             was for, which model said something, or when usage changed.",
    fix: "Write one structured record per call (JSON, say) with the model, the input and output \
          token counts, the service, and the operation, in the same field names every time.",
};

pub(super) const INJECTION_LOGGED: &str = "probe.ai-injection-logged";

/// C11.4.2 (ADR-079).
pub(super) const FLAG_NOT_GATING: Rule = Rule {
    rule_id: "probe.ai-flag-not-gating",
    requirement_ids: &["C11.4.2"],
    cwe: &["CWE-693"],
    impact: "The app notices an attack and lets it through anyway: the record says it was caught, \
             and the model was given it as it came, so the detector gives comfort and no protection.",
    fix: "Make the screen's verdict decide what happens next: refuse a flagged message, or answer it \
          without the model, instead of only writing the flag down and passing the message on.",
};

/// C11.4.2: the textbook injection, recorded in the app's output as caught (`caught`), and whether
/// it then reached the model. Stopped is credited in part; passed on is the finding; anything else,
/// with no flag seen or no plain message getting through, is said.
fn flag_gating(markers: &LogMarkers, caught: bool, out: &mut Outcome) {
    let say = |why: &str, out: &mut Outcome| {
        out.not_assessed.push((
            "C11.4.2".to_owned(),
            format!("Whether a flagged input is stopped: {why}"),
        ));
    };
    if !caught {
        say(
            "no line of the app's output recorded the textbook prompt injection as caught, so no flag \
             was seen to judge.",
            out,
        );
        return;
    }
    match markers.injection_reached {
        Some(false) => out.verified.push(
            crate::Verified::new(
                FLAG_NOT_GATING.rule_id,
                FLAG_NOT_GATING.requirement_ids,
                "a textbook prompt injection the app's output recorded as caught, kept from the model \
                 where a plain message straight after reached it; one kind of anomalous input"
                    .to_owned(),
            )
            .in_part(),
        ),
        Some(true) => out.findings.push(on(vec![question_id("INJECT", 4), question_id("PLAIN", 5)], finding(
            &FLAG_NOT_GATING,
            "A prompt injection the app flagged is passed to the model anyway",
            Severity::Medium,
            "The app's output recorded the textbook prompt injection as caught, and the model was \
             given it as it came: the flag did not stop it."
                .to_owned(),
        ))),
        None => say(
            "the app's output recorded the textbook prompt injection as caught, but a plain message \
             straight after it did not reach the model either, so whether the flag was what stopped it \
             cannot be told.",
            out,
        ),
    }
    crate::verified::unless_credited(FLAG_NOT_GATING.rule_id, &out.verified);
}

/// C12.4.2 (ADR-075): only ever credited, and in part.
pub(super) const TOOL_ACTION_LOGGED: Rule = Rule {
    rule_id: "probe.ai-tool-action-logged",
    requirement_ids: &["C12.4.2"],
    cwe: &["CWE-778"],
    impact: "An action the AI took that leaves no record of what it did, with what, when and on \
             whose say-so cannot be reviewed after something goes wrong.",
    fix: "Write each tool call the AI makes as an audit record: the tool, its arguments, the time, \
          the user it acted for, who approved it if anyone did, and what came of it.",
};

/// C12.4.2: a line of the app's output recording the MCP control question's call to `sv_lookup`,
/// with the tag as its argument. A line carrying the tag only inside the person's message as it was
/// logged records the message, not the action, and does not count. Credited in part, saying
/// whether the line has a time; never a finding.
fn tool_action_logged(markers: &LogMarkers, log: &str, out: &mut Outcome) {
    let Some(tag) = &markers.tool_call else {
        return;
    };
    let message_marker = format!("SV-PROBE-MCPPLAIN-{tag}");
    let line = log.lines().find(|line| {
        line.contains("sv_lookup") && line.replace(&message_marker, "").contains(tag.as_str())
    });
    // The line that recorded the call, kept beside the report so a person can check it (0229, part 3).
    if let Some(found) = line {
        out.log_lines.push(crate::logs::KeptLine {
            read_for: "the AI's call to the test tool (C12.4.2)".to_owned(),
            line: found.to_owned(),
        });
    }
    match line {
        Some(line) => out.verified.push(
            crate::Verified::new(
                TOOL_ACTION_LOGGED.rule_id,
                TOOL_ACTION_LOGGED.requirement_ids,
                format!(
                    "a line in the app's output recording the AI's call to the test tool `sv_lookup` \
                     with its argument{}; a read-only lookup, not a security-critical action, and no \
                     approver or outcome was seen",
                    if crate::logs::has_timestamp(line) {
                        ", and when"
                    } else {
                        ", though not when"
                    }
                ),
            )
            .in_part(),
        ),
        None => out.not_assessed.push((
            "C12.4.2".to_owned(),
            "Whether the AI's actions are audited: no line of the app's output recorded its call to \
             the test tool `sv_lookup` with the argument it was given. That is not a finding: actions \
             may be audited in another system."
                .to_owned(),
        )),
    }
    crate::verified::unless_credited(TOOL_ACTION_LOGGED.rule_id, &out.verified);
}

/// C12.1.2 (ADR-074): only ever credited, and in part.
pub(super) const SAFETY_DETAIL: Rule = Rule {
    rule_id: "probe.ai-safety-decision-detailed",
    requirement_ids: &["C12.1.2"],
    cwe: &["CWE-778"],
    impact: "A record that something was blocked, without why or when, cannot be audited, cannot \
             show a filter that blocks too much, and cannot be laid beside other events in an \
             investigation.",
    fix: "Write each safety decision as one structured record: what was decided, the rule or \
          category that decided it, when, and for which user and session.",
};

/// Field names that say why a safety decision was made.
const REASON_FIELDS: &[&str] = &["reason", "category", "rule", "policy", "score", "label"];

/// C12.1.2: the line recording the textbook injection as caught, read for why and when. Credited in
/// part when it carries both a reason field and a time; the credit says whether it also names the
/// user or a session. Never a finding: the detail may be recorded elsewhere. `line` is `None` when
/// no line recorded the injection as caught.
fn safety_detail(markers: &LogMarkers, line: Option<&str>, out: &mut Outcome) {
    let say = |why: String, out: &mut Outcome| {
        out.not_assessed.push(("C12.1.2".to_owned(), why));
    };
    let Some(line) = line else {
        say(
            "Whether safety decisions are logged in detail: no line of the app's output recorded the \
             textbook prompt injection as caught, so there was no decision to read."
                .to_owned(),
            out,
        );
        return;
    };
    let lower = line.to_lowercase();
    let field = |f: &str| lower.contains(&format!("\"{f}\"")) || lower.contains(&format!("{f}="));
    let reason = REASON_FIELDS.iter().find(|f| field(f));
    let timed = crate::logs::has_timestamp(line);
    let whose = markers
        .who
        .as_ref()
        .is_some_and(|who| lower.contains(&who.to_lowercase()))
        || SESSION_FIELDS.iter().any(|f| field(f));
    match reason {
        Some(reason) if timed => out.verified.push(
            crate::Verified::new(
                SAFETY_DETAIL.rule_id,
                SAFETY_DETAIL.requirement_ids,
                format!(
                    "the line recording the textbook prompt injection as caught carries why (a \
                     `{reason}` field) and when{}; one kind of safety decision, the injection screen",
                    if whose {
                        ", and whose request it was"
                    } else {
                        ", but not whose request it was"
                    }
                ),
            )
            .in_part(),
        ),
        _ => {
            let mut missing = Vec::new();
            if reason.is_none() {
                missing.push("why (a reason, category, rule, policy, score or label field)");
            }
            if !timed {
                missing.push("when (a timestamp)");
            }
            say(
                format!(
                    "Whether safety decisions are logged in detail: the line recording the textbook \
                     prompt injection as caught does not say {}. That is not a finding: the detail \
                     may be recorded elsewhere.",
                    missing.join(" or ")
                ),
                out,
            );
        }
    }
    crate::verified::unless_credited(SAFETY_DETAIL.rule_id, &out.verified);
}

/// Field names that tie a record to a user or a session.
const SESSION_FIELDS: &[&str] = &[
    "user",
    "user_id",
    "userid",
    "session",
    "session_id",
    "sessionid",
    "conversation_id",
    "conversationid",
    "trace_id",
];

/// C12.2.5 (ADR-073): only ever credited, and in part.
pub(super) const TOKENS_ATTRIBUTED: Rule = Rule {
    rule_id: "probe.ai-token-use-attributed",
    requirement_ids: &["C12.2.5"],
    cwe: &["CWE-778"],
    impact: "Token use nobody can tie to a user or a session cannot show who ran up a bill, or which \
             account is being used to drain the app's credit.",
    fix: "Write the user, the session, and the feature beside the token counts in the record of \
          every model call, and add them up per user and per feature where the bill is watched.",
};

/// C12.2.5: whether the line recording the model call, which carries its token counts, also says
/// whose call it was: the signed-in test user named, or a user field (per user), or a session field
/// (per session). Credited in part, since per feature endpoint and per team are not seen; never a
/// finding, since the counts may be attributed elsewhere. `line` is `None` when no line carried the
/// counts.
fn token_attribution(markers: &LogMarkers, line: Option<&str>, out: &mut Outcome) {
    let say = |why: &str, out: &mut Outcome| {
        out.not_assessed.push((
            "C12.2.5".to_owned(),
            format!("Whether token use is tracked per user: {why}"),
        ));
    };
    let Some(line) = line else {
        say(
            "no line of the app's output carried the token counts of its model call, so how they \
             are attributed cannot be seen here.",
            out,
        );
        return;
    };
    let Some(who) = &markers.who else {
        say(
            "the AI feature was asked without signing in, so there was no user to look for beside \
             the token counts.",
            out,
        );
        return;
    };
    let lower = line.to_lowercase();
    let local = who.split('@').next().unwrap_or(who).to_lowercase();
    let named = lower.contains(&who.to_lowercase()) || (local.len() >= 6 && lower.contains(&local));
    let has = |fields: &[&str]| {
        fields
            .iter()
            .any(|f| lower.contains(&format!("\"{f}\"")) || lower.contains(&format!("{f}=")))
    };
    let mut per = Vec::new();
    if named || has(&["user", "user_id", "userid"]) {
        per.push("per user");
    }
    if has(&[
        "session",
        "session_id",
        "sessionid",
        "conversation_id",
        "conversationid",
    ]) {
        per.push("per session");
    }
    if per.is_empty() {
        say(
            "the line carrying the token counts of the model call a signed-in test user's message \
             made names no user and no session. That is not a finding: they may be attributed in \
             another record.",
            out,
        );
    } else {
        out.verified.push(
            crate::Verified::new(
                TOKENS_ATTRIBUTED.rule_id,
                TOKENS_ATTRIBUTED.requirement_ids,
                format!(
                    "the line recording one model call carries its token counts and ties them {}; \
                     per feature endpoint and per team or workspace were not seen",
                    per.join(" and ")
                ),
            )
            .in_part(),
        );
    }
    crate::verified::unless_credited(TOKENS_ATTRIBUTED.rule_id, &out.verified);
}

/// C12.1.1: whether the line recording the model call also says whose session it was in. Credited
/// only when the AI feature was asked signed in and the line names that user, or carries a field
/// for a user or session.
fn session_context(markers: &LogMarkers, line: &str, out: &mut Outcome) {
    let Some(who) = &markers.who else {
        out.not_assessed.push((
            "C12.1.1".to_owned(),
            "Whether model calls are logged with their session: the AI feature was asked without \
             signing in, so there was no user or session to look for in the record."
                .to_owned(),
        ));
        return;
    };
    let lower = line.to_lowercase();
    let local = who.split('@').next().unwrap_or(who).to_lowercase();
    let named = lower.contains(&who.to_lowercase()) || (local.len() >= 6 && lower.contains(&local));
    let field = SESSION_FIELDS
        .iter()
        .find(|f| lower.contains(&format!("\"{f}\"")) || lower.contains(&format!("{f}=")));
    if named || field.is_some() {
        out.verified.push(crate::Verified::new(
            SESSION_LOG.rule_id,
            SESSION_LOG.requirement_ids,
            format!(
                "the line recording the model call a signed-in test user's message made also {}",
                if named {
                    "names that user".to_owned()
                } else {
                    format!("carries a `{}` field", field.copied().unwrap_or_default())
                }
            ),
        )
        // One log line, one model call's (ADR-053, Later).
        .in_part());
    } else {
        out.not_assessed.push((
            "C12.1.1".to_owned(),
            "The line recording the model call a signed-in test user's message made does not name \
             the user or carry a user or session field. That is not a finding: the session may be \
             recorded on another line."
                .to_owned(),
        ));
    }
    crate::verified::unless_credited(SESSION_LOG.rule_id, &out.verified);
}

/// Services a log line may name as the provider.
pub(super) const PROVIDERS: &[&str] = &[
    "openai",
    "anthropic",
    "azure",
    "bedrock",
    "vertex",
    "gemini",
    "google",
    "mistral",
    "cohere",
    "groq",
    "ollama",
    "together",
    "fireworks",
    "deepseek",
    "openrouter",
    "xai",
];

/// Words a log line may use for the kind of call.
const OPERATIONS: &[&str] = &[
    "chat",
    "completion",
    "messages",
    "responses",
    "generate",
    "embedding",
    "operation",
];

/// Words that say a message was caught: on a line carrying the injection's own tag.
pub(super) const CAUGHT: &[&str] = &[
    "blocked",
    "flagged",
    "refused",
    "rejected",
    "suspicious",
    "detected",
    "denied",
    "malicious",
    "violation",
];

/// Words that name the attack itself, which nothing else in a run could have made the app write.
pub(super) const NAMED: &[&str] = &["injection", "jailbreak", "prompt attack", "prompt-attack"];

/// Whether `word` is on `line`, in any case, with no letter or digit either side of it. Matched by
/// hand rather than with a pattern, which would be compiled once per line and word: this runs over
/// every line of the app's output.
pub(super) fn has_word(line: &str, word: &str) -> bool {
    let joined = |c: Option<char>| c.is_some_and(|c| c.is_ascii_alphanumeric());
    !word.is_empty()
        && line.char_indices().any(|(i, _)| {
            if joined(line[..i].chars().next_back()) {
                return false;
            }
            let mut rest = line[i..].chars();
            word.chars().all(|w| {
                rest.next()
                    .is_some_and(|c| c.to_lowercase().eq(w.to_lowercase()))
            }) && !joined(rest.next())
        })
}

/// Reads the app's output for the AI feature's own records, adding to what `run` found.
pub fn logged(markers: &LogMarkers, log: &str, out: &mut Outcome) {
    let say = |ids: &str, why: String, out: &mut Outcome| {
        out.not_assessed.push((ids.to_owned(), why));
    };
    failure_logged(markers, log, out);
    if !log.trim().is_empty() {
        tool_action_logged(markers, log, out);
    }
    if markers.call.is_none() && markers.injection.is_none() {
        return;
    }
    if log.trim().is_empty() {
        say(
            "C12.1.3, C12.2.1",
            "The app wrote nothing to its output during the run, so whether it records its model \
             calls and the injection it was sent cannot be seen here. That is not a finding: an \
             app that logs to a file or a service writes nothing to its output."
                .to_owned(),
            out,
        );
        if markers.call.is_some() {
            token_attribution(markers, None, out);
        }
        if markers.injection.is_some() {
            safety_detail(markers, None, out);
            flag_gating(markers, false, out);
        }
        tool_action_logged(markers, log, out);
        return;
    }

    // C12.1.3: the line carrying both token counts is the record of that call.
    if let Some(call) = &markers.call {
        let (input, output) = (
            call.input_tokens.to_string(),
            call.output_tokens.to_string(),
        );
        match log
            .lines()
            .find(|line| has_word(line, &input) && has_word(line, &output))
        {
            None => {
                out.steps.push(
                    "no line of the app's output carried the token counts of its model call"
                        .to_owned(),
                );
                say(
                    "C12.1.3",
                    format!(
                        "No line of the app's output carried the token counts the test model \
                         reported for its call ({input} in, {output} out), so whether it records \
                         its model calls cannot be seen here. That is not a finding: they may be \
                         recorded somewhere else."
                    ),
                    out,
                );
                token_attribution(markers, None, out);
            }
            Some(line) => {
                let lower = line.to_lowercase();
                let format =
                    crate::logs::common_format(line).filter(|f| *f != "the common log format");
                let mut missing = Vec::new();
                if call.model.is_empty() || !lower.contains(&call.model.to_lowercase()) {
                    missing.push("the model");
                }
                if !PROVIDERS.iter().any(|p| has_word(&lower, p)) {
                    missing.push("the service it went to");
                }
                if !OPERATIONS.iter().any(|o| lower.contains(o)) {
                    missing.push("the kind of call");
                }
                session_context(markers, line, out);
                token_attribution(markers, Some(line), out);
                out.steps.push(format!(
                    "found the line recording the model call ({}){}",
                    format.unwrap_or("not structured"),
                    if missing.is_empty() {
                        String::new()
                    } else {
                        format!(", without {}", missing.join(", "))
                    }
                ));
                if missing.is_empty() && format.is_some() {
                    out.verified.push(crate::Verified::new(
                        CALL_LOG.rule_id,
                        CALL_LOG.requirement_ids,
                        format!(
                            "the line recording the model call this run made, written as {}, with \
                             the model, both token counts, the service, and the kind of call",
                            format.unwrap_or_default()
                        ),
                    )
                    // One log line, one model call's, not each entry (ADR-053, Later).
                    .in_part());
                } else {
                    let mut short = missing
                        .iter()
                        .map(|m| format!("does not name {m}"))
                        .collect::<Vec<_>>();
                    if format.is_none() {
                        short.push("is not written as JSON or logfmt".to_owned());
                    }
                    out.findings.push(on(vec![question_id("PLAIN", 1)], finding(
                        &CALL_LOG,
                        "The app's record of a model call leaves things out",
                        Severity::Low,
                        format!(
                            "The line in the app's output recording the model call this run made \
                             ({input} tokens in, {output} out) {}.",
                            short.join(", and ")
                        ),
                    )));
                }
            }
        }
    }

    // C12.2.1: a line that says the injection was caught — one naming the attack, or one carrying
    // the injection's own tag with a word for stopping it. An app that writes every message down
    // as it came has noticed nothing, and the probe's tag (`INJECT-`) is not one of those words.
    if let Some(tag) = &markers.injection {
        let caught_line = log.lines().find(|line| {
            NAMED.iter().any(|w| has_word(line, w))
                || (line.contains(tag.as_str()) && CAUGHT.iter().any(|w| has_word(line, w)))
        });
        let caught = caught_line.is_some();
        safety_detail(markers, caught_line, out);
        flag_gating(markers, caught, out);
        out.steps.push(format!(
            "the app's output {} the prompt injection as caught",
            if caught { "recorded" } else { "did not record" }
        ));
        if caught {
            out.verified.push(crate::Verified::new(
                INJECTION_LOGGED,
                &["C12.2.1"],
                "a line in the app's output recording the textbook prompt injection this run sent \
                 as one; whether anybody is alerted beyond the log was not seen"
                    .to_owned(),
            )
            // One log line for one injection (ADR-053, Later).
            .in_part());
        } else {
            say(
                "C12.2.1",
                "No line of the app's output recorded the textbook prompt injection this run sent \
                 as caught. That is not a finding: it may be recorded or alerted on somewhere else."
                    .to_owned(),
                out,
            );
        }
        crate::verified::unless_credited(INJECTION_LOGGED, &out.verified);
    }
}
