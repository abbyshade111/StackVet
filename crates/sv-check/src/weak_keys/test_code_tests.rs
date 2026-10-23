//! Backlog 0655, ADR-137 (Later): the weak-key check reads test code as the code rules do. A weak key made in a test is
//! found, and its finding is marked as test code, so the reports list it apart and it still counts.

use super::*;
use crate::finding::mark_rust_test_code;

#[test]
fn a_weak_key_in_test_code_is_found_and_marked_as_test_code() {
    let dir = std::env::temp_dir().join(format!("sv-weak-key-tests-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    let write = |file: &str, text: &str| {
        let path = dir.join(file);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    };
    let weak = "key = rsa.generate_private_key(public_exponent=65537, key_size=1024)\n";
    write("app.py", weak);
    write("tests/test_keys.py", weak);
    write(
        "src/lib.rs",
        "pub fn run() {}\n\n#[cfg(all(test, unix))]\nmod key_tests;\n",
    );
    write(
        "src/key_tests.rs",
        "fn make() { let g = EcGroup::from_curve_name(\"prime192v1\"); }\n",
    );
    let listing = Listing::of(&dir);
    let mut report = ConfigReport::default();
    check(&listing, &mut report);
    mark_rust_test_code(&dir, &mut report.findings);
    std::fs::remove_dir_all(&dir).ok();
    let found: Vec<(&str, bool)> = report
        .findings
        .iter()
        .filter(|f| f.rule_id == WEAK_KEY)
        .map(|f| (f.location.file.as_str(), f.in_test_code()))
        .collect();
    let mut found = found;
    found.sort();
    assert_eq!(
        found,
        [
            ("app.py", false),
            ("src/key_tests.rs", true),
            ("tests/test_keys.py", true)
        ],
        "the app's own key, a test file's, and a Rust test module's"
    );
}
