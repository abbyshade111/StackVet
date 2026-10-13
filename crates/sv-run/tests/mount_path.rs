//! A folder of the owner's whose path holds a colon or a comma is still mounted (backlog 0329). Docker's
//! short `-v src:dst:ro` splits at every colon, so `Notes: 2026/app` could not be mounted and the run
//! failed without saying why; `--mount`, with its fields quoted the way Docker reads them, carries it.

use std::path::Path;
use std::process::Command;
use sv_run::Backend;
use sv_run::docker::{DockerBackend, read_only_bind};

#[test]
fn a_colon_a_comma_and_a_quote_are_carried_in_the_mount() {
    assert_eq!(
        read_only_bind(Path::new("/Users/a/Notes: 2026/app"), "/app"),
        "type=bind,src=/Users/a/Notes: 2026/app,dst=/app,readonly"
    );
    // A comma would end the field, so the field is quoted; a quote inside it is doubled.
    assert_eq!(
        read_only_bind(Path::new("/Users/a/one, two/\"app\""), "/app"),
        "type=bind,\"src=/Users/a/one, two/\"\"app\"\"\",dst=/app,readonly"
    );
}

#[test]
fn docker_mounts_a_folder_whose_name_holds_a_colon_and_a_comma() {
    if DockerBackend::new().available().is_err() {
        println!("no container backend here; only the mount's text is checked (the test above)");
        return;
    }
    let root = std::env::temp_dir().join(format!("sv-mount-path-{}", std::process::id()));
    let app = root.join("Notes: 2026, app");
    std::fs::remove_dir_all(&root).ok();
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(app.join("marker.txt"), "found it\n").unwrap();
    let docker = |mount: &[&str]| {
        Command::new("docker")
            .args(["run", "--rm", "--network", "none"])
            .args(mount)
            .args(["busybox:1.36", "cat", "/app/marker.txt"])
            .output()
            .expect("docker runs")
    };
    let bind = read_only_bind(&app, "/app");
    let fixed = docker(&["--mount", &bind]);
    // The setup: the short form really cannot carry this path, so the test is of the fault.
    let short = format!("{}:/app:ro", app.display());
    let old = docker(&["-v", &short]);
    std::fs::remove_dir_all(&root).ok();

    assert!(
        !old.status.success() || !String::from_utf8_lossy(&old.stdout).contains("found it"),
        "the short -v form mounted the folder after all, so this test shows nothing"
    );
    assert!(
        fixed.status.success() && String::from_utf8_lossy(&fixed.stdout).contains("found it"),
        "the folder was not mounted: {}",
        String::from_utf8_lossy(&fixed.stderr)
    );
}
