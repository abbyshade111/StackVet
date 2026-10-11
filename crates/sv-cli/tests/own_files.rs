//! `sv`'s own files (backlog 0237): an opt-in `SV_LOG` file of stages and the command, and a crash file under the
//! history folder when a panic reaches `main`. Neither holds the arguments, the paths the owner typed, or a panic's
//! message.

use std::path::{Path, PathBuf};
use std::process::Command;

fn scratch(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("sv-own-files-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&root).ok();
    std::fs::create_dir_all(&root).unwrap();
    root
}

fn copy_dir(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = to.join(entry.file_name());
        if entry.path().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), &target).unwrap();
        }
    }
}

#[test]
fn sv_log_names_the_stages_and_the_command_and_not_the_paths() {
    let root = scratch("log");
    let app = root.join("app");
    copy_dir(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/notes-with-users"),
        &app,
    );
    let out = root.join("report");
    let log = root.join("sv.log");
    Command::new(env!("CARGO_BIN_EXE_sv"))
        .env_remove("RUST_BACKTRACE")
        .env("XDG_CONFIG_HOME", root.join("config"))
        .env("SV_LOG", &log)
        .args([
            "report",
            app.to_str().unwrap(),
            "--out",
            out.to_str().unwrap(),
        ])
        .output()
        .expect("sv runs");
    // The setup: the log was written, so the checks on it mean something.
    let text = std::fs::read_to_string(&log).expect("SV_LOG was written");
    assert!(text.contains(" command report"), "{text}");
    assert!(text.contains(" stage 0: "), "{text}");
    // Nothing the owner typed: not the app's folder, not the output folder.
    assert!(!text.contains(app.to_str().unwrap()), "{text}");
    assert!(!text.contains(out.to_str().unwrap()), "{text}");
}

#[test]
fn without_sv_log_no_log_is_written() {
    let root = scratch("nolog");
    let app = root.join("app");
    copy_dir(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/notes-with-users"),
        &app,
    );
    let out = root.join("report");
    let log = root.join("sv.log");
    Command::new(env!("CARGO_BIN_EXE_sv"))
        .env_remove("RUST_BACKTRACE")
        .env_remove("SV_LOG")
        .env("XDG_CONFIG_HOME", root.join("config"))
        .args([
            "report",
            app.to_str().unwrap(),
            "--out",
            out.to_str().unwrap(),
        ])
        .output()
        .expect("sv runs");
    assert!(!log.exists(), "a log was written without SV_LOG");
}

#[test]
fn a_panic_leaves_a_crash_file_with_the_command_and_its_place_and_no_arguments() {
    let root = scratch("crash");
    let data = root.join("data");
    // An argument naming a folder the owner typed: it must not reach the file.
    let typed = "/owner-only-folder/secret-dir";
    let ran = Command::new(env!("CARGO_BIN_EXE_sv"))
        .env_remove("RUST_BACKTRACE")
        .env("SV_PANIC_FOR_TEST", "1")
        .env("XDG_DATA_HOME", &data)
        .args(["check", typed])
        .output()
        .expect("sv runs");
    assert_eq!(
        ran.status.code(),
        Some(3),
        "{}",
        String::from_utf8_lossy(&ran.stderr)
    );
    // The setup: a crash file was written, under the history folder.
    let folder = data.join(sv_frameworks::names::CONFIG_DIR).join("history");
    let file = std::fs::read_dir(&folder)
        .expect("the history folder was made")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .find(|p| {
            p.file_name()
                .is_some_and(|n| n.to_string_lossy().starts_with("crash-"))
        })
        .expect("a crash file was written");
    let text = std::fs::read_to_string(file).unwrap();
    assert!(text.contains("command: check"), "{text}");
    assert!(text.contains("place: "), "{text}");
    assert!(text.contains("version: "), "{text}");
    // Not the arguments, and not the panic's message.
    assert!(!text.contains(typed), "{text}");
    assert!(
        !text.contains("a panic asked for by SV_PANIC_FOR_TEST"),
        "{text}"
    );
}
