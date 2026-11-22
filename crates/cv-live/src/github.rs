//! `cv live github <owner>/<repo>`: one repository's settings, read with a token the person gives, through a fixed list
//! of `GET` requests to `api.github.com` (ADR-157, Later, 21 November 2026; backlog 0717).
//!
//! Whether the token can write is settled by its kind. A classic or OAuth token says its scopes on every answer, in
//! `X-OAuth-Scopes`, and no classic scope reads a private repository's settings without writing, so one carrying any
//! scope but a `read:` one is refused after the first request, and nothing more is asked. A fine-grained token, or a
//! GitHub App's, cannot say its permissions, so the report says, before any finding, that ConfigVet could not confirm
//! it is read-only. Nothing here ever sends a write to find out.

use crate::{Answer, MOST_REDIRECTS, MOST_REQUESTS, Sent, Transport};
use serde::Serialize;
use serde_json::Value;
use sv_core::Secret;

/// The one host every request goes to.
pub const HOST: &str = "api.github.com";
/// No more rulesets or environments than this are read one by one.
pub const MOST_EACH: usize = 10;

/// A repository named `owner/name`, each part as GitHub allows it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Repo {
    pub owner: String,
    pub name: String,
}

impl Repo {
    pub fn parse(text: &str) -> Result<Repo, String> {
        let ok = |part: &str| {
            !part.is_empty()
                && part != "."
                && part != ".."
                && part.len() <= 100
                && part
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        };
        match text.split_once('/') {
            Some((owner, name)) if ok(owner) && ok(name) => Ok(Repo {
                owner: owner.to_owned(),
                name: name.to_owned(),
            }),
            _ => Err(format!(
                "{text:?} is not a repository's name; give it as owner/name, such as abbyshade111/StackVet"
            )),
        }
    }

    pub fn full(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }
}

/// What kind of token it is, from how it begins (GitHub's documented prefixes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TokenKind {
    /// `ghp_` (a classic personal access token), `gho_` (an OAuth app's, such as `gh auth token` gives), `ghu_`.
    Classic,
    /// `github_pat_`.
    FineGrained,
    /// `ghs_`: a GitHub App's installation token.
    Installation,
    Unknown,
}

pub fn token_kind(token: &str) -> TokenKind {
    if token.starts_with("github_pat_") {
        TokenKind::FineGrained
    } else if ["ghp_", "gho_", "ghu_"]
        .iter()
        .any(|p| token.starts_with(p))
    {
        TokenKind::Classic
    } else if token.starts_with("ghs_") {
        TokenKind::Installation
    } else {
        TokenKind::Unknown
    }
}

/// Whether the token was shown to be read-only, refused, or could not be told.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "is", rename_all = "kebab-case")]
pub enum ReadOnly {
    /// GitHub named its scopes, and none can write.
    Confirmed { scopes: Vec<String> },
    /// GitHub named its scopes, and these can write: nothing more was read.
    Refused {
        scopes: Vec<String>,
        writing: Vec<String>,
    },
    /// GitHub does not say this kind of token's permissions.
    CouldNotConfirm { why: String },
}

/// The scopes that can write: every one but a `read:` scope. Strict on purpose: `gist` and `user` write too, if not to
/// a repository, and a token for reading settings needs none of them.
pub fn writing_scopes(scopes: &[String]) -> Vec<String> {
    scopes
        .iter()
        .filter(|s| !s.starts_with("read:"))
        .cloned()
        .collect()
}

#[derive(Debug, Clone, Serialize)]
pub struct TokenSaid {
    pub kind: TokenKind,
    /// The token as `Secret::redact` shows a key: its first characters and its length.
    pub token: Secret,
    pub read_only: ReadOnly,
}

/// One ruleset that applies to the repository, as GitHub showed it.
#[derive(Debug, Clone, Serialize)]
pub struct Ruleset {
    pub name: String,
    pub target: String,
    pub enforcement: String,
    /// The rule types it holds, such as `pull_request` and `deletion`.
    pub rules: Vec<String>,
    /// Who may bypass it; `None` when GitHub did not show it to this token.
    pub bypass_actors: Option<Vec<Value>>,
    pub current_user_can_bypass: Option<String>,
}

/// One environment and what protects it.
#[derive(Debug, Clone, Serialize)]
pub struct Environment {
    pub name: String,
    /// How many reviewers must approve a job that uses it.
    pub reviewers: usize,
    pub prevent_self_review: Option<bool>,
    pub wait_minutes: Option<u64>,
    /// Which branches or tags may deploy: "any branch", "protected branches", or the custom names and their kinds.
    pub deploys_from: String,
}

/// The settings as read. A field left `None` was not read; `unread` says why.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Settings {
    pub default_branch: Option<String>,
    pub private: Option<bool>,
    pub secret_scanning: Option<bool>,
    pub push_protection: Option<bool>,
    pub rulesets: Option<Vec<Ruleset>>,
    /// The rule types active on the default branch, from every ruleset that reaches it.
    pub branch_rules: Option<Vec<String>>,
    /// Classic branch protection: `Some(true)` when it asks for pull request reviews, `Some(false)` when the branch
    /// has none or it asks for none.
    pub classic_reviews: Option<bool>,
    pub actions_enabled: Option<bool>,
    pub allowed_actions: Option<String>,
    pub sha_pinning_required: Option<bool>,
    pub selected_actions: Option<Value>,
    pub default_workflow_permissions: Option<String>,
    pub can_approve_pull_request_reviews: Option<bool>,
    pub fork_approval: Option<String>,
    pub environments: Option<Vec<Environment>>,
}

/// A whole reading: the token, every request, the settings, and what could not be read.
#[derive(Debug, Clone, Serialize)]
pub struct Reading {
    pub provider: &'static str,
    pub repository: String,
    pub token: TokenSaid,
    pub requests: Vec<Sent>,
    pub settings: Settings,
    /// Each setting that could not be read, and why, in a person's words.
    pub unread: Vec<String>,
    /// Why the reading stopped before the end, when it did: nothing after it was asked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped: Option<String>,
}

/// A path segment with everything but GitHub's plain characters escaped, so a branch or environment name cannot reach
/// another endpoint.
pub fn segment(text: &str) -> String {
    let mut out = String::new();
    for b in text.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

struct Client<'t> {
    transport: &'t mut dyn Transport,
    token: &'t str,
    requests: Vec<Sent>,
}

enum Got {
    Json(Value, Answer),
    Status(u16, Answer),
}

impl Client<'_> {
    /// A `GET` of `path` on `api.github.com`, following a redirect only when it stays on that host.
    fn get(&mut self, path: &str) -> Result<Got, String> {
        let mut url = format!("https://{HOST}{path}");
        for _ in 0..=MOST_REDIRECTS {
            if self.requests.len() >= MOST_REQUESTS {
                return Err(format!(
                    "cv stopped at its limit of {MOST_REQUESTS} requests"
                ));
            }
            let shown = url
                .trim_start_matches(&format!("https://{HOST}"))
                .to_owned();
            let answer = match self.transport.get(&url, self.token) {
                Ok(answer) => answer,
                Err(why) => {
                    self.requests.push(Sent {
                        method: "GET",
                        host: HOST.to_owned(),
                        path: shown,
                        status: 0,
                    });
                    return Err(why);
                }
            };
            self.requests.push(Sent {
                method: "GET",
                host: HOST.to_owned(),
                path: shown,
                status: answer.status,
            });
            if (300..400).contains(&answer.status) {
                match &answer.redirect {
                    Some(next) if next.starts_with(&format!("https://{HOST}/")) => {
                        url = next.clone();
                        continue;
                    }
                    Some(next) => {
                        return Err(format!(
                            "GitHub redirected to {next}, which is not {HOST}, so it was not followed"
                        ));
                    }
                    None => return Ok(Got::Status(answer.status, answer)),
                }
            }
            if answer.status == 200 {
                return match serde_json::from_slice::<Value>(&answer.body) {
                    Ok(value) => Ok(Got::Json(value, answer)),
                    Err(_) => Err(format!("GitHub's answer to {path} was not JSON")),
                };
            }
            return Ok(Got::Status(answer.status, answer));
        }
        Err(format!(
            "GitHub redirected {path} more than {MOST_REDIRECTS} times"
        ))
    }
}

/// What a refused or missing answer means, in a person's words, with the permission GitHub says the endpoint needs.
fn unanswered(what: &str, status: u16, answer: &Answer) -> String {
    let needs = answer
        .header("x-accepted-github-permissions")
        .map(|p| format!("; GitHub says this needs `{p}`"))
        .unwrap_or_default();
    format!("{what}: GitHub answered {status}{needs}")
}

/// Reads `repo`'s settings with `token`, through `transport`.
pub fn read(repo: &Repo, token: &str, transport: &mut dyn Transport) -> Reading {
    let kind = token_kind(token);
    let mut reading = Reading {
        provider: "github",
        repository: repo.full(),
        token: TokenSaid {
            kind,
            token: Secret::redact(token),
            read_only: ReadOnly::CouldNotConfirm {
                why: "no answer came from GitHub".to_owned(),
            },
        },
        requests: Vec::new(),
        settings: Settings::default(),
        unread: Vec::new(),
        stopped: None,
    };
    let mut client = Client {
        transport,
        token,
        requests: Vec::new(),
    };
    let base = format!("/repos/{}/{}", segment(&repo.owner), segment(&repo.name));
    read_into(&mut client, &base, &mut reading);
    reading.requests = client.requests;
    reading
}

fn read_into(client: &mut Client, base: &str, reading: &mut Reading) {
    // 1. The repository, which also says what the token is.
    let (repository, answer) = match client.get(base) {
        Ok(Got::Json(value, answer)) => (value, answer),
        Ok(Got::Status(401, _)) => {
            reading.stopped = Some("GitHub did not accept the token (401)".to_owned());
            return;
        }
        Ok(Got::Status(status, answer)) => {
            reading.stopped = Some(unanswered(
                "The repository could not be read, or this token cannot see it",
                status,
                &answer,
            ));
            return;
        }
        Err(why) => {
            reading.stopped = Some(why);
            return;
        }
    };
    reading.token.read_only = match answer.header("x-oauth-scopes") {
        Some(listed) => {
            let scopes: Vec<String> = listed
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_owned)
                .collect();
            let writing = writing_scopes(&scopes);
            if writing.is_empty() {
                ReadOnly::Confirmed { scopes }
            } else {
                ReadOnly::Refused { scopes, writing }
            }
        }
        None => ReadOnly::CouldNotConfirm {
            why: match reading.token.kind {
                TokenKind::FineGrained => {
                    "a fine-grained token does not say its permissions to GitHub's API"
                }
                TokenKind::Installation => {
                    "a GitHub App's token says its permissions only when it is made"
                }
                TokenKind::Classic | TokenKind::Unknown => {
                    "GitHub's answer named no scopes for this token"
                }
            }
            .to_owned(),
        },
    };
    if let ReadOnly::Refused { writing, .. } = &reading.token.read_only {
        reading.stopped = Some(format!(
            "The token can write ({}), so cv read nothing more. Make a fine-grained token with read-only access to \
             this repository's Administration and Actions instead.",
            writing.join(", ")
        ));
        return;
    }
    let s = &mut reading.settings;
    s.default_branch = repository["default_branch"].as_str().map(str::to_owned);
    s.private = repository["private"].as_bool();
    let security = &repository["security_and_analysis"];
    let enabled = |v: &Value| v["status"].as_str().map(|status| status == "enabled");
    if security.is_object() {
        s.secret_scanning = enabled(&security["secret_scanning"]);
        s.push_protection = enabled(&security["secret_scanning_push_protection"]);
    }
    if s.secret_scanning.is_none() {
        reading.unread.push(
            "Secret scanning and push protection: GitHub shows them only to the repository's administrators"
                .to_owned(),
        );
    }

    // 2. The rulesets, each read whole for who may bypass it.
    match client.get(&format!(
        "{base}/rulesets?includes_parents=true&per_page=30"
    )) {
        Ok(Got::Json(Value::Array(list), _)) => {
            let mut rulesets = Vec::new();
            for item in list.iter().take(MOST_EACH) {
                let Some(id) = item["id"].as_u64() else {
                    continue;
                };
                match client.get(&format!("{base}/rulesets/{id}")) {
                    Ok(Got::Json(whole, _)) => rulesets.push(Ruleset {
                        name: whole["name"].as_str().unwrap_or("").to_owned(),
                        target: whole["target"].as_str().unwrap_or("").to_owned(),
                        enforcement: whole["enforcement"].as_str().unwrap_or("").to_owned(),
                        rules: whole["rules"]
                            .as_array()
                            .into_iter()
                            .flatten()
                            .filter_map(|r| r["type"].as_str().map(str::to_owned))
                            .collect(),
                        bypass_actors: whole["bypass_actors"].as_array().cloned(),
                        current_user_can_bypass: whole["current_user_can_bypass"]
                            .as_str()
                            .map(str::to_owned),
                    }),
                    Ok(Got::Status(status, answer)) => {
                        reading
                            .unread
                            .push(unanswered(&format!("Ruleset {id}"), status, &answer))
                    }
                    Err(why) => reading.unread.push(format!("Ruleset {id}: {why}")),
                }
            }
            if list.len() > MOST_EACH {
                reading.unread.push(format!(
                    "Rulesets after the first {MOST_EACH}: not read, of {}",
                    list.len()
                ));
            }
            s.rulesets = Some(rulesets);
        }
        Ok(Got::Json(..)) => reading
            .unread
            .push("Rulesets: GitHub's answer was not a list".to_owned()),
        Ok(Got::Status(status, answer)) => {
            reading.unread.push(unanswered("Rulesets", status, &answer))
        }
        Err(why) => reading.unread.push(format!("Rulesets: {why}")),
    }

    // 3. The rules in force on the default branch, and classic protection where they ask for no pull request.
    if let Some(branch) = s.default_branch.clone() {
        let mut asks_pull_request = false;
        match client.get(&format!("{base}/rules/branches/{}", segment(&branch))) {
            Ok(Got::Json(Value::Array(list), _)) => {
                let mut kinds: Vec<String> = list
                    .iter()
                    .filter_map(|r| r["type"].as_str().map(str::to_owned))
                    .collect();
                kinds.sort();
                kinds.dedup();
                asks_pull_request = kinds.iter().any(|k| k == "pull_request");
                s.branch_rules = Some(kinds);
            }
            Ok(Got::Json(..)) => reading
                .unread
                .push("The default branch's rules: GitHub's answer was not a list".to_owned()),
            Ok(Got::Status(status, answer)) => {
                reading
                    .unread
                    .push(unanswered("The default branch's rules", status, &answer))
            }
            Err(why) => reading
                .unread
                .push(format!("The default branch's rules: {why}")),
        }
        if !asks_pull_request {
            match client.get(&format!("{base}/branches/{}/protection", segment(&branch))) {
                Ok(Got::Json(protection, _)) => {
                    s.classic_reviews =
                        Some(protection["required_pull_request_reviews"].is_object())
                }
                Ok(Got::Status(404, answer))
                    if String::from_utf8_lossy(&answer.body).contains("Branch not protected") =>
                {
                    s.classic_reviews = Some(false)
                }
                Ok(Got::Status(status, answer)) => {
                    reading
                        .unread
                        .push(unanswered("Classic branch protection", status, &answer))
                }
                Err(why) => reading
                    .unread
                    .push(format!("Classic branch protection: {why}")),
            }
        }
    } else {
        reading
            .unread
            .push("The default branch's rules: GitHub named no default branch".to_owned());
    }

    // 4. Actions: which may run, whether they must be pinned, the workflow token, and forks.
    match client.get(&format!("{base}/actions/permissions")) {
        Ok(Got::Json(actions, _)) => {
            s.actions_enabled = actions["enabled"].as_bool();
            s.allowed_actions = actions["allowed_actions"].as_str().map(str::to_owned);
            s.sha_pinning_required = actions["sha_pinning_required"].as_bool();
            if s.allowed_actions.as_deref() == Some("selected") {
                match client.get(&format!("{base}/actions/permissions/selected-actions")) {
                    Ok(Got::Json(selected, _)) => s.selected_actions = Some(selected),
                    Ok(Got::Status(status, answer)) => {
                        reading
                            .unread
                            .push(unanswered("The actions allowed", status, &answer))
                    }
                    Err(why) => reading.unread.push(format!("The actions allowed: {why}")),
                }
            }
        }
        Ok(Got::Status(status, answer)) => {
            reading
                .unread
                .push(unanswered("Which actions may run", status, &answer))
        }
        Err(why) => reading.unread.push(format!("Which actions may run: {why}")),
    }
    match client.get(&format!("{base}/actions/permissions/workflow")) {
        Ok(Got::Json(workflow, _)) => {
            s.default_workflow_permissions = workflow["default_workflow_permissions"]
                .as_str()
                .map(str::to_owned);
            s.can_approve_pull_request_reviews =
                workflow["can_approve_pull_request_reviews"].as_bool();
        }
        Ok(Got::Status(status, answer)) => reading.unread.push(unanswered(
            "The workflow token's permissions",
            status,
            &answer,
        )),
        Err(why) => reading
            .unread
            .push(format!("The workflow token's permissions: {why}")),
    }
    match client.get(&format!(
        "{base}/actions/permissions/fork-pr-contributor-approval"
    )) {
        Ok(Got::Json(fork, _)) => {
            s.fork_approval = fork["approval_policy"].as_str().map(str::to_owned)
        }
        Ok(Got::Status(status, answer)) => {
            reading
                .unread
                .push(unanswered("Approval of forks' workflows", status, &answer))
        }
        Err(why) => reading
            .unread
            .push(format!("Approval of forks' workflows: {why}")),
    }

    // 5. The environments, and which branches or tags may deploy to each.
    match client.get(&format!("{base}/environments?per_page=30")) {
        Ok(Got::Json(listed, _)) => {
            let all = listed["environments"]
                .as_array()
                .cloned()
                .unwrap_or_default();
            let mut environments = Vec::new();
            for env in all.iter().take(MOST_EACH) {
                let name = env["name"].as_str().unwrap_or("").to_owned();
                let rules = env["protection_rules"]
                    .as_array()
                    .cloned()
                    .unwrap_or_default();
                let reviewers_rule = rules.iter().find(|r| r["type"] == "required_reviewers");
                let wait = rules
                    .iter()
                    .find(|r| r["type"] == "wait_timer")
                    .and_then(|r| r["wait_timer"].as_u64());
                let policy = &env["deployment_branch_policy"];
                let deploys_from = if policy.is_null() {
                    "any branch".to_owned()
                } else if policy["protected_branches"] == true {
                    "protected branches".to_owned()
                } else if policy["custom_branch_policies"] == true {
                    match client.get(&format!(
                        "{base}/environments/{}/deployment-branch-policies",
                        segment(&name)
                    )) {
                        Ok(Got::Json(policies, _)) => {
                            let names: Vec<String> = policies["branch_policies"]
                                .as_array()
                                .into_iter()
                                .flatten()
                                .map(|p| {
                                    format!(
                                        "{} ({})",
                                        p["name"].as_str().unwrap_or(""),
                                        p["type"].as_str().unwrap_or("branch")
                                    )
                                })
                                .collect();
                            if names.is_empty() {
                                "no branch or tag".to_owned()
                            } else {
                                names.join(", ")
                            }
                        }
                        Ok(Got::Status(status, answer)) => {
                            reading.unread.push(unanswered(
                                &format!("Environment {name}'s deployment branches"),
                                status,
                                &answer,
                            ));
                            "not read".to_owned()
                        }
                        Err(why) => {
                            reading
                                .unread
                                .push(format!("Environment {name}'s deployment branches: {why}"));
                            "not read".to_owned()
                        }
                    }
                } else {
                    "any branch".to_owned()
                };
                environments.push(Environment {
                    name,
                    reviewers: reviewers_rule
                        .and_then(|r| r["reviewers"].as_array())
                        .map_or(0, Vec::len),
                    prevent_self_review: reviewers_rule
                        .and_then(|r| r["prevent_self_review"].as_bool()),
                    wait_minutes: wait,
                    deploys_from,
                });
            }
            if all.len() > MOST_EACH {
                reading.unread.push(format!(
                    "Environments after the first {MOST_EACH}: not read, of {}",
                    all.len()
                ));
            }
            s.environments = Some(environments);
        }
        Ok(Got::Status(status, answer)) => {
            reading
                .unread
                .push(unanswered("The environments", status, &answer))
        }
        Err(why) => reading.unread.push(format!("The environments: {why}")),
    }
}

#[cfg(test)]
#[path = "github_tests.rs"]
pub(crate) mod github_tests;
