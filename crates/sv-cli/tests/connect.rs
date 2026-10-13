//! `sv connect`, run as the AI coding tool runs it, and the setup prompt that tells the tool to run it
//! (`docs/prompts/setup.md`, backlog 0217 part 1).

use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use sv_frameworks::paths::Canonical;

fn sv(folder: &Path, args: &[&str], in_container: bool) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_sv"));
    command
        .current_dir(folder)
        .args(args)
        .env_remove("RUST_BACKTRACE");
    if in_container {
        command.env("SV_CONNECT_IN_CONTAINER", "1");
    } else {
        command.env_remove("SV_CONNECT_IN_CONTAINER");
    }
    command.output().expect("sv runs")
}

fn folder(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-connect-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    dir.canonical().unwrap()
}

fn parsed(out: &Output) -> Value {
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).expect("standard output is the settings block alone")
}

fn said(out: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

fn strings(v: &Value) -> Vec<String> {
    v.as_array()
        .expect("a list")
        .iter()
        .map(|a| a.as_str().unwrap().to_owned())
        .collect()
}

#[test]
fn the_container_settings_carry_this_folders_real_path_and_say_where_they_go() {
    let dir = folder("docker");
    let out = sv(
        &dir,
        &["connect", "claude", "--docker", "/usr/local/bin/docker"],
        true,
    );
    let block = parsed(&out);
    let server = &block["mcpServers"]["stackvet"];
    assert_eq!(server["command"], "/usr/local/bin/docker");
    let args = strings(&server["args"]);
    let path = dir.display().to_string();
    assert!(args.contains(&format!("{path}:{path}")), "{args:?}");
    assert_eq!(args.last().unwrap(), &path, "{args:?}");
    assert!(
        args.contains(&sv_frameworks::names::IMAGE.to_owned()),
        "{args:?}"
    );
    assert!(
        args.windows(2).any(|w| w == ["--network", "none"]),
        "{args:?}"
    );
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(err.contains("Put this in .mcp.json"), "{err}");
    // It wrote nothing into the folder.
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 0);
}

// Unix only: it makes a symbolic link, which Windows allows only with special rights (backlog 0120).
#[cfg(unix)]
#[test]
fn a_folder_reached_through_a_link_is_named_by_its_real_place() {
    let dir = folder("real");
    let link = std::env::temp_dir().join(format!("sv-connect-link-{}", std::process::id()));
    std::fs::remove_file(&link).ok();
    std::os::unix::fs::symlink(&dir, &link).unwrap();
    // Named as the link, not entered: the system already gives a folder entered through a link by
    // its real place.
    let named = link.display().to_string();
    let out = sv(
        &std::env::temp_dir(),
        &[
            "connect",
            "cursor",
            "--docker",
            "/usr/bin/docker",
            "--folder",
            &named,
        ],
        false,
    );
    let args = strings(&parsed(&out)["mcpServers"]["stackvet"]["args"]);
    assert_eq!(args.last().unwrap(), &dir.display().to_string(), "{args:?}");
    assert!(!args.iter().any(|a| a.contains(&named)), "{args:?}");
    std::fs::remove_file(&link).ok();
}

#[test]
fn a_folder_given_by_a_relative_name_is_written_as_its_full_path() {
    let dir = folder("relative");
    std::fs::create_dir(dir.join("app")).unwrap();
    let out = sv(
        &dir,
        &[
            "connect",
            "claude",
            "--docker",
            "/usr/bin/docker",
            "--folder",
            "app/../app",
        ],
        false,
    );
    let args = strings(&parsed(&out)["mcpServers"]["stackvet"]["args"]);
    let full = dir.join("app").display().to_string();
    assert_eq!(args.last().unwrap(), &full, "{args:?}");
    assert!(args.contains(&format!("{full}:{full}")), "{args:?}");
}

#[test]
fn without_docker_an_installed_copy_names_its_own_real_place() {
    let dir = folder("installed");
    let out = sv(&dir, &["connect", "vscode"], false);
    let block = parsed(&out);
    let server = &block["servers"]["stackvet"];
    assert_eq!(server["type"], "stdio");
    let program = PathBuf::from(env!("CARGO_BIN_EXE_sv")).canonical().unwrap();
    assert_eq!(server["command"], program.display().to_string());
    assert_eq!(
        strings(&server["args"]),
        ["mcp", "--root", &dir.display().to_string()]
    );
    assert!(said(&out).contains("Put this in .vscode/mcp.json"));
}

#[test]
fn inside_the_container_it_asks_for_docker_rather_than_print_a_path_that_means_nothing_outside() {
    let dir = folder("inside");
    let out = sv(&dir, &["connect", "claude"], true);
    assert!(!out.status.success());
    let text = said(&out);
    assert!(text.contains("inside its container"), "{text}");
    assert!(text.contains("--docker"), "{text}");
    assert!(out.stdout.is_empty(), "nothing a tool could paste: {text}");
}

#[test]
fn what_it_cannot_use_is_refused_with_the_reason() {
    let dir = folder("refused");
    for (args, reason) in [
        (
            vec!["connect", "copilot"],
            "is not a tool `sv connect` knows",
        ),
        (
            vec!["connect", "claude", "--docker", "docker"],
            "docker's full path",
        ),
        (
            vec![
                "connect",
                "claude",
                "--docker",
                "/usr/bin/docker",
                "--user",
                "me",
            ],
            "--user needs UID:GID",
        ),
        (
            vec!["connect", "claude", "--user", "1000:1000"],
            "--user is for the container",
        ),
        (
            vec!["connect", "claude", "--folder", "/no/such/folder"],
            "is not a folder",
        ),
    ] {
        refused(&dir, &args, false, reason);
    }
    // Inside the container, its own path is refused too, with a tool named or none.
    refused(&dir, &["connect", "vscode"], true, "--docker");
    refused(
        &dir,
        &["connect"],
        true,
        "sv connect --docker /usr/local/bin/docker",
    );
}

fn refused(dir: &Path, args: &[&str], in_container: bool, reason: &str) {
    let out = sv(dir, args, in_container);
    assert!(!out.status.success(), "{args:?}");
    assert!(out.stdout.is_empty(), "{args:?}: {}", said(&out));
    assert!(said(&out).contains(reason), "{args:?}: {}", said(&out));
}

/// The setup prompt, between its markers.
fn setup_prompt() -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let page = std::fs::read_to_string(root.join("docs/prompts/setup.md")).unwrap();
    let start = page
        .find("<!-- setup-prompt:start -->")
        .expect("start marker");
    let end = page.find("<!-- setup-prompt:end -->").expect("end marker");
    page[start..end].to_owned()
}

#[test]
fn the_setup_prompt_fetches_and_runs_the_image_stackvet_is_published_as() {
    let prompt = setup_prompt();
    let image = sv_frameworks::names::IMAGE;
    assert!(
        prompt.contains(&format!("docker pull {image}`")),
        "{prompt}"
    );
    let run = prompt
        .lines()
        .find(|l| l.contains("docker run"))
        .expect("the line that runs `sv connect`");
    for part in [
        "--network none",
        "-v \"$PWD\":\"$PWD\"",
        "-w \"$PWD\"",
        image,
        " connect --docker DOCKER_PATH",
    ] {
        assert!(run.contains(part), "{part} in {run}");
    }
}

/// The setup prompt asks StackVet for its server in no app's format, and the AI tool adds it to its own app's
/// settings its own way (backlog 0396): run here exactly as the prompt words it, it prints that server, and the prompt
/// names no app or app's settings file for the tool to choose between.
#[test]
fn the_setup_prompts_command_prints_stackvets_server_in_no_apps_format() {
    let prompt = setup_prompt();
    let run = prompt.lines().find(|l| l.contains("docker run")).unwrap();
    let after_image = run
        .split(sv_frameworks::names::IMAGE)
        .nth(1)
        .unwrap()
        .trim()
        .trim_end_matches('`');
    let words: Vec<&str> = after_image.split_whitespace().collect();
    assert_eq!(words, ["connect", "--docker", "DOCKER_PATH"]);
    let dir = folder("prompt");
    // The prompt's own words, with a path put in, as the AI tool would.
    let args: Vec<&str> = words
        .iter()
        .map(|w| match *w {
            "DOCKER_PATH" => "/usr/local/bin/docker",
            other => other,
        })
        .collect();
    let out = sv(&dir, &args, true);
    let server = parsed(&out);
    assert_eq!(server["name"], "stackvet", "{server}");
    assert_eq!(server["transport"], "stdio", "{server}");
    assert_eq!(server["command"], "/usr/local/bin/docker", "{server}");
    let given = strings(&server["args"]);
    for part in [
        "--network",
        "none",
        sv_frameworks::names::IMAGE,
        "mcp",
        "--root",
    ] {
        assert!(given.iter().any(|a| a == part), "{part}: {given:?}");
    }
    assert_eq!(given.last().map(String::as_str), dir.to_str(), "{given:?}");
    assert!(said(&out).contains("in no app's format"), "{}", said(&out));
    // And with the Linux line's --user, in the form `id` prints, before the image, where docker reads it.
    assert!(prompt.contains("--user \"$(id -u):$(id -g)\""));
    let mut linux = args.clone();
    linux.extend(["--user", "1000:1000"]);
    let given = strings(&parsed(&sv(&dir, &linux, true))["args"]);
    let user = given.iter().position(|a| a == "--user").expect("--user");
    let image = given
        .iter()
        .position(|a| a == sv_frameworks::names::IMAGE)
        .unwrap();
    assert!(
        user < image,
        "docker reads --user only before the image: {given:?}"
    );
    // No app is named for the tool to pick, and no app's settings file.
    for app in [
        "`claude`",
        "`vscode`",
        "`cursor`",
        ".mcp.json",
        ".vscode/",
        ".cursor/",
        "connect TOOL",
    ] {
        assert!(!prompt.contains(app), "{app} in the prompt");
    }
}

#[test]
fn the_guide_offers_the_setup_prompt() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let guide = std::fs::read_to_string(root.join("docs/GETTING-STARTED.md")).unwrap();
    assert!(
        guide.contains("(prompts/setup.md)"),
        "the guide links the setup prompt"
    );
}
