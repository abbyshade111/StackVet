//! The `Dockerfile`'s dependency layer copies every workspace member's `Cargo.toml` and gives each an empty source
//! (backlog 0357), or `cargo` cannot load the workspace and the image does not build. A crate added to the workspace
//! and not here broke the `image` jobs on 21 November 2026 (`cv-live`, backlog 0717); this says so before CI does.

use std::path::PathBuf;

fn read(path: &str) -> String {
    std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(path),
    )
    .unwrap()
}

/// The `crates/...` members of the workspace's `members = [...]` list.
fn members() -> Vec<String> {
    let manifest = read("Cargo.toml");
    let start = manifest
        .find("members = [")
        .expect("the setup: the workspace lists its members");
    let list = &manifest[start..start + manifest[start..].find(']').unwrap()];
    list.split('"')
        .filter_map(|part| part.strip_prefix("crates/"))
        .map(str::to_owned)
        .collect()
}

#[test]
fn every_workspace_member_is_in_the_dockerfiles_dependency_layer() {
    let members = members();
    assert!(
        members.len() >= 10,
        "the setup: the members were read: {members:?}"
    );
    let dockerfile = read("Dockerfile");
    let empty_sources = dockerfile
        .lines()
        .find(|l| l.trim_start().starts_with("RUN for c in "))
        .expect("the setup: the layer that makes each crate's empty source");
    let named: Vec<&str> = empty_sources
        .trim_start()
        .trim_start_matches("RUN for c in ")
        .split("; do")
        .next()
        .unwrap()
        .split_whitespace()
        .collect();
    for member in &members {
        let copy = format!("COPY crates/{member}/Cargo.toml crates/{member}/");
        assert!(
            dockerfile.contains(&copy),
            "the Dockerfile does not copy {member}'s Cargo.toml: add `{copy}`"
        );
        assert!(
            named.contains(&member.as_str()),
            "the Dockerfile makes no empty source for {member}: add it to `RUN for c in ...`"
        );
    }
}
