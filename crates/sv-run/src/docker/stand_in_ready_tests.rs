//! Whether the test model and the test sign-in provider answer is asked once for the run (backlog 0369). It was asked
//! on each of the script's calls, and a stand-in that never came up cost ten seconds of waiting each time.

use super::*;

/// A backend whose `docker` answers every request with a 200, and writes a line to `calls` each time it is run.
fn backend(test: &str) -> (DockerBackend, std::path::PathBuf) {
    let dir = std::env::temp_dir().join(format!("sv-stand-in-ready-{test}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    let calls = dir.join("calls");
    let docker = dir.join("docker");
    sv_check::test_support::executable(
        &docker,
        &format!(
            // A session kept open in the sidecar (backlog 0341) is refused, so each question is its own exec.
            "#!/bin/sh\ncase \"$*\" in *@@sv-session-*) exit 1;; esac\ncat > /dev/null\necho run >> '{}'\nprintf 'HTTP/1.1 200 OK\\r\\nContent-Length: 2\\r\\n\\r\\nok'\n",
            calls.display()
        ),
    );
    let backend = DockerBackend {
        binary: docker.to_string_lossy().into_owned(),
        ..DockerBackend::new()
    };
    (backend, calls)
}

fn runs(calls: &std::path::Path) -> usize {
    std::fs::read_to_string(calls).map_or(0, |t| t.lines().count())
}

#[test]
fn the_model_and_the_provider_are_each_asked_once_for_the_run() {
    let (backend, calls) = backend("once");
    let mut manifest = sv_manifest::Manifest::default();
    manifest.stack.run.image = Some("python:3.12".to_owned());
    manifest.stack.run.start = Some("gunicorn app:app".to_owned());
    let plan = crate::RunPlan::from_manifest(&manifest, std::path::Path::new("/tmp/app")).unwrap();
    let via = Via::Sidecar("sidecar");
    let run = DockerRun {
        backend: &backend,
        via: &via,
        app: "app",
        switched_off: "app-off",
        app_args: &[],
        plan: &plan,
        mail: None,
        provider: Some("provider"),
        browser: None,
        model: Some("model"),
        model_up: OnceLock::new(),
        provider_up: OnceLock::new(),
    };
    // The setup: the stand-in `docker` is run, and its answer reads as the model answering.
    assert_eq!(run.model_answering(), Some("model"));
    assert_eq!(runs(&calls), 1, "the first question is asked");
    for _ in 0..4 {
        assert_eq!(run.model_answering(), Some("model"));
    }
    assert_eq!(runs(&calls), 1, "asked once, not on every call");
    for _ in 0..3 {
        assert_eq!(run.provider_answering(), Some("provider"));
    }
    assert_eq!(runs(&calls), 2, "the provider is asked once too");
}
