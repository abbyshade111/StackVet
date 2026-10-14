//! What a run reads of its app's container from the engine (backlog 0243). A fake `docker` answers the two
//! commands the run sends, with the words the real engine printed on 11 October 2026.

use super::*;

/// A backend whose `docker` is `script`, written to a scratch folder of this test's own.
fn backend_answering(test: &str, script: &str) -> DockerBackend {
    let dir =
        std::env::temp_dir().join(format!("sv-resource-reading-{test}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let docker = dir.join("docker");
    sv_check::test_support::executable(&docker, script);
    DockerBackend {
        binary: docker.to_string_lossy().into_owned(),
        ..DockerBackend::new()
    }
}

/// The fake engine: `stats` prints the reading line, `inspect` the limits line, anything else prints nothing.
const ANSWERS: &str = "#!/bin/sh\n\
    case \"$1\" in\n\
    stats) echo \"0.00% | 404KiB / 64MiB | 736B / 126B | 0B / 0B\" ;;\n\
    inspect) echo \"nanocpus=500000000 mem=67108864\" ;;\n\
    esac\n";

#[test]
fn a_running_app_is_read_from_the_engines_own_words() {
    let backend = backend_answering("answers", ANSWERS);
    let taken = backend
        .resource_readings("run-app")
        .expect("the engine answered both commands, so something is read");
    assert_eq!(
        taken.reading.mem_used,
        404 * 1024,
        "the reading is not empty"
    );
    assert_eq!(taken.reading.mem_limit, 64 * 1024 * 1024);
    assert_eq!(taken.limits.nano_cpus, 500_000_000);
    assert_eq!(taken.limits.mem_bytes, 67_108_864);
}

#[test]
fn an_engine_that_will_not_say_gives_nothing_rather_than_a_guess() {
    // Stats fail: nothing is read.
    let failing = backend_answering("stats-fail", "#!/bin/sh\nexit 1\n");
    assert_eq!(failing.resource_readings("run-app"), None);
    // Stats answer in words the reader does not know: nothing is read.
    let unknown = backend_answering("unknown", "#!/bin/sh\necho \"all good\"\n");
    assert_eq!(unknown.resource_readings("run-app"), None);
}
