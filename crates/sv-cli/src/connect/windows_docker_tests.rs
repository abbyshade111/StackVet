//! Backlog 0375 (ADR-153): `sv connect --docker` on Windows takes Docker's Windows path, shows the app's folder
//! inside the container at a Linux-style path with the Windows one told to it, and on a Mac or Linux is as it was.

use super::*;

fn docker(path: &str) -> Runner {
    Runner::Docker {
        docker: path.to_owned(),
        user: None,
        keys: None,
    }
}

#[test]
fn docker_s_windows_path_is_taken_on_windows_only() {
    let windows = r"C:\Program Files\Docker\Docker\resources\bin\docker.exe";
    assert!(check_docker_on(windows, true).is_ok());
    assert!(
        check_docker_on(windows, false).is_err(),
        "a drive path means nothing on a Mac"
    );
    assert!(check_docker_on("/usr/local/bin/docker", true).is_ok());
    assert!(check_docker_on("/usr/local/bin/docker", false).is_ok());
    let bare = check_docker_on("docker", true).unwrap_err().to_string();
    assert!(bare.contains("where.exe docker"), "{bare}");
}

#[test]
fn the_folder_is_shown_inside_at_a_linux_style_path_and_the_windows_one_is_told() {
    let folder = Path::new(r"C:\Users\you\app");
    let runner = docker(r"C:\Program Files\Docker\Docker\resources\bin\docker.exe");
    let (command, args) = command_and_args_on(&runner, folder, true);
    assert_eq!(
        command,
        r"C:\Program Files\Docker\Docker\resources\bin\docker.exe"
    );
    let at = |word: &str| {
        args.iter()
            .position(|a| a == word)
            .unwrap_or_else(|| panic!("{word} in {args:?}"))
    };
    assert_eq!(args[at("-v") + 1], r"C:\Users\you\app:/c/Users/you/app");
    assert_eq!(
        args[at("-e") + 1],
        format!(r"{HOST_FOLDER_ENV}=C:\Users\you\app")
    );
    assert_eq!(args[at("--root") + 1], "/c/Users/you/app");
}

#[test]
fn on_a_mac_or_linux_the_settings_are_as_they_were() {
    let folder = Path::new("/Users/you/app");
    let (_, args) = command_and_args_on(&docker("/usr/local/bin/docker"), folder, false);
    let at = |word: &str| args.iter().position(|a| a == word).unwrap();
    assert_eq!(args[at("-v") + 1], "/Users/you/app:/Users/you/app");
    assert_eq!(args[at("--root") + 1], "/Users/you/app");
    assert!(
        !args.iter().any(|a| a.starts_with(HOST_FOLDER_ENV)),
        "{args:?}"
    );
    // And on Windows with an installed `sv`, no container and nothing to change.
    let (_, args) = command_and_args_on(
        &Runner::Installed(PathBuf::from(r"C:\sv\sv.exe")),
        Path::new(r"C:\app"),
        true,
    );
    assert_eq!(args, ["mcp", "--root", r"C:\app"]);
}

#[test]
fn a_windows_folder_s_inside_path() {
    assert_eq!(
        inside_path(r"C:\Users\you\app").as_deref(),
        Some("/c/Users/you/app")
    );
    assert_eq!(inside_path(r"D:\work\app\").as_deref(), Some("/d/work/app"));
    assert_eq!(
        inside_path("C:/Users/you/app").as_deref(),
        Some("/c/Users/you/app")
    );
    assert_eq!(inside_path(r"C:\").as_deref(), Some("/c"));
    assert_eq!(inside_path("/Users/you/app"), None);
    assert_eq!(inside_path("app"), None);
}
