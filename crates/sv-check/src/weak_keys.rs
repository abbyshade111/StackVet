//! V11.6.1: a key generated with a method or a size that is no longer approved (backlog 0494).
//!
//! V11.6.1 asks that "only approved cryptographic algorithms and modes of operation are used for key generation and
//! seeding, and digital signature generation and verification", and that key generation "must not generate insecure
//! keys vulnerable to known attacks". This reads the code for a key made in a way that is not approved today: an RSA
//! key under 2048 bits, a DSA key (FIPS 186-5 no longer approves DSA for making signatures), an elliptic-curve key on
//! a 192-bit curve, and an RSA public exponent of 3.
//!
//! It can only ever show V11.6.1 failing. Finding no such line credits nothing: a key can be made in a way this does
//! not read, or generated outside the code, and V11.6.1 also asks about signing and seeding, which this does not.

use crate::config::ConfigReport;
use crate::finding::{Confidence, Finding, Location, Severity};
use regex::Regex;
use std::sync::LazyLock;
use sv_scan::files::Listing;

pub const WEAK_KEY: &str = "ast.weak-key-generation";

/// The smallest RSA modulus still approved for making keys, in bits (NIST SP 800-131A).
const SMALLEST_RSA: u32 = 2048;

/// One way of making a key the code can say, with what it means.
struct Pattern {
    regex: LazyLock<Regex>,
    /// What was found, with `{n}` for the number the pattern captures, when it captures one.
    says: &'static str,
    /// A captured number is a key size: below `SMALLEST_RSA` is weak, and at or above is not a finding.
    sized: bool,
}

static PATTERNS: [Pattern; 8] = [
    // Node: generateKeyPair('rsa', { modulusLength: 1024 }), across up to a few lines.
    Pattern {
        regex: LazyLock::new(|| {
            Regex::new(r#"(?s)generateKeyPair(?:Sync)?\s*\(\s*['"`]rsa(?:-pss)?['"`]\s*,\s*\{[^}]{0,300}?modulusLength\s*:\s*(\d+)"#)
                .expect("compiles")
        }),
        says: "an RSA key of {n} bits is made with Node's `generateKeyPair`",
        sized: true,
    },
    // Python cryptography: rsa.generate_private_key(public_exponent=65537, key_size=1024).
    Pattern {
        regex: LazyLock::new(|| {
            Regex::new(r"(?s)rsa\.generate_private_key\s*\([^)]{0,200}?key_size\s*=\s*(\d+)")
                .expect("compiles")
        }),
        says: "an RSA key of {n} bits is made with `rsa.generate_private_key`",
        sized: true,
    },
    // PyCryptodome RSA.generate(1024), Go rsa.GenerateKey(rand.Reader, 1024), Ruby OpenSSL::PKey::RSA.new(1024).
    Pattern {
        regex: LazyLock::new(|| {
            Regex::new(r"\b(?:RSA\.generate|rsa\.GenerateKey\s*\(\s*[\w.]+\s*,|PKey::RSA\.(?:new|generate))\s*\(?\s*(\d{3,5})\b")
                .expect("compiles")
        }),
        says: "an RSA key of {n} bits is made",
        sized: true,
    },
    // Java: keyPairGenerator.initialize(1024) after getInstance("RSA") in the same file is read as RSA.
    Pattern {
        regex: LazyLock::new(|| {
            Regex::new(r#"(?s)KeyPairGenerator\.getInstance\s*\(\s*"RSA"\s*\).{0,400}?\.initialize\s*\(\s*(\d+)"#)
                .expect("compiles")
        }),
        says: "an RSA key of {n} bits is made with Java's `KeyPairGenerator`",
        sized: true,
    },
    // openssl genrsa 1024 in a script.
    Pattern {
        regex: LazyLock::new(|| {
            Regex::new(r"\bopenssl\s+genrsa\b[^\n]*?\s(\d{3,5})\b").expect("compiles")
        }),
        says: "an RSA key of {n} bits is made with `openssl genrsa`",
        sized: true,
    },
    // DSA keys: Node generateKeyPair('dsa'), Python dsa.generate_private_key, PyCryptodome DSA.generate, Java "DSA",
    // openssl gendsa or dsaparam.
    Pattern {
        regex: LazyLock::new(|| {
            Regex::new(r#"generateKeyPair(?:Sync)?\s*\(\s*['"`]dsa['"`]|\bdsa\.generate_private_key\s*\(|\bDSA\.generate\s*\(|KeyPairGenerator\.getInstance\s*\(\s*"DSA"\s*\)|\bopenssl\s+(?:gendsa|dsaparam)\b"#)
                .expect("compiles")
        }),
        says: "a DSA key is made, and DSA is no longer approved for making signatures",
        sized: false,
    },
    // A 192-bit curve: secp192r1, prime192v1, P-192, ec.SECP192R1().
    Pattern {
        regex: LazyLock::new(|| {
            Regex::new(r#"(?i)['"`]?(?:secp192[rk]1|prime192v1|p-192)['"`]?|\bec\.SECP192R1\s*\("#)
                .expect("compiles")
        }),
        says: "an elliptic-curve key is made on a 192-bit curve, which is no longer approved",
        sized: false,
    },
    // An RSA public exponent of 3: public_exponent=3, publicExponent: 3.
    Pattern {
        regex: LazyLock::new(|| {
            Regex::new(r"(?:public_exponent\s*=\s*|publicExponent\s*:\s*)3\b").expect("compiles")
        }),
        says: "an RSA key is made with a public exponent of 3",
        sized: false,
    },
];

/// What the code says about each weak key it makes, with the 1-indexed line: `(line, what)`.
pub fn weak_keys_in(source: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    for pattern in PATTERNS.iter() {
        for m in pattern.regex.captures_iter(source) {
            let whole = m.get(0).expect("a match has its whole");
            let says = match m.get(1) {
                Some(n) if pattern.sized => {
                    let Ok(bits) = n.as_str().parse::<u32>() else {
                        continue;
                    };
                    if bits >= SMALLEST_RSA {
                        continue;
                    }
                    pattern.says.replace("{n}", n.as_str())
                }
                _ => pattern.says.to_owned(),
            };
            let line = source[..whole.start()].matches('\n').count() + 1;
            // A comment makes no key: it describes one, as this file's own comments do (ADR-137).
            let line_start = source[..whole.start()].rfind('\n').map_or(0, |i| i + 1);
            let line_end = source[whole.start()..]
                .find('\n')
                .map_or(source.len(), |i| whole.start() + i);
            if crate::cert_checks::commented(&source[line_start..line_end]) {
                continue;
            }
            out.push((line, says));
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Reads the app's own files, test code aside, for a key made in a way no longer approved.
pub fn check(listing: &Listing, report: &mut ConfigReport) {
    for entry in listing.app_files() {
        if entry.language.is_none() || listing.is_test_code(&entry.relative) {
            continue;
        }
        let Ok(source) = entry.read_text() else {
            continue;
        };
        for (line, says) in weak_keys_in(&source) {
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
                rule_id: WEAK_KEY.into(),
                title: "A key is made with a method or a size that is no longer approved".into(),
                severity: Severity::Medium,
                confidence: Confidence::Medium,
                location: Location {
                    file: entry.relative.clone(),
                    line,
                },
                secret: None,
                requirement_ids: vec!["V11.6.1".into()],
                cwe: vec!["CWE-326".into()],
                description: format!(
                    "Here {says}. Keys like this are no longer approved for new use: an RSA key under 2048 bits \
                     or on a 192-bit curve is within reach of a determined attacker, a DSA signature is no longer \
                     approved to make, and an exponent of 3 makes some RSA mistakes exploitable."
                ),
                impact: "Whatever the key protects, a signature, a token, an encrypted message, can be forged or \
                         read by somebody able to break the key, and that gets cheaper every year."
                    .into(),
                fix: "Make RSA keys of 2048 bits or more (3072 for anything meant to last), with the exponent \
                      65537; or use an elliptic-curve key on P-256 or larger, or Ed25519. Replace DSA with ECDSA \
                      or Ed25519. Keys already made the old way have to be made again; changing the code alone \
                      does not change them."
                    .into(),
            }));
        }
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod comment_tests;
