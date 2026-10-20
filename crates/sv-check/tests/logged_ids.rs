//! Semgrep's Python rule for credentials written to a log, on a line that logs only ids, is listed apart and decides
//! nothing (backlog 0583; ADR-111, Later). In the Opus run of 15 October 2026 it fired four times on lines logging a
//! user id or a game id.

use std::path::PathBuf;
use sv_check::adapters::{self, Adapters};
use sv_check::finding::Outranked;
use sv_check::logged_ids::{RULE, set_apart_logged_ids};

const APP: &str = "import logging\n\
logger = logging.getLogger(__name__)\n\
\n\
def reset(user_id, password):\n\
    logger.info(\"Password reset requested for user %s\", user_id)\n\
    logger.info(\"Password for %s is %s\", user_id, password)\n";

/// One Semgrep finding of `rule` on each of `lines` of `app.py`.
fn found(rule: &str, lines: &[usize]) -> Vec<sv_check::Finding> {
    let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/adapters.json");
    let all = Adapters::load(&data).expect("the adapter file loads");
    let semgrep = all
        .all()
        .iter()
        .find(|a| a.id == "semgrep")
        .expect("semgrep is listed");
    let results: Vec<serde_json::Value> = lines
        .iter()
        .map(|line| {
            serde_json::json!({
                "ruleId": rule.strip_prefix("semgrep.").unwrap(),
                "level": "warning",
                "message": { "text": "a finding" },
                "locations": [{ "physicalLocation": {
                    "artifactLocation": { "uri": "app.py" },
                    "region": { "startLine": line }
                }}]
            })
        })
        .collect();
    let sarif = serde_json::json!({
        "version": "2.1.0",
        "runs": [{ "tool": { "driver": { "name": "Semgrep OSS", "rules": [] } }, "results": results }]
    })
    .to_string();
    let findings = adapters::parse_sarif(semgrep, &sarif).expect("the report reads");
    assert_eq!(findings.len(), lines.len(), "every result made a finding");
    assert!(
        findings.iter().all(|f| f.rule_id == rule),
        "the rule is named as sv names it: {:?}",
        findings.iter().map(|f| &f.rule_id).collect::<Vec<_>>()
    );
    findings
}

/// `app.py` in a folder of its own for each test: cargo runs tests in parallel, and a shared folder let one test remove
/// it while the other was still reading it (a failure of 16 October 2026).
fn app(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-logged-ids-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("app.py"), APP).unwrap();
    dir
}

#[test]
fn a_line_logging_only_an_id_is_listed_apart_and_one_logging_a_password_is_not() {
    let dir = app("listed-apart");
    let mut findings = found(RULE, &[5, 6]);
    set_apart_logged_ids(&dir, &mut findings);
    std::fs::remove_dir_all(&dir).ok();
    assert_eq!(
        findings[0].outranked,
        Some(Outranked::LogsOnlyIds {
            names: vec!["user_id".to_owned()]
        })
    );
    assert!(findings[0].apart(), "listed apart");
    assert_eq!(
        findings[1].outranked, None,
        "a password logged still counts"
    );
}

#[test]
fn another_rule_or_a_second_tool_is_left_as_it_was() {
    let dir = app("left-as-it-was");
    let mut other = found(
        "semgrep.python.lang.security.audit.formatted-sql-query.formatted-sql-query",
        &[5],
    );
    let mut twice = found(RULE, &[5]);
    twice[0].also_reported_by = vec!["bandit.B999".to_owned()];
    set_apart_logged_ids(&dir, &mut other);
    set_apart_logged_ids(&dir, &mut twice);
    std::fs::remove_dir_all(&dir).ok();
    assert_eq!(other[0].outranked, None);
    assert_eq!(twice[0].outranked, None);
}
