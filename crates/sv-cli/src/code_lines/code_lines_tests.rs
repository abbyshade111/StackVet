//! The lines a code finding is on, kept for "The code" (backlog 0378, part 2): the line and the one either side,
//! redacted together, read only from inside the app's folder.

use super::*;

fn rules() -> SecretRules {
    SecretRules::load(
        &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/secret-rules.json"),
    )
    .expect("rules load")
}

fn at(file: &str, line: usize) -> sv_check::Finding {
    sv_check::Finding {
        evidence: Vec::new(),
        also_reported_by: Vec::new(),
        fingerprint: String::new(),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        outranked: None,
        also_on_this_line: Vec::new(),
        rule_id: "ast.x".into(),
        title: "t".into(),
        severity: sv_check::Severity::High,
        confidence: sv_check::Confidence::High,
        location: sv_check::Location {
            file: file.into(),
            line,
        },
        secret: None,
        requirement_ids: Vec::new(),
        cwe: Vec::new(),
        description: String::new(),
        impact: String::new(),
        fix: String::new(),
    }
}

fn folder(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-code-lines-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn a_finding_gets_its_line_and_the_one_either_side_with_a_key_on_them_cut() {
    let dir = folder("window");
    // A key built from pieces, so this file holds none.
    let key = ["sk_live_", "4eC39HqLyjWDarjtT1", "zdp7dc"].concat();
    std::fs::write(
        dir.join("app.py"),
        format!("import os\nSTRIPE = \"{key}\"\nquery = \"SELECT * FROM notes WHERE id = \" + id\nprint(query)\n"),
    )
    .unwrap();
    let got = read(&dir, &[at("app.py", 3)], &rules());
    let lines = got
        .get(&("app.py".to_owned(), 3))
        .expect("the finding's lines");
    assert_eq!(
        lines.iter().map(|(n, _)| *n).collect::<Vec<_>>(),
        vec![2, 3, 4]
    );
    assert!(lines[1].1.contains("SELECT * FROM notes"), "{lines:?}");
    // The setup: the key was on line 2, which is shown, so leaving it out is the redaction's doing.
    assert!(
        std::fs::read_to_string(dir.join("app.py"))
            .unwrap()
            .contains(&key)
    );
    assert!(
        lines.iter().all(|(_, text)| !text.contains(&key)),
        "the key reached the report: {lines:?}"
    );
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn nothing_outside_the_apps_folder_is_read() {
    let dir = folder("outside");
    let app = dir.join("app");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(dir.join("secret.txt"), "outside the app\n").unwrap();
    for file in ["../secret.txt", "/etc/hosts", ""] {
        assert!(
            read(&app, &[at(file, 1)], &rules()).is_empty(),
            "{file} was read"
        );
    }
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(dir.join("secret.txt"), app.join("linked.txt")).unwrap();
        assert!(
            read(&app, &[at("linked.txt", 1)], &rules()).is_empty(),
            "a link out of the folder was followed"
        );
    }
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn the_first_and_last_lines_have_no_line_past_them_and_a_long_line_is_cut() {
    let dir = folder("edges");
    std::fs::write(dir.join("a.js"), format!("{}\nend\n", "x".repeat(500))).unwrap();
    let got = read(&dir, &[at("a.js", 1), at("a.js", 2)], &rules());
    assert_eq!(
        got[&("a.js".to_owned(), 1)]
            .iter()
            .map(|(n, _)| *n)
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_eq!(
        got[&("a.js".to_owned(), 2)]
            .iter()
            .map(|(n, _)| *n)
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert!(
        got[&("a.js".to_owned(), 1)][0]
            .1
            .ends_with("(300 more characters)")
    );
    std::fs::remove_dir_all(&dir).ok();
}
