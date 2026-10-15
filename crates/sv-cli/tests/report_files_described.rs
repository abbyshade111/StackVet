//! Backlog 0413: every file of a report folder is described once, with who it is for, in `sv_report::folder::FILES`,
//! and the README, the guide, the website's reports page, and `report.html` itself say the same. The README used to
//! call `compliance.md` and `security.md` "the same, in Markdown", and the guide said they "say the same"; they are
//! written for different readers. A file added to `report_files::REPORT_FILES` and left out of the table, a row
//! dropped from one of the pages, or a reader changed in one place and not the others fails here.

use std::path::Path;

fn read(relative: &str) -> String {
    std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(relative),
    )
    .unwrap_or_else(|e| panic!("{relative}: {e}"))
}

/// The table rows of `text` that start with a file name in backticks: `| `name` | ... |`.
fn file_rows(text: &str) -> Vec<&str> {
    text.lines()
        .filter(|line| line.starts_with("| `") && line.ends_with('|'))
        .collect()
}

#[test]
fn the_described_files_are_the_files_written_in_the_same_order() {
    let described: Vec<&str> = sv_report::folder::FILES.iter().map(|f| f.name).collect();
    assert_eq!(described, sv_cli::report_files::NAMES);
}

#[test]
fn each_page_has_one_row_per_file_with_its_reader() {
    let pages = [
        "README.md",
        "docs/GETTING-STARTED.md",
        "docs/website/reports-page.md",
    ];
    for page in pages {
        let text = read(page);
        let rows = file_rows(&text);
        let names: Vec<&str> = rows
            .iter()
            .filter_map(|row| row.split('`').nth(1))
            .filter(|name| sv_cli::report_files::NAMES.contains(name))
            .collect();
        assert_eq!(
            names,
            sv_cli::report_files::NAMES,
            "{page}'s table of the report folder's files does not list each file once, in order"
        );
        for file in &sv_report::folder::FILES {
            let row = rows
                .iter()
                .find(|row| row.starts_with(&format!("| `{}` |", file.name)))
                .unwrap_or_else(|| panic!("{page} has no row for {}", file.name));
            assert!(
                row.contains(&format!("| {} |", file.for_whom)),
                "{page}'s row for {} does not say it is for \"{}\": {row}",
                file.name,
                file.for_whom
            );
        }
    }
}

#[test]
fn no_page_calls_the_two_markdown_reports_the_same() {
    for (page, words) in [
        ("README.md", "The same, in Markdown"),
        ("docs/GETTING-STARTED.md", "say the same in plain text"),
    ] {
        assert!(!read(page).contains(words), "{page} still says \"{words}\"");
    }
}

#[test]
fn the_report_lists_its_files_and_each_markdown_report_says_its_reader() {
    let app = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/flask-booking");
    let report = sv_cli::assemble_report(
        &app,
        &sv_cli::ReportOptions::reading_only("a test"),
        &sv_cli::Loaded::load().unwrap(),
    )
    .unwrap();
    assert!(
        !report.app_name.is_empty(),
        "the example app's report was not assembled"
    );

    let page = sv_report::html::page(&report);
    let section = page
        .split("<h2>The files in this folder</h2>")
        .nth(1)
        .expect("report.html has its list of files");
    for file in &sv_report::folder::FILES {
        assert!(
            section.contains(&format!("<td><code>{}</code></td>", file.name)),
            "report.html's list of files leaves out {}",
            file.name
        );
        assert!(
            section.contains(&sv_report::html::escape(file.for_whom)),
            "report.html does not say who {} is for",
            file.name
        );
    }

    for (name, text) in [
        ("compliance.md", sv_report::markdown::compliance(&report)),
        ("security.md", sv_report::markdown::security(&report)),
        (
            sv_report::LOW_FINDINGS_FILE,
            sv_report::markdown::low_findings(&report),
        ),
        (
            sv_report::checklist::FILE,
            sv_report::checklist::render(&report),
        ),
    ] {
        let opening: String = text.lines().take(6).collect::<Vec<_>>().join("\n");
        assert!(
            opening.contains(sv_report::folder::markdown_line(name).trim_end()),
            "{name} does not open by saying what it holds and who it is for:\n{opening}"
        );
    }
}
