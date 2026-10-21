//! V16.5.4: signs in the app's own code that it defines a last-resort error handler (backlog 0263, ADR-130, Later).
//!
//! # Why this can support and never fault
//!
//! V16.5.4 asks that a last-resort handler is defined *which will catch all* unhandled exceptions, so an error neither
//! loses what the logs need nor takes the app down. Files can show that a handler is defined: `process.on(
//! 'uncaughtException')`, Express's four-argument error middleware, Flask's `errorhandler(Exception)`. They cannot
//! show that it catches every one (a worker thread, a scheduled job, an error raised inside the handler), which is why
//! V16.5.4 is in `manualOnly`. So a sign found is a credit the report shows beside V16.5.4 as supporting evidence and
//! never as *checked*, and no sign found says nothing: a framework may install one of its own by default (Django,
//! Rails, Spring Boot), and the running half, `probe.app-stopped-during-questions`, is what finds an app that falls
//! over.
//!
//! Each sign is read line by line in the language's own files, as `rich_text::where_named` reads a package's name.

use crate::config::ConfigReport;
use crate::verified::Verified;
use regex::Regex;
use std::sync::OnceLock;
use sv_scan::files::Listing;

pub const LAST_RESORT: &str = "config.last-resort-error-handler";

/// The most places one credit names: the rest are counted.
const NAMED: usize = 4;

/// Each sign: the file extensions it is read in, what it is in words, and the pattern a line must match.
const SIGNS: &[(&[&str], &str, &str)] = &[
    (
        &["js", "mjs", "cjs", "ts", "mts", "cts"],
        "a handler for uncaught exceptions or unhandled rejections (`process.on`)",
        r#"process\.on\(\s*['"`](uncaughtException|unhandledRejection)['"`]"#,
    ),
    (
        &["js", "mjs", "cjs", "ts", "mts", "cts"],
        "Express error middleware, the four-argument kind",
        r"\.use\(\s*(?:async\s+)?(?:function\s*\w*\s*)?\(\s*\w+\s*(?::\s*\w+\s*)?,\s*\w+\s*(?::\s*\w+\s*)?,\s*\w+\s*(?::\s*\w+\s*)?,\s*\w+\s*(?::\s*\w+\s*)?\)",
    ),
    (
        &["py"],
        "a handler for every exception (`errorhandler(Exception)` or `exception_handler(Exception)`)",
        r"\b(errorhandler|exception_handler|register_error_handler)\(\s*(Exception|BaseException)\b",
    ),
    (
        &["py"],
        "Django's `handler500`, or a replaced `sys.excepthook`",
        r"^\s*(handler500\s*=|sys\.excepthook\s*=)",
    ),
    (
        &["rb"],
        "`rescue_from StandardError` or `rescue_from Exception`",
        r"\brescue_from\s+(StandardError|Exception)\b",
    ),
    (
        &["java", "kt"],
        "a controller advice (`@ControllerAdvice`)",
        r"@(Rest)?ControllerAdvice\b",
    ),
    (
        &["cs"],
        "ASP.NET's `UseExceptionHandler`",
        r"\bUseExceptionHandler\s*\(",
    ),
    (&["go"], "a `recover()`", r"\brecover\(\s*\)"),
];

fn compiled() -> &'static [Regex] {
    static COMPILED: OnceLock<Vec<Regex>> = OnceLock::new();
    COMPILED.get_or_init(|| {
        SIGNS
            .iter()
            .map(|(_, _, pattern)| Regex::new(pattern).expect("each sign's pattern compiles"))
            .collect()
    })
}

/// Every place a sign is found in the app's own files, as `(what, file:line)`, in file and line order.
pub(crate) fn found(listing: &Listing) -> Vec<(&'static str, String)> {
    let mut out = Vec::new();
    for entry in listing.app_files() {
        let extension = entry
            .relative
            .rsplit_once('.')
            .map(|(_, e)| e.to_lowercase())
            .unwrap_or_default();
        let signs: Vec<usize> = SIGNS
            .iter()
            .enumerate()
            .filter(|(_, (extensions, _, _))| extensions.contains(&extension.as_str()))
            .map(|(i, _)| i)
            .collect();
        if signs.is_empty() {
            continue;
        }
        let Ok(text) = entry.read_text() else {
            continue;
        };
        for (n, line) in text.lines().enumerate() {
            if let Some(&i) = signs.iter().find(|&&i| compiled()[i].is_match(line)) {
                out.push((SIGNS[i].1, format!("{}:{}", entry.relative, n + 1)));
            }
        }
    }
    out
}

/// A credit for V16.5.4 when a sign is found, naming where; nothing otherwise, and never a finding.
pub fn check(listing: &Listing, report: &mut ConfigReport) {
    let places = found(listing);
    if places.is_empty() {
        // Run, and withheld: no handler found is no credit, and said nowhere (ADR-059's census sees it here).
        crate::verified::unless_credited(LAST_RESORT, &report.passed);
        return;
    }
    // Each kind once, in the order first found.
    let mut kinds: Vec<&str> = Vec::new();
    for (what, _) in &places {
        if !kinds.contains(what) {
            kinds.push(what);
        }
    }
    let named: Vec<&str> = places
        .iter()
        .take(NAMED)
        .map(|(_, at)| at.as_str())
        .collect();
    let more = match places.len().saturating_sub(NAMED) {
        0 => String::new(),
        n => format!(", and {n} more"),
    };
    report.passed.push(Verified::new(
        LAST_RESORT,
        &["V16.5.4"],
        format!(
            "a last-resort error handler is defined in the app's code: {}, at {}{more}. That it catches every unhandled \
             exception, a worker's and a scheduled job's among them, is not something files show",
            kinds.join("; "),
            named.join(", ")
        ),
    ));
}

#[cfg(test)]
mod tests;
