//! The notes of an app on Windows, written by `sv` in its Linux container and read by `av report` on the computer,
//! have one name (backlog 0728). Pure: the paths are given, so this is tested on every system.

use super::name_of;

const HOST: &str = r"C:\Users\you\app";

#[test]
fn the_container_and_the_computer_name_one_folder_s_notes_alike() {
    // What `sv` in the container sees, told the folder's Windows path in SV_HOST_FOLDER.
    let container = name_of("/c/Users/you/app", Some(HOST));
    // What `av report` on the computer names it from: its own canonical path, in each form Windows can give it.
    for computer in [
        r"C:\Users\you\app",
        r"c:\Users\you\app",
        r"\\?\C:\Users\you\app",
        r"C:\Users\you\app\",
    ] {
        assert_eq!(name_of(computer, None), container, "{computer}");
    }
    // A folder under the app's, as the MCP server may be asked about.
    assert_eq!(
        name_of("/c/Users/you/app/web", Some(HOST)),
        name_of(r"C:\Users\you\app\web", None)
    );
}

#[test]
fn without_the_folder_it_was_told_the_container_names_its_own_path() {
    // The way it was before: a note `av report` on Windows would never find. Shown so a change that drops the
    // turning back fails the test above and this one's pair.
    assert_ne!(name_of("/c/Users/you/app", None), name_of(HOST, None));
    // Told a folder that is not the one served: nothing is turned.
    assert_eq!(
        name_of("/c/Users/you/other", Some(HOST)),
        name_of("/c/Users/you/other", None)
    );
}

#[test]
fn a_path_that_is_no_windows_one_keeps_the_name_it_always_had() {
    // Nothing kept on a Mac or Linux is renamed: the name is the first 16 of the SHA-256 of the path as it was.
    for app in ["/Users/you/app", "/home/you/app", "/app"] {
        let before = crate::plain::sha256(app.as_bytes())[..16].to_owned();
        assert_eq!(name_of(app, None), before, "{app}");
    }
    // Nor one on Windows from its canonical path, which is already in the one form.
    let before = crate::plain::sha256(HOST.as_bytes())[..16].to_owned();
    assert_eq!(name_of(HOST, None), before);
}
