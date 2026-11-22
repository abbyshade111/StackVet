use super::*;
use crate::{Answer, MOST_REQUESTS};
use std::collections::BTreeMap;

/// One canned answer: its status, its headers, its body, and where it redirects.
type Canned = (u16, Vec<(String, String)>, String, Option<String>);

/// A stand-in for GitHub: an answer for each path, and every request it was asked, with the token it was given.
#[derive(Default)]
pub(crate) struct Fake {
    pub answers: BTreeMap<String, Canned>,
    pub asked: Vec<(String, String)>,
}

impl Fake {
    pub fn ok(mut self, path: &str, body: &str) -> Self {
        self.answers
            .insert(path.to_owned(), (200, Vec::new(), body.to_owned(), None));
        self
    }
    pub fn answer(mut self, path: &str, status: u16, headers: &[(&str, &str)], body: &str) -> Self {
        let headers = headers
            .iter()
            .map(|(n, v)| (n.to_ascii_lowercase(), (*v).to_owned()))
            .collect();
        self.answers
            .insert(path.to_owned(), (status, headers, body.to_owned(), None));
        self
    }
    pub fn redirect(mut self, path: &str, to: &str) -> Self {
        self.answers.insert(
            path.to_owned(),
            (301, Vec::new(), String::new(), Some(to.to_owned())),
        );
        self
    }
}

impl Transport for Fake {
    fn get(&mut self, url: &str, token: &str) -> Result<Answer, String> {
        self.asked.push((url.to_owned(), token.to_owned()));
        let path = url
            .strip_prefix("https://api.github.com")
            .ok_or("not api.github.com")?;
        let (status, headers, body, redirect) = self.answers.get(path).cloned().unwrap_or((
            404,
            Vec::new(),
            "{\"message\":\"Not Found\"}".to_owned(),
            None,
        ));
        Ok(Answer {
            status,
            headers,
            body: body.into_bytes(),
            redirect,
        })
    }
}

const R: &str = "/repos/o/r";

/// A token made from pieces at run time, as CLAUDE.md asks of test data that looks like a key.
pub(crate) fn fine() -> String {
    ["github", "_pat_", "11AAAA", "x".repeat(70).as_str()].concat()
}
pub(crate) fn classic() -> String {
    ["gh", "o_", "y".repeat(36).as_str()].concat()
}

/// This repository's settings as GitHub gave them on 21 November 2026, trimmed.
pub(crate) fn whole_repository() -> Fake {
    Fake::default()
        .ok(R, r#"{"default_branch":"main","private":false,"security_and_analysis":{"secret_scanning":{"status":"enabled"},"secret_scanning_push_protection":{"status":"enabled"}}}"#)
        .ok(&format!("{R}/rulesets?includes_parents=true&per_page=30"), r#"[{"id":1},{"id":2}]"#)
        .ok(&format!("{R}/rulesets/1"), r#"{"name":"Main","target":"branch","enforcement":"active","rules":[{"type":"pull_request"},{"type":"deletion"}],"bypass_actors":[],"current_user_can_bypass":"never"}"#)
        .ok(&format!("{R}/rulesets/2"), r#"{"name":"Release tags","target":"tag","enforcement":"active","rules":[{"type":"creation"}],"bypass_actors":[{"actor_id":5}],"current_user_can_bypass":"always"}"#)
        .ok(&format!("{R}/rules/branches/main"), r#"[{"type":"pull_request"},{"type":"deletion"}]"#)
        .ok(&format!("{R}/actions/permissions"), r#"{"enabled":true,"allowed_actions":"all","sha_pinning_required":false}"#)
        .ok(&format!("{R}/actions/permissions/workflow"), r#"{"default_workflow_permissions":"read","can_approve_pull_request_reviews":false}"#)
        .ok(&format!("{R}/actions/permissions/fork-pr-contributor-approval"), r#"{"approval_policy":"first_time_contributors"}"#)
        .ok(&format!("{R}/environments?per_page=30"), r#"{"environments":[{"name":"release","protection_rules":[{"type":"required_reviewers","prevent_self_review":false,"reviewers":[{"type":"User"}]},{"type":"branch_policy"}],"deployment_branch_policy":{"protected_branches":false,"custom_branch_policies":true}},{"name":"staging","protection_rules":[],"deployment_branch_policy":null}]}"#)
        .ok(&format!("{R}/environments/release/deployment-branch-policies"), r#"{"branch_policies":[{"name":"v*","type":"tag"}]}"#)
}

fn repo() -> Repo {
    Repo::parse("o/r").unwrap()
}

#[test]
fn a_repository_name_is_owner_and_name_and_nothing_that_reaches_another_path() {
    assert_eq!(
        Repo::parse("abbyshade111/StackVet").unwrap().full(),
        "abbyshade111/StackVet"
    );
    for bad in [
        "", "o", "o/", "/r", "o/r/x", "../r", "o/..", "o/r?x=1", "o/r#x", "o r/x", "o/%2e",
    ] {
        assert!(Repo::parse(bad).is_err(), "{bad:?} was taken");
    }
}

#[test]
fn a_branch_or_environment_name_is_one_path_segment() {
    assert_eq!(segment("main"), "main");
    assert_eq!(segment("feature/x"), "feature%2Fx");
    assert_eq!(segment("../../user"), "..%2F..%2Fuser");
    assert_eq!(segment("a?b#c d"), "a%3Fb%23c%20d");
}

#[test]
fn token_kinds_are_told_from_their_prefixes() {
    assert_eq!(token_kind(&fine()), TokenKind::FineGrained);
    assert_eq!(token_kind(&classic()), TokenKind::Classic);
    assert_eq!(token_kind(&["gh", "p_x"].concat()), TokenKind::Classic);
    assert_eq!(token_kind(&["gh", "s_x"].concat()), TokenKind::Installation);
    assert_eq!(token_kind("something-else"), TokenKind::Unknown);
}

#[test]
fn every_scope_but_a_read_scope_writes() {
    let scopes = |s: &[&str]| s.iter().map(|x| (*x).to_owned()).collect::<Vec<_>>();
    assert_eq!(
        writing_scopes(&scopes(&["read:org", "read:user"])),
        Vec::<String>::new()
    );
    assert_eq!(
        writing_scopes(&scopes(&["gist", "read:org", "repo", "workflow"])),
        scopes(&["gist", "repo", "workflow"])
    );
    assert_eq!(
        writing_scopes(&scopes(&["public_repo"])),
        scopes(&["public_repo"])
    );
}

#[test]
fn a_classic_token_that_can_write_is_refused_after_one_request() {
    let mut fake = whole_repository().answer(
        R,
        200,
        &[("X-OAuth-Scopes", "gist, read:org, repo, workflow")],
        r#"{"default_branch":"main"}"#,
    );
    let reading = read(&repo(), &classic(), &mut fake);
    assert_eq!(
        fake.asked.len(),
        1,
        "nothing more is asked of a token that can write: {:?}",
        fake.asked
    );
    assert!(
        matches!(&reading.token.read_only, ReadOnly::Refused { writing, .. } if writing == &["gist", "repo", "workflow"])
    );
    assert!(
        reading.stopped.as_deref().unwrap().contains("can write"),
        "{:?}",
        reading.stopped
    );
    assert!(
        reading.settings.default_branch.is_none(),
        "nothing is taken from its answer"
    );
}

#[test]
fn a_classic_token_with_read_scopes_only_is_confirmed() {
    let mut fake = whole_repository().answer(
        R,
        200,
        &[("X-OAuth-Scopes", "read:org")],
        r#"{"default_branch":"main"}"#,
    );
    let reading = read(&repo(), &classic(), &mut fake);
    assert!(
        matches!(&reading.token.read_only, ReadOnly::Confirmed { scopes } if scopes == &["read:org"])
    );
    assert!(reading.stopped.is_none());
}

#[test]
fn a_fine_grained_token_is_never_called_read_only() {
    let mut fake = whole_repository();
    let reading = read(&repo(), &fine(), &mut fake);
    assert!(
        matches!(&reading.token.read_only, ReadOnly::CouldNotConfirm { why } if why.contains("fine-grained")),
        "{:?}",
        reading.token.read_only
    );
    assert!(reading.stopped.is_none(), "{:?}", reading.stopped);
}

#[test]
fn a_whole_reading_asks_only_the_fixed_list_and_lists_every_request() {
    let mut fake = whole_repository();
    let reading = read(&repo(), &fine(), &mut fake);
    let paths: Vec<&str> = reading.requests.iter().map(|r| r.path.as_str()).collect();
    assert_eq!(
        paths,
        [
            "/repos/o/r",
            "/repos/o/r/rulesets?includes_parents=true&per_page=30",
            "/repos/o/r/rulesets/1",
            "/repos/o/r/rulesets/2",
            "/repos/o/r/rules/branches/main",
            "/repos/o/r/actions/permissions",
            "/repos/o/r/actions/permissions/workflow",
            "/repos/o/r/actions/permissions/fork-pr-contributor-approval",
            "/repos/o/r/environments?per_page=30",
            "/repos/o/r/environments/release/deployment-branch-policies",
        ]
    );
    assert!(
        reading
            .requests
            .iter()
            .all(|r| r.method == "GET" && r.host == "api.github.com" && r.status == 200)
    );
    assert_eq!(
        fake.asked.len(),
        reading.requests.len(),
        "every request sent is listed"
    );
    assert!(
        fake.asked
            .iter()
            .all(|(url, _)| url.starts_with("https://api.github.com/"))
    );
    let s = &reading.settings;
    assert_eq!(s.allowed_actions.as_deref(), Some("all"));
    assert_eq!(s.sha_pinning_required, Some(false));
    assert_eq!(s.secret_scanning, Some(true));
    let envs = s.environments.as_ref().unwrap();
    assert_eq!(envs[0].deploys_from, "v* (tag)");
    assert_eq!(envs[0].reviewers, 1);
    assert_eq!(envs[1].deploys_from, "any branch");
}

#[test]
fn the_token_is_nowhere_in_the_reading() {
    let mut fake = whole_repository();
    let token = fine();
    let reading = read(&repo(), &token, &mut fake);
    let text = serde_json::to_string(&reading).unwrap();
    assert!(
        text.contains("\"kind\":\"fine-grained\""),
        "the setup: the reading was written: {text}"
    );
    assert!(!text.contains(&token), "the token is in the reading");
    assert!(
        !text.contains(&token[..20]),
        "a long piece of the token is in the reading"
    );
}

#[test]
fn a_redirect_to_another_host_is_not_followed() {
    let mut fake = whole_repository().redirect(
        &format!("{R}/actions/permissions"),
        "https://elsewhere.example/steal",
    );
    let reading = read(&repo(), &fine(), &mut fake);
    assert!(
        fake.asked.iter().all(|(url, _)| !url.contains("elsewhere")),
        "{:?}",
        fake.asked
    );
    assert!(
        reading
            .unread
            .iter()
            .any(|u| u.contains("elsewhere.example") && u.contains("not followed")),
        "{:?}",
        reading.unread
    );
    assert!(reading.settings.allowed_actions.is_none());
}

#[test]
fn a_redirect_on_the_same_host_is_followed_and_listed() {
    let mut fake = whole_repository()
        .redirect(
            &format!("{R}/actions/permissions"),
            "https://api.github.com/repositories/9/actions/permissions",
        )
        .ok(
            "/repositories/9/actions/permissions",
            r#"{"enabled":true,"allowed_actions":"selected","sha_pinning_required":true}"#,
        );
    let reading = read(&repo(), &fine(), &mut fake);
    assert_eq!(reading.settings.sha_pinning_required, Some(true));
    assert!(
        reading
            .requests
            .iter()
            .any(|r| r.path == "/repositories/9/actions/permissions")
    );
    assert!(reading.requests.iter().any(|r| r.status == 301));
}

#[test]
fn a_refused_endpoint_is_unread_with_what_it_needs() {
    let mut fake = whole_repository().answer(
        &format!("{R}/actions/permissions/workflow"),
        403,
        &[("X-Accepted-GitHub-Permissions", "administration=read")],
        r#"{"message":"Resource not accessible"}"#,
    );
    let reading = read(&repo(), &fine(), &mut fake);
    assert!(reading.settings.default_workflow_permissions.is_none());
    assert!(
        reading
            .unread
            .iter()
            .any(|u| u.contains("403") && u.contains("administration=read")),
        "{:?}",
        reading.unread
    );
}

#[test]
fn hidden_settings_are_unread_never_assumed() {
    let mut fake = whole_repository()
        .ok(R, r#"{"default_branch":"main"}"#)
        .ok(&format!("{R}/rulesets/1"), r#"{"name":"Main","target":"branch","enforcement":"active","rules":[{"type":"pull_request"}]}"#);
    let reading = read(&repo(), &fine(), &mut fake);
    assert_eq!(reading.settings.secret_scanning, None);
    assert!(reading.unread.iter().any(|u| u.contains("Secret scanning")));
    assert!(
        reading.settings.rulesets.as_ref().unwrap()[0]
            .bypass_actors
            .is_none()
    );
}

#[test]
fn the_number_of_requests_is_limited() {
    let many: String = format!(
        "[{}]",
        (1..=200)
            .map(|i| format!("{{\"id\":{i}}}"))
            .collect::<Vec<_>>()
            .join(",")
    );
    let mut fake = whole_repository().ok(
        &format!("{R}/rulesets?includes_parents=true&per_page=30"),
        &many,
    );
    let reading = read(&repo(), &fine(), &mut fake);
    assert!(
        fake.asked.len() <= MOST_REQUESTS,
        "{} requests",
        fake.asked.len()
    );
    assert!(
        reading.unread.iter().any(|u| u.contains("after the first")),
        "{:?}",
        reading.unread
    );
}

#[test]
fn a_token_github_does_not_accept_stops_the_reading() {
    let mut fake = Fake::default().answer(R, 401, &[], r#"{"message":"Bad credentials"}"#);
    let reading = read(&repo(), &fine(), &mut fake);
    assert_eq!(fake.asked.len(), 1);
    assert!(reading.stopped.as_deref().unwrap().contains("401"));
}

#[test]
fn the_client_stops_at_its_request_limit() {
    // The fixed plan stays well under the limit, so the client is driven past it directly.
    let mut fake = Fake::default().ok("/x", "{}");
    let token = fine();
    let mut client = Client::new(HOST, "", "GitHub", &mut fake, &token);
    let mut refused = None;
    for n in 0..MOST_REQUESTS + 5 {
        if let Err(why) = client.get("/x") {
            refused = Some((n, why));
            break;
        }
    }
    let (n, why) = refused.expect("the client never stopped");
    assert_eq!(n, MOST_REQUESTS, "it stopped after {n} requests");
    assert!(why.contains("limit"), "{why}");
    assert_eq!(
        fake.asked.len(),
        MOST_REQUESTS,
        "a refused request is never sent"
    );
}
