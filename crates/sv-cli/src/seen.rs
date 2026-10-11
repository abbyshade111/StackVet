//! What `sv` saw of the running app, made ready to keep beside the report (ADR-082): each question
//! asked as somebody not signed in, and what came back, with every credential cut down first.
//!
//! Two passes, and a header pass before them. A header that carries a session or a sign-in
//! (`set-cookie`, `authorization`, and the others in `SESSION_HEADERS`) keeps its name and, for a
//! cookie, the cookie's name, and loses its value whatever it looks like: a session id is random
//! letters and digits, and no rule for keys knows it from any other. Then every header value and
//! every body goes through `secrets::redact_text`, which cuts what the secrets scan would find and
//! any value a name says is a credential. The test that plants a key built from pieces in each
//! place (`seen_tests`) fails if any of it reaches the record.

use serde_json::Value;
use sv_check::probes::{ProbeRequest, ProbeResponse};
use sv_check::secrets::{SecretRules, redact_text};
use sv_report::seen::{Exchange, KEPT_CHARS, MOST_EXCHANGES, MOST_TOOL_CHARS, Seen};
use sv_run::stand_ins::TEST_SECRET;

/// Headers whose value is a session or a sign-in, so it is cut whatever it looks like.
const SESSION_HEADERS: &[&str] = &[
    "set-cookie",
    "cookie",
    "authorization",
    "proxy-authorization",
    "www-authenticate",
    "x-api-key",
    "x-auth-token",
    "x-csrf-token",
    "x-xsrf-token",
];

/// The record of `asked` and the `answered` that came back. `rate_limited` are the ids the app's
/// rate limiter answered in its place, which the run leaves out of `answered`.
pub fn record(
    rules: &SecretRules,
    asked: &[ProbeRequest],
    answered: &[ProbeResponse],
    rate_limited: &[String],
) -> Seen {
    let mut seen = Seen::default();
    for request in asked {
        let Some(response) = answered.iter().find(|r| r.id == request.id) else {
            let why = if rate_limited
                .iter()
                .any(|l| l.starts_with(&format!("{} (", request.id)))
            {
                "answered by the app's rate limiter in its place"
            } else {
                "no answer"
            };
            seen.not_answered.push(format!("{} ({why})", request.id));
            continue;
        };
        if seen.exchanges.len() == MOST_EXCHANGES {
            seen.left_out += 1;
            continue;
        }
        let kept = kept_exchange(
            rules,
            &mut seen.credentials_removed,
            &request.id,
            &request.method,
            &request.path,
            response.status,
            &response.headers,
            &response.body,
        );
        seen.exchanges.push(kept);
    }
    seen
}

/// The signed-in suite's questions and their answers, made ready to keep (backlog 0229, part 1):
/// each through the same header and body rules as the anonymous answers, numbered `signed-in-N` in
/// the order asked. A question with no answer is counted, not kept.
pub fn signed_in(
    rules: &SecretRules,
    asked: Option<&sv_check::signed_in::Outcome>,
    seen: &mut Seen,
) {
    let Some(asked) = asked else {
        return;
    };
    for exchange in &asked.exchanges {
        let Some(status) = exchange.status else {
            seen.signed_in_unanswered += 1;
            continue;
        };
        if seen.signed_in.len() == MOST_EXCHANGES {
            seen.left_out += 1;
            continue;
        }
        let kept = kept_exchange(
            rules,
            &mut seen.credentials_removed,
            &exchange.id,
            &exchange.method,
            &exchange.path,
            status,
            &exchange.headers,
            &exchange.body,
        );
        seen.signed_in.push(kept);
    }
}

/// The report's sentence on the names the app looked up on its own network (ADR-085, backlog 0240): how many distinct
/// names, and where the list is kept. The names themselves are not printed here: they are the app's text, and a
/// name can carry a secret in a subdomain, so the record, where they are redacted, is the place to read them. `None`
/// when the name server did not run.
pub fn names_sentence(names: Option<&[sv_run::name_server::Lookup]>) -> Option<String> {
    let distinct: std::collections::BTreeSet<&str> =
        names?.iter().map(|l| l.name.as_str()).collect();
    Some(match distinct.len() {
        0 => " The app looked up no name on its own network.".to_owned(),
        n => format!(
            " The app looked up {n} distinct name{} on its own network; {} keeps them under \
             stand_ins.names, with the credentials cut out.",
            if n == 1 { "" } else { "s" },
            sv_report::seen::FILE
        ),
    })
}

/// The app's container as it was read between the stages of the questions (backlog 229 part 1),
/// numbered as the credits and findings name them. Its text goes through `redact_text` like the
/// rest, and the credentials cut from it are counted.
pub fn liveness(rules: &SecretRules, readings: &[sv_check::running::Liveness], seen: &mut Seen) {
    for (index, reading) in readings.iter().enumerate() {
        let (after, cut_after) = redact_text(rules, &reading.after);
        let (status, cut_status) = redact_text(rules, &reading.status);
        seen.credentials_removed += cut_after + cut_status;
        seen.liveness.push(sv_report::seen::Reading {
            id: sv_check::running::liveness_id(index),
            after,
            status,
            restarts: reading.restarts,
            exit_code: reading.exit_code,
            out_of_memory: reading.out_of_memory,
            answered: reading.answered,
        });
    }
}

/// One answer, kept: its session and sign-in headers' values taken out, and every string in it
/// through `redact_text`, counting the credentials cut into `removed`.
#[allow(clippy::too_many_arguments)]
fn kept_exchange(
    rules: &SecretRules,
    removed: &mut usize,
    id: &str,
    method: &str,
    path: &str,
    status: u16,
    headers: &[(String, String)],
    body: &str,
) -> Exchange {
    let mut cut = |text: &str| {
        let (text, n) = redact_around_markers(rules, text);
        *removed += n;
        text
    };
    let headers = headers
        .iter()
        .map(|(name, value)| {
            let value = if SESSION_HEADERS.contains(&name.as_str()) {
                session_value(name, value)
            } else {
                value.clone()
            };
            (name.clone(), cut(&value))
        })
        .collect();
    let body = cut(body);
    let path = cut(path);
    Exchange {
        id: id.to_owned(),
        method: method.to_owned(),
        path,
        status,
        headers,
        body,
    }
}

/// A session header's value with the value itself taken out: a cookie keeps its name and its
/// attributes (`Path`, `HttpOnly`, `Secure`, `SameSite`), which the checks read and a person
/// following a finding needs, and anything else keeps its scheme word (`Bearer`, `Basic`).
fn session_value(name: &str, value: &str) -> String {
    if name == "set-cookie" || name == "cookie" {
        return value
            .split(';')
            .map(str::trim)
            .enumerate()
            .map(|(i, part)| match part.split_once('=') {
                // The cookie itself, in `set-cookie` the first part and in `cookie` every one.
                Some((cookie, v)) if i == 0 || name == "cookie" => {
                    format!("{cookie}=[removed, {} characters]", v.chars().count())
                }
                _ => part.to_owned(),
            })
            .collect::<Vec<_>>()
            .join("; ");
    }
    match value.split_once(' ') {
        Some((scheme, rest))
            if !scheme.is_empty() && scheme.chars().all(|c| c.is_ascii_alphabetic()) =>
        {
            format!("{scheme} [removed, {} characters]", rest.chars().count())
        }
        _ => format!("[removed, {} characters]", value.chars().count()),
    }
}

/// What the stand-ins received, made ready to keep (backlog 0229, part 2): every piece of text in
/// it passed through `redact_text`, cut at `KEPT_CHARS` characters, and every list at
/// `MOST_EXCHANGES` entries. `sv`'s own test secrets were blanked by value in `sv-run`, which alone
/// knows them. Adds the credentials cut to `seen.credentials_removed`.
pub fn stand_ins(rules: &SecretRules, received: &sv_run::stand_ins::StandIns, seen: &mut Seen) {
    use std::cell::Cell;
    let (removed, cuts) = (Cell::new(0), Cell::new(0));
    let cut = |text: &str| {
        let (text, n) = redact_around_markers(rules, text);
        removed.set(removed.get() + n);
        bounded(text, &cuts)
    };
    let mail = received.mail.as_ref().map(|mail| {
        cuts.set(cuts.get() + mail.len().saturating_sub(MOST_EXCHANGES));
        mail.iter()
            .take(MOST_EXCHANGES)
            .map(|m| sv_report::seen::Mail {
                to: m.to.iter().map(|t| cut(t)).collect(),
                subject: cut(&m.subject),
                at: m.at.clone(),
            })
            .collect()
    });
    // Each name is redacted like every other string here, then counted: the same name asked for
    // twice (once as an IPv4 address, once as an IPv6 one, or on a retry) is one entry, asked twice
    // (ADR-085).
    let names = received.names.as_ref().map(|lookups| {
        let mut kept: Vec<sv_report::seen::NameAsked> = Vec::new();
        for lookup in lookups {
            let name = cut(&lookup.name);
            let kind = sv_run::name_server::kind_name(lookup.kind);
            match kept.iter_mut().find(|k| k.name == name && k.kind == kind) {
                Some(entry) => entry.asked += 1,
                None => kept.push(sv_report::seen::NameAsked {
                    name,
                    kind,
                    asked: 1,
                    first: lookup.at.clone(),
                }),
            }
        }
        cuts.set(cuts.get() + kept.len().saturating_sub(MOST_EXCHANGES));
        kept.truncate(MOST_EXCHANGES);
        kept
    });
    seen.stand_ins = sv_report::seen::StandIns {
        model: received.model.clone().map(|v| walk(v, &cut, &cuts)),
        sign_in_provider: received.provider.clone().map(|v| walk(v, &cut, &cuts)),
        mail,
        names,
        not_read: received.unread.iter().map(|s| (*s).to_owned()).collect(),
        cut: cuts.get(),
    };
    seen.credentials_removed += removed.get();
}

/// The lines of the app's own output the log checks read, and its last lines, made ready to keep
/// (backlog 0229, part 3): each through `redact_text` and cut at `KEPT_CHARS` characters. `sv`'s
/// test secrets were blanked in `sv-run`. Adds the credentials cut to `seen.credentials_removed`.
pub fn app_log(
    rules: &SecretRules,
    asked: Option<&sv_check::signed_in::Outcome>,
    ai: Option<&sv_check::signed_in::Outcome>,
    seen: &mut Seen,
) {
    // The lines the AI feature's log checks read are kept beside the signed-in suite's (0229, part 3).
    if asked.is_none() && ai.is_none() {
        return;
    }
    let cuts = std::cell::Cell::new(0);
    let mut removed = 0;
    let mut cut = |text: &str| {
        let (text, n) = redact_around_markers(rules, text);
        removed += n;
        bounded(text, &cuts)
    };
    let lines_read = asked
        .into_iter()
        .chain(ai)
        .flat_map(|o| o.log_lines.iter())
        .map(|k| sv_report::seen::LogLine {
            read_for: k.read_for.clone(),
            line: cut(&k.line),
        })
        .collect();
    let last_lines = asked.map_or_else(Vec::new, |o| o.log_tail.iter().map(|l| cut(l)).collect());
    seen.app_log = sv_report::seen::AppLog {
        lines_read,
        last_lines,
        cut: cuts.get(),
    };
    seen.credentials_removed += removed;
}

/// Each outside tool's own report, from the record of its run in `examined`, made ready to keep
/// (backlog 0229, part 4): through `redact_text`, and cut at `MOST_TOOL_CHARS` characters. Adds the
/// credentials cut to `seen.credentials_removed`.
pub fn tool_output(rules: &SecretRules, examined: &[sv_report::Examined], seen: &mut Seen) {
    for e in examined {
        let Some(tool) = &e.tool else { continue };
        let Some(text) = &tool.output else { continue };
        let (text, n) = redact_text(rules, text);
        seen.credentials_removed += n;
        let total = text.chars().count();
        let cut_chars = total.saturating_sub(MOST_TOOL_CHARS);
        let report = if cut_chars == 0 {
            text
        } else {
            text.chars().take(MOST_TOOL_CHARS).collect()
        };
        seen.tool_output.push(sv_report::seen::ToolOutput {
            program: tool.program.clone(),
            rules: e.rules.clone(),
            report,
            cut_chars,
        });
    }
}

/// `redact_text`, with each `TEST_SECRET` marker left as it is. A marker follows `password=` in an
/// address, and the generic rule would take its first word for the password's value and cut it,
/// leaving a record that reads as if the secret was only partly removed. Each piece between the
/// markers is redacted on its own, so the markers are never seen by it.
pub fn redact_around_markers(rules: &SecretRules, text: &str) -> (String, usize) {
    let mut out = String::with_capacity(text.len());
    let mut removed = 0;
    for (i, piece) in text.split(TEST_SECRET).enumerate() {
        if i > 0 {
            out.push_str(TEST_SECRET);
        }
        let (redacted, n) = redact_text(rules, piece);
        out.push_str(&redacted);
        removed += n;
    }
    (out, removed)
}

/// `text`, cut at `KEPT_CHARS` characters with how many more there were said, counting a cut.
fn bounded(text: String, cuts: &std::cell::Cell<usize>) -> String {
    let n = text.chars().count();
    if n <= KEPT_CHARS {
        return text;
    }
    cuts.set(cuts.get() + 1);
    let head: String = text.chars().take(KEPT_CHARS).collect();
    format!("{head}… ({} more characters)", n - KEPT_CHARS)
}

/// `value` with `each` applied to every string in it, keys included, and every list cut at
/// `MOST_EXCHANGES` entries, counting each entry left out in `cuts`.
fn walk(value: Value, each: &dyn Fn(&str) -> String, cuts: &std::cell::Cell<usize>) -> Value {
    match value {
        Value::String(s) => Value::String(each(&s)),
        Value::Array(items) => {
            cuts.set(cuts.get() + items.len().saturating_sub(MOST_EXCHANGES));
            Value::Array(
                items
                    .into_iter()
                    .take(MOST_EXCHANGES)
                    .map(|v| walk(v, each, cuts))
                    .collect(),
            )
        }
        Value::Object(map) => Value::Object(
            map.into_iter()
                .map(|(k, v)| (each(&k), walk(v, each, cuts)))
                .collect(),
        ),
        other => other,
    }
}

#[cfg(test)]
#[path = "seen_tests.rs"]
mod seen_tests;

#[cfg(test)]
#[path = "seen_names_tests.rs"]
mod seen_names_tests;
