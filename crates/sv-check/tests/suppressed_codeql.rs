//! A CodeQL result under an `lgtm[...]` or `codeql[...]` comment is still a finding (backlog 0509).
//!
//! The report lists every such comment (`crates/sv-check/src/look_away.rs`), but whether CodeQL itself
//! drops a result one marks, as `sv` runs it, was not known. The reports kept here are CodeQL 2.27.2's
//! own, over the small apps beside them (see `fixtures/suppressed/README.md`): the command-line tool
//! reports every marked result and marks none as suppressed. If a later CodeQL started dropping them,
//! these kept reports would no longer say what it does, and the test that reads a fresh run would be
//! the one to add.

use std::path::PathBuf;
use sv_check::adapters::{self, Adapters};

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/suppressed")
}

fn real_adapters() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/adapters.json")
}

/// The findings `sv` makes of one kept CodeQL report, through the real adapter entry.
fn findings(id: &str, report: &str) -> Vec<sv_check::finding::Finding> {
    let all = Adapters::load(&real_adapters()).unwrap();
    let adapter = all
        .all()
        .iter()
        .find(|a| a.id == id)
        .unwrap_or_else(|| panic!("{id} is listed"));
    let sarif = std::fs::read_to_string(fixtures().join(report)).unwrap();
    adapters::parse_sarif(adapter, &sarif).unwrap()
}

/// The lines a rule was found on, from the kept report itself, so the test does not lean on `sv`.
fn lines_in_report(report: &str, rule: &str) -> Vec<u64> {
    let sarif: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(fixtures().join(report)).unwrap()).unwrap();
    let mut lines: Vec<u64> = sarif["runs"][0]["results"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["ruleId"] == rule)
        .inspect(|r| assert!(r.get("suppressions").is_none(), "{r}"))
        .map(|r| {
            r["locations"][0]["physicalLocation"]["region"]["startLine"]
                .as_u64()
                .unwrap()
        })
        .collect();
    lines.sort_unstable();
    lines
}

fn check(id: &str, report: &str, rule: &str, marked_lines: &[u64]) {
    // The setup first: the kept report holds the control and every marked line, none suppressed.
    let in_report = lines_in_report(report, rule);
    assert_eq!(in_report.len(), 4, "{in_report:?}");
    for line in marked_lines {
        assert!(in_report.contains(line), "line {line} in {in_report:?}");
    }
    // Then what `sv` makes of it: each marked line is a finding, not dropped and not set aside.
    let found = findings(id, report);
    for line in marked_lines {
        assert!(
            found
                .iter()
                .any(|f| f.rule_id.ends_with(rule) && f.location.line == *line as usize),
            "line {line} is a finding: {found:?}"
        );
    }
}

#[test]
fn a_javascript_result_under_lgtm_or_codeql_comments_is_still_a_finding() {
    // app.js: 11 has `lgtm[...]` on the line, 17 has `codeql[...]` on the line before, 22 on the line.
    check(
        "codeql-javascript",
        "codeql-2.27.2-js-lgtm.sarif",
        "js/command-line-injection",
        &[11, 17, 22],
    );
}

#[test]
fn a_python_result_under_lgtm_or_codeql_comments_is_still_a_finding() {
    // app.py: 15 has `lgtm[...]` on the line, 22 has `codeql[...]` on the line before, 28 on the line.
    check(
        "codeql-python",
        "codeql-2.27.2-py-lgtm.sarif",
        "py/command-line-injection",
        &[15, 22, 28],
    );
}
