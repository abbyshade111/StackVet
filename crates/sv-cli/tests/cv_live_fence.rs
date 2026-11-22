//! `sv`'s binary never reaches ConfigVet's live clients (ADR-157, decision 4; backlog 0717). `cv-live` is the only
//! crate that reads a hosting account; `sv` opens no network connection of its own beyond "The fence"'s exceptions,
//! and holding that by a crate boundary means no crate `sv-cli` depends on, however far down, may depend on it.
//!
//! Read from each crate's own `Cargo.toml`, following every `path` dependency from `sv-cli`, so the test needs no
//! registry, no network, and no TOML library. A crate from the registry cannot depend on a path crate of this workspace.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Every crate of this workspace a crate reaches through `path` dependencies, by its folder under `crates/`, itself
/// included. Development and build dependencies count too: a test or build script of `sv` is `sv` as much as its code.
fn reaches(start: &str) -> BTreeSet<String> {
    let mut seen = BTreeSet::new();
    let mut todo = vec![start.to_owned()];
    while let Some(name) = todo.pop() {
        if !seen.insert(name.clone()) {
            continue;
        }
        let manifest = root().join("crates").join(&name).join("Cargo.toml");
        let text = std::fs::read_to_string(&manifest)
            .unwrap_or_else(|e| panic!("{} could not be read: {e}", manifest.display()));
        // Every `path = "..."` outside a comment, in whatever table: `[dependencies]`, `[dev-dependencies]`,
        // `[build-dependencies]`, a `[target...]` table, or the `[dependencies.name]` form.
        for line in text.lines().map(str::trim).filter(|l| !l.starts_with('#')) {
            let Some(at) = line.find("path") else {
                continue;
            };
            let rest = line[at + 4..].trim_start();
            let Some(rest) = rest.strip_prefix('=') else {
                continue;
            };
            let Some(quoted) = rest.trim_start().strip_prefix('"') else {
                continue;
            };
            let Some(end) = quoted.find('"') else {
                continue;
            };
            // A dependency's path names a sibling crate; a `[[bin]]` or `[lib]` path names a file of this one.
            if !quoted[..end].starts_with("../") {
                continue;
            }
            let folder = Path::new(&quoted[..end])
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned();
            todo.push(folder);
        }
    }
    seen
}

#[test]
fn sv_never_depends_on_the_live_clients() {
    let sv = reaches("sv-cli");
    // The setup: the walk went past sv-cli, through sv-check to sv-core, so a walk that read nothing cannot pass.
    for expected in ["sv-check", "sv-core", "sv-run"] {
        assert!(
            sv.contains(expected),
            "the walk from sv-cli did not reach {expected}: {sv:?}"
        );
    }
    assert!(
        !sv.contains("cv-live"),
        "sv-cli reaches cv-live, so sv's binary could read a hosting account (ADR-157, decision 4): {sv:?}"
    );
}

#[test]
fn cv_does_reach_them() {
    // The other side: cv-cli is where the live clients are used, so the walk finds cv-live there.
    assert!(reaches("cv-cli").contains("cv-live"));
}
