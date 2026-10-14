//! One answer to "did two looks at a path see one plain file", for the report lock and the MCP server's resources
//! alike (backlog 0461).

use super::*;

#[test]
fn two_looks_at_one_file_see_one_file() {
    let dir = sv_check::test_support::scratch("same-file");
    let path = dir.join("a.txt");
    std::fs::write(&path, "a").unwrap();
    let first = std::fs::symlink_metadata(&path).unwrap();
    let second = std::fs::symlink_metadata(&path).unwrap();
    assert!(same_file(&first, &second));
}

#[cfg(unix)]
#[test]
fn two_files_are_not_one() {
    let dir = sv_check::test_support::scratch("same-file");
    std::fs::write(dir.join("a.txt"), "a").unwrap();
    std::fs::write(dir.join("b.txt"), "b").unwrap();
    let a = std::fs::symlink_metadata(dir.join("a.txt")).unwrap();
    let b = std::fs::symlink_metadata(dir.join("b.txt")).unwrap();
    // The same length and both plain files: only which file each is tells them apart.
    assert!(!same_file(&a, &b));
}

#[test]
fn a_folder_is_never_the_file() {
    let dir = sv_check::test_support::scratch("same-file");
    let folder = std::fs::symlink_metadata(&dir).unwrap();
    assert!(!same_file(&folder, &folder));
}

#[cfg(unix)]
#[test]
fn a_file_swapped_for_another_at_the_same_path_is_seen() {
    let dir = sv_check::test_support::scratch("same-file");
    let path = dir.join("a.txt");
    std::fs::write(&path, "a").unwrap();
    let before = std::fs::symlink_metadata(&path).unwrap();
    // Kept open, so the new file cannot be given the old one's number.
    let _held = std::fs::File::open(&path).unwrap();
    std::fs::remove_file(&path).unwrap();
    std::fs::write(&path, "a").unwrap();
    let after = std::fs::symlink_metadata(&path).unwrap();
    assert!(!same_file(&before, &after));
}
