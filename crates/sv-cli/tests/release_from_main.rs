//! A release is made only from a tag at a commit `main` holds (backlog 0712; ADR-108, Later, 21 November 2026): the
//! signing job asks GitHub whether `main` holds the tagged commit before it signs or releases anything, and accepts
//! only GitHub's "ahead" or "identical".

use std::path::PathBuf;

fn release_job() -> String {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.github/workflows/release.yml");
    let text = std::fs::read_to_string(path).unwrap();
    let start = text
        .find("\n  release:\n")
        .expect("the setup: release.yml has a release job");
    text[start..].to_owned()
}

#[test]
fn the_release_job_checks_main_holds_the_tag_before_it_signs() {
    let job = release_job();
    let check = job
        .find("- name: The tag's commit is on main")
        .expect("the release job checks the tag's commit is on main");
    for later in ["uses: actions/attest@", "gh release create"] {
        let at = job
            .find(later)
            .unwrap_or_else(|| panic!("the setup: {later} is in the job"));
        assert!(
            check < at,
            "{later} comes before the check that main holds the tag"
        );
    }
    let step = &job[check
        ..job[check..]
            .find("\n      - name: Fetch")
            .map_or(job.len(), |i| check + i)];
    assert!(
        step.contains("compare/$GITHUB_SHA...main"),
        "the check does not compare the tagged commit with main: {step}"
    );
    assert!(
        step.contains("ahead|identical)") && step.contains("exit 1"),
        "the check does not stop on anything but ahead or identical: {step}"
    );
}
