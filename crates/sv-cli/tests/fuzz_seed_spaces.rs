//! `fuzz/seed.sh` copies a fixture whose name holds a space whole (backlog 0320). It split `find`'s output into words,
//! so such a file became two paths that do not exist and `set -eu` stopped the seeding step at the first one.
#![cfg(unix)]

use std::path::{Path, PathBuf};
use std::process::Command;

fn scratch() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-fuzz-seed-spaces-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    for folder in ["fuzz", "examples/my app", "crates", "data"] {
        std::fs::create_dir_all(dir.join(folder)).unwrap();
    }
    dir
}

fn names(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

#[test]
fn a_fixture_whose_name_holds_a_space_is_seeded_whole() {
    let top = scratch();
    let script = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fuzz/seed.sh");
    std::fs::copy(&script, top.join("fuzz/seed.sh")).unwrap();
    std::fs::write(
        top.join("examples/my app/stackvet.toml"),
        "manifest-version = 1\n",
    )
    .unwrap();
    std::fs::write(top.join("examples/my app/package-lock.json"), "{}\n").unwrap();
    std::fs::write(top.join("examples/stackvet.toml"), "manifest-version = 1\n").unwrap();
    // The setup: the space is really in the path the script finds.
    assert!(top.join("examples/my app/stackvet.toml").is_file());

    let ran = Command::new("sh")
        .arg("fuzz/seed.sh")
        .current_dir(&top)
        .output()
        .expect("sh runs");
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    );
    assert!(ran.status.success(), "{said}");
    let corpus = top.join("fuzz/corpus");
    let manifests = names(&corpus.join("manifest"));
    assert_eq!(manifests.len(), 2, "{manifests:?}");
    assert!(
        manifests.iter().all(|n| n.ends_with("-stackvet.toml")),
        "{manifests:?}"
    );
    // Each copied whole, not as a broken half of its name.
    for name in &manifests {
        let text = std::fs::read_to_string(corpus.join("manifest").join(name)).unwrap();
        assert_eq!(text, "manifest-version = 1\n");
    }
    assert_eq!(names(&corpus.join("lockfiles")).len(), 1);
    assert!(said.contains("seeded: 2 manifests, 1 lockfiles"), "{said}");
    let _ = std::fs::remove_dir_all(&top);
}
