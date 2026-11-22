//! `sv connect --docker --notes DIR` (backlog 0716, part 4): the container is given the one folder of notes and told
//! where its records are, only when asked; the folder must be there and the person's alone; and Windows is refused
//! with the reason.

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
fn windows_is_refused_with_the_reason_whatever_the_path() {
    for notes in [r"C:\Users\someone\state\stackvet", "/c/Users/someone/state"] {
        let why = said(notes, true);
        assert!(
            why.contains("not offered on Windows yet")
                && why.contains("another path inside the container"),
            "{notes}: {why}"
        );
    }
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
    std::fs::set_permissions(&notes, std::fs::Permissions::from_mode(0o755)).unwrap();
    assert_eq!(
        std::fs::metadata(&notes).unwrap().permissions().mode() & 0o777,
        0o755,
        "the setup opened the folder"
    );
    assert!(said(&given, false).contains("can be read or written by others"));
    // The person's alone: taken, without a trailing slash.
    std::fs::set_permissions(&notes, std::fs::Permissions::from_mode(0o700)).unwrap();
    assert_eq!(said(&format!("{given}/"), false), format!("kept {given}"));
}
