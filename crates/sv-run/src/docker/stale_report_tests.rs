//! Backlog 0646: the stale test report is removed with its path as an argument of its own, so a quote in the path is
//! part of the name, and nothing after it runs. Takes the honest-absence path where no container backend answers; CI,
//! which must have one, fails `a_container_backend_is_here_where_ci_says_it_must_be` instead.

use super::*;

/// What `sh` in the container says of `test`: whether it holds.
fn holds(backend: &DockerBackend, app: &str, test: &str) -> bool {
    backend
        .docker(&["exec", app, "sh", "-c", test])
        .is_ok_and(|(code, _)| code == 0)
}

/// Removes the test's container when the test ends, passed or failed.
struct Removed<'a>(&'a DockerBackend, String);

impl Drop for Removed<'_> {
    fn drop(&mut self) {
        self.0.docker(&["rm", "-f", &self.1]).ok();
    }
}

#[test]
fn a_quote_in_the_report_path_is_part_of_its_name_and_runs_nothing() {
    let backend = DockerBackend::new();
    if backend.available().is_err() {
        println!("no container backend here; the removal is not tried");
        return;
    }
    let app = format!("sv-stale-report-{}", std::process::id());
    // Read-only, as the backend starts every container, with the report folder the app's own run gets.
    let started = backend.docker(&[
        "run",
        "-d",
        "--rm",
        "--name",
        &app,
        "--tmpfs",
        crate::REPORT_DIR,
        "busybox:1.36",
        "sleep",
        "120",
    ]);
    let _removed = Removed(&backend, app.clone());
    assert!(
        started.as_ref().is_ok_and(|(code, _)| *code == 0),
        "the container did not start: {started:?}"
    );
    // The setup, checked before anything rests on it.
    assert!(holds(
        &backend,
        &app,
        "touch \"/sv-reports/it's.xml\" /sv-reports/keep && test -f \"/sv-reports/it's.xml\" && test -f /sv-reports/keep"
    ));

    // What a quote could do, first: a path that closes the old quoting and runs two commands after it.
    let injected = backend.remove_stale_report(
        &app,
        "/sv-reports/x'; touch /sv-reports/ran; rm -f /sv-reports/keep; '",
    );
    let ran = holds(&backend, &app, "test -e /sv-reports/ran");
    let kept = holds(&backend, &app, "test -e /sv-reports/keep");
    assert!(
        !ran && kept,
        "nothing written after a quote in the path ran"
    );
    assert!(
        injected,
        "a name that is not there is removed as nothing, as `rm -f` does"
    );

    // Then a report whose name holds a quote is removed.
    let quoted = backend.remove_stale_report(&app, "/sv-reports/it's.xml");
    let gone = holds(&backend, &app, "test ! -e \"/sv-reports/it's.xml\"");
    assert!(quoted && gone, "a report named with a quote is removed");
}
