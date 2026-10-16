//! Backlog 0610: the hand check on data protection (V14.2.4) shows the checks that measure parts of it, each with this
//! report's own verdict, and says they do not answer the question.

use std::path::PathBuf;
use std::process::Command;

fn app() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-measured-parts-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("app.py"), "def home():\n    return 'hi'\n").unwrap();
    std::fs::write(
        dir.join("stackvet.toml"),
        "manifest-version = 1\n\n[app]\nname = \"Parts\"\naudience = \"customers\"\n\
         deployment = \"local-only\"\n\n[stack]\nlanguages = [\"python\"]\n\n[capabilities]\nauth = true\n",
    )
    .unwrap();
    dir
}

#[test]
fn the_data_protection_check_lists_its_measured_parts_with_their_verdicts() {
    let dir = app();
    let out = Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(["questions", dir.to_str().unwrap()])
        .output()
        .expect("sv runs");
    std::fs::remove_dir_all(&dir).ok();
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let start = text
        .find(" - V14.2.4:")
        .unwrap_or_else(|| panic!("V14.2.4 is asked:\n{text}"));
    let rest = &text[start..];
    let question = &rest[..rest[1..].find("\n - ").map_or(rest.len(), |e| e + 1)];
    assert!(
        question.contains(sv_check::human::MEASURED_SAY),
        "{question}"
    );
    for (id, what) in [
        ("V14.2.1", "sensitive data kept out of web addresses"),
        ("V14.3.2", "private pages not kept in a browser's cache"),
        (
            "V16.2.5",
            "sensitive data kept out of the logs, or masked in them",
        ),
        (
            "V14.3.1",
            "what the browser kept for a signed-in person, cleared when they sign out",
        ),
    ] {
        let line = question
            .lines()
            .find(|l| l.trim_start().starts_with(&format!("{id}, {what}: ")))
            .unwrap_or_else(|| panic!("{id} is listed:\n{question}"));
        // Each verdict is one the report gives, never a word of the app's.
        assert!(
            [
                "needs attention",
                "checked",
                "not verified",
                "not one this app is held to"
            ]
            .iter()
            .any(|v| line.trim_end().ends_with(v)),
            "{line}"
        );
    }
}
