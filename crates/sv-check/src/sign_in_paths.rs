//! The ways into the app the code shows, for the sign-in pathway questions (backlog 0491; ADR-022, Later).
//!
//! V6.1.3 asks the owner to write down every way a person or a program can sign in, and V6.3.4 asks that there be no
//! way in that is not written down. The ways in are mechanical to find: a route whose path says sign-in (`/login`,
//! `/auth/google`, `/oauth/callback`, `/token`), a Passport strategy, an API key read from a request. This lists the
//! ones `sv` can read, so the owner can hold their own list against it.
//!
//! # It finds, and does not judge
//!
//! Like a locator (`crate::locate`) and the route table (`crate::route_guards`), this makes no finding and credits
//! nothing. A path named `/auth` may not sign anybody in, and a way in may be named something none of these words
//! say, so the list is where to look, never evidence.

use crate::finding::{Confidence, Finding, Location, Severity};
use crate::route_guards::RouteLine;
use regex::Regex;
use std::sync::LazyLock;

/// The most ways in a question shows; the rest are counted.
pub const MOST_PATHWAYS: usize = 20;

/// What the list is, and what it is not, shown above it.
pub const PATHWAYS_SAY: &str = "The ways into the app StackVet found in the code: routes and paths whose words say \
     sign-in, Passport strategies, and API keys read from a request. These are where to look, not evidence: a route named for \
     sign-in may not sign anybody in, and a way in named something else, or declared in a way StackVet does not read, \
     is not in this list. Are these all the ways in, and is each held to the same rules?";

/// A path whose words say sign-in, a token, or a session.
static SIGN_IN_PATH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(^|[/_.-])(log-?in|sign-?in|sign-?on|auth|authenticate|oauth2?|callback|token|tokens|sso|saml|magic(-?link)?|api-?keys?|sessions?|webauthn|passkeys?|otp|2fa|mfa)([/_.-]|$)",
    )
    .expect("the sign-in path pattern compiles")
});

/// `passport.use(new GoogleStrategy(...))`, with or without a name before the strategy.
static PASSPORT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"passport\s*\.\s*use\s*\(\s*(?:['"][\w-]+['"]\s*,\s*)?new\s+([A-Za-z_$][\w$.]*Strategy)\b"#)
        .expect("the Passport pattern compiles")
});

/// An API key read by its header name.
static API_KEY_HEADER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)['"](x-api-key|x-api-token|api-key|apikey)['"]"#)
        .expect("the API key pattern compiles")
});

/// A request's path compared with a literal, as an app with no router matches its pages: `self.path == "/login"`,
/// `url.pathname === '/login/google'`.
static PATH_COMPARED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:path|pathname|url|route)\b\s*(?:===?|\.startswith\(|\.startsWith\()\s*['"](/[^'"]*)['"]"#)
        .expect("the compared path pattern compiles")
});

/// One way in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pathway {
    /// What it is, in plain words, with what the code calls it: "a route, POST /login", "a Passport strategy,
    /// GoogleStrategy", "an API key read from the `x-api-key` header".
    pub what: String,
    /// What a list in `stackvet.toml` names it by: its path, the strategy's name, or the header's name.
    pub key: String,
    pub file: String,
    pub line: usize,
}

/// Every way in found in an app.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Pathways {
    pub ways: Vec<Pathway>,
    /// Ways in past `MOST_PATHWAYS`, counted and not kept.
    pub more: usize,
}

/// The ways in, from the app's routes (`route_guards::read_all`) and its own files, not counting test code, the first
/// `MOST_PATHWAYS` kept.
pub fn read(listing: &sv_scan::files::Listing) -> Pathways {
    let mut ways = read_all(listing);
    let more = ways.len().saturating_sub(MOST_PATHWAYS);
    ways.truncate(MOST_PATHWAYS);
    Pathways { ways, more }
}

/// Every way in, in file and line order.
pub fn read_all(listing: &sv_scan::files::Listing) -> Vec<Pathway> {
    let mut ways: Vec<Pathway> = crate::route_guards::read_all(listing)
        .into_iter()
        .filter_map(|r| from_route(&r))
        .collect();
    for entry in listing.app_files() {
        if !matches!(entry.language, Some("python" | "javascript" | "typescript"))
            || listing.is_test_code(&entry.relative)
        {
            continue;
        }
        let Ok(source) = entry.read_text() else {
            continue;
        };
        ways.extend(in_text(&entry.relative, &source));
    }
    ways.sort_by(|a, b| a.file.cmp(&b.file).then(a.line.cmp(&b.line)));
    ways.dedup();
    ways
}

/// A route whose path says sign-in, as a way in.
pub fn from_route(route: &RouteLine) -> Option<Pathway> {
    SIGN_IN_PATH.is_match(&route.path).then(|| Pathway {
        what: format!("a route, {} {}", route.method, route.path),
        key: route.path.clone(),
        file: route.file.clone(),
        line: route.line,
    })
}

/// The Passport strategies, the sign-in paths a request is compared with, and the API keys read by header in one file.
pub fn in_text(file: &str, source: &str) -> Vec<Pathway> {
    let mut out = Vec::new();
    for (n, line) in source.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("//") || trimmed.starts_with('#') {
            continue;
        }
        for m in PASSPORT.captures_iter(line) {
            out.push(Pathway {
                what: format!("a Passport strategy, {}", &m[1]),
                key: m[1].to_owned(),
                file: file.to_owned(),
                line: n + 1,
            });
        }
        for m in PATH_COMPARED.captures_iter(line) {
            if SIGN_IN_PATH.is_match(&m[1]) {
                out.push(Pathway {
                    what: format!("a path the code compares a request with, {}", &m[1]),
                    key: m[1].to_owned(),
                    file: file.to_owned(),
                    line: n + 1,
                });
            }
        }
        if let Some(m) = API_KEY_HEADER.captures(line) {
            out.push(Pathway {
                what: format!("an API key read by the name `{}`", &m[1]),
                key: m[1].to_owned(),
                file: file.to_owned(),
                line: n + 1,
            });
        }
    }
    out
}

/// The rule for a way in the code shows that `[stack.auth] pathways` does not name (backlog 0491; ADR-123).
pub const NOT_LISTED: &str = "config.sign-in-pathway-not-listed";

/// Whether an entry of `[stack.auth] pathways` names this way in. Case and spaces aside, an entry names a way in whose
/// key it is, or, for a path, one below it (`/auth/google` names `/auth/google/callback`), or, for a Passport
/// strategy, its name without `Strategy` (`google` names `GoogleStrategy`).
pub fn named_by(way: &Pathway, entry: &str) -> bool {
    let entry = entry.trim().to_lowercase();
    let entry = entry.trim_end_matches('/');
    let key = way.key.to_lowercase();
    if entry.is_empty() {
        return false;
    }
    key == entry
        || key.starts_with('/') && key.starts_with(&format!("{entry}/"))
        || key
            .strip_suffix("strategy")
            .is_some_and(|name| name == entry)
}

/// The ways in that no entry of `listed` names.
pub fn not_listed<'a>(ways: &'a [Pathway], listed: &[String]) -> Vec<&'a Pathway> {
    ways.iter()
        .filter(|way| !listed.iter().any(|entry| named_by(way, entry)))
        .collect()
}

/// A finding for each way in the app's code shows that `listed`, the owner's `[stack.auth] pathways`, does not name
/// (backlog 0491; ADR-123). It can only ever show V6.3.4 failing: a list that names every way found credits nothing,
/// since a way in may be one this does not find.
pub fn not_listed_findings(listing: &sv_scan::files::Listing, listed: &[String]) -> Vec<Finding> {
    let ways = read_all(listing);
    not_listed(&ways, listed)
        .into_iter()
        .map(|way| {
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
                rule_id: NOT_LISTED.into(),
                title: "A way into the app is not in stackvet.toml's list of sign-in pathways".into(),
                severity: Severity::Medium,
                confidence: Confidence::Low,
                location: Location {
                    file: way.file.clone(),
                    line: way.line,
                },
                secret: None,
                requirement_ids: vec!["V6.3.4".into()],
                cwe: vec!["CWE-288".into()],
                description: format!(
                    "The code has {}, which looks like a way to sign in, and `[stack.auth] pathways` in \
                     stackvet.toml does not name `{}`. A way in nobody wrote down is one nobody checks \
                     is held to the same rules as the rest.",
                    way.what, way.key
                ),
                impact: "A forgotten way in, such as an old sign-in route, a test login, or an API key \
                         check, can skip what the main sign-in requires: a second factor, a limit on \
                         wrong passwords, or a check that the account is still active."
                    .into(),
                fix: format!(
                    "If `{}` is a way to sign in, add it to `pathways` under `[stack.auth]` in \
                     stackvet.toml, and write in your security notes how it is held to the same rules as \
                     the others. If it should not exist, remove it from the code. If it is not a way in \
                     at all, this is a false alarm and can be set aside in `sv review`.",
                    way.key
                ),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod not_listed_tests;
