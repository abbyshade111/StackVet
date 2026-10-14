//! Backlog 0510: on Windows, `sv doctor` says beside its Docker line that the checks that start the app are untried
//! there, even when Docker answers; elsewhere it says nothing of Windows. "Can't tell" counts as nothing missing.

use super::*;
use std::path::PathBuf;

fn answers_on(windows: bool, backend: &dyn Fn() -> Result<(), CannotRun>) -> Vec<Line> {
    let dir = std::env::temp_dir().join(format!(
        "sv-doctor-windows-{windows}-{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    let lines = answers(
        &dir,
        &Asked {
            version: "sv 0.1.0 (commit test)",
            data: Ok(PathBuf::from("/data")),
            in_container: false,
            backend,
            windows,
        },
    );
    std::fs::remove_dir_all(&dir).ok();
    lines
}

#[test]
fn on_windows_the_line_after_docker_says_the_running_app_checks_are_untried() {
    let docker_ok = || Ok(());
    let lines = answers_on(true, &docker_ok);
    let docker = lines
        .iter()
        .position(|l| l.topic == "Docker")
        .expect("a Docker line");
    let windows = &lines[docker + 1];
    assert_eq!(windows.topic, "on Windows");
    assert_eq!(
        windows.state,
        State::CannotTell,
        "untried is neither ready nor missing"
    );
    assert!(
        windows.said.contains("have not been tried"),
        "{}",
        windows.said
    );
    assert!(windows.said.contains("sv report --run"), "{}", windows.said);
    assert!(
        text(std::path::Path::new("app"), &lines)
            .contains("have not been tried on a Windows computer")
    );
}

#[test]
fn elsewhere_nothing_is_said_of_windows() {
    let docker_ok = || Ok(());
    let lines = answers_on(false, &docker_ok);
    assert!(
        lines.iter().any(|l| l.topic == "Docker"),
        "the setup gave a Docker line"
    );
    assert!(!lines.iter().any(|l| l.topic == "on Windows"), "{lines:?}");
}
