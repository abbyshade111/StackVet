//! `fuzz/seed.sh` prints its counts as bare numbers wherever it runs (backlog 0395). macOS's `wc` pads a count with
//! spaces, so on a Mac the script said `seeded:        2 manifests, ...` and `fuzz_seed_spaces.rs` failed there, though
//! the seeding was right. Here a `wc` that pads as macOS's does is put first on the `PATH`.
#![cfg(unix)]

use std::path::{Path, PathBuf};
use std::process::Command;

fn scratch() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-fuzz-seed-counts-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    for folder in ["fuzz", "examples/app", "crates", "data", "bin"] {
        std::fs::create_dir_all(dir.join(folder)).unwrap();
    }
    dir
}

/// The `wc` the system has, by its full path, for the padding one to call.
fn real_wc() -> String {
    let found = Command::new("sh")
        .args(["-c", "command -v wc"])
        .output()
        .expect("sh runs");
    let path = String::from_utf8_lossy(&found.stdout).trim().to_owned();
    assert!(path.starts_with('/'), "the setup: wc was found: {path:?}");
    path
}

#[test]
fn the_counts_are_bare_numbers_where_wc_pads_them() {
    let top = scratch();
    let script = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fuzz/seed.sh");
    std::fs::copy(&script, top.join("fuzz/seed.sh")).unwrap();
    std::fs::write(
        top.join("examples/app/stackvet.toml"),
        "manifest-version = 1\n",
    )
    .unwrap();
    std::fs::write(top.join("examples/app/package-lock.json"), "{}\n").unwrap();
    // A `wc` that pads its count to eight places, as macOS's does.
    let wc = top.join("bin/wc");
    sv_check::test_support::executable(
        &wc,
        &format!(
            "#!/bin/sh\nprintf '%8s\\n' \"$({} \"$@\" | tr -d ' ')\"\n",
            real_wc()
        ),
    );
    let path = format!(
        "{}:{}",
        top.join("bin").display(),
        std::env::var("PATH").unwrap_or_default()
    );
    // The setup: the padding `wc` is the one the script will find, and it pads.
    let padded = Command::new("sh")
        .args(["-c", "printf 'a\\nb\\n' | wc -l"])
        .env("PATH", &path)
        .output()
        .expect("sh runs");
    assert_eq!(String::from_utf8_lossy(&padded.stdout), "       2\n");

    let ran = Command::new("sh")
        .arg("fuzz/seed.sh")
        .current_dir(&top)
        .env("PATH", &path)
        .output()
        .expect("sh runs");
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    );
    let _ = std::fs::remove_dir_all(&top);
    assert!(ran.status.success(), "{said}");
    assert!(
        said.contains(
            "seeded: 1 manifests, 1 lockfiles and test reports, 0 tool reports, 1 MCP session"
        ),
        "{said}"
    );
}
