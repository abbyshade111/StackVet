//! The Docker Hub token is off the runner's disk before a branch's own code runs (backlog 0287). In the
//! jobs that run a pull request's tests, build scripts, and image (`suite`, `image`), the step that
//! signs in fetches what it needs and signs out again, so `docker login` never leaves the token in
//! `~/.docker/config.json` while `cargo test`, `cargo build`, or `tools/image_smoke.py` run as the same
//! user. The `publish` job, which runs on `main` alone after a merge, is not held to this.

use std::path::Path;

/// The text of each step of job `job` in a workflow, split at its `- name:` lines.
fn steps_of(text: &str, job: &str) -> Vec<String> {
    let mut in_jobs = false;
    let mut in_job = false;
    let mut steps: Vec<String> = Vec::new();
    for line in text.lines() {
        if line == "jobs:" {
            in_jobs = true;
            continue;
        }
        if !in_jobs {
            continue;
        }
        // A job's own key, two spaces in.
        if line.starts_with("  ") && !line.starts_with("   ") && line.trim_end().ends_with(':') {
            in_job = line.trim() == format!("{job}:");
            continue;
        }
        if !in_job {
            continue;
        }
        if line.trim_start().starts_with("- name:") {
            steps.push(String::new());
        }
        if let Some(step) = steps.last_mut() {
            step.push_str(line);
            step.push('\n');
        }
    }
    steps
}

/// The steps of `job` that sign in to Docker Hub and do not sign out again before they end.
fn left_signed_in(text: &str, job: &str) -> Vec<String> {
    steps_of(text, job)
        .into_iter()
        .filter(|s| {
            let login = s.find("docker login -u");
            let logout = s.rfind("docker logout");
            login.is_some_and(|at| logout.is_none_or(|out| out < at))
        })
        .map(|s| s.lines().next().unwrap_or("").trim().to_owned())
        .collect()
}

#[test]
fn the_reader_sees_a_sign_in_left_open() {
    // The setup: the reader finds a sign-in step with no sign-out, and passes one that has it.
    let workflow = "jobs:\n  suite:\n    steps:\n      - name: In\n        run: |\n          docker login -u a --password-stdin\n      \
                    - name: Tests\n        run: cargo test\n  image:\n    steps:\n      - name: In and out\n        run: |\n          \
                    docker login -u a --password-stdin\n          docker pull x\n          docker logout\n";
    assert_eq!(left_signed_in(workflow, "suite"), ["- name: In"]);
    assert!(left_signed_in(workflow, "image").is_empty());
    assert_eq!(steps_of(workflow, "suite").len(), 2);
}

#[test]
fn the_jobs_that_run_a_branchs_code_sign_out_in_the_step_that_signs_in() {
    let text = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.github/workflows/rust.yml"),
    )
    .unwrap()
    .replace("\r\n", "\n");
    for job in ["suite", "image"] {
        // The setup: the job and its sign-in are where the test looks.
        let steps = steps_of(&text, job);
        assert!(
            steps.iter().any(|s| s.contains("docker login -u")),
            "the setup: no Docker Hub sign-in found in the {job} job"
        );
        let open = left_signed_in(&text, job);
        assert!(
            open.is_empty(),
            "the {job} job leaves the token on disk after: {open:?}"
        );
    }
}
