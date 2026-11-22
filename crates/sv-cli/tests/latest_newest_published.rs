//! `latest` names the newest commit on main whose image is published, moved after the run's own image is signed
//! (backlog 0721, part 4; ADR-080, Later, 22 November 2026). Until then only the run of main's newest commit moved it,
//! so a failed last run of a burst of merges would have left it behind every green image before it. The step's script
//! was run against a stand-in `docker` and a real git history in five cases (the newest run failed; a late older run;
//! the newest run itself; a copy whose digest differed; nothing published), and each did as this file says.

use std::path::PathBuf;

fn publish_job() -> String {
    let text = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.github/workflows/rust.yml"),
    )
    .unwrap();
    let start = text
        .find("\n  publish:\n")
        .expect("the setup: the publish job");
    text[start..].to_owned()
}

#[test]
fn latest_moves_only_in_its_own_step_after_signing() {
    let job = publish_job();
    let step = job
        .find("- name: Point latest at the newest image published from main")
        .expect("the step that moves latest");
    let signed = job
        .find("- name: Sign the arm64 image")
        .expect("the setup: the signing steps");
    assert!(signed < step, "latest is moved before the image is signed");
    assert_eq!(
        job.matches("-t \"$IMAGE:latest\"").count(),
        1,
        "latest is tagged somewhere other than its own step"
    );
    assert!(
        !job.contains("latest is left to the newer commit's run"),
        "the old rule is still there"
    );
}

#[test]
fn latest_is_the_newest_published_image_and_the_signed_one() {
    let job = publish_job();
    let step = &job[job
        .find("- name: Point latest at the newest image published from main")
        .unwrap()..];
    for line in [
        "git rev-list --first-parent --max-count=50 FETCH_HEAD",
        r#"docker buildx imagetools inspect "$IMAGE:$c""#,
        r#"docker buildx imagetools create -t "$IMAGE:latest" "$IMAGE:$newest""#,
        r#"test "$(digest_of "$IMAGE:latest")" = "$(digest_of "$IMAGE:$newest")""#,
    ] {
        assert!(step.contains(line), "the step does not {line:?}");
    }
    assert!(
        step.contains("exit 1"),
        "a step that cannot move latest fails, never passes quietly"
    );
}
