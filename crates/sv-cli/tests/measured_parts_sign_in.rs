//! Backlog 0609 (from 0491): the notes question on how sign-in is defended (V6.1.1) shows what the signed-in run saw
//! of wrong passwords, the check of V6.3.1, with this report's own verdict, and says it does not answer the question.

use std::process::Command;

#[test]
fn the_sign_in_defenses_question_lists_the_wrong_password_check_with_its_verdict() {
    let dir = std::env::temp_dir().join(format!("sv-measured-sign-in-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("app.py"), "def home():\n    return 'hi'\n").unwrap();
    std::fs::write(
        dir.join("stackvet.toml"),
        "manifest-version = 1\n\n[app]\nname = \"Notes\"\naudience = \"customers\"\n\
         deployment = \"local-only\"\n\n[stack]\nlanguages = [\"python\"]\n\n[capabilities]\nauth = true\n",
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
        .find(" - V6.1.1:")
        .unwrap_or_else(|| panic!("V6.1.1 is asked:\n{text}"));
    let rest = &text[start..];
    let question = &rest[..rest[1..].find("\n - ").map_or(rest.len(), |e| e + 1)];
    assert!(
        question.contains(sv_check::human::MEASURED_SAY),
        "{question}"
    );
    let line = question
        .lines()
        .find(|l| {
            l.trim_start()
                .starts_with("V6.3.1, wrong passwords in a row, pushed back on at the number stackvet.toml states: ")
        })
        .unwrap_or_else(|| panic!("V6.3.1 is listed:\n{question}"));
    // Read without a run, the check of the running app has not been made, and the verdict says so.
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
