//! `sv connect --docker --notes DIR` (backlog 0716, part 4): the container is given the one folder of notes and told
//! where its records are, only when asked; the folder must be there and the person's alone; and on Windows (backlog 0728) a
//! drive's folder is taken, with PowerShell's command to make it.

use super::*;

fn docker(notes: Option<&str>) -> Runner {
    Runner::Docker {
        docker: "/usr/local/bin/docker".to_owned(),
        user: None,
        keys: None,
        notes: notes.map(str::to_owned),
    }
}

fn args(runner: &Runner) -> Vec<String> {
    command_and_args_on(runner, Path::new("/Users/someone/app"), false).1
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-connect-notes-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn said(notes: &str, windows: bool) -> String {
    match check_notes_on(notes, windows) {
        Ok(kept) => format!("kept {kept}"),
        Err(e) => e.to_string(),
    }
}

#[test]
fn only_the_folder_of_notes_is_given_and_only_when_asked() {
    let with = args(&docker(Some("/Users/someone/.local/state/stackvet")));
    let mount = "/Users/someone/.local/state/stackvet/loop-notes:/sv-state/stackvet/loop-notes";
    assert!(
        with.windows(2).any(|w| w == ["-v", mount]),
        "the notes' folder is mounted where sv keeps it: {with:?}"
    );
    assert!(
        with.windows(2)
            .any(|w| w == ["-e", "XDG_STATE_HOME=/sv-state"]),
        "{with:?}"
    );
    // Nothing else of the records folder: not the folder itself, and not the hook or pass-through records.
    let mounts: Vec<&String> = with
        .windows(2)
        .filter(|w| w[0] == "-v")
        .map(|w| &w[1])
        .collect();
    assert_eq!(
        mounts.len(),
        2,
        "the app's folder and the notes: {mounts:?}"
    );
    assert!(
        !with.iter().any(|a| a.contains("hook-records")
            || a.contains("proxy-records")
            || a.starts_with("/Users/someone/.local/state/stackvet:")),
        "{with:?}"
    );
    // The container still has no network, and the image still comes after every option of Docker's.
    assert!(with.windows(2).any(|w| w == ["--network", "none"]));
    let image = with
        .iter()
        .position(|a| a == sv_frameworks::names::IMAGE)
        .expect("the image");
    let state = with
        .iter()
        .position(|a| a == "XDG_STATE_HOME=/sv-state")
        .unwrap();
    assert!(state < image, "{with:?}");

    // Not asked for: no mount, and no word of where records are.
    let without = args(&docker(None));
    assert!(
        !without
            .iter()
            .any(|a| a.contains("loop-notes") || a.contains("XDG_STATE_HOME")),
        "{without:?}"
    );
}

#[test]
fn the_folder_sv_mounts_is_the_one_the_notes_are_written_to() {
    // The name here and the name the notes are kept under are one name: a change to either shows.
    let file = av_record::outside_note::file_of(Path::new("/app")).map(|f| {
        f.parent()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().into_owned())
    });
    if let Some(name) = file {
        assert_eq!(name.as_deref(), Some(NOTES_DIR));
    }
}

#[test]
fn on_windows_a_drive_s_folder_is_taken_and_it_says_how_to_make_it_in_powershell() {
    // Backlog 0728: no longer refused. A folder that is not there yet is named, with PowerShell's command to make it.
    let why = said(r"C:\Users\someone\.local\state\stackvet", true);
    if !in_a_container() {
        assert!(
            why.contains(r"C:\Users\someone\.local\state\stackvet\loop-notes")
                && why.contains("New-Item -ItemType Directory -Force -Path")
                && !why.contains("chmod"),
            "{why}"
        );
    }
    // A path that is not a full one is still refused, with a Windows example; and a colon past the drive's.
    assert!(said(r"state\stackvet", true).contains("PowerShell fills in $HOME"));
    assert!(said(r"C:\Users\a:b\stackvet", true).contains("`:` or `,`"));
    // A quote in the folder's name is doubled, as PowerShell reads it inside single quotes.
    assert_eq!(
        make_notes_on(r"C:\Users\O'Neil\.local\state\stackvet", true),
        r"New-Item -ItemType Directory -Force -Path 'C:\Users\O''Neil\.local\state\stackvet\loop-notes'"
    );
}

#[test]
fn on_windows_the_notes_are_mounted_from_the_windows_folder_and_the_container_is_told_the_app_s() {
    let runner = docker(Some(r"C:\Users\someone\.local\state\stackvet"));
    let with = command_and_args_on(&runner, Path::new(r"C:\Users\someone\app"), true).1;
    assert!(
        with.windows(2).any(|w| w
            == [
                "-v",
                r"C:\Users\someone\.local\state\stackvet\loop-notes:/sv-state/stackvet/loop-notes"
            ]),
        "{with:?}"
    );
    // The folder the notes are named by (backlog 0728) is the one the container is told.
    assert!(
        with.windows(2)
            .any(|w| w == ["-e", r"SV_HOST_FOLDER=C:\Users\someone\app"]),
        "{with:?}"
    );
}

#[test]
fn a_path_docker_would_misread_is_refused() {
    assert!(said("state/stackvet", false).contains("full path"));
    assert!(said("~/.local/state/stackvet", false).contains("full path"));
    assert!(said("/home/a:b/stackvet", false).contains("`:` or `,`"));
    assert!(said("/home/a,b/stackvet", false).contains("`:` or `,`"));
}

// Unix only: it makes a link and sets a folder's permissions.
#[cfg(unix)]
#[test]
fn the_folder_must_be_there_a_real_folder_and_the_person_s_alone() {
    use std::os::unix::fs::PermissionsExt;
    if in_a_container() {
        println!("inside a container the person's folders cannot be seen; nothing to check here");
        return;
    }
    let dir = scratch("there");
    let given = dir.display().to_string();
    // Not there: said, with the commands that make it.
    let why = said(&given, false);
    assert!(
        why.contains("is not there yet") && why.contains("mkdir -p") && why.contains("chmod 700"),
        "{why}"
    );
    // A link in its place is not a folder of its own.
    let elsewhere = scratch("elsewhere");
    std::os::unix::fs::symlink(&elsewhere, dir.join(NOTES_DIR)).unwrap();
    assert!(said(&given, false).contains("is not there yet as a folder of its own"));
    std::fs::remove_file(dir.join(NOTES_DIR)).unwrap();
    // There, and open to others: refused.
    let notes = dir.join(NOTES_DIR);
    std::fs::create_dir(&notes).unwrap();
    std::fs::set_permissions(&notes, std::fs::Permissions::from_mode(0o750)).unwrap();
    assert_eq!(
        std::fs::metadata(&notes).unwrap().permissions().mode() & 0o777,
        0o750,
        "the setup opened the folder"
    );
    assert!(said(&given, false).contains("can be read or written by others"));
    // The person's alone: taken, without a trailing slash.
    std::fs::set_permissions(&notes, std::fs::Permissions::from_mode(0o700)).unwrap();
    assert_eq!(said(&format!("{given}/"), false), format!("kept {given}"));
}
