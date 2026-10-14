use super::*;
#[cfg(unix)]
use std::process::Command;

#[cfg(unix)]
#[test]
fn a_script_written_while_other_threads_start_programs_always_runs() {
    let dir = std::env::temp_dir().join(format!("sv-executable-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let busy = std::sync::atomic::AtomicUsize::new(0);
    std::thread::scope(|threads| {
        for t in 0..8 {
            let (dir, busy) = (&dir, &busy);
            threads.spawn(move || {
                for i in 0..60 {
                    let path = dir.join(format!("s{t}-{i}"));
                    executable(&path, "#!/bin/sh\nexit 0\n");
                    match Command::new(&path).status() {
                        Ok(status) => assert!(status.success(), "{}", path.display()),
                        // 26 is ETXTBSY, "text file busy", on Linux and macOS alike.
                        Err(e) if e.raw_os_error() == Some(26) => {
                            busy.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        }
                        Err(e) => panic!("{}: {e}", path.display()),
                    }
                }
            });
        }
    });
    std::fs::remove_dir_all(&dir).ok();
    assert_eq!(busy.into_inner(), 0, "a script was busy when run");
}

#[test]
fn no_test_here_writes_a_program_in_its_own_process() {
    // Every test of the workspace that writes a program to run goes through `executable`: this
    // crate's own since 9 October 2026, and `sv-run`'s, `sv-cli`'s, and this crate's `tests/` since
    // backlog 0501, when `sv-run`'s stand-in `docker` met the race in CI.
    let crates = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let mut files = Vec::new();
    fn walk(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
        for entry in std::fs::read_dir(dir).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push(path);
            }
        }
    }
    for krate in std::fs::read_dir(&crates).unwrap().flatten() {
        for part in ["src", "tests"] {
            let dir = krate.path().join(part);
            if dir.is_dir() {
                walk(&dir, &mut files);
            }
        }
    }
    // The setup: the walk reached the crates whose tests write programs.
    for seen in [
        "sv-check/src/git.rs",
        "sv-run/src/docker/container_record_tests.rs",
        "sv-cli/tests/moved.rs",
    ] {
        assert!(
            files.iter().any(|f| f.ends_with(seen)),
            "the walk did not reach {seen}: {} files",
            files.len()
        );
    }
    let own = Path::new("test_support");
    // A folder's permissions put back after a test made it unreadable: no program is written.
    let folder_only = Path::new("sv-scan/src/files.rs");
    let offenders: Vec<String> = files
        .iter()
        .filter(|f| !f.ends_with("test_support.rs") && !f.parent().unwrap().ends_with(own))
        .filter(|f| !f.ends_with(folder_only))
        .filter(|f| {
            std::fs::read_to_string(f)
                .unwrap()
                .contains("from_mode(0o755)")
        })
        .map(|f| f.display().to_string())
        .collect();
    assert!(
        offenders.is_empty(),
        "write the program with sv_check::test_support::executable: {offenders:?}"
    );
}
