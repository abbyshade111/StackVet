//! A repository that says part of its history is kept elsewhere, and names the program that brings it, has that
//! program run by plain `git log` when a commit is missing. These plant such a program two ways, watch plain git run
//! it (so the setup is the attack), and then watch every reader here leave it alone (ADR-032, Later, 21 November
//! 2026).

use super::*;

/// Runs `git` in `dir`, and asserts it worked.
fn run(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args([
            "-c",
            "user.email=t@t",
            "-c",
            "user.name=t",
            "-c",
            "commit.gpgsign=false",
        ])
        .args(args)
        .output()
        .expect("git runs");
    assert!(
        out.status.success(),
        "git {args:?} failed in the test's setup"
    );
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}

fn no_git() -> bool {
    if Command::new("git").arg("--version").output().is_err() {
        println!("no git here; this needs it");
        return true;
    }
    false
}

fn fresh(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-git-fetch-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// A checkout with two commits, the newer one's list of files (its tree object) taken away, and a config that calls the repository a
/// partial copy of a remote whose "upload" program leaves a mark.
fn missing_tree(name: &str) -> (std::path::PathBuf, std::path::PathBuf) {
    let dir = fresh(name);
    let mark = dir.join("ran");
    run(&dir, &["init", "-q", "-b", "main"]);
    for text in ["one\n", "two\n"] {
        std::fs::write(dir.join("app.py"), text).unwrap();
        run(&dir, &["add", "app.py"]);
        run(&dir, &["commit", "-qm", text.trim()]);
    }
    let tree = run(&dir, &["rev-parse", "HEAD^{tree}"]);
    let object = dir
        .join(".git")
        .join("objects")
        .join(&tree[..2])
        .join(&tree[2..]);
    std::fs::remove_file(&object).expect("the tree is a loose object");
    // Set last, so the setup itself never runs it.
    let program = format!("touch '{}'; false", mark.display());
    run(&dir, &["config", "extensions.partialClone", "origin"]);
    run(&dir, &["config", "remote.origin.promisor", "true"]);
    run(
        &dir,
        &["config", "remote.origin.url", dir.to_str().unwrap()],
    );
    run(&dir, &["config", "remote.origin.uploadpack", &program]);
    (dir, mark)
}

/// A folder of plain files that git takes for a repository with no checkout: `HEAD`, `config`, `objects`, and
/// `refs`, as an unpacked archive can hold. Its branch names a commit that is not there, and its config names the
/// program to reach the remote with.
fn plain_files(name: &str) -> (std::path::PathBuf, std::path::PathBuf) {
    let dir = fresh(name);
    let mark = dir.join("ran");
    std::fs::create_dir_all(dir.join("objects").join("info")).unwrap();
    std::fs::create_dir_all(dir.join("refs").join("heads")).unwrap();
    std::fs::write(dir.join("HEAD"), "ref: refs/heads/main\n").unwrap();
    std::fs::write(
        dir.join("refs").join("heads").join("main"),
        "1111111111111111111111111111111111111111\n",
    )
    .unwrap();
    std::fs::write(dir.join("app.py"), "print(1)\n").unwrap();
    let config = dir.join("config");
    let set = |key: &str, value: &str| {
        let ok = Command::new("git")
            .args(["config", "-f"])
            .arg(&config)
            .args([key, value])
            .output()
            .is_ok_and(|o| o.status.success());
        assert!(ok, "git config {key} failed in the test's setup");
    };
    set("core.repositoryformatversion", "1");
    set("core.bare", "true");
    set("extensions.partialClone", "origin");
    set("remote.origin.promisor", "true");
    set("remote.origin.url", "ssh://nowhere.invalid/x");
    set(
        "core.sshCommand",
        &format!("touch '{}'; false", mark.display()),
    );
    (dir, mark)
}

/// Every reader of this module, run in `dir`; what each answers is not the point here.
fn read_everything(dir: &Path) {
    let _ = ls_files(dir);
    let _ = ever_added(dir, &[]);
    let _ = every_version(dir, "app.py");
    let _ = is_shallow(dir);
    let _ = head_commit(dir);
    let _ = has_uncommitted_changes(dir);
    let _ = ai_commits_off_merge(dir);
}

#[test]
fn a_missing_part_of_the_history_is_not_fetched_with_the_program_the_repository_names() {
    if no_git() {
        return;
    }
    let (dir, mark) = missing_tree("missing-control");
    let _ = Command::new("git")
        .arg("-C")
        .arg(&dir)
        .args(["ls-tree", "-r", "HEAD"])
        .output()
        .unwrap();
    assert!(
        mark.exists(),
        "plain git did not run the planted program, so this proves nothing"
    );
    std::fs::remove_dir_all(&dir).ok();

    let (dir, mark) = missing_tree("missing-guarded");
    read_everything(&dir);
    assert!(
        !mark.exists(),
        "git ran the program the app's repository named to fetch what was missing"
    );
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_folder_of_plain_files_shaped_like_a_repository_runs_nothing() {
    if no_git() {
        return;
    }
    let (dir, mark) = plain_files("plain-control");
    let _ = Command::new("git")
        .arg("-C")
        .arg(&dir)
        .args(["log", "--format=%H", "main"])
        .output()
        .unwrap();
    assert!(
        mark.exists(),
        "plain git did not run the planted program, so this proves nothing"
    );
    std::fs::remove_dir_all(&dir).ok();

    let (dir, mark) = plain_files("plain-guarded");
    read_everything(&dir);
    assert!(
        !mark.exists(),
        "git ran the program a folder of plain files named"
    );
    std::fs::remove_dir_all(&dir).ok();
}

/// The same settings reach a program `sv` starts that runs git itself, through its environment.
#[test]
fn the_environment_given_to_outside_tools_fetches_nothing_either() {
    if no_git() {
        return;
    }
    for (name, planted) in [
        (
            "env-missing",
            missing_tree as fn(&str) -> (std::path::PathBuf, std::path::PathBuf),
        ),
        ("env-plain", plain_files),
    ] {
        let (dir, mark) = planted(name);
        let _ = Command::new("git")
            .arg("-C")
            .arg(&dir)
            .args(["ls-tree", "-r", "HEAD"])
            .envs(ENV_OVERRIDES)
            .output()
            .unwrap();
        assert!(
            !mark.exists(),
            "git ran the planted program under the outside tools' environment ({name})"
        );
        std::fs::remove_dir_all(&dir).ok();
    }
}
