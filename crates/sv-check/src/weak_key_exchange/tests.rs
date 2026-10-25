//! What the key-exchange reader reports, and what it leaves alone (backlog 0625, part 12).

use super::weak_exchanges_in;

fn found(source: &str) -> Vec<(usize, String)> {
    weak_exchanges_in(source)
}

#[test]
fn small_diffie_hellman_primes_are_reported_with_their_size_and_line() {
    let python = "from cryptography.hazmat.primitives.asymmetric import dh\nparams = dh.generate_parameters(generator=2, key_size=1024)\n";
    assert_eq!(
        found(python),
        vec![(
            2,
            "a Diffie-Hellman exchange is set up with a 1024-bit prime by `dh.generate_parameters`"
                .to_owned()
        )]
    );
    for (source, bits) in [
        ("const dh = crypto.createDiffieHellman(1024);", "1024"),
        ("dh = OpenSSL::PKey::DH.new(1024)", "1024"),
        ("openssl dhparam -out dh.pem 1024", "1024"),
        (
            "KeyPairGenerator g = KeyPairGenerator.getInstance(\"DH\");\ng.initialize(1024);",
            "1024",
        ),
        (
            "AlgorithmParameterGenerator g = AlgorithmParameterGenerator.getInstance(\"DiffieHellman\");\ng.init(512);",
            "512",
        ),
    ] {
        let got = found(source);
        assert_eq!(got.len(), 1, "{source}: {got:?}");
        assert!(
            got[0].1.contains(&format!("{bits}-bit prime")),
            "{source}: {got:?}"
        );
    }
}

#[test]
fn small_built_in_groups_and_curves_under_192_bits_are_reported() {
    for source in [
        "const dh = crypto.getDiffieHellman('modp1');",
        "const dh = crypto.getDiffieHellman(\"modp2\");",
        "const dh = crypto.createDiffieHellmanGroup('modp5');",
        "const ecdh = crypto.createECDH('secp160r1');",
        "const ecdh = crypto.createECDH('secp112r1');",
    ] {
        assert_eq!(found(source).len(), 1, "{source}: {:?}", found(source));
    }
}

#[test]
fn safe_exchanges_are_left_alone() {
    for source in [
        "params = dh.generate_parameters(generator=2, key_size=2048)",
        "const dh = crypto.createDiffieHellman(2048);",
        "const dh = crypto.getDiffieHellman('modp14');",
        "const ecdh = crypto.createECDH('prime256v1');",
        "openssl dhparam -out dh.pem 4096",
        // A prime given as bytes, not a size: nothing to read.
        "const dh = crypto.createDiffieHellman(prime, 'hex');",
    ] {
        assert!(found(source).is_empty(), "{source}: {:?}", found(source));
    }
}

#[test]
fn a_192_bit_curve_is_left_to_the_weak_key_reader_so_it_is_not_reported_twice() {
    let source = "const ecdh = crypto.createECDH('prime192v1');";
    assert!(found(source).is_empty(), "{:?}", found(source));
    // The setup: the weak-key reader does report it, so it is not lost.
    assert_eq!(crate::weak_keys::weak_keys_in(source).len(), 1);
}

#[test]
fn a_comment_that_names_a_weak_exchange_is_not_one() {
    for source in [
        "// never: crypto.getDiffieHellman('modp2')",
        "# dh.generate_parameters(generator=2, key_size=1024) was the old way",
    ] {
        assert!(found(source).is_empty(), "{source}: {:?}", found(source));
    }
}
