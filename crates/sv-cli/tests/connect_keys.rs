//! `sv connect --docker --keys DIR` gives the container the person's list of trusted keys, one file, read-only, and
//! nothing else of their key folder (backlog 0581, the owner's choice of 15 October 2026): without it the container
//! could not count what they recorded with `sv review`, and the folder also holds the key that signs as them.

use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use sv_frameworks::paths::Canonical;

fn sv(folder: &Path, args: &[&str], in_container: bool) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_sv"));
    command.current_dir(folder).args(args);
    if in_container {
        command.env("SV_CONNECT_IN_CONTAINER", "1");
    } else {
        command.env_remove("SV_CONNECT_IN_CONTAINER");
    }
    command.output().expect("sv runs")
}

fn folder(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-connect-keys-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    dir.canonical().unwrap()
}

fn args(out: &Output) -> Vec<String> {
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let server: Value = serde_json::from_slice(&out.stdout).unwrap();
    server["args"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a.as_str().unwrap().to_owned())
        .collect()
}

fn refused(out: &Output) -> String {
    assert!(!out.status.success(), "refused");
    String::from_utf8_lossy(&out.stderr).into_owned()
}

const DOCKER: [&str; 3] = ["connect", "--docker", "/usr/local/bin/docker"];

#[test]
fn the_container_reads_the_list_of_trusted_keys_alone_read_only() {
    let dir = folder("list");
    let mut with = DOCKER.to_vec();
    with.extend(["--keys", "/home/someone/.config/stackvet/"]);
    let given = args(&sv(&dir, &with, true));
    let image = given
        .iter()
        .position(|a| a == sv_frameworks::names::IMAGE)
        .unwrap();
    let mount = given
        .iter()
        .position(|a| {
            a == "/home/someone/.config/stackvet/allowed_signers:/sv-config/stackvet/allowed_signers:ro"
        })
        .unwrap_or_else(|| panic!("the list mounted read-only: {given:?}"));
    assert_eq!(given[mount - 1], "-v");
    let config = given
        .iter()
        .position(|a| a == "XDG_CONFIG_HOME=/sv-config")
        .unwrap_or_else(|| panic!("sv told where the folder is: {given:?}"));
    assert_eq!(given[config - 1], "-e");
    assert!(
        mount < image && config < image,
        "docker reads them before the image: {given:?}"
    );
    // Nothing else of the folder: never the folder itself, never the key that signs.
    let from_folder: Vec<&String> = given
        .iter()
        .filter(|a| a.starts_with("/home/someone/.config/stackvet"))
        .collect();
    assert_eq!(from_folder.len(), 1, "{given:?}");
    assert!(
        !given.iter().any(|a| a.contains("review-signing-key")),
        "{given:?}"
    );

    // Without --keys, nothing of the kind, as before.
    let given = args(&sv(&dir, &DOCKER, true));
    std::fs::remove_dir_all(&dir).ok();
    assert!(
        !given
            .iter()
            .any(|a| a.contains("allowed_signers") || a.contains("XDG_CONFIG_HOME")),
        "{given:?}"
    );
}

#[test]
fn a_key_folder_docker_cannot_read_rightly_is_refused_with_the_reason() {
    let dir = folder("refused");
    for (keys, reason) in [
        ("~/.config/stackvet", "full path"),
        (".config/stackvet", "full path"),
        ("/home/a:b/.config/stackvet", "separators"),
    ] {
        let mut with = DOCKER.to_vec();
        with.extend(["--keys", keys]);
        let said = refused(&sv(&dir, &with, true));
        assert!(said.contains(reason), "{keys}: {said}");
    }
    let said = refused(&sv(
        &dir,
        &["connect", "--keys", "/home/someone/.config/stackvet"],
        true,
    ));
    assert!(said.contains("--keys is for the container"), "{said}");
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn outside_the_container_the_list_must_be_there_before_docker_mounts_it() {
    // Docker makes a folder where a mounted file is not yet: `sv review` could then never add its key to the list.
    let dir = folder("outside");
    let keys = dir.join("keys");
    std::fs::create_dir_all(&keys).unwrap();
    let mut with = DOCKER.to_vec();
    let keys_arg = keys.display().to_string();
    with.extend(["--keys", &keys_arg]);
    let said = refused(&sv(&dir, &with, false));
    assert!(
        said.contains("is not there yet") && said.contains("touch"),
        "{said}"
    );
    std::fs::write(keys.join("allowed_signers"), "").unwrap();
    let given = args(&sv(&dir, &with, false));
    std::fs::remove_dir_all(&dir).ok();
    assert!(
        given
            .iter()
            .any(|a| a.ends_with("/sv-config/stackvet/allowed_signers:ro")),
        "{given:?}"
    );
}
