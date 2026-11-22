//! The weekly check of stackvet.dev (backlog 0702; ADR-132, Later, 22 November 2026): Mondays, the owner's choice; it
//! builds the site with its own test and runs the site's tests; it says what it found in one issue; and it deploys
//! nothing and reads no secret, with a token that reads the code and writes issues only.

use std::path::PathBuf;

fn workflow() -> String {
    std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.github/workflows/site-weekly.yml"),
    )
    .unwrap()
}

#[test]
fn it_runs_on_mondays_and_checks_what_the_deploy_builds() {
    let w = workflow();
    assert!(
        w.contains("- cron: '0 10 * * 1'"),
        "not Mondays at 10:00 UTC, as the owner chose"
    );
    for line in [
        "python3 -I tools/website.py --self-test && python3 -I tools/website.py --out",
        "--test website --test website_fence_words --test website_example_report",
        "--test website_configvet_install",
    ] {
        assert!(w.contains(line), "the weekly check does not run {line:?}");
    }
}

#[test]
fn it_deploys_nothing_and_its_token_writes_issues_only() {
    let w = workflow();
    let permissions =
        &w[w.find("\npermissions:\n")
            .expect("the setup: its permissions")..w.find("\nconcurrency:").unwrap()];
    let granted: Vec<&str> = permissions
        .lines()
        .map(str::trim)
        .filter(|l| !l.starts_with('#') && l.contains(": "))
        .collect();
    assert_eq!(
        granted,
        ["contents: read", "issues: write"],
        "{permissions}"
    );
    for never in [
        "secrets.",
        "wrangler",
        "environment:",
        "cloudflare",
        "persist-credentials: true",
    ] {
        assert!(
            !w.to_lowercase().contains(&never.to_lowercase()),
            "the weekly check names {never:?}"
        );
    }
    assert!(w.contains("persist-credentials: false"));
}

#[test]
fn it_says_so_in_one_issue_and_fails_only_on_a_failure() {
    let w = workflow();
    assert!(w.contains(r#"title="stackvet.dev weekly freshness check""#));
    assert!(
        w.contains("gh issue comment \"$open\"")
            && w.contains("gh issue create --repo \"$GITHUB_REPOSITORY\" --title \"$title\""),
        "it does not comment on the open issue, or open one"
    );
    assert!(
        w.contains("grep 'warning:' \"$RUNNER_TEMP/site.log\""),
        "the build's warnings are not read"
    );
    assert!(
        w.trim_end()
            .ends_with(r#"[ "$SITE" = 0 ] && [ "$TESTS" = 0 ]"#),
        "the run is not red when the build or the tests failed"
    );
}
