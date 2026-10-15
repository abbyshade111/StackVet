//! Semgrep's Python rule for credentials written to a log, read against the call it matched (backlog 0583; ADR-111,
//! Later).
//!
//! The rule fires on any `logger.info("... %s ...", ...)` whose message text says password, token, secret,
//! credential, or API key, whatever the call hands it to log. In the Opus run of 15 October 2026 it fired four times
//! on lines that log a user id or a game id. When every value the call logs is an id, and none is named for a
//! secret, nothing secret is written to the log by that line, and the finding is listed apart with what was logged:
//! shown in full, deciding nothing. Anything `sv` cannot read for certain (a message built with `%`, `+`, `.format`,
//! or an f-string, a value that is not a plain name, an `extra=` mapping, a call it cannot find) leaves the finding
//! as it was.

use crate::finding::{Finding, Outranked};
use std::path::Path;

/// The rule, as `sv` names its findings.
pub const RULE: &str = "semgrep.python.lang.security.audit.logging.logger-credential-leak.python-logger-credential-disclosure";

/// The most lines a logger call is followed across.
const MOST_LINES: usize = 20;

/// Words that, in any part of a logged name, mean it may hold a secret: such a value is never read as an id.
const SECRET_WORDS: &[&str] = &[
    "password",
    "passwd",
    "pwd",
    "secret",
    "token",
    "credential",
    "key",
    "cookie",
    "session",
];

/// Lists apart each finding of `RULE` whose logger call logs only ids, with the names it logs. One another tool also
/// reported, or sharing its line with a problem of another kind, is left as it was.
pub fn set_apart_logged_ids(app_dir: &Path, findings: &mut [Finding]) {
    for f in findings.iter_mut() {
        if f.rule_id != RULE
            || f.outranked.is_some()
            || !f.also_reported_by.is_empty()
            || f.also_on_this_line.iter().any(|o| o.rule_id != RULE)
        {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(app_dir.join(&f.location.file)) else {
            continue;
        };
        if let Some(names) = logged_ids(&text, f.location.line) {
            f.outranked = Some(Outranked::LogsOnlyIds { names });
        }
    }
}

/// The names the logger call starting on `line` (1-indexed) of `text` logs, when every one is an id and its message
/// is a plain string; `None` otherwise.
pub fn logged_ids(text: &str, line: usize) -> Option<Vec<String>> {
    let lines: Vec<&str> = text.lines().collect();
    let start = lines.get(line.checked_sub(1)?)?;
    let open = call_start(start)?;
    let mut call = start[open + 1..].to_owned();
    for more in lines.iter().skip(line).take(MOST_LINES) {
        if closing(&call).is_some() {
            break;
        }
        call.push('\n');
        call.push_str(more);
    }
    let inside = &call[..closing(&call)?];
    let args = split_arguments(inside)?;
    let (message, values) = args.split_first()?;
    if !is_plain_string(message) {
        return None;
    }
    let mut names = Vec::new();
    for value in values {
        if let Some((name, _)) = value.split_once('=').filter(|(n, _)| is_name(n.trim())) {
            // A keyword that says how to log, not what: anything else (`extra=`) may carry a value.
            if ["exc_info", "stack_info", "stacklevel"].contains(&name.trim()) {
                continue;
            }
            return None;
        }
        if !is_id(value) {
            return None;
        }
        names.push(value.to_owned());
    }
    (!names.is_empty()).then_some(names)
}

/// Where the logger call's `(` is on the line: after a call such as `logger.info`, `log.warning`, or `self.logger.error`.
fn call_start(line: &str) -> Option<usize> {
    let re = regex::Regex::new(
        r"(?i)\b[\w.]*(?:_logger|logger|log)\.(?:debug|info|warn|warning|error|exception|critical)\s*\(",
    )
    .ok()?;
    Some(re.find(line)?.end() - 1)
}

/// The place of the `)` that closes the call, reading past strings and nested brackets.
fn closing(text: &str) -> Option<usize> {
    let mut depth = 0usize;
    let mut quote: Option<char> = None;
    let mut escaped = false;
    for (i, c) in text.char_indices() {
        if let Some(q) = quote {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == q {
                quote = None;
            }
            continue;
        }
        match c {
            '"' | '\'' => quote = Some(c),
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' if depth == 0 => return (c == ')').then_some(i),
            ')' | ']' | '}' => depth -= 1,
            '#' => return None,
            _ => {}
        }
    }
    None
}

/// The call's arguments, split at the commas between them, each trimmed; `None` when one is empty but the last.
fn split_arguments(inside: &str) -> Option<Vec<String>> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut depth = 0usize;
    let mut quote: Option<char> = None;
    let mut escaped = false;
    for c in inside.chars() {
        if let Some(q) = quote {
            current.push(c);
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == q {
                quote = None;
            }
            continue;
        }
        match c {
            '"' | '\'' => quote = Some(c),
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth = depth.checked_sub(1)?,
            ',' if depth == 0 => {
                args.push(current.trim().to_owned());
                current.clear();
                continue;
            }
            _ => {}
        }
        current.push(c);
    }
    let last = current.trim();
    if !last.is_empty() {
        args.push(last.to_owned());
    }
    args.iter().all(|a| !a.is_empty()).then_some(args)
}

/// Whether the message is string literals alone, side by side (Python joins them): no `%`, `+`, `.format`, or
/// f-string that could put a value into it.
fn is_plain_string(message: &str) -> bool {
    let mut rest = message.trim();
    if rest.is_empty() {
        return false;
    }
    while !rest.is_empty() {
        let prefix = rest.find(['"', '\'']).map_or(rest, |q| &rest[..q]);
        if !prefix
            .chars()
            .all(|c| matches!(c.to_ascii_lowercase(), 'r' | 'b' | 'u'))
            || prefix.len() > 2
        {
            return false;
        }
        rest = &rest[prefix.len()..];
        let Some(q) = rest.chars().next() else {
            return false;
        };
        let mut end = None;
        let mut escaped = false;
        for (i, c) in rest.char_indices().skip(1) {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == q {
                end = Some(i);
                break;
            }
        }
        let Some(end) = end else {
            return false;
        };
        rest = rest[end + 1..].trim_start();
    }
    true
}

fn is_name(text: &str) -> bool {
    let mut chars = text.chars();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Whether a logged value is a plain name, or names joined by dots, whose last part is an id (`id`, `user_id`,
/// `gameId`, `uuid`) and no part of which is named for a secret.
fn is_id(value: &str) -> bool {
    let parts: Vec<&str> = value.split('.').collect();
    if !parts.iter().all(|p| is_name(p)) {
        return false;
    }
    if parts.iter().any(|p| {
        let p = p.to_lowercase();
        SECRET_WORDS.iter().any(|w| p.contains(w))
    }) {
        return false;
    }
    let last = parts[parts.len() - 1];
    let lower = last.to_lowercase();
    lower == "id"
        || lower == "uid"
        || lower == "uuid"
        || lower.ends_with("_id")
        || lower.ends_with("_uuid")
        || (last.ends_with("Id") || last.ends_with("ID")) && last.len() > 2
}

#[cfg(test)]
mod tests;
