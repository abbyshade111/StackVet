//! The coverage charts (`docs/coverage-by-chapter.svg`, `docs/coverage-by-level.svg`; backlog 0483) draw the tables
//! in `docs/COVERAGE.md` as bars. `coverage_doc.rs` holds both current with the checks; this holds each bar to its
//! table row, read from the document rather than from the script that wrote both, so a chart that drew another column
//! than the table it pictures fails here.

use std::path::PathBuf;

fn read(path: &str) -> String {
    std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(path),
    )
    .unwrap()
}

/// The rows of the table under `heading`, as cells.
fn table(doc: &str, heading: &str) -> Vec<Vec<String>> {
    let after = doc
        .split(heading)
        .nth(1)
        .expect("the heading is in docs/COVERAGE.md");
    after
        .lines()
        .skip_while(|l| !l.starts_with('|'))
        .take_while(|l| l.starts_with('|'))
        .skip(2)
        .map(|l| {
            l.trim_matches('|')
                .split('|')
                .map(|c| c.trim().to_owned())
                .collect()
        })
        .collect()
}

/// What a bar's counts read as beside it: can be credited, can only be found failing, supporting only, the owner
/// answers, and no check yet (backlog 0624).
fn counts(credited: u32, settle: u32, supporting: u32, owner: u32, nothing: u32) -> String {
    let total = settle + supporting + owner + nothing;
    format!(
        "{credited} · {} · {supporting} · {owner} · {nothing}  of {total}",
        settle - credited
    )
}

#[test]
fn each_chapter_bar_is_its_row_in_the_table() {
    let doc = read("docs/COVERAGE.md");
    let svg = read("docs/coverage-by-chapter.svg");
    let rows = table(&doc, "## ASVS 5.0 by chapter");
    assert_eq!(
        rows.len(),
        17,
        "the setup read ASVS's seventeen chapters: {rows:?}"
    );
    for row in rows {
        let n: Vec<u32> = row[1..].iter().map(|c| c.parse().unwrap()).collect();
        let (requirements, settle, credited, supporting, owner, nothing) =
            (n[0], n[1], n[2], n[3], n[4], n[5]);
        assert_eq!(
            settle + supporting + owner + nothing,
            requirements,
            "{row:?}"
        );
        let label = format!(">{}</text>", row[0].replace('&', "&amp;"));
        assert!(svg.contains(&label), "no bar for {}", row[0]);
        let want = counts(credited, settle, supporting, owner, nothing);
        assert!(
            svg.contains(&want),
            "{}: the chart does not say {want}",
            row[0]
        );
    }
}

#[test]
fn each_level_bar_has_the_table_s_requirements_settled_and_credited() {
    let doc = read("docs/COVERAGE.md");
    let svg = read("docs/coverage-by-level.svg");
    let rows = table(&doc, "## ASVS 5.0 by level");
    assert_eq!(
        rows.len(),
        3,
        "the setup read ASVS's three levels: {rows:?}"
    );
    for row in rows {
        let n: Vec<u32> = row[1..4].iter().map(|c| c.parse().unwrap()).collect();
        let (requirements, settle, credited) = (n[0], n[1], n[2]);
        let level = row[0].trim_start_matches('L');
        let line = svg
            .lines()
            .skip_while(|l| !l.contains(&format!(">Level {level}</text>")))
            .find(|l| l.contains("  of "))
            .unwrap_or_else(|| panic!("no counts for level {level}"));
        let parts: Vec<u32> = line
            .split('>')
            .nth(1)
            .unwrap()
            .split("  of ")
            .next()
            .unwrap()
            .split(" · ")
            .map(|c| c.trim().parse().unwrap())
            .collect();
        assert_eq!(parts[0], credited, "level {level}: {line}");
        assert_eq!(parts[0] + parts[1], settle, "level {level}: {line}");
        assert_eq!(
            parts.iter().sum::<u32>(),
            requirements,
            "level {level}: {line}"
        );
    }
}
