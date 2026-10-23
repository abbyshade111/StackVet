//! Backlog 0645, ADR-137: a comment makes no key, so the weak-key check does not read one; a key made in code is still
//! found, on a line of its own or beside a comment.

use super::weak_keys_in;

#[test]
fn a_key_described_in_a_comment_is_not_a_key_made() {
    for source in [
        "// Node: generateKeyPair('rsa', { modulusLength: 1024 })\nfn f() {}\n",
        "# Python: rsa.generate_private_key(public_exponent=65537, key_size=1024)\n",
        "/*\n * RSA.generate(1024) is too small.\n */\n",
    ] {
        assert!(
            weak_keys_in(source).is_empty(),
            "{source}: {:?}",
            weak_keys_in(source)
        );
    }
}

#[test]
fn a_key_made_in_code_is_still_found() {
    for (source, line) in [
        (
            "const { generateKeyPair } = require('crypto');\ngenerateKeyPair('rsa', { modulusLength: 1024 }, done);\n",
            2,
        ),
        (
            "# A test key.\nkey = rsa.generate_private_key(public_exponent=65537, key_size=1024)\n",
            2,
        ),
        (
            "key = RSA.generate(1024)  # too small, kept for an old client\n",
            1,
        ),
    ] {
        let found = weak_keys_in(source);
        assert!(found.iter().any(|(l, _)| *l == line), "{source}: {found:?}");
    }
}
