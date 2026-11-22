//! `sv connect --docker --notes DIR`, run as a person runs it (backlog 0716, part 4): the block mounts the one folder
//! of notes, the line beside it says how to make that folder, and it is refused without `--docker`.

use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use sv_frameworks::paths::Canonical;

fn sv(folder: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_sv"))
        .current_dir(folder)
        .args(args)
        .env_remove("RUST_BACKTRACE")
        // As inside the container, where the person's own folders cannot be seen and so are not looked for.
        .env("SV_CONNECT_IN_CONTAINER", "1")
        .output()
        .expect("sv runs")
}

fn folder(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "sv-connect-notes-run-{name}-{}",
        std::process::id()
    ));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    dir.canonical().unwrap()
}

// Not on Windows: there `--notes` is refused, which the module's own tests hold on every computer.
#[cfg(not(windows))]
#[test]
fn the_block_mounts_the_notes_folder_alone_and_the_line_beside_it_says_how_to_make_it() {
    let dir = folder("block");
    let out = sv(
        &dir,
        &[
            "connect",
            "claude",
            "--docker",
            "/usr/local/bin/docker",
            "--notes",
            "/home/someone/.local/state/stackvet/",
        ],
    );
    let err = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(out.status.success(), "{err}");
    let block: Value =
        serde_json::from_slice(&out.stdout).expect("standard output is the block alone");
    let args: Vec<String> = block["mcpServers"]["stackvet"]["args"]
        .as_array()
        .expect("args")
        .iter()
        .map(|a| a.as_str().unwrap().to_owned())
        .collect();
    assert!(
        args.contains(
            &"/home/someone/.local/state/stackvet/loop-notes:/sv-state/stackvet/loop-notes"
                .to_owned()
        ),
        "{args:?}"
    );
    assert!(
        args.contains(&"XDG_STATE_HOME=/sv-state".to_owned()),
        "{args:?}"
    );
    assert!(err.contains("Put this in .mcp.json"), "{err}");
    assert!(
        err.contains(
            "mkdir -p '/home/someone/.local/state/stackvet/loop-notes' && chmod 700 \
             '/home/someone/.local/state/stackvet' '/home/someone/.local/state/stackvet/loop-notes'"
        ),
        "{err}"
    );
    // It wrote nothing into the folder.
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 0);

    // Without --notes, nothing of it.
    let out = sv(
        &dir,
        &["connect", "claude", "--docker", "/usr/local/bin/docker"],
    );
    let all = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.status.success(), "{all}");
    assert!(
        !all.contains("loop-notes") && !all.contains("XDG_STATE_HOME"),
        "{all}"
    );
}

#[test]
fn notes_without_docker_or_without_a_folder_is_refused_with_the_reason() {
    let dir = folder("refused");
    for (args, reason) in [
        (
            vec![
                "connect",
                "claude",
                "--notes",
                "/home/someone/.local/state/stackvet",
            ],
            "--notes is for the container",
        ),
        (
            vec![
                "connect",
                "claude",
                "--docker",
                "/usr/local/bin/docker",
                "--notes",
            ],
            "`--notes` needs a value",
        ),
    ] {
        let out = sv(&dir, &args);
        let err = String::from_utf8_lossy(&out.stderr).into_owned();
        assert!(!out.status.success(), "{args:?}");
        assert!(
            out.stdout.is_empty(),
            "{args:?}: nothing a tool could paste"
        );
        assert!(err.contains(reason), "{args:?}: {err}");
    }
}

/// The settings name the image with no tag, so the line beside them says to pull it first, and which `sv` printed
/// them (backlog 0721, part 5; the owner's choice, 22 November 2026).
#[test]
fn with_docker_the_line_beside_the_settings_says_to_pull_the_image_first() {
    let dir = folder("pull");
    let out = sv(
        &dir,
        &["connect", "claude", "--docker", "/usr/local/bin/docker"],
    );
    let err = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(out.status.success(), "{err}");
    let image = sv_frameworks::names::IMAGE;
    assert!(
        err.contains(&format!("Run `docker pull {image}` first")) && err.contains("(sv "),
        "{err}"
    );
    // The settings themselves still name the image with no tag.
    let block = String::from_utf8_lossy(&out.stdout).into_owned();
    assert!(block.contains(&format!("\"{image}\"")), "{block}");
}

/// With `--docker`, the line beside the settings says what a container session does not keep: history, and a report
/// from an earlier session (backlog 0729). Without it, neither: a copy of `sv` installed on the computer keeps both.
#[test]
fn with_docker_the_line_says_what_does_not_carry_over_between_sessions_and_without_it_says_neither()
{
    let dir = folder("carry");
    for args in [
        &["connect", "claude", "--docker", "/usr/local/bin/docker"][..],
        // With no tool named, the server in no tool's format: the same is said.
        &["connect", "--docker", "/usr/local/bin/docker"],
    ] {
        let out = sv(&dir, args);
        let err = String::from_utf8_lossy(&out.stderr).into_owned();
        assert!(out.status.success(), "{args:?}: {err}");
        for said in [
            "history is not kept for runs made in the container, even if you turned history on",
            "a report the container wrote in an earlier session is not offered again as one sv wrote",
            "your AI coding tool asks for a new report each session",
        ] {
            assert!(err.contains(said), "{args:?}: missing {said:?} in:\n{err}");
        }
        // Said beside the settings, never inside them: standard output is still the block alone.
        let block = String::from_utf8_lossy(&out.stdout).into_owned();
        assert!(
            !block.contains("history") && serde_json::from_str::<Value>(&block).is_ok(),
            "{block}"
        );
    }
}

#[test]
fn without_docker_nothing_is_said_of_a_container() {
    // Called as the program calls it, with no --docker: an installed copy keeps its history and its report key.
    for args in [&["claude"][..], &[], &["claude", "--hook"]] {
        let args: Vec<String> = args.iter().map(|a| (*a).to_owned()).collect();
        let said = sv_cli::connect::where_it_goes(&args);
        assert!(
            !said.contains("history is not kept") && !said.contains("earlier session"),
            "{args:?}: {said}"
        );
    }
}

/// The install guide says the same two things in its own words. (The getting-started guide has a budget of words,
/// which this would have passed: ADR-120.)
#[test]
fn the_install_guide_says_the_same_two_things() {
    let guide = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/INSTALL.md"),
    )
    .unwrap();
    let one_line = guide.split_whitespace().collect::<Vec<_>>().join(" ");
    for said in [
        "history is not kept for runs made in the container, even if you turned history on",
        "a report the container wrote in an earlier session is not offered again",
    ] {
        assert!(
            one_line.contains(said),
            "docs/INSTALL.md does not say: {said}"
        );
    }
}
