//! Backlog 0518, end to end through the binary: on `notes-with-users`, the question whether authorization is enforced
//! on the server (V8.3.1) points at the admin page's refusal, never at the refusal of a forged form.

use std::process::Command;

#[test]
fn notes_with_users_is_pointed_at_its_admin_refusal_and_not_at_a_forged_form() {
    let app = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/notes-with-users");
    let run = Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(["questions", app.to_str().unwrap()])
        .output()
        .expect("sv runs");
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    let text = String::from_utf8_lossy(&run.stdout);
    let v831 = &text[text.find(" - V8.3.1: ").expect("V8.3.1 is asked")..];
    let v831 = &v831[..v831[1..].find("\n - ").map_or(v831.len(), |n| n + 1)];
    // The setup worked: the question carries lines, or not finding :137 would mean nothing.
    assert!(v831.contains("In the code"), "{v831}");
    assert!(
        v831.contains("app.py, line 213:") || v831.contains("app.py, line 214:"),
        "the admin page's refusal is not shown: {v831}"
    );
    assert!(
        !v831.contains("app.py, line 137:"),
        "the refusal of a forged form is shown: {v831}"
    );
}
