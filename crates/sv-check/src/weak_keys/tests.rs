//! What the weak-key reader reports, and what it leaves alone (backlog 0494).

use super::weak_keys_in;

fn found(source: &str) -> Vec<(usize, String)> {
    weak_keys_in(source)
}

#[test]
fn small_rsa_keys_are_reported_with_their_size_and_line() {
    let node = "const { generateKeyPair } = require('crypto');\ngenerateKeyPair('rsa', {\n  modulusLength: 1024,\n  publicExponent: 0x10001,\n}, cb);\n";
    assert_eq!(
        found(node),
        vec![(
            2,
            "an RSA key of 1024 bits is made with Node's `generateKeyPair`".to_owned()
        )]
    );
    let python = "from cryptography.hazmat.primitives.asymmetric import rsa\nkey = rsa.generate_private_key(public_exponent=65537, key_size=1024)\n";
    assert_eq!(
        found(python),
        vec![(
            2,
            "an RSA key of 1024 bits is made with `rsa.generate_private_key`".to_owned()
        )]
    );
    for (source, bits) in [
        ("key = RSA.generate(1024)", "1024"),
        ("k, _ := rsa.GenerateKey(rand.Reader, 512)", "512"),
        ("key = OpenSSL::PKey::RSA.new(1024)", "1024"),
        ("openssl genrsa -out key.pem 1024", "1024"),
    ] {
        let got = found(source);
        assert_eq!(got.len(), 1, "{source}: {got:?}");
        assert!(
            got[0].1.contains(&format!("of {bits} bits")),
            "{source}: {got:?}"
        );
    }
    let java = "KeyPairGenerator g = KeyPairGenerator.getInstance(\"RSA\");\ng.initialize(1024);\n";
    assert_eq!(found(java).len(), 1, "{:?}", found(java));
}

#[test]
fn dsa_small_curves_and_exponent_three_are_reported() {
    for source in [
        "generateKeyPair('dsa', { modulusLength: 2048 }, cb)",
        "key = dsa.generate_private_key(key_size=2048)",
        "openssl dsaparam -out p.pem 2048",
        "generateKeyPair('ec', { namedCurve: 'prime192v1' }, cb)",
        "key = ec.generate_private_key(ec.SECP192R1())",
        "rsa.generate_private_key(public_exponent=3, key_size=4096)",
    ] {
        assert_eq!(found(source).len(), 1, "{source}: {:?}", found(source));
    }
}

#[test]
fn approved_keys_are_left_alone() {
    for source in [
        "generateKeyPair('rsa', { modulusLength: 2048 }, cb)",
        "generateKeyPair('rsa', { modulusLength: 4096, publicExponent: 65537 }, cb)",
        "key = rsa.generate_private_key(public_exponent=65537, key_size=3072)",
        "key = RSA.generate(2048)",
        "generateKeyPair('ec', { namedCurve: 'P-256' }, cb)",
        "generateKeyPair('ed25519', cb)",
        "openssl genrsa -out key.pem 4096",
        "const rounds = 3;",
    ] {
        assert_eq!(found(source), Vec::new(), "{source}");
    }
}
