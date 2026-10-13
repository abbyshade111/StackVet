//! The fuzz crate's lock (`fuzz/Cargo.lock`, ADR-077) keeps every crate it shares with the workspace at the
//! workspace's version. It is a crate of its own, outside the workspace, so nothing else keeps the two locks
//! together: by 12 October 2026, 24 shared crates had drifted, some to versions the manifests no longer accept,
//! and `cargo fuzz run` rewrote the lock on first use (backlog 0360).
use std::collections::{BTreeMap, BTreeSet};

/// Each package's versions in a `Cargo.lock`, read from its `name` and `version` lines.
fn versions(lock: &str) -> BTreeMap<String, BTreeSet<String>> {
    let mut out: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut name = None;
    for line in lock.lines() {
        if line == "[[package]]" {
            name = None;
        } else if let Some(n) = line.strip_prefix("name = ") {
            name = Some(n.trim_matches('"').to_owned());
        } else if let Some(v) = line.strip_prefix("version = ")
            && let Some(n) = name.take()
        {
            out.entry(n)
                .or_default()
                .insert(v.trim_matches('"').to_owned());
        }
    }
    out
}

#[test]
fn the_fuzz_lock_keeps_the_workspaces_versions() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace = versions(&std::fs::read_to_string(root.join("Cargo.lock")).unwrap());
    let fuzz = versions(&std::fs::read_to_string(root.join("fuzz/Cargo.lock")).unwrap());
    assert!(
        fuzz.contains_key("libfuzzer-sys") && fuzz.contains_key("sv-check"),
        "the fuzz lock was not read: {fuzz:?}"
    );
    let drifted: Vec<String> = fuzz
        .iter()
        .filter_map(|(name, got)| {
            let want = workspace.get(name)?;
            (!got.is_subset(want))
                .then(|| format!("{name}: {got:?} in fuzz/, {want:?} in the workspace"))
        })
        .collect();
    assert!(
        drifted.is_empty(),
        "regenerate fuzz/Cargo.lock from the workspace's lock (copy it, then `cargo update -w` in fuzz/):\n{}",
        drifted.join("\n")
    );
}
