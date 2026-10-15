//! Backlog 0544: setup prompt 6 says what `sv doctor` says when Docker is installed and the AI coding tool's shell
//! cannot reach it, in the doctor's own words, so the tool asks to be allowed rather than to install Docker.

use std::path::Path;

/// Prompt 6 as one line of words, as the AI tool reads it. The markers are asserted, so a renamed one fails here.
fn prompt_six() -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let page = std::fs::read_to_string(root.join("docs/prompts/setup.md")).unwrap();
    let start = page
        .find("<!-- setup-step-6:start -->")
        .expect("prompt 6's start marker");
    let end = page
        .find("<!-- setup-step-6:end -->")
        .expect("prompt 6's end marker");
    assert!(start < end, "the markers in order");
    page[start..end]
        .lines()
        .filter(|l| l.starts_with('>'))
        .map(|l| l.trim_start_matches('>').trim())
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn prompt_six_says_what_doctor_says_of_docker_out_of_reach() {
    let doctor = sv_run::CannotRun::NoBackend {
        checked: "`docker info` failed".to_owned(),
        unreachable: Some("/var/run/docker.sock".to_owned()),
    }
    .explain();
    let words = "Docker is installed but this shell cannot reach it";
    assert!(
        doctor.starts_with(words),
        "the doctor's words changed: {doctor}"
    );
    let prompt = prompt_six();
    assert!(
        prompt.contains(words),
        "prompt 6 does not say {words:?}: {prompt}"
    );
    assert!(
        prompt.contains("nothing needs installing")
            && prompt.contains("run `sv doctor` in my own terminal"),
        "{prompt}"
    );
}
