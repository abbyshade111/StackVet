//! Backlog 0544: a report made in a shell kept from Docker's socket files the run as stopped (`docker info` ran and
//! was refused), never as a container backend that is not installed.

use super::*;

#[test]
fn docker_out_of_reach_is_stopped_and_docker_absent_is_not_installed() {
    let out_of_reach = sv_run::CannotRun::NoBackend {
        checked: "`docker info` failed: permission denied".to_owned(),
        unreachable: Some("/var/run/docker.sock".to_owned()),
    };
    assert_eq!(run_gap_reason(&out_of_reach), sv_report::GapReason::Stopped);
    let absent = sv_run::CannotRun::NoBackend {
        checked: "`docker` could not be started: not found".to_owned(),
        unreachable: None,
    };
    assert_eq!(run_gap_reason(&absent), sv_report::GapReason::NotInstalled);
}
