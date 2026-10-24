//! The notes module's tests, a file of its own declared `#[cfg(test)] #[path = "notes_tests.rs"]` (ADR-136). The
//! helper below is not under `#[test]`, so only the declaring line says it is test code.

fn fixture_hash() -> md5::Digest {
    md5::compute(b"fixture") // sv:test weak-hash-function
}

#[test]
fn a_fixture_hash() {
    let _ = fixture_hash();
}
