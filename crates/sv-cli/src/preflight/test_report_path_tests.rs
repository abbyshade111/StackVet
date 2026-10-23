//! Backlog 0639: the preflight says when the test command writes its report into the app's own folder, read-only while
//! the app runs, as the owner's Cursor build did with `--junitxml=junit.xml`, and is content when it writes under
//! /sv-reports.

use super::*;
use sv_report::fence::Fence;

fn said(test: &str) -> Option<(Answer, String)> {
    let toml = format!(
        "manifest-version = 1\n[app]\nname = \"t\"\n[stack.run]\nimage = \"python:3.12-slim\"\n\
         start = \"python app.py\"\nhealth = \"/\"\ntest = {test:?}\n"
    );
    let manifest = Manifest::parse(&toml, Path::new("stackvet.toml")).expect("the manifest parses");
    assert_eq!(
        manifest.stack.run.test.as_deref(),
        Some(test),
        "the setup reaches the manifest"
    );
    report_path_item(&manifest).map(|item| {
        assert_eq!(item.topic, "test-report-path");
        (item.answer, item.text(&Fence::none()))
    })
}

#[test]
fn a_report_written_into_the_app_s_folder_is_said_before_the_run() {
    // The owner's build.
    let (answer, text) = said("pytest -q --tb=short --junitxml=junit.xml").expect("an item");
    assert_eq!(answer, Answer::Look);
    assert!(
        text.contains("`junit.xml`") && text.contains("read-only while the app runs"),
        "{text}"
    );
    assert!(text.contains("--junitxml=/sv-reports/junit.xml"), "{text}");
    for test in [
        "pytest --junitxml reports/junit.xml",
        "npx jest --json --outputFile=results.json",
        "go test -json ./... > go-test.json",
        "node --test --test-reporter=tap --test-reporter-destination=./tests.tap",
        "gotestsum --junitfile=/app/junit.xml",
    ] {
        assert_eq!(said(test).map(|(a, _)| a), Some(Answer::Look), "{test}");
    }
}

#[test]
fn a_report_under_sv_reports_looks_right_and_no_report_says_nothing() {
    for test in [
        "pytest -q --junitxml=/sv-reports/junit.xml",
        "go test -json ./... > /sv-reports/go-test.json",
        "npx vitest run --reporter=json --outputFile=/sv-reports/vitest.json",
    ] {
        assert_eq!(said(test).map(|(a, _)| a), Some(Answer::Looks), "{test}");
    }
    for test in ["pytest -q", "npm test", "cargo test 2>&1"] {
        assert!(said(test).is_none(), "{test}");
    }
}
