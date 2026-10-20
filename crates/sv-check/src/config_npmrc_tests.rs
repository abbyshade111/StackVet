//! An `.npmrc`, a `.pypirc`, or a `.netrc` is a secrets file only when a line in one of its committed versions holds
//! a credential (backlog 0614): until 20 October 2026 the name alone made it a critical finding, and `sv`'s own `v1`
//! `.npmrc` (five settings, no credential) was its one critical finding outside test code. The credentials below are
//! built from pieces at run time, as every key in a test is.

use super::*;
use std::process::Command;

fn token() -> String {
    format!("npm_{}{}", "Zq8".repeat(6), "x7Wk".repeat(5))
}

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

/// A repository holding `files`, committed in one commit.
fn repo(tag: &str, files: &[(&str, &str)]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-config-npmrc-{tag}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    git(&dir, &["init", "-q"]);
    for (name, text) in files {
        std::fs::write(dir.join(name), text).unwrap();
    }
    git(&dir, &["add", "-A"]);
    git(&dir, &["commit", "-q", "-m", "one"]);
    dir
}

fn has_git() -> bool {
    Command::new("git").arg("--version").output().is_ok()
}

#[test]
fn a_credential_line_is_found_in_each_kind_of_settings_file_and_a_setting_is_not() {
    let t = token();
    let found = |name: &str, text: &str| credential_line(name, text).map(|(n, key, _)| (n, key));
    assert_eq!(
        found(".npmrc", &format!("ignore-scripts=true\n_authToken={t}\n")),
        Some((2, "_authToken".into()))
    );
    assert_eq!(
        found(".npmrc", &format!("//registry.npmjs.org/:_authToken={t}\n")),
        Some((1, "_authToken".into()))
    );
    assert_eq!(
        found(".npmrc", "_auth=\"\"\n_password = 'x'\n"),
        Some((2, "_password".into()))
    );
    assert_eq!(
        found(
            ".pypirc",
            &format!("[pypi]\nusername = __token__\npassword = {t}\n")
        ),
        Some((3, "password".into()))
    );
    assert_eq!(
        found(
            ".netrc",
            &format!("machine example.com login me password {t}\n")
        ),
        Some((1, "password".into()))
    );
    // Not a credential: settings, a reference to the environment, an empty value, a comment.
    for (name, text) in [
        (
            ".npmrc",
            "ignore-scripts=true\nmin-release-age=7\nfund=false\naudit=false\nupdate-notifier=false\n",
        ),
        (".npmrc", "//registry.npmjs.org/:_authToken=${NPM_TOKEN}\n"),
        (".npmrc", "_authToken=\n"),
        (".npmrc", &format!("# _authToken={t}\n")),
        (
            ".pypirc",
            "[pypi]\nusername = __token__\npassword = ${PYPI_TOKEN}\n",
        ),
        (".netrc", "machine example.com login me\n"),
    ] {
        assert_eq!(credential_line(name, text), None, "{name}: {text:?}");
    }
}

#[test]
fn a_committed_npmrc_with_settings_alone_is_no_finding_and_one_with_a_token_is() {
    if !has_git() {
        println!("no git here; this needs it");
        return;
    }
    // The setup this item was filed from: `sv`'s own v1 `.npmrc`, five settings and no credential.
    let settings = repo(
        "settings",
        &[(
            ".npmrc",
            "ignore-scripts=true\nmin-release-age=7\nfund=false\naudit=false\nupdate-notifier=false\n",
        )],
    );
    assert_eq!(held(&settings, ".npmrc"), None);
    assert!(matches!(
        secrets_file_committed(&settings),
        Outcome::Passed(_)
    ));
    std::fs::remove_dir_all(&settings).ok();

    let t = token();
    let with = repo(
        "token",
        &[(".npmrc", &format!("ignore-scripts=true\n_authToken={t}\n"))],
    );
    let Outcome::Failed(f) = secrets_file_committed(&with) else {
        panic!("a committed npm token is not a finding")
    };
    assert_eq!((f.location.file.as_str(), f.location.line), (".npmrc", 2));
    assert!(
        f.description.contains("its line 2 holds `_authToken`"),
        "{}",
        f.description
    );
    // The value is cut short, never written out.
    let secret = f.secret.as_ref().expect("the value, masked");
    assert_eq!(secret.length(), t.chars().count());
    assert!(!secret.as_str().contains(&t[4..]), "{}", secret.as_str());
    assert!(
        !format!("{f:?}").contains(&t),
        "the finding holds the token"
    );
    std::fs::remove_dir_all(&with).ok();
}

#[test]
fn a_token_taken_out_of_a_later_version_is_still_in_the_history() {
    if !has_git() {
        println!("no git here; this needs it");
        return;
    }
    let t = token();
    let dir = repo("history", &[(".npmrc", &format!("_authToken={t}\n"))]);
    // The token removed in the next commit, and the file then untracked: only the first commit holds it.
    std::fs::write(dir.join(".npmrc"), "ignore-scripts=true\n").unwrap();
    git(&dir, &["commit", "-q", "-am", "token out"]);
    git(&dir, &["rm", "-q", "--cached", ".npmrc"]);
    git(&dir, &["commit", "-q", "-m", "untracked"]);
    let Outcome::Failed(f) = secrets_file_committed(&dir) else {
        panic!("a token in an earlier version is not a finding")
    };
    assert!(f.title.contains("still in the history"), "{}", f.title);
    assert!(
        f.description.contains("holds `_authToken`"),
        "{}",
        f.description
    );
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_file_whose_only_job_is_a_credential_still_goes_by_its_name() {
    if !has_git() {
        println!("no git here; this needs it");
        return;
    }
    // An .env with nothing that looks like a key in it is still a secrets file: that is all it is for.
    let dir = repo("env", &[(".env", "DEBUG=1\n")]);
    assert_eq!(held(&dir, ".env"), Some(Held::ByName));
    let Outcome::Failed(f) = secrets_file_committed(&dir) else {
        panic!(".env committed is not a finding")
    };
    assert!(
        f.description
            .contains("files with that name hold credentials"),
        "{}",
        f.description
    );
    std::fs::remove_dir_all(&dir).ok();
}
