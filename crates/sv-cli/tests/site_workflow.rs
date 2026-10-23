//! `.github/workflows/site.yml`, which publishes stackvet.dev to Cloudflare (backlog 0376; ADR-132, Later). Nothing
//! publishes the site but the owner starting it: not a push, not a schedule, and not a pull request, whose check job
//! sees no secret. A workflow that published on its own, or handed the token to the job a pull request runs, fails
//! here.

use std::path::{Path, PathBuf};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn workflow() -> String {
    std::fs::read_to_string(repo().join(".github/workflows/site.yml"))
        .expect("the setup: site.yml is there")
}

/// The text of one job: its `  name:` line and the lines after it, up to the next line indented by two spaces only.
fn job<'a>(text: &'a str, name: &str) -> &'a str {
    let start = text
        .find(&format!("\n  {name}:\n"))
        .unwrap_or_else(|| panic!("no job {name}"))
        + 1;
    let body = start + name.len() + 4;
    let end = text[body..]
        .match_indices('\n')
        .map(|(i, _)| body + i + 1)
        .find(|&i| text[i..].starts_with("  ") && !text[i..].starts_with("   "))
        .unwrap_or(text.len());
    &text[start..end]
}

#[test]
fn only_the_owner_starting_it_publishes() {
    let text = workflow();
    let on =
        &text[text.find("\non:\n").expect("an on: block")..text.find("\npermissions:").unwrap()];
    assert!(on.contains("workflow_dispatch:"), "{on}");
    for publishes_on_its_own in [
        "push:",
        "schedule:",
        "pull_request_target:",
        "workflow_run:",
        "release:",
    ] {
        assert!(
            !on.contains(publishes_on_its_own),
            "the site would publish on {publishes_on_its_own}"
        );
    }
    let deploy = job(&text, "deploy");
    assert!(
        deploy.contains("if: github.event_name == 'workflow_dispatch'"),
        "the deploy runs only when started by hand:\n{deploy}"
    );
    assert!(
        deploy.contains("command: deploy --config deploy/stackvet.dev/wrangler.jsonc"),
        "{deploy}"
    );
}

#[test]
fn the_token_reaches_the_deploy_step_alone() {
    let text = workflow();
    let check = job(&text, "check");
    assert!(
        check.contains("--dry-run"),
        "the setup: the check job reads the deploy:\n{check}"
    );
    assert!(
        !check.contains("secrets."),
        "the job a pull request runs sees a secret:\n{check}"
    );
    let deploy = job(&text, "deploy");
    // Only the names of the repository's secrets are read here, as text in the workflow, and the messages name them in
    // words, so nothing that looks like a secret is printed.
    for (reference, what) in [
        (
            "${{ secrets.CLOUDFLARE_API_TOKEN }}",
            "the Cloudflare token",
        ),
        (
            "${{ secrets.CLOUDFLARE_ACCOUNT_ID }}",
            "the Cloudflare account",
        ),
    ] {
        assert_eq!(text.matches(reference).count(), 1, "{what} is used once");
        assert!(
            deploy.contains(reference),
            "{what} is not the deploy step's"
        );
    }
    // Handed through the action's inputs, never written into a script (backlog 0318).
    let handed_in_a_script = deploy
        .lines()
        .filter(|l| l.contains("secrets."))
        .filter(|l| {
            let l = l.trim_start();
            !(l.starts_with("apiToken:") || l.starts_with("accountId:"))
        })
        .count();
    assert_eq!(
        handed_in_a_script, 0,
        "a secret reaches the deploy other than through the action's inputs"
    );
    assert!(
        text.contains("permissions:\n  contents: read\n"),
        "the workflow can only read the repository"
    );
}

#[test]
fn every_step_is_pinned() {
    let text = workflow();
    for line in text
        .lines()
        .filter(|l| l.trim_start().trim_start_matches("- ").starts_with("uses:"))
    {
        let at = line
            .find('@')
            .unwrap_or_else(|| panic!("not pinned: {line}"));
        let pin: String = line[at + 1..]
            .chars()
            .take_while(|c| c.is_ascii_hexdigit())
            .collect();
        assert_eq!(pin.len(), 40, "pinned to a commit fingerprint: {line}");
    }
    // wrangler, which the action and the check install, at one exact version.
    let versions: Vec<&str> = text
        .match_indices("4.148.0")
        .map(|(i, _)| &text[i..i + 7])
        .collect();
    assert_eq!(
        versions.len(),
        2,
        "the check's and the deploy's wrangler: {versions:?}"
    );
    assert!(!text.contains("wrangler@latest") && !text.contains("wranglerVersion: 'latest'"));
}

#[test]
fn the_deploy_serves_the_built_site_at_stackvet_dev_only() {
    let config =
        std::fs::read_to_string(repo().join("deploy/stackvet.dev/wrangler.jsonc")).unwrap();
    assert!(
        config.contains(r#""directory": "../../exports/site""#),
        "{config}"
    );
    assert!(
        config.contains(r#"{ "pattern": "stackvet.dev", "custom_domain": true }"#),
        "{config}"
    );
    assert!(
        config.contains(r#""workers_dev": false"#) && config.contains(r#""preview_urls": false"#)
    );
    assert!(
        !config.contains("\"main\":"),
        "a script would run on Cloudflare:\n{config}"
    );
}
