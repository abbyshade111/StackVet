//! The coverage charts for the other two frameworks (`docs/coverage-aisvs-by-level.svg`,
//! `docs/coverage-sbd-by-domain.svg`) draw tables in `docs/COVERAGE.md` as bars, as `coverage_charts.rs` holds the
//! ASVS charts to theirs: each bar is read against its row in the document, not the script that wrote both.

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

/// The five counts written beside the bar labeled `label`, in the key's order: can be credited, can only be found
/// failing, supporting only, the owner answers, and no check yet.
fn bar_counts(svg: &str, label: &str) -> Vec<u32> {
    let line = svg
        .lines()
        .skip_while(|l| !l.contains(&format!(">{label}</text>")))
        .find(|l| l.contains("  of "))
        .unwrap_or_else(|| panic!("no counts for {label}"));
    line.split('>')
        .nth(1)
        .unwrap()
        .split("  of ")
        .next()
        .unwrap()
        .split(" · ")
        .map(|c| c.trim().parse().unwrap())
        .collect()
}

#[test]
fn each_aisvs_level_bar_has_the_table_s_requirements_settled_and_credited() {
    let doc = read("docs/COVERAGE.md");
    let svg = read("docs/coverage-aisvs-by-level.svg");
    let rows = table(&doc, "## AISVS 1.0 by level");
    assert_eq!(
        rows.len(),
        3,
        "the setup read AISVS's three levels: {rows:?}"
    );
    for row in rows {
        let n: Vec<u32> = row[1..4].iter().map(|c| c.parse().unwrap()).collect();
        let (requirements, settle, credited) = (n[0], n[1], n[2]);
        let level = row[0].trim_start_matches('L');
        let parts = bar_counts(&svg, &format!("Level {level}"));
        assert_eq!(parts.len(), 5, "level {level}: {parts:?}");
        assert_eq!(parts[0], credited, "level {level}: {parts:?}");
        assert_eq!(parts[0] + parts[1], settle, "level {level}: {parts:?}");
        assert_eq!(
            parts.iter().sum::<u32>(),
            requirements,
            "level {level}: {parts:?}"
        );
    }
}

#[test]
fn each_secure_by_design_bar_is_its_domain_s_controls_and_supporting_evidence() {
    let doc = read("docs/COVERAGE.md");
    let svg = read("docs/coverage-sbd-by-domain.svg");
    let rows = table(&doc, "## Secure by Design checklist 0.5.0 by domain");
    assert_eq!(rows.len(), 5, "the setup read the five domains: {rows:?}");
    for row in rows {
        let controls: u32 = row[1].parse().unwrap();
        let supported: u32 = row[4].parse().unwrap();
        let parts = bar_counts(&svg, &row[0].replace('&', "&amp;"));
        assert_eq!(parts.len(), 5, "{}: {parts:?}", row[0]);
        // A person answers every control, so no check credits one or finds it failing.
        assert_eq!((parts[0], parts[1]), (0, 0), "{}: {parts:?}", row[0]);
        assert_eq!(parts[2], supported, "{}: {parts:?}", row[0]);
        assert_eq!(parts.iter().sum::<u32>(), controls, "{}: {parts:?}", row[0]);
    }
}
