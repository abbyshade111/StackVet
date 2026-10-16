//! Backlog 0498: the hand check on availability defenses (V15.2.2) shows the running checks that measure parts of it,
//! creating without limit (V2.4.1) and a file too large (V5.2.1), each with this report's own verdict, and says they do
//! not answer the question.

use std::process::Command;

#[test]
fn the_availability_check_lists_its_measured_parts_with_their_verdicts() {
    let dir = std::env::temp_dir().join(format!("sv-measured-availability-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("app.py"), "def home():\n    return 'hi'\n").unwrap();
    std::fs::write(
        dir.join("stackvet.toml"),
        "manifest-version = 1\n\n[app]\nname = \"Busy\"\naudience = \"customers\"\n\
         deployment = \"local-only\"\n\n[stack]\nlanguages = [\"python\"]\n\n[capabilities]\nauth = true\n\
         uploads = true\n",
    )
    .unwrap();
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
        .find(" - V15.2.2:")
        .unwrap_or_else(|| panic!("V15.2.2 is asked:\n{text}"));
    let rest = &text[start..];
    let question = &rest[..rest[1..].find("\n - ").map_or(rest.len(), |e| e + 1)];
    assert!(
        question.contains(sv_check::human::MEASURED_SAY),
        "{question}"
    );
    for (id, what) in [
        (
            "V2.4.1",
            "how fast one person can make the app do work, limited",
        ),
        ("V5.2.1", "a file too large for the app to handle, refused"),
    ] {
        let line = question
            .lines()
            .find(|l| l.trim_start().starts_with(&format!("{id}, {what}: ")))
            .unwrap_or_else(|| panic!("{id} is listed:\n{question}"));
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
