//! The starter file's first lines send the person to the report's table and to `sv review`, not to
//! reading the file itself, and the table they name is one the report really has (backlog 0477).

#[test]
fn the_starter_names_the_reports_own_table_and_sv_review() {
    let starter = sv_manifest::spec::STARTER_MANIFEST;
    let head: String = starter.lines().take(4).collect::<Vec<_>>().join("\n");
    assert!(!head.contains("read it yourself"), "{head}");
    assert!(head.contains("`sv review`"), "{head}");
    let table = "What the app says about itself";
    assert!(head.contains(&format!("\"{table}\"")), "{head}");
    // The report's heading, as the code that writes it has it: renamed there, this fails.
    let report = include_str!("../../sv-report/src/markdown.rs");
    assert!(
        report.contains(&format!("\"## {table}\\n")),
        "the report has no such table"
    );
}
