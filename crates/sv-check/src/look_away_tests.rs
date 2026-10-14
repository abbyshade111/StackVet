//! The comments that tell a scanner to look away, one of each scanner's, read from a real folder (backlog 0400).

use super::*;

fn rules() -> &'static SecretRules {
    SecretRules::shared().expect("the rules sv ships")
}

/// An app folder holding `files`, read.
fn read_app(files: &[(&str, &str)]) -> LookAway {
    let dir = crate::test_support::scratch("look-away");
    for (relative, text) in files {
        let path = dir.join(relative);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }
    let read = read(&Listing::of(&dir), rules());
    let _ = std::fs::remove_dir_all(&dir);
    read
}

fn listed(read: &LookAway) -> Vec<(String, usize, &'static str, String)> {
    read.comments
        .iter()
        .map(|c| (c.file.clone(), c.line, c.scanner, c.says.clone()))
        .collect()
}

#[test]
fn each_scanners_comment_is_listed_with_its_file_line_and_scanner() {
    let read = read_app(&[
        (
            "app.py",
            "import subprocess\nhost = '0.0.0.0'  # nosec B104\nsubprocess.call(cmd, shell=True)  # nosemgrep: python.lang.security.audit.subprocess-shell-true\n",
        ),
        (
            "main.go",
            "package main\n\nfunc f() {\n\th := md5.New() // #nosec G401 -- checksum only\n\t//gosec:disable G101\n}\n",
        ),
        (
            "web/index.js",
            "eval(input); // lgtm[js/code-injection]\n/* codeql[js/xss] the page escapes it */\nconst ok = 1;\n",
        ),
        (
            "views/page.html",
            "<!-- nosemgrep --><div>{{ raw }}</div>\n",
        ),
    ]);
    assert_eq!(
        listed(&read),
        vec![
            ("app.py".into(), 2, "Bandit", "nosec B104".into()),
            (
                "app.py".into(),
                3,
                "Semgrep",
                "nosemgrep: python.lang.security.audit.subprocess-shell-true".into()
            ),
            (
                "main.go".into(),
                4,
                "gosec",
                "nosec G401 -- checksum only".into()
            ),
            ("main.go".into(), 5, "gosec", "gosec:disable G101".into()),
            ("views/page.html".into(), 1, "Semgrep", "nosemgrep".into()),
            (
                "web/index.js".into(),
                1,
                "CodeQL",
                "lgtm[js/code-injection]".into()
            ),
            (
                "web/index.js".into(),
                2,
                "CodeQL",
                "codeql[js/xss] the page escapes it".into()
            ),
        ]
    );
    assert!(read.not_read.is_empty(), "{:?}", read.not_read);
}

#[test]
fn the_words_outside_a_comment_or_inside_another_word_are_not_listed() {
    let read = read_app(&[(
        "app.py",
        "nosec = True\nprint('lgtm')\nnosecurity = 1  # nosecurity is a name, not the word\nlgtms = []\n",
    )]);
    assert!(read.comments.is_empty(), "{:#?}", read.comments);
}

#[test]
fn files_that_are_not_code_are_not_read_for_them() {
    let read = read_app(&[
        ("NOTES.md", "We added # nosec to app.py.\n"),
        ("app.py", "x = 1\n"),
    ]);
    assert!(read.comments.is_empty(), "{:#?}", read.comments);
}

#[test]
fn a_copy_of_another_library_is_named_and_not_read() {
    let read = read_app(&[(
        "public/js/jquery.min.js",
        "/*! jQuery v3.6.1 | (c) OpenJS Foundation and other contributors | jquery.org/license */\n// lgtm\n",
    )]);
    assert!(read.comments.is_empty(), "{:#?}", read.comments);
    assert_eq!(read.not_read.len(), 1, "{:?}", read.not_read);
    assert!(
        read.not_read[0].contains("public/js/jquery.min.js"),
        "{:?}",
        read.not_read
    );
}

#[test]
fn each_warning_brakemans_ignore_file_lists_is_listed_where_the_warning_is() {
    let ignore = serde_json::json!({
        "ignored_warnings": [
            { "warning_type": "SQL Injection", "fingerprint": "abc", "file": "app/models/user.rb", "line": 12,
              "note": "the AI tool said it was safe" },
            { "warning_type": "Cross-Site Scripting", "fingerprint": "def", "file": "app/views/a.html.erb", "line": 3,
              "note": "" }
        ],
        "brakeman_version": "6.2.1"
    });
    let read = read_app(&[("config/brakeman.ignore", &ignore.to_string())]);
    assert_eq!(
        listed(&read),
        vec![
            (
                "app/models/user.rb".into(),
                12,
                "Brakeman",
                "SQL Injection, listed in config/brakeman.ignore: the AI tool said it was safe"
                    .into()
            ),
            (
                "app/views/a.html.erb".into(),
                3,
                "Brakeman",
                "Cross-Site Scripting, listed in config/brakeman.ignore".into()
            ),
        ]
    );
    let read = read_app(&[("config/brakeman.ignore", "not json")]);
    assert!(read.comments.is_empty());
    assert_eq!(read.not_read.len(), 1, "{:?}", read.not_read);
}

#[test]
fn a_key_on_the_line_is_cut_before_it_is_quoted() {
    // Built from pieces, so this file holds no key.
    let key = ["sk", "ant", "api03", "Hp4Tz8Wq2Lr6Vn1Kd5Yb0Xm3"].join("-");
    let read = read_app(&[(
        "app.py",
        &format!("KEY = load()  # nosec B105 the old key was {key}\n"),
    )]);
    assert_eq!(read.comments.len(), 1, "the setup: {:#?}", read.comments);
    assert!(
        !read.comments[0].says.contains(&key),
        "{}",
        read.comments[0].says
    );
}
