//! A security contact that is a placeholder (backlog 0405; ADR-112). An AI coding tool asked for a `SECURITY.md` may
//! write one whose contact is `security@example.com`: a name kept for examples (RFC 2606) that takes no mail, so a
//! report sent there reaches nobody. `config.security-contact` credits the file for being there; this check reads it.
//!
//! It cites no requirement, as `config.security-contact` cites none: nothing in ASVS 5.0, AISVS 1.0, its Appendix C,
//! or the Secure by Design checklist asks for a way to report a vulnerability (ADR-049).

use crate::config::Outcome;
use crate::finding::{Confidence, Finding, Location, Severity};
use regex::Regex;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

/// The check's id.
pub const PLACEHOLDER: &str = "config.security-contact-placeholder";

/// Reads each of the app's security files for a contact that is a stand-in.
pub(crate) fn placeholder(app_dir: &Path, files: &[PathBuf]) -> Outcome {
    if files.is_empty() {
        return Outcome::NotAssessed(
            "there is no SECURITY.md or security.txt to read, which config.security-contact reports".into(),
        );
    }
    let mut found = Vec::new();
    let mut unread = Vec::new();
    for path in files {
        let relative = path
            .strip_prefix(app_dir)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");
        match std::fs::read_to_string(path) {
            Ok(text) => {
                for (i, line) in text.lines().enumerate() {
                    if let Some((what, why)) = placeholder_on(line) {
                        found.push((relative.clone(), i + 1, what, why));
                    }
                }
            }
            Err(_) => unread.push(relative),
        }
    }
    let Some((file, line, what, why)) = found.first() else {
        return if unread.is_empty() {
            Outcome::Passed(&[])
        } else {
            Outcome::NotAssessed(format!("{} could not be read as text", unread.join(", ")))
        };
    };
    let more = match found.len() - 1 {
        0 => String::new(),
        1 => " One more line in the app's security files gives a stand-in too.".to_owned(),
        n => format!(" {n} more lines in the app's security files give stand-ins too."),
    };
    Outcome::Failed(Box::new(crate::finding::found(Finding {
        evidence: Vec::new(),
        also_reported_by: Vec::new(),
        fingerprint: String::new(),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        outranked: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
        rule_id: PLACEHOLDER.into(),
        title: "The place to report a security problem is a placeholder".into(),
        severity: Severity::Low,
        confidence: Confidence::High,
        location: Location {
            file: file.clone(),
            line: *line,
        },
        secret: None,
        requirement_ids: vec![],
        cwe: vec![],
        description: format!(
            "`{file}` gives `{what}` as where to report a security problem, and {why}, so a report sent there \
             reaches nobody.{more}"
        ),
        impact: "Somebody who finds a problem and does what the file says tells nobody.".into(),
        fix: "Put an address you read in its place. If the file promises an answer within a set time, make sure it is \
              a promise you will keep: an AI coding tool may have written it for you."
            .into(),
    })))
}

/// The stand-in on `line`, and why it reaches nobody, or `None`.
fn placeholder_on(line: &str) -> Option<(String, &'static str)> {
    static EMAIL: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?i)[a-z0-9._%+-]+@([a-z0-9-]+(?:\.[a-z0-9-]+)*)").expect("a fixed pattern")
    });
    // A web address counts only on a `Contact:` line, as security.txt gives one (RFC 9116); elsewhere in a
    // SECURITY.md an `example.com` address may be an example of something else.
    static CONTACT_URL: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?i)^\s*contact:\s*(https?://([a-z0-9.-]+)\S*)").expect("a fixed pattern")
    });
    static STAND_IN: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?i)\byou@|\byour[-_]?e-?mail\b|\bchange[-_]?me\b|<your[^>]*>|\[your[^\]]*\]")
            .expect("a fixed pattern")
    });
    for found in EMAIL.captures_iter(line) {
        if let Some(why) = reserved(&found[1]) {
            return Some((found[0].to_owned(), why));
        }
    }
    if let Some(found) = CONTACT_URL.captures(line)
        && let Some(why) = reserved(&found[2])
    {
        return Some((found[1].to_owned(), why));
    }
    STAND_IN.find(line).map(|m| {
        (
            m.as_str().to_owned(),
            "it is a stand-in left to be filled in",
        )
    })
}

/// Why a host name reaches nobody, or `None` for one that may.
fn reserved(host: &str) -> Option<&'static str> {
    let host = host.trim_end_matches('.').to_ascii_lowercase();
    let under = |name: &str| host == name || host.ends_with(&format!(".{name}"));
    if ["example.com", "example.org", "example.net"]
        .iter()
        .any(|name| under(name))
    {
        Some("that name is kept for examples (RFC 2606) and takes no mail")
    } else if ["example", "test", "invalid"]
        .iter()
        .any(|name| under(name))
    {
        Some("that name is kept for examples and tests (RFC 2606) and reaches no one")
    } else if under("localhost") {
        Some("`localhost` is whichever computer reads it, not yours")
    } else if under("local") {
        // In the trial of 15 October 2026 an AI coding tool cleared the finding with an address on `.local`
        // (backlog 0594).
        Some(
            "a `.local` name is found only on the network it is on (RFC 6762), so no mail from outside reaches it",
        )
    } else if host.starts_with("example.") {
        Some("a name beginning `example.` is one made for examples, almost certainly a stand-in")
    } else if host.starts_with("your") {
        Some("it is a stand-in left to be filled in")
    } else {
        None
    }
}

#[cfg(test)]
#[path = "security_contact_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "security_contact_unreachable_tests.rs"]
mod unreachable_tests;
