//! A file whose name git writes in quotes (backlog 0713). Git writes a name that holds a letter outside ASCII, a quote,
//! a backslash, or a control character as `"caf\303\251.py"`, in the list of a commit's files and in the patch's
//! headers alike, and ends an unquoted header name that holds a space with a tab. The reader took each as written, so
//! nothing a tool's commit added to such a file was seen. Each test plants a real repository and reads it with the
//! same `git` the section runs.

use super::tests::run;
use super::*;

/// A repository with a person's commit, then an AI tool's commit that adds `name` with a comment telling a scanner to
/// look away, and changes a line of it in a third commit, also the tool's.
fn planted(test: &str, name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-git-names-{test}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    run(&dir, &["init", "-q", "-b", "main"]);
    std::fs::write(dir.join("app.py"), "print(1)\n").unwrap();
    run(&dir, &["add", "-A"]);
    run(&dir, &["commit", "-qm", "Start the app"]);
    for (text, subject) in [
        ("eval(x)  # nosec\n", "Add a file"),
        ("eval(y)  # nosec\n", "Change it"),
    ] {
        std::fs::write(dir.join(name), text).unwrap();
        run(&dir, &["add", "-A"]);
        run(
            &dir,
            &[
                "commit",
                "-qm",
                &format!("{subject}\n\nCo-Authored-By: Claude <noreply@anthropic.com>"),
            ],
        );
    }
    dir
}

/// What plain `git show` writes for the newest commit, so a test can show its setup is the case it means.
fn shown(dir: &Path) -> String {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(dir)
        .args([
            "show",
            "--format=",
            "--raw",
            "--patch",
            "--unified=0",
            "HEAD",
        ])
        .output()
        .unwrap();
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// The tool's two commits are read with the file under its own name: listed, and each line added or removed there
/// kept under that name.
fn read_under_its_own_name(dir: &Path, name: &str) {
    let (commits, _) = since(dir, None, 20).expect("git could read the planted repository");
    let tools: Vec<&Commit> = commits.iter().filter(|c| c.tool.is_some()).collect();
    assert_eq!(tools.len(), 2, "the setup: two commits name a tool");
    let (changed, added) = (tools[0], tools[1]);
    assert_eq!(added.files, vec![('A', name.to_owned())], "{added:?}");
    assert_eq!(
        added.added,
        vec![(name.to_owned(), "eval(x)  # nosec".to_owned())],
        "{added:?}"
    );
    assert_eq!(changed.files, vec![('M', name.to_owned())], "{changed:?}");
    assert_eq!(
        changed.added,
        vec![(name.to_owned(), "eval(y)  # nosec".to_owned())],
        "{changed:?}"
    );
    assert_eq!(
        changed.removed,
        vec![(name.to_owned(), "eval(x)  # nosec".to_owned())],
        "{changed:?}"
    );
}

#[test]
fn a_name_with_a_letter_outside_ascii_is_read_as_the_file_s_own() {
    let name = "caf\u{e9}.py";
    let dir = planted("accent", name);
    // The setup: git writes this name in quotes, with the letter as two escaped bytes.
    assert!(
        shown(&dir).contains(r#"+++ "b/caf\303\251.py""#),
        "the setup: git quotes the name: {}",
        shown(&dir)
    );
    read_under_its_own_name(&dir, name);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_name_with_a_space_is_read_as_the_file_s_own() {
    let name = "my file.py";
    let dir = planted("space", name);
    // The setup: git leaves this name unquoted and ends it with a tab in the patch's headers.
    assert!(
        shown(&dir).contains("+++ b/my file.py\t"),
        "the setup: git ends the name with a tab: {:?}",
        shown(&dir)
    );
    read_under_its_own_name(&dir, name);
    std::fs::remove_dir_all(&dir).ok();
}

/// Names Windows does not allow in a file, so planted only elsewhere: a quote, a backslash, a tab, and the character
/// that starts a terminal's escape sequences.
#[cfg(unix)]
#[test]
fn a_name_with_a_quote_a_backslash_or_a_control_character_is_read_as_the_file_s_own() {
    for (test, name) in [
        ("quote", "a\"b.py"),
        ("backslash", "a\\b.py"),
        ("tab", "a\tb.py"),
        ("escape", "a\u{1b}b.py"),
    ] {
        let dir = planted(test, name);
        assert!(
            shown(&dir).contains("+++ \"b/a"),
            "the setup: git quotes the name ({test}): {:?}",
            shown(&dir)
        );
        read_under_its_own_name(&dir, name);
        std::fs::remove_dir_all(&dir).ok();
    }
}

#[test]
fn a_name_is_taken_out_of_git_s_quotes_and_one_not_in_quotes_is_left_as_it_is() {
    for (written, held) in [
        ("plain.py", "plain.py"),
        (r#""caf\303\251.py""#, "caf\u{e9}.py"),
        (r#""a\"b\\c\td.py""#, "a\"b\\c\td.py"),
        (r#""a\033b\a.py""#, "a\u{1b}b\u{7}.py"),
        // Not git's quoting: left alone, never guessed at.
        ("\"half", "\"half"),
        (r#"a"b"#, r#"a"b"#),
    ] {
        assert_eq!(unquoted(written), held, "{written}");
    }
}
