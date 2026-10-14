//! The shared scratch folder (backlog 0462): empty, made, named after the calling file, and never the same twice.

use super::*;

#[test]
fn each_call_has_an_empty_folder_of_its_own_named_after_the_calling_file() {
    let a = scratch("same");
    let b = scratch("same");
    assert_ne!(a, b, "two calls with one name share a folder");
    for dir in [&a, &b] {
        assert!(dir.is_dir(), "{} was not made", dir.display());
        assert_eq!(
            std::fs::read_dir(dir).unwrap().count(),
            0,
            "{} is not empty",
            dir.display()
        );
        let name = dir.file_name().unwrap().to_string_lossy().into_owned();
        assert!(name.starts_with("sv-scratch_tests-same-"), "{name}");
        assert!(name.contains(&std::process::id().to_string()), "{name}");
    }
    assert!(
        scratch("")
            .file_name()
            .unwrap()
            .to_string_lossy()
            .starts_with("sv-scratch_tests-")
    );
    std::fs::remove_dir_all(a).ok();
    std::fs::remove_dir_all(b).ok();
}

#[test]
fn a_row_s_status_is_read_and_a_missing_row_fails() {
    let compliance = "| row-a | checked | x |\n| row-b | needs attention | y |\n";
    assert_eq!(status_of(compliance, "row-b"), "needs attention");
    assert!(std::panic::catch_unwind(|| status_of(compliance, "row-c")).is_err());
}
