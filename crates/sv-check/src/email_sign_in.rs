//! V6.3.6: email used as a way of signing in (backlog 0625, part 12).
//!
//! V6.3.6 asks that "email is not used as either a single-factor or multi-factor authentication mechanism". This reads
//! the app's dependencies for a package whose purpose is signing people in with a link or a code sent by email. A
//! package that offers email among many ways in is not read, since an app may use another of them.
//!
//! It also reads stackvet.toml for an `email-code` entry under `[stack.run.users]`: the owner saying that the app signs
//! people in with a code or link it emails, which the signed-in suite then follows (`signed_in::codes`).
//!
//! It can only ever show V6.3.6 failing. Finding none credits nothing: an emailed link can be written by hand.

use crate::config::ConfigReport;
use crate::finding::{Confidence, Finding, Severity};
use crate::sbom::Sbom;
use sv_scan::files::Listing;

pub const EMAIL_SIGN_IN_PACKAGE: &str = "config.email-sign-in-package";
pub const EMAIL_CODE_DECLARED: &str = "config.email-code-sign-in";

/// Packages made for signing in by email, by ecosystem, each with what it does.
const PACKAGES: &[(&str, &str, &str)] = &[
    (
        "Python",
        "django-sesame",
        "signs people in with a link sent by email",
    ),
    (
        "Python",
        "django-magiclink",
        "signs people in with a link sent by email",
    ),
    (
        "Python",
        "drfpasswordless",
        "signs people in with a code sent by email or text message",
    ),
    (
        "npm",
        "passport-magic-link",
        "signs people in with a link sent by email",
    ),
    (
        "npm",
        "passwordless",
        "signs people in with a token sent by email",
    ),
    (
        "npm",
        "magic-sdk",
        "signs people in with Magic's emailed links",
    ),
    (
        "npm",
        "@magic-sdk/admin",
        "checks sign-ins made with Magic's emailed links",
    ),
    (
        "Ruby",
        "passwordless",
        "signs people in with a link sent by email",
    ),
    (
        "Ruby",
        "devise-passwordless",
        "signs people in with a link sent by email",
    ),
    (
        "PHP",
        "grosv/laravel-passwordless-login",
        "signs people in with a link sent by email",
    ),
];

/// The email sign-in packages the app depends on: `(ecosystem, name, version, what it does)`.
pub fn email_sign_in_packages(sbom: &Sbom) -> Vec<(&str, &str, &str, &'static str)> {
    let norm = |s: &str| s.to_lowercase().replace('_', "-");
    let mut out: Vec<_> = sbom
        .components
        .iter()
        .filter_map(|c| {
            PACKAGES
                .iter()
                .find(|(ecosystem, name, _)| c.ecosystem == *ecosystem && norm(&c.name) == *name)
                .map(|(_, _, does)| {
                    (
                        c.ecosystem.as_str(),
                        c.name.as_str(),
                        c.version.as_str(),
                        *does,
                    )
                })
        })
        .collect();
    out.sort();
    out.dedup();
    out
}

/// The line of stackvet.toml that declares an `email-code` entry, when the file declares one: an uncommented
/// `email-code = …` or `[stack.run.users.email-code]`. `None` when there is no such entry, or the file does not parse.
pub fn email_code_declared(app_dir: &std::path::Path) -> Option<usize> {
    let path = app_dir.join("stackvet.toml");
    let text = std::fs::read_to_string(&path).ok()?;
    let manifest = sv_manifest::Manifest::parse(&text, &path).ok()?;
    manifest.stack.run.users.as_ref()?.email_code.as_ref()?;
    let at = text.lines().position(|line| {
        let line = line.trim_start();
        line.starts_with("email-code") || line.starts_with("[stack.run.users.email-code]")
    });
    Some(at.map_or(1, |i| i + 1))
}

fn finding(
    location: crate::finding::Location,
    rule_id: &str,
    title: &str,
    description: String,
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
        rule_id: rule_id.into(),
        title: title.into(),
        severity: Severity::Low,
        confidence: Confidence::Medium,
        location,
        secret: None,
        requirement_ids: vec!["V6.3.6".into()],
        cwe: vec!["CWE-308".into()],
        description,
        impact: "Whoever can read a person's email, through a reused password, a shared computer, or a forwarding \
                 rule, can sign in as them, and email is not protected the way a sign-in should be."
            .into(),
        fix: "Sign people in with a password and a second step that is not email: an authenticator app's code, a \
              passkey, or a security key. Keep email for telling people what happened, not for letting them in."
            .into(),
    })
}

/// Reads the bill of materials for email sign-in packages, each placed at the dependency file's line that names it,
/// and stackvet.toml for an `email-code` entry.
pub fn check(listing: &Listing, sbom: &Sbom, report: &mut ConfigReport) {
    if let Some(line) = email_code_declared(&listing.root) {
        report.findings.push(finding(
            crate::finding::Location {
                file: "stackvet.toml".into(),
                line,
            },
            EMAIL_CODE_DECLARED,
            "The app signs people in with a code or link it sends by email",
            "stackvet.toml's `email-code` entry under `[stack.run.users]` says the app signs people in with a code or \
             link it emails. V6.3.6 asks that email not be used as a way of signing in, alone or as a second step."
                .to_owned(),
        ));
    }
    for (ecosystem, name, version, does) in email_sign_in_packages(sbom) {
        report.findings.push(finding(
            crate::rich_text::where_named(listing, name),
            EMAIL_SIGN_IN_PACKAGE,
            "The app depends on a package for signing in by email",
            format!(
                "The app depends on `{name}` {version} ({ecosystem}), which {does}. V6.3.6 asks that email not be \
                 used as a way of signing in, alone or as a second step, so check whether the app uses it to sign \
                 people in."
            ),
        ));
    }
}

#[cfg(test)]
mod tests;
