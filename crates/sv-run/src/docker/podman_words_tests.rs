//! Backlog 0519, from the Podman measurement of 0508: Podman named as Podman when it answers for `docker`, and a
//! container that has gone recognized whatever case the backend writes it in, and told in `sv`'s own words.

use super::*;

/// What Podman 4.9.3 answers `docker info --format {{.OSType}}`, through its `docker` command.
const PODMAN_INFO: &str = "Error: template: info:1:2: executing \"info\" at <.OSType>: can't evaluate field OSType \
                           in type *define.Info\n";

#[test]
fn podman_answering_for_docker_is_named_and_is_no_backend() {
    let Err(absent) = linux_containers(125, PODMAN_INFO) else {
        panic!("Podman was taken for a backend");
    };
    assert!(matches!(absent, CannotRun::NoBackend { .. }));
    let said = absent.explain();
    for part in [
        "Podman",
        "does not run the app under yet",
        "need Docker",
        "not assessed",
    ] {
        assert!(said.contains(part), "{part} in {said}");
    }
    assert!(
        !said.contains("define.Info"),
        "Podman's template error was shown: {said}"
    );
    // Docker failing for its own reasons is still told in Docker's words.
    let Err(docker) = linux_containers(1, "Cannot connect to the Docker daemon\n") else {
        panic!("a failed `docker info` was taken for a backend");
    };
    assert!(!docker.explain().contains("Podman"), "{}", docker.explain());
}

#[test]
fn a_gone_container_is_recognized_in_podmans_words_and_told_in_svs() {
    let backend = DockerBackend::new();
    assert!(backend.note_if_sidecar_lost(
        "probe-1",
        125,
        "Error: no container with name or ID \"sv-1-probe\" found: no such container\n"
    ));
    let lost = backend.sidecar_lost().expect("written down");
    assert!(
        lost.contains("`probe-1`") && lost.contains("had stopped, or been removed"),
        "{lost}"
    );
    assert!(
        !lost.contains("no such container"),
        "the backend's words were passed on: {lost}"
    );
}
