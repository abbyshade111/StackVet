//! Backlog 0562: a refusal written as its log line and then its answer (`event(..., status=403)` above `send(403, ...)`)
//! is one place a question points at, not two; a refusal further down is still a place of its own. In the pointer's
//! second reading `examples/notes-with-users` said "and 2 more places", one of them app.py:210, the answer under
//! the log line at :209 already shown.

use serde_json::Value;
use std::process::Command;

/// The lines V8.1.1 points at in this app, and how many more it says there are.
fn pointed(source: &str) -> (Vec<u64>, u64) {
    let dir = sv_check::test_support::scratch("app");
    std::fs::write(
        dir.join("stackvet.toml"),
        "manifest-version = 1\n[app]\nname = \"x\"\n",
    )
    .unwrap();
    std::fs::write(dir.join("app.py"), source).unwrap();
    let out = dir.join("report");
    let run = Command::new(env!("CARGO_BIN_EXE_sv"))
        .args([
            "report",
            dir.to_str().unwrap(),
            "--no-tools",
            "--out",
            out.to_str().unwrap(),
        ])
        .output()
        .expect("sv runs");
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    let report: Value =
        serde_json::from_str(&std::fs::read_to_string(out.join("report.json")).unwrap()).unwrap();
    std::fs::remove_dir_all(&dir).ok();
    let question = report["questions_for_you"]
        .as_array()
        .expect("the questions are listed")
        .iter()
        .find(|q| q["id"] == "V8.1.1")
        .expect("V8.1.1 is asked of an app with no answers")
        .clone();
    let lines = question["rests_on"]["places"]
        .as_array()
        .expect("V8.1.1 carries the places its locators found")
        .iter()
        .map(|p| p["line"].as_u64().unwrap())
        .collect();
    (lines, question["rests_on"]["more"].as_u64().unwrap_or(0))
}

#[test]
fn a_refusal_and_its_log_line_are_one_place_and_one_further_down_is_another() {
    let (lines, more) = pointed(
        "def admin(self, ok):\n\
         \x20   if not ok:\n\
         \x20       self.event(\"refused\", status=403)\n\
         \x20       return self.send(403, \"No\")\n\
         \x20   return self.send(200, \"Yes\")\n\
         \n\
         \n\
         \n\
         def other(self):\n\
         \x20   return self.send(403, \"No\")\n",
    );
    // The setup: the locator found the refusals at all, or nothing here says anything.
    assert!(
        lines.contains(&3),
        "the log line of the first refusal is shown: {lines:?}"
    );
    assert!(
        !lines.contains(&4),
        "its answer, a line below, is the same place: {lines:?}"
    );
    assert!(
        lines.contains(&10),
        "a refusal further down is a place of its own: {lines:?}"
    );
    assert_eq!(
        more, 0,
        "nothing is counted twice among the more: {lines:?}"
    );
}

#[test]
fn the_example_app_says_one_more_place_the_owner_scoped_update() {
    // The item's own case: notes-with-users shows 241, 209, and 316, and the one more is the update at :336 that moves
    // a person's notes to their new email, a query by owner.
    let example = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/notes-with-users/app.py");
    let source = std::fs::read_to_string(example).unwrap();
    assert!(
        source
            .lines()
            .nth(335)
            .is_some_and(|l| l.contains("where owner = ?")),
        "the setup: app.py:336 is the update"
    );
    let (lines, more) = pointed(&source);
    assert_eq!(lines, [241, 209, 316], "{lines:?}");
    assert_eq!(
        more, 1,
        "app.py:210 is the refusal at :209, not a place of its own"
    );
}
