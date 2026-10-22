use super::{write_report_files, write_without_following};

use sv_check::test_support::scratch;

#[test]
fn a_link_put_where_a_file_is_written_is_replaced_and_what_it_points_to_is_left_alone() {
    // The race `refuse_link` cannot close by looking first: a link put at the name after it looked.
    let dir = scratch("replace");
    std::fs::write(dir.join("precious.txt"), "keep me\n").unwrap();
    std::fs::create_dir(dir.join("out")).unwrap();
    std::os::unix::fs::symlink(dir.join("precious.txt"), dir.join("out/report.json")).unwrap();
    assert_eq!(
        std::fs::read_to_string(dir.join("out/report.json")).unwrap(),
        "keep me\n"
    );

    write_without_following(&dir.join("out"), "report.json", b"{}\n").unwrap();
    let kept = std::fs::read_to_string(dir.join("precious.txt")).unwrap();
    let meta = std::fs::symlink_metadata(dir.join("out/report.json")).unwrap();
    let written = std::fs::read_to_string(dir.join("out/report.json")).unwrap();
    let left: Vec<_> = std::fs::read_dir(dir.join("out"))
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect();
    std::fs::remove_dir_all(&dir).ok();
    assert_eq!(kept, "keep me\n", "the write went through the link");
    assert!(!meta.file_type().is_symlink() && written == "{}\n");
    assert_eq!(left.len(), 1, "a staging file was left behind: {left:?}");
}

#[test]
fn a_report_folder_that_is_a_link_is_refused_and_nothing_is_written() {
    // `sv report` writes to `<app>/stackvet-report` unless told otherwise, and an app can ship
    // that name as a link to a folder of the owner's.
    let dir = scratch("folder");
    std::fs::create_dir(dir.join("theirs")).unwrap();
    std::os::unix::fs::symlink(dir.join("theirs"), dir.join("stackvet-report")).unwrap();
    let app = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/flask-booking");
    let report = super::assemble_report(
        &app,
        &super::ReportOptions::reading_only("a test"),
        &super::Loaded::load().unwrap(),
    )
    .unwrap();
    let result = write_report_files(&report, &dir.join("stackvet-report"));
    let written: Vec<_> = std::fs::read_dir(dir.join("theirs")).unwrap().collect();
    std::fs::remove_dir_all(&dir).ok();
    let err = result.expect_err("a report folder that is a link was written through");
    assert!(format!("{err:#}").contains("is a link"), "{err:#}");
    assert!(
        written.is_empty(),
        "files were written through the link: {written:?}"
    );
}
