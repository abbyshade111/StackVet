//! Backlog 0544: when `docker info` is refused the socket (permission denied), as inside an AI coding tool's sandbox,
//! `sv` says Docker is installed and out of this shell's reach, names the socket, and says where to run instead;
//! when nothing is listening, it still says there is no backend.

use super::*;

#[test]
fn each_way_docker_says_permission_denied_names_the_socket() {
    for (answer, socket) in [
        // Docker 24 to 27.
        (
            "permission denied while trying to connect to the Docker daemon socket at unix:///var/run/docker.sock: \
             Get \"http://%2Fvar%2Frun%2Fdocker.sock/v1.24/info\": dial unix /var/run/docker.sock: connect: \
             permission denied",
            "/var/run/docker.sock",
        ),
        // Docker 28 and later, and Colima's socket in the home folder.
        (
            "permission denied while trying to connect to the docker API at unix:///Users/sam/.colima/default/docker.sock",
            "/Users/sam/.colima/default/docker.sock",
        ),
        // A macOS sandbox refuses the connection itself.
        (
            "Cannot connect to the Docker daemon at unix:///Users/sam/.docker/run/docker.sock. Is the docker daemon \
             running?: dial unix /Users/sam/.docker/run/docker.sock: connect: operation not permitted",
            "/Users/sam/.docker/run/docker.sock",
        ),
        // Docker Desktop on Windows.
        (
            "permission denied while trying to connect to the Docker daemon at npipe:////./pipe/docker_engine",
            "//./pipe/docker_engine",
        ),
        // No address in the answer: the socket is still said to be the cause, without a path.
        ("ERROR: permission denied", "Docker's socket"),
    ] {
        assert_eq!(
            unreachable_socket(answer).as_deref(),
            Some(socket),
            "{answer}"
        );
        let Err(why) = linux_containers(1, answer) else {
            panic!("a refused `docker info` was taken for a backend: {answer}");
        };
        assert!(
            matches!(
                why,
                CannotRun::NoBackend {
                    unreachable: Some(_),
                    ..
                }
            ),
            "{why:?}"
        );
        let said = why.explain();
        assert!(
            said.starts_with(
                "Docker is installed but this shell cannot reach it (permission denied on "
            ) && said.contains(socket)
                && said
                    .contains("run `sv doctor` outside the tool's sandbox or in your own terminal")
                && said.contains("not assessed")
                && !said.contains("No container backend"),
            "{said}"
        );
        assert_eq!(
            why.kind(),
            "Docker is installed and this shell cannot reach it"
        );
    }
}

#[test]
fn nothing_listening_is_still_no_backend() {
    // The control: a daemon that is not running, or no `docker` at all, is not called installed.
    for answer in [
        "Cannot connect to the Docker daemon at unix:///var/run/docker.sock. Is the docker daemon running?",
        "error during connect: this error may indicate that the docker daemon is not running",
    ] {
        assert_eq!(unreachable_socket(answer), None, "{answer}");
        let Err(why) = linux_containers(1, answer) else {
            panic!("a failed `docker info` was taken for a backend: {answer}");
        };
        assert!(
            why.explain()
                .starts_with("No container backend on this computer"),
            "{}",
            why.explain()
        );
    }
}
