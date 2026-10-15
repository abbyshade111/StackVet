//! Backlog 0540: `tools/install.sh` builds into the checkout's own `target` folder, wherever `CARGO_TARGET_DIR` points,
//! and installs nothing when the program it would copy was built from another commit. In the Cursor trial of 14
//! October 2026 the sandbox set `CARGO_TARGET_DIR`, the fresh build landed there, and an older `sv` was copied and
//! reported "Installed."
//!
//! Each test runs a copy of the script in a scratch checkout of its own, with a stand-in `cargo` that "builds" a
//! program saying the commit it is given, into the folder `--target-dir` names, or `CARGO_TARGET_DIR` without one.
#![cfg(unix)]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use sv_check::test_support::{executable, scratch};

fn git(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["-c", "user.name=t", "-c", "user.email=t@example.com"])
        .args(args)
        .output()
        .expect("git runs");
    assert!(out.status.success(), "git {args:?}: {out:?}");
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}

/// A checkout with the install script, a data folder, and one commit; its `HEAD`.
fn checkout(root: &Path) -> (PathBuf, String) {
    let repository = root.join("repository");
    std::fs::create_dir_all(repository.join("tools")).unwrap();
    std::fs::create_dir_all(repository.join("data")).unwrap();
    let script = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tools/install.sh");
    std::fs::copy(&script, repository.join("tools/install.sh")).unwrap();
    std::fs::write(repository.join("data/README.md"), "data\n").unwrap();
    git(&repository, &["init", "-q"]);
    git(&repository, &["add", "-A"]);
    git(&repository, &["commit", "-q", "-m", "one"]);
    let head = git(&repository, &["rev-parse", "HEAD"]);
    assert_eq!(head.len(), 40, "the setup: a commit was made");
    (repository, head)
}

/// A program that says it is `sv` built from `commit`, at `dir/release/sv`.
fn built_sv(dir: &Path, commit: &str) {
    std::fs::create_dir_all(dir.join("release")).unwrap();
    executable(
        &dir.join("release/sv"),
        &format!("#!/bin/sh\necho \"sv 0.1.0 (commit {commit})\"\n"),
    );
}

/// Runs the checkout's install script with a stand-in `cargo` that builds `commit` and `CARGO_TARGET_DIR` set to a
/// folder outside the checkout, as a sandbox sets it.
fn install(root: &Path, repository: &Path, commit: &str) -> Output {
    let bin = root.join("bin");
    std::fs::create_dir_all(&bin).unwrap();
    executable(
        &bin.join("cargo"),
        &format!(
            "#!/bin/sh\ndir=\"$CARGO_TARGET_DIR\"\nwhile [ $# -gt 0 ]; do\n  [ \"$1\" = --target-dir ] && dir=\"$2\"\n  shift\ndone\n\
             mkdir -p \"$dir/release\"\nprintf '#!/bin/sh\\necho \"sv 0.1.0 (commit {commit})\"\\n' > \"$dir/release/sv\"\n\
             chmod 755 \"$dir/release/sv\"\n"
        ),
    );
    Command::new("sh")
        .arg(repository.join("tools/install.sh"))
        .env("SV_PREFIX", root.join("prefix"))
        .env("CARGO_TARGET_DIR", root.join("elsewhere"))
        .env_remove("SV_BINARY")
        .env(
            "PATH",
            format!(
                "{}:{}",
                bin.display(),
                std::env::var("PATH").unwrap_or_default()
            ),
        )
        .output()
        .unwrap()
}

#[test]
fn a_build_sent_elsewhere_by_cargo_target_dir_is_still_the_one_installed() {
    let root = scratch("elsewhere");
    let (repository, head) = checkout(&root);
    // An older build already in the checkout's own target folder, as in the trial.
    built_sv(&repository.join("target"), "10b39332");
    let ran = install(&root, &repository, &head);
    let said = String::from_utf8_lossy(&ran.stdout).into_owned();
    assert!(ran.status.success(), "{ran:?}");
    assert!(
        said.contains(&format!("(commit {head})")) && said.contains("Installed."),
        "the fresh build is the one installed: {said}"
    );
}

#[test]
fn a_program_built_from_another_commit_is_not_installed() {
    let root = scratch("stale");
    let (repository, head) = checkout(&root);
    let older = "10b393320000000000000000000000000000000a";
    let ran = install(&root, &repository, older);
    let told = String::from_utf8_lossy(&ran.stderr).into_owned();
    assert!(
        !ran.status.success(),
        "a stale build was installed: {ran:?}"
    );
    assert!(
        told.contains(older) && told.contains(&head) && told.contains("nothing was installed"),
        "both commits are named: {told}"
    );
    assert!(
        !root.join("prefix/bin/sv").exists() && !root.join("prefix/share/stackvet/sv").exists(),
        "nothing was put in place"
    );
}
