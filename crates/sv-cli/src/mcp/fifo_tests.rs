//! A pipe under a name the server reads or writes (the review of 8 October 2026, item 6, for
//! `stackvet.toml`; backlog 0282 for the build-loop record and `report.json`): opening it waits for
//! the other end, and with it the server's one thread for answering. It is refused, with the reason.
#![cfg(unix)]
use super::tests::{call, scratch_app, text};
use super::*;

#[test]
fn a_pipe_under_a_name_the_server_reads_is_refused_rather_than_waited_on() {
    let root = scratch_app("fifo-manifest", "flask-booking");
    let manifest = root.join("app/stackvet.toml");
    std::fs::remove_file(&manifest).ok();
    let made = std::process::Command::new("mkfifo")
        .arg(&manifest)
        .status()
        .expect("mkfifo runs");
    // The setup: a pipe is really there, under the name the server reads.
    assert!(made.success());
    let kind = std::fs::symlink_metadata(&manifest).unwrap().file_type();
    assert!(
        std::os::unix::fs::FileTypeExt::is_fifo(&kind),
        "the setup: not a pipe"
    );
    let server = Server::new(&root).unwrap();
    let (sent, answered) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let result = call(&server, "stackvet_check", json!({ "path": "app" }));
        let _ = sent.send(result);
    });
    let result = answered.recv_timeout(std::time::Duration::from_secs(30));
    std::fs::remove_dir_all(&root).ok();
    let result = result.expect("the server was still reading the pipe after 30 seconds");
    assert_eq!(result["isError"], true, "{}", text(&result));
    assert!(
        text(&result).contains("stackvet.toml in app is not an ordinary file"),
        "{}",
        text(&result)
    );
}

/// A pipe made at `path`, checked to be one, so a test that finds it refused has really met a pipe.
fn pipe_at(path: &std::path::Path) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::remove_file(path).ok();
    let made = std::process::Command::new("mkfifo")
        .arg(path)
        .status()
        .expect("mkfifo runs");
    assert!(made.success());
    let kind = std::fs::symlink_metadata(path).unwrap().file_type();
    assert!(
        std::os::unix::fs::FileTypeExt::is_fifo(&kind),
        "the setup: not a pipe at {}",
        path.display()
    );
}

/// What `work` gave, or a failure naming `what` when it was still waiting after 30 seconds.
fn within_30_seconds<T: Send + 'static>(
    what: &str,
    work: impl FnOnce() -> T + Send + 'static,
) -> T {
    let (sent, answered) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = sent.send(work());
    });
    answered
        .recv_timeout(std::time::Duration::from_secs(30))
        .unwrap_or_else(|_| panic!("{what} was still waiting on the pipe after 30 seconds"))
}

#[test]
fn a_pipe_where_the_build_loop_record_goes_is_refused_and_the_server_still_answers() {
    // Backlog 0282: opening a pipe to write waits for a reader, and the record is written after
    // every tool call, on the server's one thread for answering.
    let root = scratch_app("fifo-build-loop", "flask-booking");
    let app = root.join("app");
    let record = sv_scan::ecosystems::build_loop_record_in(&app);
    pipe_at(&record);
    let refused = {
        let app = app.clone();
        within_30_seconds("writing the record", move || {
            crate::build_loop::record(&app, &crate::build_loop::Line::default())
        })
    };
    let server = Server::new(&root).unwrap();
    let answered = within_30_seconds("the server", move || {
        call(
            &server,
            "stackvet_prompts",
            json!({ "requirement": "V6.2.1" }),
        )
    });
    let still_a_pipe = std::fs::symlink_metadata(&record)
        .map(|m| std::os::unix::fs::FileTypeExt::is_fifo(&m.file_type()))
        .unwrap_or(false);
    std::fs::remove_dir_all(&root).ok();
    let error = format!("{:#}", refused.expect_err("a pipe is not written to"));
    assert!(error.contains("is not an ordinary file"), "{error}");
    assert!(answered.is_object(), "{answered}");
    assert!(still_a_pipe, "the pipe is left as it was, not replaced");
}

#[test]
fn a_pipe_where_the_report_sits_is_refused_rather_than_waited_on() {
    let root = scratch_app("fifo-report", "flask-booking");
    let report = sv_scan::ecosystems::default_report_dir_in(&root.join("app")).join("report.json");
    pipe_at(&report);
    let server = Server::new(&root).unwrap();
    let result = within_30_seconds("stackvet_prompts", move || {
        call(&server, "stackvet_prompts", json!({ "path": "app" }))
    });
    std::fs::remove_dir_all(&root).ok();
    assert_eq!(result["isError"], true, "{}", text(&result));
    assert!(
        text(&result).contains("report.json is not an ordinary file"),
        "{}",
        text(&result)
    );
}

#[test]
fn a_report_larger_than_the_server_reads_of_any_file_is_not_read() {
    // Backlog 0282: the report is read with the same cap as every other file the server reads.
    let root = scratch_app("oversized-report", "flask-booking");
    let report = sv_scan::ecosystems::default_report_dir_in(&root.join("app")).join("report.json");
    std::fs::create_dir_all(report.parent().unwrap()).unwrap();
    std::fs::File::create(&report)
        .unwrap()
        .set_len(MAX_RESOURCE_BYTES + 1)
        .unwrap();
    let server = Server::new(&root).unwrap();
    let result = call(&server, "stackvet_prompts", json!({ "path": "app" }));
    std::fs::remove_dir_all(&root).ok();
    assert_eq!(result["isError"], true, "{}", text(&result));
    assert!(
        text(&result).contains("is larger than the 16 MB"),
        "{}",
        text(&result)
    );
}
