//! The notes themselves, a module of its own: its findings are the app's, and its test modules are files of their own,
//! declared below as Rust declares them.

use openssl::symm::Cipher;
use rusqlite::Connection;
use sha2::{Digest, Sha256};

// Finding a person's notes by joining their name into the query, and by a parameter.
pub fn by_owner_joined(conn: &Connection, owner: &str) {
    let _ = conn.execute(&format!("delete from notes where owner = '{owner}'"), []); // sv:fault sql-built-by-hand
}

pub fn by_owner_bound(conn: &Connection, owner: &str) {
    let _ = conn.execute("delete from notes where owner = ?1", [owner]); // sv:safe sql-built-by-hand
}

// Reading a file at a path the app was given, and one it names itself.
pub fn load(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_default() // sv:fault file-path-from-value
}

pub fn load_settings() -> String {
    std::fs::read_to_string("settings.toml").unwrap_or_default() // sv:safe file-path-from-value
}

// A hash that is broken, and one that is not.
pub fn fingerprint_md5(b: &[u8]) -> String {
    let digest = md5::compute(b); // sv:fault weak-hash-function
    format!("{digest:x}")
}

// The same hash made inside a macro's arguments: Rust's grammar keeps a macro's arguments as unread tokens, so no rule
// sees the call there yet (backlog 0660). Held as known, so the day it is read, the test says so.
pub fn fingerprint_in_a_macro(b: &[u8]) -> String {
    format!("{:x}", md5::compute(b)) // sv:missed weak-hash-function
}

pub fn fingerprint_sha256(b: &[u8]) -> Vec<u8> {
    Sha256::new().chain_update(b).finalize().to_vec() // sv:safe weak-hash-function
}

// Ciphers: a retired one, one that does not show whether the data was changed, and one that does.
pub fn old_cipher() -> Cipher {
    Cipher::des_ede3_cbc() // sv:fault weak-cipher
}

pub fn unchecked_cipher() -> Cipher {
    Cipher::aes_256_cbc() // sv:fault unauthenticated-encryption
}

pub fn checked_cipher() -> Cipher {
    Cipher::aes_256_gcm() // sv:safe weak-cipher sv:safe unauthenticated-encryption
}

// A key made from a password with a weak hash, and with an approved one at enough rounds.
pub fn key_weak(pw: &[u8], salt: &[u8], key: &mut [u8]) {
    pbkdf2::pbkdf2::<Hmac<Sha1>>(pw, salt, 1_000_000, key); // sv:fault weak-password-key-derivation
}

pub fn key_strong(pw: &[u8], salt: &[u8], key: &mut [u8]) {
    pbkdf2::pbkdf2_hmac::<Sha256>(pw, salt, 600_000, key); // sv:safe weak-password-key-derivation
}

#[cfg(test)]
#[path = "notes_tests.rs"]
mod tests;

#[cfg(all(test, unix))]
mod fence_tests;
