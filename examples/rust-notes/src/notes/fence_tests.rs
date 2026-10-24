//! Tests that run only on Unix, declared `#[cfg(all(test, unix))] mod fence_tests;` (backlog 0652, ADR-139). The
//! helper below is not under `#[test]`, so only the declaring line says it is test code.

fn fixture_cipher() -> openssl::symm::Cipher {
    openssl::symm::Cipher::des_ede3_cbc() // sv:test weak-cipher
}

#[test]
fn a_fixture_cipher() {
    let _ = fixture_cipher();
}
