//! The session lifetime the code sets, read beside the one stackvet.toml's policy states (backlog 0612; ADR-123,
//! Later).
//!
//! V7.1.1 asks that the session's inactivity timeout and absolute lifetime be documented and appropriate.
//! `[policy] session-lifetime-minutes` is the owner's documented lifetime, and `sv run --slow` holds the running app to
//! it (V7.3.2). The code often says its own: Django's `SESSION_COOKIE_AGE`, Flask's `PERMANENT_SESSION_LIFETIME`, and
//! the `maxAge` among an Express session's options. When one of those is longer than the policy allows, the
//! documentation is not what the app does, and that is a finding. Agreement credits nothing: a lifetime set elsewhere
//! (a hosting console, a library default) is not read, and the documentation half is the owner's.
//!
//! Only a number the code writes out is read: an integer, a product or sum of integers, `timedelta(...)` with
//! integer arguments, or a duration string such as `"7d"`. A value read from a setting is not, nor a cache's `maxAge`,
//! nor a token's `expiresIn`, which may be a long-lived refresh token's rather than the session's.

use crate::finding::{Confidence, Finding, Location, Severity};
use tree_sitter::{Node, Parser};

/// The finding's rule.
pub const DIFFERS: &str = "config.session-timeout-differs-from-policy";

/// One session lifetime the code sets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Setting {
    pub file: String,
    pub line: usize,
    /// The name as the code writes it.
    pub name: String,
    pub seconds: u64,
}

/// The names read, each with how many seconds one of its units is: seconds for Django's and Flask's settings, and
/// milliseconds for a session's `maxAge` in JavaScript. A token's `expiresIn` is not read: a long-lived refresh token is
/// not plainly the session.
fn unit_of(name: &str, javascript: bool) -> Option<f64> {
    let plain: String = name
        .chars()
        .filter(|c| *c != '_')
        .flat_map(char::to_lowercase)
        .collect();
    match plain.trim_matches(|c| c == '\'' || c == '"') {
        "sessioncookieage" | "permanentsessionlifetime" if !javascript => Some(1.0),
        "maxage" if javascript => Some(0.001),
        _ => None,
    }
}

/// Whether a JavaScript `pair` is among a session's options: in the object given to `session(...)` or
/// `cookieSession(...)`, or in its `cookie: {...}`. `express.static`'s `maxAge` is a cache's, not a session's.
fn in_session_options(pair: Node, source: &str) -> bool {
    let Some(object) = pair.parent() else {
        return false;
    };
    let Some(holder) = object.parent() else {
        return false;
    };
    match holder.kind() {
        "pair" => {
            holder
                .child_by_field_name("key")
                .is_some_and(|k| text(k, source) == "cookie")
                && holder
                    .parent()
                    .and_then(|o| o.parent())
                    .is_some_and(|h| session_call(h, source))
        }
        "arguments" => holder.parent().is_some_and(|c| session_call(c, source)),
        _ => false,
    }
}

/// Whether a node is a call to a session middleware: `session(...)`, `cookieSession(...)`, `expressSession(...)`.
fn session_call(node: Node, source: &str) -> bool {
    if node.kind() == "arguments" {
        return node.parent().is_some_and(|c| session_call(c, source));
    }
    node.kind() == "call_expression"
        && node.child_by_field_name("function").is_some_and(|f| {
            let name = text(f, source)
                .rsplit('.')
                .next()
                .unwrap_or("")
                .to_lowercase();
            matches!(
                name.as_str(),
                "session" | "cookiesession" | "expresssession"
            )
        })
}

/// Every session lifetime the app's own files set, not counting test code.
pub fn read_all(listing: &sv_scan::files::Listing) -> Vec<Setting> {
    let mut out = Vec::new();
    for entry in listing.app_files() {
        let Some(language) = entry.language else {
            continue;
        };
        if sv_scan::files::is_test_path(&entry.relative) {
            continue;
        }
        let key = match language {
            "python" => "python",
            "javascript" => "javascript",
            "typescript" if entry.relative.to_lowercase().ends_with(".tsx") => "tsx",
            "typescript" => "typescript",
            _ => continue,
        };
        let Ok(source) = entry.read_text() else {
            continue;
        };
        let lower = source.to_lowercase();
        if !["session_cookie_age", "permanent_session_lifetime", "maxage"]
            .iter()
            .any(|w| lower.contains(w))
        {
            continue;
        }
        out.extend(in_source(key, &entry.relative, &source));
    }
    out
}

/// The session lifetimes one file sets.
pub fn in_source(key: &str, file: &str, source: &str) -> Vec<Setting> {
    let Some(grammar) = crate::ast::grammar(key) else {
        return Vec::new();
    };
    let mut parser = Parser::new();
    if parser.set_language(&grammar).is_err() {
        return Vec::new();
    }
    let Some(tree) = parser.parse(source, None) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    walk(tree.root_node(), &mut |node| {
        let (name, value) = match node.kind() {
            // `SESSION_COOKIE_AGE = ...`, `app.permanent_session_lifetime = ...`,
            // `app.config['PERMANENT_SESSION_LIFETIME'] = ...`
            "assignment" => {
                let (Some(left), Some(right)) = (
                    node.child_by_field_name("left"),
                    node.child_by_field_name("right"),
                ) else {
                    return;
                };
                let name = match left.kind() {
                    "identifier" => text(left, source),
                    "attribute" => left
                        .child_by_field_name("attribute")
                        .map_or("", |a| text(a, source)),
                    "subscript" => left
                        .child_by_field_name("subscript")
                        .map_or("", |s| text(s, source)),
                    _ => return,
                };
                (name, right)
            }
            // `maxAge: ...` among a session's options.
            "pair" if in_session_options(node, source) => {
                let (Some(k), Some(v)) = (
                    node.child_by_field_name("key"),
                    node.child_by_field_name("value"),
                ) else {
                    return;
                };
                (text(k, source), v)
            }
            // `app.config.update(PERMANENT_SESSION_LIFETIME=...)`
            "keyword_argument" => {
                let (Some(k), Some(v)) = (
                    node.child_by_field_name("name"),
                    node.child_by_field_name("value"),
                ) else {
                    return;
                };
                (text(k, source), v)
            }
            _ => return,
        };
        let Some(unit) = unit_of(name, key != "python") else {
            return;
        };
        let Some(seconds) = seconds_of(value, source, unit) else {
            return;
        };
        out.push(Setting {
            file: file.to_owned(),
            line: node.start_position().row + 1,
            name: name.trim_matches(|c| c == '\'' || c == '"').to_owned(),
            seconds,
        });
    });
    out
}

/// The seconds a value written out in the code stands for, `unit` seconds to one of its units, or `None` when it is
/// not written out.
fn seconds_of(node: Node, source: &str, unit: f64) -> Option<u64> {
    if let Some(n) = number(node, source) {
        return Some((n * unit).round() as u64);
    }
    match node.kind() {
        "string" => {
            duration(text(node, source).trim_matches(|c| c == '\'' || c == '"' || c == '`'))
        }
        "call" => {
            // `timedelta(days=7)`, `datetime.timedelta(hours=8)`
            let function = text(node.child_by_field_name("function")?, source);
            if !function.ends_with("timedelta") {
                return None;
            }
            let arguments = node.child_by_field_name("arguments")?;
            let mut cursor = arguments.walk();
            let mut total = 0.0;
            for argument in arguments.named_children(&mut cursor) {
                let (name, value) = if argument.kind() == "keyword_argument" {
                    (
                        text(argument.child_by_field_name("name")?, source),
                        argument.child_by_field_name("value")?,
                    )
                } else {
                    ("days", argument)
                };
                let per = match name {
                    "weeks" => 604_800.0,
                    "days" => 86_400.0,
                    "hours" => 3_600.0,
                    "minutes" => 60.0,
                    "seconds" => 1.0,
                    _ => return None,
                };
                total += number(value, source)? * per;
            }
            Some(total.round() as u64)
        }
        _ => None,
    }
}

/// An integer, or a product or sum of integers, as written.
fn number(node: Node, source: &str) -> Option<f64> {
    match node.kind() {
        "integer" | "number" => text(node, source).replace('_', "").parse::<f64>().ok(),
        "parenthesized_expression" => number(node.named_child(0)?, source),
        "binary_operator" | "binary_expression" => {
            let left = number(node.child_by_field_name("left")?, source)?;
            let right = number(node.child_by_field_name("right")?, source)?;
            match text(node.child_by_field_name("operator")?, source) {
                "*" => Some(left * right),
                "+" => Some(left + right),
                _ => None,
            }
        }
        _ => None,
    }
}

/// A duration string as the `ms` and `jsonwebtoken` libraries read one: `"7d"`, `"12h"`, `"30m"`, `"60s"`, `"2 days"`.
fn duration(text: &str) -> Option<u64> {
    let text = text.trim().to_lowercase();
    let split = text.find(|c: char| !c.is_ascii_digit())?;
    let (n, unit) = text.split_at(split);
    let n: u64 = n.parse().ok()?;
    let per = match unit.trim() {
        "s" | "sec" | "secs" | "second" | "seconds" => 1,
        "m" | "min" | "mins" | "minute" | "minutes" => 60,
        "h" | "hr" | "hrs" | "hour" | "hours" => 3_600,
        "d" | "day" | "days" => 86_400,
        "w" | "week" | "weeks" => 604_800,
        _ => return None,
    };
    Some(n * per)
}

/// A number of seconds in words: "7 days", "8 hours", "45 minutes".
pub fn in_words(seconds: u64) -> String {
    let plural = |n: u64, one: &str| {
        if n == 1 {
            format!("1 {one}")
        } else {
            format!("{n} {one}s")
        }
    };
    if seconds.is_multiple_of(86_400) {
        plural(seconds / 86_400, "day")
    } else if seconds.is_multiple_of(3_600) {
        plural(seconds / 3_600, "hour")
    } else if seconds.is_multiple_of(60) {
        plural(seconds / 60, "minute")
    } else {
        plural(seconds, "second")
    }
}

/// A finding for each lifetime the code sets that is longer than `lifetime_minutes`, the policy's.
pub fn longer_than_policy(settings: &[Setting], lifetime_minutes: u32) -> Vec<Finding> {
    let allowed = u64::from(lifetime_minutes) * 60;
    settings
        .iter()
        .filter(|s| s.seconds > allowed)
        .map(|s| {
            crate::finding::found(Finding {
                evidence: Vec::new(),
                also_reported_by: Vec::new(),
                fingerprint: String::new(),
                earlier_fingerprints: Vec::new(),
                marked_test_code: false,
                bundled_library: None,
                outranked: None,
                also_on_this_line: Vec::new(),
                help_uri: None,
                rule_id: DIFFERS.into(),
                title: "The code keeps a session longer than the documented lifetime".into(),
                severity: Severity::Medium,
                confidence: Confidence::Medium,
                location: Location {
                    file: s.file.clone(),
                    line: s.line,
                },
                secret: None,
                requirement_ids: vec!["V7.1.1".into()],
                cwe: vec!["CWE-613".into()],
                description: format!(
                    "`{}` here sets {}, and `[policy] session-lifetime-minutes` in stackvet.toml says a session lasts \
                     {} at most. The documented lifetime is not the one the code sets.",
                    s.name,
                    in_words(s.seconds),
                    in_words(allowed)
                ),
                impact: "A session that lasts longer than the policy says stays usable on a shared or stolen device \
                         well after the person thinks it has ended, and the documentation anyone relies on is wrong \
                         about it."
                    .into(),
                fix: format!(
                    "Set `{}` to {} or less, or, if the longer lifetime is what you want, change \
                     `session-lifetime-minutes` and say in your security notes why it is right for this app.",
                    s.name,
                    in_words(allowed)
                ),
            })
        })
        .collect()
}

fn walk<'a>(node: Node<'a>, visit: &mut impl FnMut(Node<'a>)) {
    visit(node);
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        walk(child, visit);
    }
}

fn text<'a>(node: Node, source: &'a str) -> &'a str {
    node.utf8_text(source.as_bytes()).unwrap_or("")
}

#[cfg(test)]
#[path = "session_settings_tests.rs"]
mod tests;
