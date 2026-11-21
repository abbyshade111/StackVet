//! The task envelope's reader (backlog 0715, part 1): what is read, what is refused and why, what is left as a
//! question, and what an entry covers.

use super::*;

const WHOLE: &str = r#"
envelope-version = 1
ask-first = ["delete", "push"]

[task]
goal = "Add a sign-out link to the header."

[may]
write = ["src/", "tests/", "README.md"]
run = ["pytest", "npm"]
reach = ["pypi.org"]

[guards]
files = [".github/workflows/", "AGENTS.md"]
"#;

const LEAST: &str = "envelope-version = 1\n[task]\ngoal = \"Tidy the README.\"\n";

fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("av-envelope-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn a_whole_envelope_is_read_and_leaves_no_question() {
    let e = parse(WHOLE).unwrap();
    assert_eq!(e.task.goal, "Add a sign-out link to the header.");
    assert_eq!(
        e.may.run.as_deref(),
        Some(&["pytest".to_owned(), "npm".to_owned()][..])
    );
    assert_eq!(
        e.ask_first.as_deref(),
        Some(&["delete".to_owned(), "push".to_owned()][..])
    );
    assert_eq!(e.questions(), Vec::<String>::new());
    assert_eq!(e.guards(), [".github/workflows/", "AGENTS.md"]);
}

#[test]
fn a_line_left_out_is_a_question_and_never_a_yes_or_a_no() {
    let e = parse(LEAST).unwrap();
    // Nobody said what it may write: neither "anything" nor "nothing".
    assert_eq!(e.may_write("src/app.py"), None);
    assert_eq!(e.questions().len(), 4, "{:?}", e.questions());
    for asked in ["[may] write", "[may] run", "[may] reach", "ask-first"] {
        assert!(
            e.questions().iter().any(|q| q.contains(asked)),
            "no question about {asked}: {:?}",
            e.questions()
        );
    }
    // An empty list is an answer: nothing.
    let e = parse(&format!("{LEAST}[may]\nwrite = []\n")).unwrap();
    assert_eq!(e.may_write("src/app.py"), Some(false));
    assert_eq!(e.questions().len(), 3);
    // With no guards named, the default list holds, the envelope itself among them.
    assert!(e.guards_file(FILE) && e.guards_file(".claude/settings.json"));
    assert!(e.guards_file(".github/workflows/ci.yml") && !e.guards_file("src/app.py"));
}

#[test]
fn what_is_not_an_envelope_is_refused_with_its_reason() {
    for (text, why) in [
        ("[task]\ngoal = \"x\"\n", "envelope-version"),
        (
            "envelope-version = 2\n[task]\ngoal = \"x\"\n",
            "reads version 1 only",
        ),
        ("envelope-version = 1\n", "task"),
        (
            "envelope-version = 1\n[task]\ngoal = \"  \"\n",
            "empty [task] goal",
        ),
        // A key it does not know is refused, never skipped: a misspelled guard would otherwise guard nothing.
        (&format!("{LEAST}[guard]\nfiles = []\n") as &str, "guard"),
        (&format!("{LEAST}[may]\nwrites = [\"src/\"]\n"), "writes"),
        (
            &format!("{LEAST}[may]\nwrite = [\"../other/\"]\n"),
            "leaves this folder",
        ),
        (
            &format!("{LEAST}[may]\nwrite = [\"/etc/\"]\n"),
            "not a place below this folder",
        ),
        (
            &format!("{LEAST}[may]\nwrite = [\"C:/Users/\"]\n"),
            "not a place below this folder",
        ),
        (
            &format!("{LEAST}[may]\nwrite = [\"src\\\\app\"]\n"),
            "backslash",
        ),
        (
            &format!("{LEAST}[guards]\nfiles = [\"\"]\n"),
            "names nothing",
        ),
        (
            &format!("{LEAST}[may]\nrun = [\"rm -rf\"]\n"),
            "not one name",
        ),
        (&format!("{LEAST}[may]\nreach = [\"\"]\n"), "not one name"),
        (
            "envelope-version = 1\nask-first = [\"deploy\"]\n[task]\ngoal = \"x\"\n",
            "not one of delete, push, install, send, spend",
        ),
    ] {
        let refused = parse(text).expect_err(text);
        assert!(refused.contains(why), "{text:?}: {refused}");
        assert!(refused.starts_with(FILE), "it names the file: {refused}");
        assert!(!refused.contains('\n'), "on one line: {refused:?}");
    }
}

#[test]
fn an_entry_covers_a_folder_s_files_or_one_file_and_nothing_by_pattern() {
    let entries: Vec<String> = ["src/", "README.md", "docs/guide/"]
        .iter()
        .map(|e| (*e).to_owned())
        .collect();
    for covered in [
        "src/app.py",
        "src/deep/er/x.py",
        "README.md",
        "./README.md",
        "docs/guide/a.md",
    ] {
        assert!(covers(&entries, covered), "{covered}");
    }
    for not in [
        "srcs/app.py", // a folder is not a prefix of a longer name
        "src",         // the folder itself is not a file in it
        "README.md.bak",
        "docs/other.md",
        "tests/test_app.py",
        "SRC/app.py", // case is not folded
    ] {
        assert!(!covers(&entries, not), "{not}");
    }
    // The whole folder, said either way.
    for whole in ["./", "."] {
        assert!(covers(&[whole.to_owned()], "anything/at/all.py"), "{whole}");
    }
    assert!(!covers(&[], "src/app.py"));
}

#[test]
fn a_folder_s_envelope_is_read_only_as_an_ordinary_file_of_a_page_s_size() {
    let dir = scratch("read");
    assert_eq!(read(&dir), Read::NotThere);
    std::fs::write(dir.join(FILE), WHOLE).unwrap();
    assert!(matches!(read(&dir), Read::Held(_)));
    std::fs::write(dir.join(FILE), "envelope-version = 1\n[task\n").unwrap();
    let Read::Unreadable(why) = read(&dir) else {
        panic!("TOML that does not parse is not an envelope");
    };
    assert!(why.starts_with(FILE) && !why.contains('\n'), "{why:?}");
    std::fs::write(dir.join(FILE), vec![b'#'; (MAX_BYTES + 1) as usize]).unwrap();
    assert!(matches!(read(&dir), Read::Unreadable(why) if why.contains("larger than")));
    std::fs::remove_file(dir.join(FILE)).unwrap();
    std::fs::create_dir(dir.join(FILE)).unwrap();
    assert!(matches!(read(&dir), Read::Unreadable(why) if why.contains("not an ordinary file")));
    std::fs::remove_dir_all(&dir).ok();
}

#[cfg(unix)]
#[test]
fn an_envelope_that_is_a_link_is_not_read_through() {
    let dir = scratch("link");
    let elsewhere = scratch("link-elsewhere");
    std::fs::write(elsewhere.join("real.toml"), WHOLE).unwrap();
    std::os::unix::fs::symlink(elsewhere.join("real.toml"), dir.join(FILE)).unwrap();
    // The setup: the link leads to a whole envelope, so only the refusal keeps it from being read.
    assert!(parse(&std::fs::read_to_string(dir.join(FILE)).unwrap()).is_ok());
    assert!(
        matches!(read(&dir), Read::Unreadable(why) if why.contains("a link to somewhere else"))
    );
    std::fs::remove_dir_all(&dir).ok();
    std::fs::remove_dir_all(&elsewhere).ok();
}
