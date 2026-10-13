//! Reading the history through `secret_file_pathspecs` finds every secrets file that reading all
//! of it finds (backlog 0366): the pathspecs narrow what git compares, never what `sv` reports.

use super::*;
use std::process::Command;

fn git(dir: &Path, args: &[&str]) {
    let ok = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["-c", "user.email=t@example.com", "-c", "user.name=t"])
        .args(args)
        .status()
        .is_ok_and(|s| s.success());
    assert!(ok, "git {args:?} failed");
}

#[test]
fn the_narrowed_history_finds_every_secrets_file_the_whole_history_does() {
    if Command::new("git").arg("--version").output().is_err() {
        println!("no git here; this needs it");
        return;
    }
    let dir = std::env::temp_dir().join(format!("sv-config-pathspecs-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    git(&dir, &["init", "-q"]);

    // Every kind of name `is_secret_file` accepts, at the top and two folders down, beside files
    // that are not secrets; all committed, then all removed, so only the history holds them.
    let mut secrets: Vec<String> = SECRET_FILES.iter().map(|n| (*n).to_owned()).collect();
    secrets.extend([
        ".env.staging".to_owned(),
        "my-app-firebase-adminsdk-abc12-0123456789.json".to_owned(),
    ]);
    let others = ["app.py", ".env.example", "notes.json", "id_rsa.pub"];
    let mut written = Vec::new();
    for name in secrets.iter().map(String::as_str).chain(others) {
        for folder in ["", "deploy/keys/"] {
            let path = format!("{folder}{name}");
            let full = dir.join(&path);
            std::fs::create_dir_all(full.parent().unwrap()).unwrap();
            std::fs::write(&full, "x\n").unwrap();
            written.push(path);
        }
    }
    git(&dir, &["add", "-A"]);
    git(&dir, &["commit", "-q", "-m", "all"]);
    git(&dir, &["rm", "-q", "-r", "."]);
    git(&dir, &["commit", "-q", "-m", "none"]);

    let whole = crate::git::ever_added(&dir, &[]).expect("git answers");
    let narrowed = crate::git::ever_added(&dir, &secret_file_pathspecs()).expect("git answers");
    std::fs::remove_dir_all(&dir).ok();

    // The setup: the whole history holds every file written, the secrets among them.
    for path in &written {
        assert!(whole.contains(path), "{path} not in the history: {whole:?}");
    }
    let secret = |path: &&String| is_secret_file(path.rsplit('/').next().unwrap_or(path));
    let from_whole: Vec<&String> = whole.iter().filter(secret).collect();
    let from_narrowed: Vec<&String> = narrowed.iter().filter(secret).collect();
    assert_eq!(from_whole.len(), secrets.len() * 2, "{from_whole:?}");
    // The point: the narrowed history misses none of them.
    assert_eq!(from_narrowed, from_whole);
    // And it is narrowed: a file no pattern names is not compared.
    assert!(
        !narrowed.iter().any(|n| n.ends_with("app.py")),
        "{narrowed:?}"
    );
}
