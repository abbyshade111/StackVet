use super::{REPORT_FOLDER_NAMES, claim_report_folder, is_staging};

#[test]
fn a_part_written_file_of_svs_is_svs_and_is_cleared_once_the_folder_is_held() {
    // What a run killed while writing its marker leaves: the folder not yet marked, and the
    // marker's part-written file. Seen on 4 October 2026, when the next run called the folder
    // someone else's and refused it.
    let dir = std::env::temp_dir().join(format!("sv-staging-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    for name in ["..stackvet-report.sv-98295", ".report.json.sv-7"] {
        std::fs::write(dir.join(name), "part").unwrap();
    }
    let held =
        claim_report_folder(&dir, "sv report", "give --out", false).expect("sv's own folder");
    assert!(
        dir.join(sv_scan::ecosystems::REPORT_MARKER).is_file(),
        "marked"
    );
    assert!(!dir.join("..stackvet-report.sv-98295").exists(), "cleared");
    assert!(!dir.join(".report.json.sv-7").exists(), "cleared");
    drop(held);
    std::fs::remove_dir_all(&dir).ok();

    // Names only like them are still someone else's.
    for name in [
        ".notes.md.sv-1",
        ".report.json.sv-",
        ".report.json.sv-12a",
        "report.json.sv-1",
    ] {
        assert!(!is_staging(name, REPORT_FOLDER_NAMES), "{name}");
    }
    let theirs = std::env::temp_dir().join(format!("sv-staging-theirs-{}", std::process::id()));
    std::fs::remove_dir_all(&theirs).ok();
    std::fs::create_dir_all(&theirs).unwrap();
    std::fs::write(theirs.join(".notes.md.sv-1"), "theirs").unwrap();
    let refused = claim_report_folder(&theirs, "sv report", "give --out", false)
        .err()
        .expect("not sv's folder")
        .to_string();
    assert!(refused.contains("files sv did not write"), "{refused}");
    assert!(
        !theirs.join(".stackvet-report.lock").exists(),
        "refused before a lock was put in someone else's folder"
    );
    std::fs::remove_dir_all(&theirs).ok();
}
