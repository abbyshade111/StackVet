//! V10.4.3 and V10.4.8, read from an authorization server's own settings (backlog 0625, part 6; ADR-147).
//!
//! Two libraries most Python and Node authorization servers use set how long an authorization code and a refresh token
//! live. Their documentation, read 24 October 2026:
//!
//! - django-oauth-toolkit, in the `OAUTH2_PROVIDER` dictionary of Django's settings: `AUTHORIZATION_CODE_EXPIRE_SECONDS`
//!   defaults to 60; `REFRESH_TOKEN_EXPIRE_SECONDS` defaults to `None`, "which means refresh tokens never expire";
//!   `ROTATE_REFRESH_TOKEN` defaults to true.
//! - node-oidc-provider, in the provider's configuration: `ttl.AuthorizationCode` defaults to 60 seconds; the default
//!   `rotateRefreshToken` stops rotating a token once it has lived a year, after which "its TTL is final", and `true`
//!   rotates with no such end.
//!
//! What is credited is in part: a setting, not the server's behavior. A code lifetime is credited only at a minute or
//! less, which meets every level (level 3 asks a minute, levels 1 and 2 ten), and a finding only past ten minutes,
//! since the app's level is not known here; between the two, nothing is said. A django-oauth-toolkit refresh lifetime
//! is credited only with rotation off, since whether a rotated token's lifetime starts again was not read. A value
//! given by a function, or read from the environment, is not judged.

use crate::config::ConfigReport;
use crate::finding::{Confidence, Finding, Location, Severity};
use crate::sbom::Sbom;
use crate::verified::Verified;
use regex::Regex;
use std::sync::LazyLock;
use sv_scan::files::Listing;

pub const CODE_LIFETIME: &str = "config.authorization-code-long-lived";
pub const REFRESH_EXPIRY: &str = "config.refresh-token-never-expires";

/// A lifetime a whole level set allows, in seconds: level 3's minute, and levels 1 and 2's ten minutes.
const EVERY_LEVEL: u64 = 60;
const LEVELS_ONE_AND_TWO: u64 = 600;

/// A setting as written: a number of seconds (a product or sum of whole numbers), none, or something not judged.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Written {
    Seconds(u64),
    Never,
    NotJudged(String),
}

fn written(value: &str) -> Written {
    let value = value
        .split('#')
        .next()
        .unwrap_or("")
        .split("//")
        .next()
        .unwrap_or("");
    let value = value.trim().trim_end_matches(',').trim();
    if matches!(value, "None" | "null" | "undefined") {
        return Written::Never;
    }
    static ARITHMETIC: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^[\d_]+(\s*[*+]\s*[\d_]+)*$").unwrap());
    if !ARITHMETIC.is_match(value) {
        return Written::NotJudged(value.to_owned());
    }
    let mut total: u64 = 0;
    for term in value.split('+') {
        let product = term
            .split('*')
            .map(|n| n.trim().replace('_', "").parse::<u64>().unwrap_or(0))
            .fold(1u64, u64::saturating_mul);
        total = total.saturating_add(product);
    }
    Written::Seconds(total)
}

/// What one file sets: a setting's name, how it is written, and its line.
type Set = (String, Written, String, usize);

fn installed(sbom: &Sbom, ecosystem: &str, package: &str) -> bool {
    let norm = |s: &str| s.to_lowercase().replace(['_', '.'], "-");
    sbom.components
        .iter()
        .any(|c| c.ecosystem == ecosystem && norm(&c.name) == norm(package))
}

/// django-oauth-toolkit's settings, from the Python files that name `OAUTH2_PROVIDER`, and whether any Python file
/// names the library at all (`oauth2_provider`, its app's name): one that keeps every default writes no setting.
fn django(listing: &Listing) -> (Vec<Set>, bool) {
    static KEY: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r#"["'](AUTHORIZATION_CODE_EXPIRE_SECONDS|REFRESH_TOKEN_EXPIRE_SECONDS|ROTATE_REFRESH_TOKEN)["']\s*\]?\s*[:=]\s*([^,\n}]+)"#,
        )
        .unwrap()
    });
    let mut out = Vec::new();
    let mut named = false;
    for entry in listing
        .app_files()
        .filter(|f| f.extension.as_deref() == Some("py"))
    {
        let Ok(text) = entry.read_text() else {
            continue;
        };
        named |= text.contains("oauth2_provider") || text.contains("OAUTH2_PROVIDER");
        if !text.contains("OAUTH2_PROVIDER") {
            continue;
        }
        for c in KEY.captures_iter(&text) {
            let line = sv_scan::text::line_of(&text, c.get(0).map_or(0, |m| m.start()));
            let value = c[2].trim();
            let how = if c[1].starts_with("ROTATE") {
                Written::NotJudged(value.to_owned())
            } else {
                written(value)
            };
            out.push((c[1].to_owned(), how, entry.relative.clone(), line));
        }
    }
    (out, named)
}

/// node-oidc-provider's settings, from the files that name the package, and whether any does: one that keeps every
/// default writes no setting.
fn node(listing: &Listing) -> (Vec<Set>, bool) {
    static KEY: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"\b(AuthorizationCode|rotateRefreshToken)\s*:\s*([^,\n}]+)").unwrap()
    });
    let mut out = Vec::new();
    let mut named = false;
    for entry in listing.app_files().filter(|f| {
        f.extension
            .as_deref()
            .is_some_and(|e| ["js", "mjs", "cjs", "ts", "mts", "cts"].contains(&e))
    }) {
        let Ok(text) = entry.read_text() else {
            continue;
        };
        if !text.contains("oidc-provider") {
            continue;
        }
        named = true;
        for c in KEY.captures_iter(&text) {
            let line = sv_scan::text::line_of(&text, c.get(0).map_or(0, |m| m.start()));
            let value = c[2].trim().to_owned();
            let how = if &c[1] == "rotateRefreshToken" {
                Written::NotJudged(value)
            } else {
                written(&value)
            };
            out.push((c[1].to_owned(), how, entry.relative.clone(), line));
        }
    }
    (out, named)
}

fn finding(
    id: &str,
    title: &str,
    requirement: &str,
    file: &str,
    line: usize,
    description: String,
    fix: &str,
) -> Finding {
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
        rule_id: id.into(),
        title: title.into(),
        severity: Severity::Medium,
        confidence: Confidence::Medium,
        location: Location {
            file: file.into(),
            line,
        },
        secret: None,
        requirement_ids: vec![requirement.into()],
        cwe: vec!["CWE-613".into()],
        description,
        impact: if requirement == "V10.4.3" {
            "A sign-in code that lives long gives whoever catches it, from a log or a browser's history, longer to trade \
             it for the person's tokens."
        } else {
            "A refresh token that never expires lets whoever has it, once, keep getting new access tokens as the person \
             for as long as nobody revokes it."
        }
        .into(),
        fix: fix.into(),
    })
}

/// Where a finding about settings nobody wrote points: Django's settings file, or else the file that names the package.
fn where_named(listing: &Listing) -> String {
    let named = |pick: &dyn Fn(&str) -> bool| {
        listing
            .app_files()
            .find(|f| pick(f.relative.rsplit('/').next().unwrap_or(&f.relative)))
            .map(|f| f.relative.clone())
    };
    named(&|n| n == "settings.py")
        .or_else(|| {
            named(&|n| {
                matches!(
                    n,
                    "requirements.txt" | "pyproject.toml" | "Pipfile" | "setup.py"
                )
            })
        })
        .unwrap_or_else(|| "the app's settings".to_owned())
}

/// The first setting of `name` among `sets`, as written, with its place.
fn first<'a>(sets: &'a [Set], name: &str) -> Option<&'a Set> {
    sets.iter().find(|(n, _, _, _)| n == name)
}

pub fn check(listing: &Listing, sbom: &Sbom, report: &mut ConfigReport) {
    let (django_sets, django_named) = django(listing);
    let (node_sets, node_named) = node(listing);
    let has_django = installed(sbom, "Python", "django-oauth-toolkit") || django_named;
    let has_node = installed(sbom, "npm", "oidc-provider") || node_named;
    let mut code_ok = has_django || has_node;
    let mut code_said = Vec::new();
    let mut refresh_ok = has_django || has_node;
    let mut refresh_said = Vec::new();

    for (library, sets, code_key, default_place) in [
        (
            has_django.then_some("django-oauth-toolkit"),
            &django_sets,
            "AUTHORIZATION_CODE_EXPIRE_SECONDS",
            "Django's settings",
        ),
        (
            has_node.then_some("node-oidc-provider"),
            &node_sets,
            "AuthorizationCode",
            "the provider's configuration",
        ),
    ] {
        let Some(library) = library else { continue };
        // The authorization code: 60 seconds by default in both.
        let code = first(sets, code_key);
        match code.map(|(_, w, f, l)| (w.clone(), f.as_str(), *l)) {
            None => code_said.push(format!(
                "{library} leaves the code's lifetime at its default, 60 seconds"
            )),
            Some((Written::Seconds(s), _, _)) if s <= EVERY_LEVEL => {
                code_said.push(format!("{library} sets the code's lifetime to {s} seconds"))
            }
            Some((Written::Seconds(s), file, line)) if s > LEVELS_ONE_AND_TWO => {
                code_ok = false;
                report.findings.push(finding(
                    CODE_LIFETIME,
                    "Authorization codes live longer than ten minutes",
                    "V10.4.3",
                    file,
                    line,
                    format!(
                        "{library}'s `{code_key}` is {s} seconds. V10.4.3 allows ten minutes at levels 1 and 2, and a \
                         minute at level 3."
                    ),
                    "Set the authorization code's lifetime to 60 seconds, the library's own default: a client trades \
                     the code for tokens at once.",
                ));
            }
            // Between a minute and ten, or not a number: nothing is said either way.
            Some(_) => code_ok = false,
        }
        // The refresh token.
        if library == "django-oauth-toolkit" {
            let expiry = first(sets, "REFRESH_TOKEN_EXPIRE_SECONDS");
            let rotation_off = first(sets, "ROTATE_REFRESH_TOKEN")
                .is_some_and(|(_, w, _, _)| matches!(w, Written::NotJudged(v) if v == "False"));
            match expiry.map(|(_, w, f, l)| (w.clone(), f.clone(), *l)) {
                None | Some((Written::Never, _, _)) => {
                    refresh_ok = false;
                    let (file, line) = expiry
                        .map(|(_, _, f, l)| (f.clone(), *l))
                        .or_else(|| sets.first().map(|(_, _, f, l)| (f.clone(), *l)))
                        .unwrap_or_else(|| (where_named(listing), 1));
                    report.findings.push(finding(
                        REFRESH_EXPIRY,
                        "Refresh tokens never expire",
                        "V10.4.8",
                        &file,
                        line,
                        format!(
                            "django-oauth-toolkit's `REFRESH_TOKEN_EXPIRE_SECONDS` is {} in {default_place}, and \
                             `None` means refresh tokens never expire.",
                            if expiry.is_none() { "not set, so `None`," } else { "`None`" }
                        ),
                        "Set `REFRESH_TOKEN_EXPIRE_SECONDS` in `OAUTH2_PROVIDER` to the longest a person should stay \
                         signed in to a client without signing in again, such as 30 days.",
                    ));
                }
                Some((Written::Seconds(s), _, _)) if rotation_off => refresh_said.push(format!(
                    "django-oauth-toolkit's refresh tokens expire after {s} seconds, with rotation off"
                )),
                // Set, but with rotation on, or not a number: whether the limit holds is not read.
                Some(_) => refresh_ok = false,
            }
        } else {
            match first(sets, "rotateRefreshToken").map(|(_, w, f, l)| (w.clone(), f.clone(), *l)) {
                None => refresh_said.push(
                    "node-oidc-provider keeps its default rotation, which stops once a refresh token has lived a year"
                        .to_owned(),
                ),
                Some((Written::NotJudged(v), file, line)) if v == "true" => {
                    refresh_ok = false;
                    report.findings.push(finding(
                        REFRESH_EXPIRY,
                        "Refresh tokens are rotated with no end",
                        "V10.4.8",
                        &file,
                        line,
                        "node-oidc-provider's `rotateRefreshToken` is `true`, which rotates every refresh token with no \
                         end, so one token kept in use never expires."
                            .to_owned(),
                        "Remove `rotateRefreshToken: true` to keep the default, which stops rotating once a token has \
                         lived a year, or give a function that returns false past the longest you allow.",
                    ));
                }
                Some(_) => refresh_ok = false,
            }
        }
    }

    if code_ok && !code_said.is_empty() {
        report.passed.push(
            Verified::new(
                CODE_LIFETIME,
                &["V10.4.3"],
                format!(
                    "{}: a setting, not the server's behavior",
                    code_said.join("; ")
                ),
            )
            .in_part(),
        );
    } else {
        crate::verified::unless_credited(CODE_LIFETIME, &report.passed);
    }
    if refresh_ok && !refresh_said.is_empty() {
        report.passed.push(
            Verified::new(
                REFRESH_EXPIRY,
                &["V10.4.8"],
                format!(
                    "{}: a setting, not the server's behavior",
                    refresh_said.join("; ")
                ),
            )
            .in_part(),
        );
    } else {
        crate::verified::unless_credited(REFRESH_EXPIRY, &report.passed);
    }
}

#[cfg(test)]
mod tests;
