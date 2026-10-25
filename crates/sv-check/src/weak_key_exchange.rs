//! V11.6.2: a key exchange with parameters no longer safe (backlog 0625, part 12).
//!
//! V11.6.2 asks that "approved cryptographic algorithms are used for key exchange (such as Diffie-Hellman) with a
//! focus on ensuring that key exchange mechanisms use secure parameters". This reads the code for a Diffie-Hellman
//! exchange set up with a prime under 2048 bits, one of the small built-in groups (`modp1`, `modp2`, `modp5`), or an
//! elliptic-curve exchange on a curve under 192 bits. A 192-bit curve is read by `weak_keys`, for V11.6.1, wherever it
//! is named, so it is not read again here.
//!
//! It can only ever show V11.6.2 failing. Finding no such line credits nothing: an exchange can be set up in a way
//! this does not read, or by a library or a server's settings outside the code.

use crate::config::ConfigReport;
use crate::finding::{Confidence, Finding, Location, Severity};
use regex::Regex;
use std::sync::LazyLock;
use sv_scan::files::Listing;

pub const WEAK_EXCHANGE: &str = "ast.weak-key-exchange";

/// The smallest Diffie-Hellman prime still approved, in bits (NIST SP 800-131A).
const SMALLEST_DH: u32 = 2048;

/// One way of setting up an exchange the code can say, with what it means.
struct Pattern {
    regex: LazyLock<Regex>,
    /// What was found, with `{n}` for the number or the name the pattern captures.
    says: &'static str,
    /// A captured number is a prime's size: below `SMALLEST_DH` is weak, and at or above is not a finding.
    sized: bool,
}

static PATTERNS: [Pattern; 7] = [
    // Python cryptography: dh.generate_parameters(generator=2, key_size=1024).
    Pattern {
        regex: LazyLock::new(|| {
            Regex::new(r"(?s)\bdh\.generate_parameters\s*\([^)]{0,200}?key_size\s*=\s*(\d+)")
                .expect("compiles")
        }),
        says: "a Diffie-Hellman exchange is set up with a {n}-bit prime by `dh.generate_parameters`",
        sized: true,
    },
    // Node: crypto.createDiffieHellman(1024), a prime of that many bits made on the spot.
    Pattern {
        regex: LazyLock::new(|| {
            Regex::new(r"\bcreateDiffieHellman\s*\(\s*(\d{2,5})\s*[,)]").expect("compiles")
        }),
        says: "a Diffie-Hellman exchange is set up with a {n}-bit prime by Node's `createDiffieHellman`",
        sized: true,
    },
    // Node: getDiffieHellman('modp2'), createDiffieHellmanGroup('modp5'): the 768-, 1024- and 1536-bit groups.
    Pattern {
        regex: LazyLock::new(|| {
            Regex::new(
                r#"\b(?:getDiffieHellman|createDiffieHellmanGroup)\s*\(\s*['"`](modp[125])['"`]"#,
            )
            .expect("compiles")
        }),
        says: "a Diffie-Hellman exchange uses Node's built-in group `{n}`, whose prime is under 2048 bits",
        sized: false,
    },
    // Node: createECDH('secp160r1') and the other curves under 192 bits.
    Pattern {
        regex: LazyLock::new(|| {
            Regex::new(r#"\bcreateECDH\s*\(\s*['"`](secp1(?:12|28)r[12]|secp160(?:k1|r1|r2))['"`]"#)
                .expect("compiles")
        }),
        says: "an elliptic-curve exchange uses the curve `{n}`, which is under 192 bits",
        sized: false,
    },
    // Java: KeyPairGenerator.getInstance("DH").initialize(1024), or AlgorithmParameterGenerator's init(1024).
    Pattern {
        regex: LazyLock::new(|| {
            Regex::new(r#"(?s)(?:KeyPairGenerator|AlgorithmParameterGenerator)\.getInstance\s*\(\s*"(?:DH|DiffieHellman)"\s*\).{0,400}?\.init(?:ialize)?\s*\(\s*(\d+)"#)
                .expect("compiles")
        }),
        says: "a Diffie-Hellman exchange is set up with a {n}-bit prime by Java's key generators",
        sized: true,
    },
    // Ruby: OpenSSL::PKey::DH.new(1024) or .generate(1024).
    Pattern {
        regex: LazyLock::new(|| {
            Regex::new(r"\bPKey::DH\.(?:new|generate)\s*\(\s*(\d{3,5})\b").expect("compiles")
        }),
        says: "a Diffie-Hellman exchange is set up with a {n}-bit prime by Ruby's `OpenSSL::PKey::DH`",
        sized: true,
    },
    // openssl dhparam 1024 in a script.
    Pattern {
        regex: LazyLock::new(|| {
            Regex::new(r"\bopenssl\s+dhparam\b[^\n]*?\s(\d{3,5})\b").expect("compiles")
        }),
        says: "Diffie-Hellman parameters with a {n}-bit prime are made by `openssl dhparam`",
        sized: true,
    },
];

/// What the code says about each weak exchange it sets up, with the 1-indexed line: `(line, what)`.
pub fn weak_exchanges_in(source: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    for pattern in PATTERNS.iter() {
        for m in pattern.regex.captures_iter(source) {
            let whole = m.get(0).expect("a match has its whole");
            let Some(caught) = m.get(1) else {
                continue;
            };
            if pattern.sized {
                let Ok(bits) = caught.as_str().parse::<u32>() else {
                    continue;
                };
                if bits >= SMALLEST_DH {
                    continue;
                }
            }
            let line = source[..whole.start()].matches('\n').count() + 1;
            // A comment sets up no exchange: it describes one, as this file's own comments do (ADR-137).
            let line_start = source[..whole.start()].rfind('\n').map_or(0, |i| i + 1);
            let line_end = source[whole.start()..]
                .find('\n')
                .map_or(source.len(), |i| whole.start() + i);
            if crate::cert_checks::commented(&source[line_start..line_end]) {
                continue;
            }
            out.push((line, pattern.says.replace("{n}", caught.as_str())));
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Reads the app's files for a key exchange set up with parameters no longer safe. Test code is read too, as
/// `weak_keys` reads it: a finding there is marked as test code where the report marks every finding.
pub fn check(listing: &Listing, report: &mut ConfigReport) {
    for entry in listing.app_files() {
        if entry.language.is_none() {
            continue;
        }
        let Ok(source) = entry.read_text() else {
            continue;
        };
        for (line, says) in weak_exchanges_in(&source) {
            report.findings.push(crate::finding::found(Finding {
                evidence: Vec::new(),
                also_reported_by: Vec::new(),
                fingerprint: String::new(),
                earlier_fingerprints: Vec::new(),
                marked_test_code: false,
                bundled_library: None,
                outranked: None,
                also_on_this_line: Vec::new(),
                help_uri: None,
                rule_id: WEAK_EXCHANGE.into(),
                title: "A key exchange is set up with parameters that are no longer safe".into(),
                severity: Severity::Medium,
                confidence: Confidence::Medium,
                location: Location {
                    file: entry.relative.clone(),
                    line,
                },
                secret: None,
                requirement_ids: vec!["V11.6.2".into()],
                cwe: vec!["CWE-326".into()],
                description: format!(
                    "Here {says}. A Diffie-Hellman prime under 2048 bits, or an elliptic curve under 192 bits, is \
                     within reach of a determined attacker, and the shared secret the exchange makes is only as \
                     strong as its parameters."
                ),
                impact: "Somebody able to break the exchange can work out the shared secret, then read, or change, \
                         whatever the two sides send with it, without either side noticing."
                    .into(),
                fix: "Use a Diffie-Hellman group of 2048 bits or more (`modp14` or larger, `ffdhe2048` or larger), \
                      or better, an elliptic-curve exchange on P-256 or larger, or X25519."
                    .into(),
            }));
        }
    }
}

#[cfg(test)]
mod run_tests;

#[cfg(test)]
mod tests;
