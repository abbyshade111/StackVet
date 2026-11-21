//! No workflow runs a downloaded installer it has not checked (backlog 0708): every step that downloads rustup names
//! a version and a SHA-256, checks the file against it, and runs it only after; and no workflow pipes a download
//! straight into a shell. Measured on 21 November 2026 in an arm64 Linux container without rustup, as the
//! `image-arm64` runner would be without one: with the checksum one character off, `sha256sum` failed the step before
//! `rustup-init` ran; with the real one, rustup 1.29.3 installed.

use std::path::PathBuf;

fn workflows() -> Vec<(String, String)> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.github/workflows");
    let mut out: Vec<(String, String)> = std::fs::read_dir(&dir)
        .unwrap()
        .flatten()
        .filter(|e| e.path().extension().is_some_and(|x| x == "yml"))
        .map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            (name, std::fs::read_to_string(e.path()).unwrap())
        })
        .collect();
    out.sort();
    out
}

#[test]
fn no_workflow_pipes_a_download_into_a_shell() {
    let all = workflows();
    assert!(
        all.len() > 5,
        "the setup: the workflows were read ({})",
        all.len()
    );
    for (name, text) in &all {
        for line in text.lines().filter(|l| !l.trim_start().starts_with('#')) {
            let piped = (line.contains("curl ") || line.contains("wget "))
                && ["| sh", "| bash", "|sh", "|bash"]
                    .iter()
                    .any(|p| line.contains(p));
            assert!(
                !piped,
                "{name}: a download is run without a check: {}",
                line.trim()
            );
        }
    }
}

#[test]
fn the_arm64_job_runs_rustup_only_after_checking_it() {
    let (_, rust) = workflows()
        .into_iter()
        .find(|(n, _)| n == "rust.yml")
        .unwrap();
    let job = &rust[rust
        .find("\n  image-arm64:\n")
        .expect("the setup: the arm64 job is there")..];
    let step = &job[job
        .find("- name: Build sv natively")
        .expect("the setup: its native build")..];
    let step = &step[..step[1..]
        .find("\n      - name:")
        .map_or(step.len(), |i| i + 1)];
    let version = step
        .lines()
        .find_map(|l| l.trim().strip_prefix("RUSTUP_VERSION: "))
        .expect("a rustup version is named");
    assert!(
        version.split('.').count() == 3 && version.split('.').all(|p| p.parse::<u32>().is_ok()),
        "the version is exact: {version}"
    );
    let sum = step
        .lines()
        .find_map(|l| l.trim().strip_prefix("RUSTUP_SHA256: "))
        .expect("its SHA-256 is written");
    assert!(
        sum.len() == 64 && sum.chars().all(|c| c.is_ascii_hexdigit()),
        "the checksum is a SHA-256: {sum}"
    );
    let download = step
        .find("rustup/archive/$RUSTUP_VERSION/")
        .expect("the named version is downloaded");
    let check = step
        .find("sha256sum --check --strict")
        .expect("the download is checked");
    let run = step
        .find("\"$RUNNER_TEMP/rustup-init\" -y")
        .expect("the checked file is what runs");
    assert!(
        download < check && check < run,
        "downloaded, checked, then run: {step}"
    );
}
