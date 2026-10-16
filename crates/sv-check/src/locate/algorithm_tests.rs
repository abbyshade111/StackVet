//! Backlog 0611: `locate.algorithm-names` points the design question on crypto agility (V11.2.2) at the algorithm names
//! and key sizes written into the code. Held to what it must find and to the look-alikes it must leave alone.

use super::*;
use crate::test_support::scratch;

fn shipped() -> Locators {
    Locators::load(
        &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/locators.json"),
    )
    .expect("the shipped locators load, every query compiled")
}

/// The places one locator found in a folder holding one file, as `file:line`.
fn found(locator: &str, name: &str, text: &str) -> Vec<String> {
    let dir = scratch("");
    std::fs::write(dir.join(name), text).unwrap();
    let listing = sv_scan::files::Listing::of(&dir);
    // The setup worked: the file was listed as the app's own code, or finding nothing would mean nothing.
    assert_eq!(listing.app_files().count(), 1, "{name} was not listed");
    shipped()
        .find(&listing)
        .places
        .get(locator)
        .map(|places| {
            places
                .iter()
                .map(|p| format!("{}:{}", p.file, p.line))
                .collect()
        })
        .unwrap_or_default()
}

#[test]
fn algorithm_names_and_key_sizes_are_found_and_other_text_is_not() {
    let python = found(
        "locate.algorithm-names",
        "crypto.py",
        "digest = hashlib.new('sha256', data)\n\
         key = rsa.generate_private_key(public_exponent=65537, key_size=2048)\n\
         token = jwt.encode(claims, secret, algorithm='HS256')\n\
         hashed = bcrypt.hashpw(pw, bcrypt.gensalt(rounds=12))\n\
         name = 'shampoo'\n\
         size = 2048\n",
    );
    assert_eq!(
        python,
        ["crypto.py:1", "crypto.py:2", "crypto.py:3", "crypto.py:4"]
    );
    let javascript = found(
        "locate.algorithm-names",
        "crypto.js",
        "const c = crypto.createCipheriv('aes-256-gcm', key, iv);\n\
         const { publicKey } = crypto.generateKeyPairSync('rsa', { modulusLength: 2048 });\n\
         const label = 'aesthetic';\n",
    );
    assert_eq!(javascript, ["crypto.js:1", "crypto.js:2"]);
}

#[test]
fn a_file_with_only_a_quoted_algorithm_name_is_still_read() {
    // The whole-file screen must not miss a name written only in quotes (a first version did).
    let python = found(
        "locate.algorithm-names",
        "hash.py",
        "def hash_password(p, salt):\n\
         \x20   return hashlib.pbkdf2_hmac(\"sha256\", p, salt, 200_000)\n",
    );
    assert_eq!(python, ["hash.py:2"]);
}
