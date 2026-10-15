//! Backlog 0544: `sv doctor` in a shell kept from Docker's socket, as an AI coding tool's sandbox keeps it, says Docker
//! is installed and out of reach, as "can't tell" rather than "not ready", so nobody is sent to install what they have.

use super::*;
use std::path::PathBuf;

fn docker_line(backend: &dyn Fn() -> Result<(), CannotRun>) -> (Line, String) {
    let dir = std::env::temp_dir().join(format!("sv-doctor-unreachable-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let lines = answers(
        &dir,
        &Asked {
            version: "sv 0.1.0 (commit test)",
            data: Ok(PathBuf::from("/data")),
            in_container: false,
            backend,
            windows: false,
        },
    );
    let printed = text(&dir, &lines);
    std::fs::remove_dir_all(&dir).ok();
    let docker = lines
        .into_iter()
        .find(|l| l.topic == "Docker")
        .expect("doctor answers about Docker");
    (docker, printed)
}

#[test]
fn a_shell_kept_from_the_socket_is_told_docker_is_installed_and_where_to_run() {
    let refused = || {
        Err(CannotRun::NoBackend {
            checked: "`docker info` failed: permission denied".to_owned(),
            unreachable: Some("/var/run/docker.sock".to_owned()),
        })
    };
    let (docker, printed) = docker_line(&refused);
    assert_eq!(docker.state, State::CannotTell, "{}", docker.said);
    assert!(
        printed.contains(
            "Docker is installed but this shell cannot reach it (permission denied on /var/run/docker.sock)"
        ) && printed.contains("run `sv doctor` outside the tool's sandbox or in your own terminal"),
        "{printed}"
    );
    assert!(!printed.contains("No container backend"), "{printed}");
}

#[test]
fn no_docker_at_all_is_still_not_ready() {
    // The control: with nothing there, the line is "not ready" and says so, as before.
    let absent = || {
        Err(CannotRun::NoBackend {
            checked: "`docker` could not be started: not found".to_owned(),
            unreachable: None,
        })
    };
    let (docker, _) = docker_line(&absent);
    assert_eq!(docker.state, State::NotReady);
    assert!(
        docker.said.starts_with("No container backend"),
        "{}",
        docker.said
    );
}
