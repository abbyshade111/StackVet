//! The site's deploy takes the Cloudflare keys from the `site` Environment, which only `main` can deploy to and whose
//! required reviewer is the owner (backlog 0704; ADR-132, Later). Without `environment: site` the job would read
//! the repository's secrets instead, which every workflow can reach. The Environment's own rules are GitHub
//! settings, the owner's, and are not read here.

use std::path::PathBuf;

fn workflow() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.github/workflows/site.yml");
    std::fs::read_to_string(path).expect("the setup: site.yml is there")
}

/// The lines of one job, from its name to the next job or the end.
fn job(text: &str, name: &str) -> String {
    let start = format!("  {name}:");
    let mut lines = text.lines().skip_while(|l| *l != start);
    let first = lines
        .next()
        .unwrap_or_else(|| panic!("the setup: site.yml has a job {name}"));
    let rest =
        lines.take_while(|l| !(l.starts_with("  ") && !l.starts_with("   ") && l.ends_with(':')));
    std::iter::once(first)
        .chain(rest)
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn the_deploy_job_takes_its_keys_from_the_site_environment() {
    let text = workflow();
    let deploy = job(&text, "deploy");
    assert!(
        deploy.contains("secrets.CLOUDFLARE_API_TOKEN"),
        "the setup: the deploy job is the one that uses the Cloudflare token:\n{deploy}"
    );
    assert!(
        deploy.lines().any(|l| l.trim() == "environment: site"),
        "the deploy job names no `site` Environment, so it reads the repository's secrets:\n{deploy}"
    );
    let check = job(&text, "check");
    assert!(
        !check.contains("environment:"),
        "the check job, which a pull request runs, names an Environment:\n{check}"
    );
}
