//! What each setting read from a live account counts as (ADR-157, decision 6, and its Later entry of 21 November 2026;
//! backlog 0717). A live reading is the provider's account of its own settings, so a setting that is safe is a credit
//! *in part*, for its value only, never for the behavior it causes; one that is unsafe is a finding; one that could not
//! be read is *not assessed*, never a pass. Each requirement cited was read in `data/frameworks/` first, and a setting
//! no requirement speaks to is reported with none.

use crate::github::{ReadOnly, Reading};
use cv_rules::outcome::{Evidence, NotAssessed, Rule};
use sv_core::{Confidence, Finding, Location, Severity, Verified};

pub const DEFAULT_BRANCH_REVIEW: Rule = Rule {
    id: "cv.github-default-branch-review",
    setting: "a pull request rule on the default branch",
    // AISVS Appendix C, AC.8.3: agents cannot bypass branch protection or required reviews. In part: the setting that
    // asks for a pull request, not the alert AC.8.3 also asks for.
    requirement_ids: &["AC.8.3"],
    cwe: &[],
    severity: Severity::Medium,
    title: "The default branch takes changes without a pull request",
    impact: "Anyone who can push, a person or an AI coding agent, can change the default branch directly, with no \
             review and no required checks in the way.",
    fix: "Add a ruleset (Settings, Rules, Rulesets) that targets the default branch and requires a pull request \
          before merging, with the checks that must pass.",
    scope: "The rules on the default branch of",
};

pub const WORKFLOW_APPROVES: Rule = Rule {
    id: "cv.github-workflow-approves-pull-requests",
    setting: "can_approve_pull_request_reviews",
    // AC.8.4: each stage performed by a distinct principal; AC.4.1: the agent does not count as the human reviewer. In
    // part: it keeps a workflow's own token from approving; it cannot show who the reviewer is.
    requirement_ids: &["AC.8.4", "AC.4.1"],
    cwe: &[],
    severity: Severity::Medium,
    title: "Workflows may approve pull requests",
    impact: "A workflow's own token can approve a pull request, so a change can be written and approved by \
             automation with no person in between.",
    fix: "Turn off \"Allow GitHub Actions to create and approve pull requests\" (Settings, Actions, General, Workflow \
          permissions).",
    scope: "Whether a workflow's token may approve pull requests in",
};

pub const WORKFLOW_TOKEN_WRITES: Rule = Rule {
    id: "cv.github-workflow-token-writes",
    setting: "default_workflow_permissions",
    // No requirement read speaks to it directly (ADR-157, Later, 21 November 2026).
    requirement_ids: &[],
    cwe: &["CWE-250"],
    severity: Severity::Low,
    title: "Workflows get a token that can write by default",
    impact: "Every workflow that does not set `permissions:` runs with a token that can write to the repository, so \
             a step that is tricked into running someone else's code can push with it.",
    fix: "Set the default to read (Settings, Actions, General, Workflow permissions, \"Read repository contents and \
          packages permissions\"), and give write access only in the workflows that need it.",
    scope: "The default permissions of a workflow's token in",
};

pub const ACTIONS_ANYWHERE: Rule = Rule {
    id: "cv.github-actions-from-anywhere",
    setting: "allowed_actions",
    // V15.2.4 (level 3): third-party components come from the expected repository. In part: where actions may come
    // from, not the dependency-confusion half.
    requirement_ids: &["V15.2.4"],
    cwe: &["CWE-829"],
    severity: Severity::Low,
    title: "Any action from anywhere may run",
    impact: "A workflow can use any action published by anyone, so one edited workflow, or one that names the wrong \
             action, runs code nobody here chose.",
    fix: "Allow only actions from this repository's owner, GitHub, and the verified creators you use, or a named list \
          (Settings, Actions, General, Actions permissions).",
    scope: "Which actions GitHub lets workflows use in",
};

pub const ACTIONS_UNPINNED: Rule = Rule {
    id: "cv.github-actions-pinning-not-required",
    setting: "sha_pinning_required",
    requirement_ids: &["V15.2.4"],
    cwe: &["CWE-829"],
    severity: Severity::Low,
    title: "Actions need not be pinned to a commit",
    impact: "A workflow may name an action by a tag, which its owner can move to other code at any time; GitHub does \
             not refuse it.",
    fix: "Require actions to be pinned to a full commit SHA (Settings, Actions, General, Actions permissions).",
    scope: "Whether GitHub requires actions pinned to a commit in",
};

pub const SECRET_SCANNING: Rule = Rule {
    id: "cv.github-secret-scanning-off",
    setting: "secret_scanning",
    // V13.3.1: secrets not included in source code; AC.4.2: secret scanning among the automated testing. In part:
    // known secret patterns only, and AC.4.2's other kinds of testing are not read.
    requirement_ids: &["V13.3.1", "AC.4.2"],
    cwe: &[],
    severity: Severity::Medium,
    title: "Secret scanning is off",
    impact: "A key committed to the repository is not found by GitHub, so it can sit there, readable, until someone \
             else finds it.",
    fix: "Turn on secret scanning (Settings, Advanced Security).",
    scope: "Whether GitHub scans for committed secrets in",
};

pub const PUSH_PROTECTION: Rule = Rule {
    id: "cv.github-push-protection-off",
    setting: "secret_scanning_push_protection",
    requirement_ids: &["V13.3.1", "AC.4.2"],
    cwe: &[],
    severity: Severity::Medium,
    title: "Push protection is off",
    impact: "A push that holds a known kind of key is accepted, so the key is in the history before anyone sees it.",
    fix: "Turn on push protection (Settings, Advanced Security).",
    scope: "Whether GitHub refuses a push that holds a known kind of secret, in",
};

pub const ENVIRONMENT_UNREVIEWED: Rule = Rule {
    id: "cv.github-environment-unreviewed",
    setting: "required_reviewers",
    // AC.7.2: human review and approval before running beyond a sandbox. In part: an Environment holds only for a job
    // that names it, and this reads the setting, not the workflows.
    requirement_ids: &["AC.7.2"],
    cwe: &[],
    severity: Severity::Low,
    title: "An environment deploys without anyone's approval",
    impact: "A job that uses this environment runs, with its secrets, as soon as it is started: no person approves it.",
    fix: "Add required reviewers to the environment (Settings, Environments), and limit which branches or tags may \
          deploy to it.",
    scope: "The protection of",
};

pub const FORK_APPROVAL: Rule = Rule {
    id: "cv.github-fork-approval-weakest",
    setting: "approval_policy",
    // No requirement read speaks to it directly (ADR-157, Later, 21 November 2026).
    requirement_ids: &[],
    cwe: &[],
    severity: Severity::Low,
    title: "Forks' workflows need approval only from accounts new to GitHub",
    impact: "Someone who has used GitHub before can open a pull request from a fork and have its workflows run here \
             with no one approving them first.",
    fix: "Require approval for first-time contributors, or for all outside contributors (Settings, Actions, \
          General, Fork pull request workflows).",
    scope: "When forks' workflows wait for approval in",
};

/// Every rule this module can report, for the tests and the listing.
pub const RULES: &[Rule] = &[
    DEFAULT_BRANCH_REVIEW,
    WORKFLOW_APPROVES,
    WORKFLOW_TOKEN_WRITES,
    ACTIONS_ANYWHERE,
    ACTIONS_UNPINNED,
    SECRET_SCANNING,
    PUSH_PROTECTION,
    ENVIRONMENT_UNREVIEWED,
    FORK_APPROVAL,
];

fn place(reading: &Reading, area: &str) -> Location {
    Location {
        file: format!("github.com/{} settings: {area}", reading.repository),
        line: 1,
    }
}

fn found(rule: &Rule, location: Location, description: String) -> Finding {
    sv_core::finding::found(Finding {
        rule_id: rule.id.to_owned(),
        title: rule.title.to_owned(),
        severity: rule.severity,
        // GitHub's own account of its setting, read just now.
        confidence: Confidence::High,
        location,
        secret: None,
        requirement_ids: rule
            .requirement_ids
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        cwe: rule.cwe.iter().map(|s| (*s).to_owned()).collect(),
        description,
        impact: rule.impact.to_owned(),
        fix: rule.fix.to_owned(),
        also_reported_by: Vec::new(),
        evidence: Vec::new(),
        fingerprint: String::new(),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
        outranked: None,
    })
}

fn credit(rule: &Rule, what: String) -> Verified {
    let credit = Verified::new(rule.id, rule.requirement_ids, what);
    // A live reading is the provider's account of a setting's value: never more than in part (ADR-157, decision 6).
    credit.in_part()
}

fn unknown(rule: &Rule, unit: &str, why: &str) -> NotAssessed {
    NotAssessed {
        rule_id: rule.id.to_owned(),
        requirement_ids: rule
            .requirement_ids
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        unit: unit.to_owned(),
        why: why.to_owned(),
    }
}

/// Each setting of `reading`, settled. A reading that stopped settles nothing: everything is not assessed.
pub fn judge(reading: &Reading) -> Evidence {
    let mut out = Evidence::default();
    let repo = reading.repository.as_str();
    if let Some(why) = &reading.stopped {
        for rule in RULES {
            out.not_assessed.push(unknown(rule, repo, why));
        }
        return out;
    }
    let s = &reading.settings;
    let said = |rule: &Rule| format!("{} {repo}, as GitHub reports it", rule.scope);

    // The default branch: a pull request rule from a ruleset, or classic protection that asks for reviews.
    let rule = &DEFAULT_BRANCH_REVIEW;
    let by_rules = s
        .branch_rules
        .as_ref()
        .map(|k| k.iter().any(|k| k == "pull_request"));
    match (by_rules, s.classic_reviews) {
        (Some(true), _) | (_, Some(true)) => {
            let bypass = bypass_words(reading);
            out.verified.push(credit(
                rule,
                format!("{}: a pull request is required{bypass}", said(rule)),
            ));
            if bypass.contains("not shown") {
                out.not_assessed.push(unknown(
                    rule,
                    repo,
                    "who may bypass the default branch's rulesets: GitHub did not show it to this token",
                ));
            }
        }
        (Some(false), Some(false)) => out.findings.push(found(
            rule,
            place(reading, "rules"),
            format!(
                "No ruleset requires a pull request on {}, and it has no classic protection that asks for reviews.",
                s.default_branch.as_deref().unwrap_or("the default branch")
            ),
        )),
        _ => out.not_assessed.push(unknown(rule, repo, "the default branch's rules could not all be read")),
    }

    one(
        &mut out,
        reading,
        &WORKFLOW_APPROVES,
        "Actions",
        s.can_approve_pull_request_reviews.map(|v| !v),
        |v| {
            if v {
                "workflows may not approve pull requests".into()
            } else {
                "workflows may approve pull requests".into()
            }
        },
    );
    one(
        &mut out,
        reading,
        &WORKFLOW_TOKEN_WRITES,
        "Actions",
        s.default_workflow_permissions
            .as_deref()
            .map(|p| p == "read"),
        |v| {
            if v {
                "the default is read".into()
            } else {
                "the default is write".into()
            }
        },
    );
    let anywhere = s.allowed_actions.as_deref().map(|a| a != "all");
    one(
        &mut out,
        reading,
        &ACTIONS_ANYWHERE,
        "Actions",
        anywhere,
        |_| {
            format!(
                "`allowed_actions` is `{}`",
                s.allowed_actions.as_deref().unwrap_or("")
            )
        },
    );
    one(
        &mut out,
        reading,
        &ACTIONS_UNPINNED,
        "Actions",
        s.sha_pinning_required,
        |v| {
            if v {
                "pinning to a full commit SHA is required".into()
            } else {
                "pinning is not required".into()
            }
        },
    );
    one(
        &mut out,
        reading,
        &SECRET_SCANNING,
        "Advanced Security",
        s.secret_scanning,
        |v| {
            if v {
                "it is on".into()
            } else {
                "it is off".into()
            }
        },
    );
    one(
        &mut out,
        reading,
        &PUSH_PROTECTION,
        "Advanced Security",
        s.push_protection,
        |v| {
            if v {
                "it is on".into()
            } else {
                "it is off".into()
            }
        },
    );
    let weakest = s
        .fork_approval
        .as_deref()
        .map(|p| p != "first_time_contributors_new_to_github");
    one(
        &mut out,
        reading,
        &FORK_APPROVAL,
        "Actions",
        weakest,
        |_| {
            format!(
                "`approval_policy` is `{}`",
                s.fork_approval.as_deref().unwrap_or("")
            )
        },
    );

    // Each environment: reviewers, and which branches or tags may deploy.
    let rule = &ENVIRONMENT_UNREVIEWED;
    match &s.environments {
        Some(environments) if environments.is_empty() => {}
        Some(environments) => {
            for env in environments {
                let unit = format!("the environment {} of {repo}", env.name);
                let from = format!("it deploys from {}", env.deploys_from);
                if env.reviewers > 0 {
                    let own = match env.prevent_self_review {
                        Some(true) => ", and a reviewer may not approve their own run",
                        Some(false) => ", and a reviewer may approve their own run",
                        None => "",
                    };
                    out.verified.push(credit(
                        rule,
                        format!(
                            "{} {unit}, as GitHub reports it: {} required reviewer(s){own}; {from}",
                            rule.scope, env.reviewers
                        ),
                    ));
                } else {
                    out.findings.push(found(
                        rule,
                        place(reading, &format!("environment {}", env.name)),
                        format!(
                            "The environment {} has no required reviewers; {from}.",
                            env.name
                        ),
                    ));
                }
            }
        }
        None => out
            .not_assessed
            .push(unknown(rule, repo, "the environments could not be read")),
    }
    out
}

/// One setting with a safe or an unsafe value, or none read.
fn one(
    out: &mut Evidence,
    reading: &Reading,
    rule: &Rule,
    area: &str,
    safe: Option<bool>,
    words: impl Fn(bool) -> String,
) {
    let repo = reading.repository.as_str();
    match safe {
        Some(true) => out.verified.push(credit(
            rule,
            format!(
                "{} {repo}, as GitHub reports it: {}",
                rule.scope,
                words(true)
            ),
        )),
        Some(false) => out.findings.push(found(
            rule,
            place(reading, area),
            format!("GitHub reports that, in {repo}, {}.", words(false)),
        )),
        None => out.not_assessed.push(unknown(
            rule,
            repo,
            &format!("`{}` was not read", rule.setting),
        )),
    }
}

/// Who may bypass the rulesets that reach the default branch, in words for a credit's scope.
fn bypass_words(reading: &Reading) -> String {
    let Some(rulesets) = &reading.settings.rulesets else {
        return String::new();
    };
    let branch: Vec<_> = rulesets
        .iter()
        .filter(|r| r.target == "branch" && r.enforcement == "active")
        .collect();
    if branch.iter().any(|r| r.bypass_actors.is_none()) {
        return "; who may bypass it was not shown to this token".to_owned();
    }
    let open: Vec<&str> = branch
        .iter()
        .filter(|r| r.bypass_actors.as_ref().is_some_and(|b| !b.is_empty()))
        .map(|r| r.name.as_str())
        .collect();
    if open.is_empty() {
        "; no one may bypass the branch rulesets".to_owned()
    } else {
        format!("; someone may bypass {}", open.join(", "))
    }
}

/// The words a report puts before any finding about the token (ADR-157, decision 5).
pub fn token_words(reading: &Reading) -> String {
    match &reading.token.read_only {
        ReadOnly::Confirmed { scopes } => format!(
            "The token's scopes, as GitHub names them, cannot write: {}.",
            if scopes.is_empty() {
                "none".to_owned()
            } else {
                scopes.join(", ")
            }
        ),
        ReadOnly::Refused { writing, .. } => format!(
            "The token can write ({}), so cv refused it and read nothing more.",
            writing.join(", ")
        ),
        ReadOnly::CouldNotConfirm { why } => format!(
            "cv could not confirm the token is read-only: {why}. Use a token with read-only access only."
        ),
    }
}

#[cfg(test)]
#[path = "judge_tests.rs"]
mod judge_tests;
