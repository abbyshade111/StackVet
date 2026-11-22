//! What each setting read from a Cloudflare zone counts as (ADR-157, decision 6, and its Later entry of 22 November
//! 2026; backlog 0732). As for GitHub: a safe value is a credit *in part*, for the value only, never for the behavior it
//! causes; an unsafe one is a finding; one not read is *not assessed*, never a pass. Each requirement cited was read in
//! `data/frameworks/asvs-5.0.0.json` first.

use crate::cloudflare::{Reading, Standing};
use cv_rules::outcome::{Evidence, NotAssessed, Rule};
use serde_json::Value;
use sv_core::{Confidence, Finding, Location, Severity, Verified};

/// HSTS's shortest max-age V3.4.1 accepts: one year, in seconds.
pub const ONE_YEAR: u64 = 31_536_000;

pub const ORIGIN_UNENCRYPTED: Rule = Rule {
    id: "cv.cloudflare-origin-unencrypted",
    setting: "ssl",
    // V12.3.1: an encrypted protocol for all inbound and outbound connections to and from the application. In part:
    // Cloudflare's connection to the origin, as its mode says, not every other connection the app makes.
    requirement_ids: &["V12.3.1"],
    cwe: &["CWE-319"],
    severity: Severity::High,
    title: "Cloudflare reaches the app over plain HTTP",
    impact: "Visitors see a padlock, but Cloudflare sends their requests on to the app unencrypted, so anyone on the \
             path between Cloudflare and the app's server can read and change them.",
    fix: "Give the app's server a certificate (Cloudflare's free Origin CA certificate will do) and set the SSL/TLS \
          mode to Full (strict) (SSL/TLS, Overview).",
    scope: "Cloudflare's connection to the app's server, by the SSL/TLS mode of",
};

pub const ORIGIN_UNCHECKED: Rule = Rule {
    id: "cv.cloudflare-origin-certificate-unchecked",
    setting: "ssl",
    // V12.3.2: TLS clients validate certificates received before communicating with a TLS server. Cloudflare is the
    // client here; under Full it encrypts without checking the origin's certificate.
    requirement_ids: &["V12.3.2"],
    cwe: &["CWE-295"],
    severity: Severity::Medium,
    title: "Cloudflare does not check the app's certificate",
    impact: "Cloudflare encrypts its connection to the app but accepts any certificate, so someone who can answer for \
             the app's address can stand in for it.",
    fix: "Set the SSL/TLS mode to Full (strict), once the app's server has a certificate Cloudflare trusts, such as an \
          Origin CA certificate (SSL/TLS, Overview).",
    scope: "Whether Cloudflare checks the app server's certificate, by the SSL/TLS mode of",
};

pub const OLD_TLS: Rule = Rule {
    id: "cv.cloudflare-old-tls-accepted",
    setting: "min_tls_version",
    // V12.1.1: only the latest recommended TLS versions enabled, such as 1.2 and 1.3. In part: the visitor's side.
    requirement_ids: &["V12.1.1"],
    cwe: &["CWE-327"],
    severity: Severity::Medium,
    title: "Visitors may connect with TLS older than 1.2",
    impact: "Cloudflare accepts TLS 1.0 or 1.1, which are no longer considered safe, so a visitor's browser can be \
             talked into a weaker connection.",
    fix: "Set Minimum TLS Version to TLS 1.2 or higher (SSL/TLS, Edge Certificates).",
    scope: "The lowest TLS version Cloudflare accepts from visitors to",
};

pub const NO_TLS_13: Rule = Rule {
    id: "cv.cloudflare-tls-1-3-off",
    setting: "tls_1_3",
    // V12.1.1: "The latest version of the TLS protocol must be the preferred option."
    requirement_ids: &["V12.1.1"],
    cwe: &[],
    severity: Severity::Low,
    title: "TLS 1.3 is off",
    impact: "Visitors cannot use the newest version of TLS, which V12.1.1 asks to be the preferred one.",
    fix: "Turn on TLS 1.3 (SSL/TLS, Edge Certificates).",
    scope: "Whether TLS 1.3 is on for visitors to",
};

pub const HTTP_ALLOWED: Rule = Rule {
    id: "cv.cloudflare-http-not-redirected",
    setting: "always_use_https",
    // V12.2.1: TLS for all connectivity between a client and external-facing HTTP services, with no fall back to
    // unencrypted communications. In part: Cloudflare's redirect, not what the app itself does.
    requirement_ids: &["V12.2.1"],
    cwe: &["CWE-319"],
    severity: Severity::Medium,
    title: "Plain HTTP is not redirected to HTTPS",
    impact: "A visitor who types the address without https, or follows an old link, is served over plain HTTP, which \
             anyone on the network can read and change.",
    fix: "Turn on Always Use HTTPS (SSL/TLS, Edge Certificates), or redirect HTTP in a rule of your own.",
    scope: "Whether Cloudflare redirects plain HTTP to HTTPS for",
};

pub const HSTS: Rule = Rule {
    id: "cv.cloudflare-hsts-weak",
    setting: "security_header",
    // V3.4.1: a Strict-Transport-Security header on all responses, max-age at least one year, and, for L2 and up, all
    // subdomains. In part: the header Cloudflare is set to add, not what each response carries.
    requirement_ids: &["V3.4.1"],
    cwe: &["CWE-319"],
    severity: Severity::Medium,
    title: "HSTS is off or too short",
    impact: "Browsers are not told to use HTTPS only, or are told for less than a year, so a visitor's first request \
             after that can be made over plain HTTP and taken over.",
    fix: "Turn on HSTS with a max-age of at least 12 months (SSL/TLS, Edge Certificates, HTTP Strict Transport \
          Security), once every subdomain it covers serves HTTPS.",
    scope: "The HSTS header Cloudflare adds for",
};

pub const CACHE_RULES: Rule = Rule {
    id: "cv.cloudflare-cache-rules",
    setting: "http_request_cache_settings",
    // V14.2.5's cache half (ADR-157, Later, 20 November 2026): caches keep only responses of the expected content type.
    // Shown, never credited, until a real zone's rules have been seen (ADR-157, Later, 22 November 2026).
    requirement_ids: &["V14.2.5"],
    cwe: &[],
    severity: Severity::Info,
    title: "The cache rules",
    impact: "",
    fix: "",
    scope: "The cache rules of",
};

/// Every rule this module can report, for the tests and the listing.
pub const RULES: &[Rule] = &[
    ORIGIN_UNENCRYPTED,
    ORIGIN_UNCHECKED,
    OLD_TLS,
    NO_TLS_13,
    HTTP_ALLOWED,
    HSTS,
    CACHE_RULES,
];

fn place(reading: &Reading, area: &str) -> Location {
    Location {
        file: format!("dash.cloudflare.com {} settings: {area}", reading.zone),
        line: 1,
    }
}

fn found(rule: &Rule, location: Location, description: String) -> Finding {
    sv_core::finding::found(Finding {
        rule_id: rule.id.to_owned(),
        title: rule.title.to_owned(),
        severity: rule.severity,
        // Cloudflare's own account of its setting, read just now.
        confidence: Confidence::High,
        location,
        secret: None,
        requirement_ids: rule
            .requirement_ids
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        cwe: rule.cwe.iter().map(|s| (*s).to_owned()).collect(),
        description,
        impact: rule.impact.to_owned(),
        fix: rule.fix.to_owned(),
        also_reported_by: Vec::new(),
        evidence: Vec::new(),
        fingerprint: String::new(),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
        outranked: None,
    })
}

fn credit(rule: &Rule, what: String) -> Verified {
    // A live reading is the provider's account of a setting's value: never more than in part (ADR-157, decision 6).
    Verified::new(rule.id, rule.requirement_ids, what).in_part()
}

fn unknown(rule: &Rule, unit: &str, why: &str) -> NotAssessed {
    NotAssessed {
        rule_id: rule.id.to_owned(),
        requirement_ids: rule
            .requirement_ids
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        unit: unit.to_owned(),
        why: why.to_owned(),
    }
}

/// Each setting of `reading`, settled. A reading that stopped settles nothing: everything is not assessed.
pub fn judge(reading: &Reading) -> Evidence {
    let mut out = Evidence::default();
    let zone = reading.zone.as_str();
    if let Some(why) = &reading.stopped {
        for rule in RULES {
            out.not_assessed.push(unknown(rule, zone, why));
        }
        return out;
    }
    let s = &reading.settings;
    let said = |rule: &Rule, words: &str| {
        format!("{} {zone}, as Cloudflare reports it: {words}", rule.scope)
    };
    let not_read = |rule: &Rule| format!("`{}` was not read", rule.setting);

    // The SSL/TLS mode: the connection from Cloudflare to the app's server.
    // Cloudflare's documentation shows one value of `ssl_automatic_mode`, `custom`, for a mode the person set; any other
    // is taken to be its Automatic SSL/TLS, which may raise the mode by itself, and is said with its own word.
    let automatic = match s.text("ssl_automatic_mode") {
        Some("custom") | None => String::new(),
        Some(other) => format!(
            "; SSL/TLS is set to `{other}`, not `custom`, so Cloudflare may raise this mode by itself"
        ),
    };
    match s.text("ssl") {
        Some(mode @ ("off" | "flexible")) => {
            out.findings.push(found(
                &ORIGIN_UNENCRYPTED,
                place(reading, "SSL/TLS"),
                format!(
                    "The SSL/TLS mode of {zone} is {}, so Cloudflare connects to the app's server over plain HTTP{automatic}.",
                    if mode == "off" { "Off" } else { "Flexible" }
                ),
            ));
        }
        Some("full") => {
            out.verified.push(credit(
                &ORIGIN_UNENCRYPTED,
                said(
                    &ORIGIN_UNENCRYPTED,
                    &format!("the mode is Full, so the connection is encrypted{automatic}"),
                ),
            ));
            out.findings.push(found(
                &ORIGIN_UNCHECKED,
                place(reading, "SSL/TLS"),
                format!(
                    "The SSL/TLS mode of {zone} is Full: Cloudflare encrypts its connection to the app's server but \
                     does not check its certificate{automatic}."
                ),
            ));
        }
        Some("strict") => {
            for rule in [&ORIGIN_UNENCRYPTED, &ORIGIN_UNCHECKED] {
                out.verified.push(credit(
                    rule,
                    said(rule, &format!("the mode is Full (strict){automatic}")),
                ));
            }
        }
        Some(other) => {
            for rule in [&ORIGIN_UNENCRYPTED, &ORIGIN_UNCHECKED] {
                out.not_assessed.push(unknown(
                    rule,
                    zone,
                    &format!("the SSL/TLS mode is `{other}`, which cv does not know how to read"),
                ));
            }
        }
        None => {
            for rule in [&ORIGIN_UNENCRYPTED, &ORIGIN_UNCHECKED] {
                out.not_assessed.push(unknown(rule, zone, &not_read(rule)));
            }
        }
    }

    // The lowest TLS version accepted, and TLS 1.3.
    let rule = &OLD_TLS;
    match s.text("min_tls_version") {
        Some(v @ ("1.0" | "1.1")) => out.findings.push(found(
            rule,
            place(reading, "Edge Certificates"),
            format!("Cloudflare accepts TLS {v} from visitors to {zone}."),
        )),
        Some(v @ ("1.2" | "1.3")) => out
            .verified
            .push(credit(rule, said(rule, &format!("TLS {v} and up")))),
        Some(other) => out.not_assessed.push(unknown(
            rule,
            zone,
            &format!("the lowest TLS version is `{other}`, which cv does not know how to read"),
        )),
        None => out.not_assessed.push(unknown(rule, zone, &not_read(rule))),
    }
    let rule = &NO_TLS_13;
    match s.text("tls_1_3") {
        Some("off") => out.findings.push(found(
            rule,
            place(reading, "Edge Certificates"),
            format!("TLS 1.3 is off for {zone}."),
        )),
        Some(v @ ("on" | "zrt")) => out.verified.push(credit(
            rule,
            said(
                rule,
                if v == "zrt" {
                    "it is on, with 0-RTT, which lets a request be replayed and is worth knowing about"
                } else {
                    "it is on"
                },
            ),
        )),
        Some(other) => out.not_assessed.push(unknown(
            rule,
            zone,
            &format!("TLS 1.3 is `{other}`, which cv does not know how to read"),
        )),
        None => out.not_assessed.push(unknown(rule, zone, &not_read(rule))),
    }

    // Always Use HTTPS. It redirects every address of the zone, which V4.1.2 asks not to do for an API: said, never
    // credited or counted.
    let rule = &HTTP_ALLOWED;
    match s.text("always_use_https") {
        Some("off") => out.findings.push(found(
            rule,
            place(reading, "Edge Certificates"),
            format!("Always Use HTTPS is off for {zone}."),
        )),
        Some("on") => out.verified.push(credit(
            rule,
            said(
                rule,
                "it is on. It redirects every address of the zone, API ones too, which V4.1.2 asks only of addresses \
                 people open in a browser",
            ),
        )),
        Some(other) => out.not_assessed.push(unknown(
            rule,
            zone,
            &format!("Always Use HTTPS is `{other}`, which cv does not know how to read"),
        )),
        None => out.not_assessed.push(unknown(rule, zone, &not_read(rule))),
    }

    // HSTS: on, and for a year at least. Whether it must cover subdomains depends on the app's level, which cv is not
    // told, so that half is not assessed rather than guessed.
    let rule = &HSTS;
    match s.values.get("security_header") {
        Some(value) => {
            let hsts = &value["strict_transport_security"];
            let enabled = hsts["enabled"].as_bool();
            let max_age = hsts["max_age"].as_u64();
            let subdomains = hsts["include_subdomains"].as_bool();
            let preload = hsts["preload"].as_bool() == Some(true);
            match (enabled, max_age) {
                (Some(false), _) => out.findings.push(found(
                    rule,
                    place(reading, "Edge Certificates"),
                    format!("HSTS is off for {zone}."),
                )),
                (Some(true), Some(age)) if age < ONE_YEAR => out.findings.push(found(
                    rule,
                    place(reading, "Edge Certificates"),
                    format!(
                        "HSTS for {zone} has a max-age of {age} seconds, under a year ({ONE_YEAR})."
                    ),
                )),
                (Some(true), Some(age)) => {
                    out.verified.push(credit(
                        rule,
                        said(
                            rule,
                            &format!(
                                "on, with a max-age of {age} seconds{}{}",
                                match subdomains {
                                    Some(true) => ", including subdomains",
                                    _ => ", not including subdomains",
                                },
                                if preload {
                                    "; it is marked for preloading, which is not the same as being on the preload \
                                     list (V3.7.4)"
                                } else {
                                    ""
                                }
                            ),
                        ),
                    ));
                    if subdomains != Some(true) {
                        out.not_assessed.push(unknown(
                            rule,
                            zone,
                            "HSTS does not include subdomains, which V3.4.1 asks for at level 2 and up; cv is not \
                             told the app's level",
                        ));
                    }
                }
                _ => out.not_assessed.push(unknown(
                    rule,
                    zone,
                    "Cloudflare's answer did not say whether HSTS is on and for how long",
                )),
            }
        }
        None => out.not_assessed.push(unknown(rule, zone, &not_read(rule))),
    }

    // The cache rules: shown, never credited, until a real zone's rules have been seen.
    let rule = &CACHE_RULES;
    out.not_assessed.push(unknown(
        rule,
        zone,
        &match &s.cache_rules {
            Some(rules) if rules.is_empty() => {
                "the zone has no cache rules; whether Cloudflare's default caching keeps only the expected content \
                 types is not read"
                    .to_owned()
            }
            Some(rules) => format!(
                "{} cache rule(s) were read and are listed; what to make of a rule is settled once a real zone's \
                 rules have been seen",
                rules.len()
            ),
            None => "the cache rules could not be read".to_owned(),
        },
    ));
    out
}

/// The words a report puts before any finding about the token (ADR-157, decision 5).
pub fn token_words(reading: &Reading) -> String {
    match &reading.token.standing {
        Standing::Refused { why } => format!("cv refused the token and sent nothing: {why}."),
        Standing::ActiveCouldNotConfirm => "Cloudflare says the token is active. cv could not confirm it is read-only: \
            Cloudflare shows a token's permissions only to a token that may read every API token, which cv does not \
            ask for. Use a token with Zone Read, Zone Settings Read, and Cache Rules Read for this zone only."
            .to_owned(),
        Standing::NotActive { why } => format!("{why}, so cv read nothing more."),
        Standing::NotChecked => "Cloudflare was not asked about the token.".to_owned(),
    }
}

/// A setting's value as the listing shows it.
pub fn shown(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

#[cfg(test)]
#[path = "cloudflare_judge_tests.rs"]
mod cloudflare_judge_tests;
