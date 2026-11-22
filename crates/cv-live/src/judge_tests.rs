use super::*;
use crate::github::github_tests::{Fake, fine, whole_repository};
use crate::github::{Repo, read};

fn judged(fake: &mut Fake) -> (crate::github::Reading, cv_rules::outcome::Evidence) {
    let reading = read(&Repo::parse("o/r").unwrap(), &fine(), fake);
    let evidence = judge(&reading);
    (reading, evidence)
}

fn ids(findings: &[Finding]) -> Vec<&str> {
    findings.iter().map(|f| f.rule_id.as_str()).collect()
}

#[test]
fn this_repositorys_settings_are_judged_as_read() {
    let (_, e) = judged(&mut whole_repository());
    assert_eq!(
        ids(&e.findings),
        [
            ACTIONS_ANYWHERE.id,
            ACTIONS_UNPINNED.id,
            ENVIRONMENT_UNREVIEWED.id
        ],
        "{:#?}",
        e.findings
    );
    let credited: Vec<&str> = e.verified.iter().map(|v| v.check_id.as_str()).collect();
    for rule in [
        DEFAULT_BRANCH_REVIEW,
        WORKFLOW_APPROVES,
        WORKFLOW_TOKEN_WRITES,
        SECRET_SCANNING,
        PUSH_PROTECTION,
        FORK_APPROVAL,
        ENVIRONMENT_UNREVIEWED,
    ] {
        assert!(
            credited.contains(&rule.id),
            "{} not credited: {credited:?}",
            rule.id
        );
    }
    assert!(e.not_assessed.is_empty(), "{:?}", e.not_assessed);
}

#[test]
fn every_credit_is_in_part() {
    let (_, e) = judged(&mut whole_repository());
    assert!(!e.verified.is_empty(), "the setup: something was credited");
    for credit in &e.verified {
        assert!(credit.in_part, "{} was credited whole", credit.check_id);
        assert!(
            credit.scope.contains("as GitHub reports it"),
            "{}",
            credit.scope
        );
    }
}

#[test]
fn unsafe_values_are_findings() {
    let r = "/repos/o/r";
    let mut fake = whole_repository()
        .ok(&format!("{r}/actions/permissions/workflow"), r#"{"default_workflow_permissions":"write","can_approve_pull_request_reviews":true}"#)
        .ok(r, r#"{"default_branch":"main","security_and_analysis":{"secret_scanning":{"status":"disabled"},"secret_scanning_push_protection":{"status":"disabled"}}}"#)
        .ok(&format!("{r}/actions/permissions/fork-pr-contributor-approval"), r#"{"approval_policy":"first_time_contributors_new_to_github"}"#)
        .ok(&format!("{r}/rules/branches/main"), r#"[{"type":"deletion"}]"#)
        .answer(&format!("{r}/branches/main/protection"), 404, &[], r#"{"message":"Branch not protected"}"#);
    let (_, e) = judged(&mut fake);
    let found = ids(&e.findings);
    for rule in [
        WORKFLOW_APPROVES,
        WORKFLOW_TOKEN_WRITES,
        SECRET_SCANNING,
        PUSH_PROTECTION,
        FORK_APPROVAL,
        DEFAULT_BRANCH_REVIEW,
    ] {
        assert!(found.contains(&rule.id), "{} not found: {found:?}", rule.id);
    }
    let approves = e
        .findings
        .iter()
        .find(|f| f.rule_id == WORKFLOW_APPROVES.id)
        .unwrap();
    assert_eq!(approves.requirement_ids, ["AC.8.4", "AC.4.1"]);
    assert!(approves.location.file.contains("github.com/o/r settings"));
}

#[test]
fn what_could_not_be_read_is_not_assessed_never_a_pass() {
    let r = "/repos/o/r";
    let mut fake = whole_repository()
        .ok(r, r#"{"default_branch":"main"}"#)
        .answer(
            &format!("{r}/actions/permissions"),
            403,
            &[("X-Accepted-GitHub-Permissions", "administration=read")],
            "{}",
        );
    let (_, e) = judged(&mut fake);
    let unread: Vec<&str> = e.not_assessed.iter().map(|n| n.rule_id.as_str()).collect();
    for rule in [
        SECRET_SCANNING,
        PUSH_PROTECTION,
        ACTIONS_ANYWHERE,
        ACTIONS_UNPINNED,
    ] {
        assert!(
            unread.contains(&rule.id),
            "{} not listed as not assessed: {unread:?}",
            rule.id
        );
        assert!(
            !e.verified.iter().any(|v| v.check_id == rule.id),
            "{} credited unread",
            rule.id
        );
        assert!(
            !e.findings.iter().any(|f| f.rule_id == rule.id),
            "{} found unread",
            rule.id
        );
    }
}

#[test]
fn a_bypass_list_github_did_not_show_is_not_assessed() {
    let mut fake = whole_repository()
        .ok("/repos/o/r/rulesets/1", r#"{"name":"Main","target":"branch","enforcement":"active","rules":[{"type":"pull_request"}]}"#);
    let (_, e) = judged(&mut fake);
    assert!(
        e.verified
            .iter()
            .any(|v| v.check_id == DEFAULT_BRANCH_REVIEW.id && v.scope.contains("not shown"))
    );
    assert!(
        e.not_assessed
            .iter()
            .any(|n| n.rule_id == DEFAULT_BRANCH_REVIEW.id && n.why.contains("bypass"))
    );
}

#[test]
fn a_stopped_reading_settles_nothing() {
    let mut fake =
        whole_repository().answer("/repos/o/r", 200, &[("X-OAuth-Scopes", "repo")], "{}");
    let (reading, e) = judged(&mut fake);
    assert!(reading.stopped.is_some());
    assert!(e.findings.is_empty() && e.verified.is_empty());
    assert_eq!(e.not_assessed.len(), RULES.len());
    assert!(token_words(&reading).contains("refused"));
}

#[test]
fn the_token_words_never_call_a_fine_grained_token_read_only() {
    let (reading, _) = judged(&mut whole_repository());
    let words = token_words(&reading);
    assert!(words.contains("could not confirm"), "{words}");
}
