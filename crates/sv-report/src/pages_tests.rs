//! The report in short pages (backlog 0590): each page holds the sections for its purpose, cut whole from the reports,
//! and nothing else; each says what it is; and report.html leads to each.

use crate::checklist_tests::report;
use crate::pages::{self, PAGES};

/// The `## ` headings of a Markdown page, in order.
fn headings(page: &str) -> Vec<&str> {
    page.lines().filter_map(|l| l.strip_prefix("## ")).collect()
}

#[test]
fn each_page_holds_its_own_sections_and_says_what_it_is() {
    let report = report();
    let fix = pages::fix_now(&report);
    let fix_headings = headings(&fix);
    assert!(
        fix_headings.len() == 1 && fix_headings[0].ends_with(" things to fix"),
        "{fix}"
    );
    assert!(fix.contains("eval of a value the user gives"), "{fix}");

    let decisions = pages::decisions(&report);
    assert_eq!(
        headings(&decisions),
        ["Requirements nobody has placed"],
        "{decisions}"
    );
    assert!(
        decisions.contains("Does anyone sign in? Or: Is there an admin page?"),
        "{decisions}"
    );

    let not_examined = pages::not_examined(&report);
    assert_eq!(
        headings(&not_examined),
        ["What was not examined"],
        "{not_examined}"
    );
    assert!(
        not_examined.contains("SESSION_COOKIE_SECURE=0"),
        "{not_examined}"
    );

    for (page, text) in PAGES.iter().zip([
        &fix,
        &decisions,
        &not_examined,
        &pages::information(&report),
    ]) {
        assert!(
            text.starts_with(&format!("# {}: test\n", page.title)),
            "{text}"
        );
        assert!(
            text.contains(crate::folder::markdown_line(&report, page.file).trim_end()),
            "{}",
            page.file
        );
        assert!(
            text.contains("it credits nothing on its own"),
            "{}",
            page.file
        );
    }
}

#[test]
fn every_section_of_the_long_reports_is_on_one_page_or_kept_in_compliance_md() {
    // A section cut from compliance.md or security.md lands on one page at most; none is cut twice.
    let report = report();
    let pages: Vec<String> = [
        pages::fix_now,
        pages::decisions,
        pages::not_examined,
        pages::information,
    ]
    .iter()
    .map(|render| render(&report))
    .collect();
    let mut seen: Vec<&str> = Vec::new();
    for page in &pages {
        for heading in headings(page) {
            assert!(!seen.contains(&heading), "{heading} is on two pages");
            seen.push(heading);
        }
    }
    // compliance.md stays whole: the long reference keeps every section it had.
    let compliance = crate::markdown::compliance(&report);
    for heading in ["What was not examined", "Requirements that apply"] {
        assert!(headings(&compliance).contains(&heading), "{heading}");
    }
}

#[test]
fn an_empty_page_says_so_rather_than_nothing() {
    let report = crate::next_step_tests::empty_report(Vec::new());
    // Nothing found: security.md's own section says so, and to read it with what was not examined.
    let fix = pages::fix_now(&report);
    assert_eq!(headings(&fix), ["Nothing was found"], "{fix}");
    assert!(fix.contains("What was not examined"), "{fix}");
    assert!(pages::decisions(&report).contains("Nothing is waiting on your answer"));
}

#[test]
fn the_front_page_leads_to_each_page_and_says_what_it_is_for() {
    let html = crate::html::page(&report());
    let nav = html
        .split("<nav class=\"pages\"")
        .nth(1)
        .expect("the front page lists the pages");
    let nav = nav.split("</nav>").next().unwrap();
    for page in PAGES {
        assert!(
            nav.contains(&format!(
                "<a href=\"{0}\">{0}</a>: {1}.",
                page.file,
                crate::html::escape(page.purpose)
            )),
            "{}: {nav}",
            page.file
        );
    }
    assert!(
        nav.contains("before-going-live.md") && nav.contains("compliance.md"),
        "{nav}"
    );
}
