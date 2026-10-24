//! `ast.hand-rolled-cryptography` (backlog 0611, V11.2.1): a function named as encrypting that combines bytes with XOR
//! itself. Only ever a finding: hand-rolled cryptography written another way is not looked for.

use super::*;
use std::path::PathBuf;

const RULE: &str = "ast.hand-rolled-cryptography";

fn rules() -> &'static AstRules {
    static RULES: OnceLock<AstRules> = OnceLock::new();
    RULES.get_or_init(|| {
        AstRules::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/ast-rules.json"))
            .expect("rules load")
    })
}

/// Whether the rule reports `source`, read as `file`. A file the parser could not read, or a query that did not
/// compile, fails the test rather than reading as clean.
fn reported(language: &str, file: &str, source: &str) -> bool {
    let read = read_file(rules(), language, file, source);
    assert!(!read.parse_error, "{file} did not parse: {source}");
    assert!(
        read.broken.is_empty(),
        "a query did not compile: {:?}",
        read.broken
    );
    read.findings.iter().any(|f| f.rule_id == RULE)
}

fn check(language: &str, file: &str, wrap: impl Fn(&str) -> String, cases: &[(&str, bool)]) {
    for (code, expected) in cases {
        assert_eq!(
            reported(language, file, &wrap(code)),
            *expected,
            "{language}: `{code}` should{} be reported",
            if *expected { "" } else { " not" }
        );
    }
}

#[test]
fn the_rule_is_only_ever_a_finding_against_v11_2_1() {
    let rule = rules()
        .rules()
        .find(|r| r.id == RULE)
        .expect("the rule is shipped");
    assert_eq!(rule.requirement_ids, ["V11.2.1"]);
    assert!(rule.findings_only, "a clean result must credit nothing");
}

#[test]
fn python_xor_in_an_encrypt_function_is_found_and_a_library_call_is_not() {
    check(
        "python",
        "crypt.py",
        |code| format!("{code}\n"),
        &[
            (
                "def encrypt(data, key):\n    return bytes(b ^ key[i % len(key)] for i, b in enumerate(data))",
                true,
            ),
            (
                "def xor_cipher(data, k):\n    out = bytearray(data)\n    for i in range(len(out)):\n        out[i] ^= k\n    return bytes(out)",
                true,
            ),
            (
                "def encrypt(data, key):\n    return Fernet(key).encrypt(data)",
                false,
            ),
            ("def checksum(data):\n    return data[0] ^ data[1]", false),
            ("def encrypt_power(x):\n    return x ** 2", false),
        ],
    );
}

#[test]
fn javascript_xor_in_an_encrypt_function_is_found_and_a_library_call_is_not() {
    for (language, file) in [("javascript", "crypt.js"), ("typescript", "crypt.ts")] {
        check(
            language,
            file,
            |code| format!("{code}\n"),
            &[
                (
                    "function encrypt(text, key) { return text.split('').map((c, i) => String.fromCharCode(c.charCodeAt(0) ^ key)).join(''); }",
                    true,
                ),
                (
                    "class Box { obfuscate(s) { let o = ''; for (const c of s) o += String.fromCharCode(c.charCodeAt(0) ^ 42); return o; } }",
                    true,
                ),
                (
                    "function encrypt(text, key) { const c = crypto.createCipheriv('aes-256-gcm', key, iv); return c.update(text); }",
                    false,
                ),
                (
                    "function hashCode(s) { let h = 0; for (const c of s) h = (h << 5) ^ c.charCodeAt(0); return h; }",
                    false,
                ),
            ],
        );
    }
}

#[test]
fn rust_xor_in_an_encrypt_function_is_found_and_a_library_call_is_not() {
    // Backlog 0661: a free function and a method alike; `^` in a format string's alignment (`{:^8}`) is not XOR.
    check(
        "rust",
        "src/crypt.rs",
        |code| format!("{code}\n"),
        &[
            (
                "fn encrypt(data: &[u8], key: &[u8]) -> Vec<u8> { data.iter().enumerate().map(|(i, b)| b ^ key[i % key.len()]).collect() }",
                true,
            ),
            (
                "impl Vault { fn obfuscate(&self, out: &mut [u8]) { for b in out.iter_mut() { *b ^= self.key; } } }",
                true,
            ),
            (
                "fn encrypt(key: &Key<Aes256Gcm>, nonce: &Nonce, data: &[u8]) -> Vec<u8> { Aes256Gcm::new(key).encrypt(nonce, data).unwrap() }",
                false,
            ),
            (
                "fn checksum(data: &[u8]) -> u8 { data[0] ^ data[1] }",
                false,
            ),
            (
                "fn encrypt_label(name: &str) -> String { format!(\"{name:^8}\") }",
                false,
            ),
        ],
    );
}
