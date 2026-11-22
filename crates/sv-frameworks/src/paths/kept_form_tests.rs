//! One form of a folder's path for what is kept about it, and the person's own path of a folder in a Linux container on
//! Windows (backlog 0728). Pure, so they are tested on every system.

use super::*;

#[test]
fn a_windows_path_is_put_in_one_form() {
    let one = r"C:\Users\you\app";
    for given in [
        r"C:\Users\you\app",
        r"C:\Users\you\app\",
        r"c:\Users\you\app",
        r"\\?\C:\Users\you\app",
        r"\\?\c:\Users\you\app\",
        "C:/Users/you/app",
    ] {
        assert_eq!(kept_form(given), one, "{given}");
    }
    // A drive's root keeps its separator.
    assert_eq!(kept_form(r"d:\"), r"D:\");
    // The rest keeps its case: both sides take it from `canonical`, which gives the disk's.
    assert_eq!(kept_form(r"C:\Users\You\App"), r"C:\Users\You\App");
}

#[test]
fn any_other_path_is_as_it_was() {
    for given in [
        "/Users/you/app",
        "/c/Users/you/app",
        "/",
        r"\\server\share\app",
        r"\\?\UNC\server\share",
        "app",
    ] {
        assert_eq!(kept_form(given), given);
    }
}

#[test]
fn a_path_in_the_container_is_the_person_s_own_under_the_folder_it_was_told() {
    let host = r"C:\Users\you\app";
    assert_eq!(host_path("/c/Users/you/app", host).as_deref(), Some(host));
    assert_eq!(host_path("/c/Users/you/app/", host).as_deref(), Some(host));
    assert_eq!(
        host_path("/c/Users/you/app/web/src", host).as_deref(),
        Some(r"C:\Users\you\app\web\src")
    );
    // Told with a separator at the end, or with `/`: the same folder.
    assert_eq!(
        host_path("/c/Users/you/app", r"C:\Users\you\app\").as_deref(),
        Some(host)
    );
    assert_eq!(
        host_path("/c/Users/you/app", "C:/Users/you/app").as_deref(),
        Some(host)
    );
    // A drive's root.
    assert_eq!(host_path("/d/work", r"D:\").as_deref(), Some(r"D:\work"));
}

#[test]
fn a_path_outside_that_folder_or_a_host_that_is_no_windows_folder_is_not_turned() {
    let host = r"C:\Users\you\app";
    assert_eq!(
        host_path("/c/Users/you/apple", host),
        None,
        "a longer name is another folder"
    );
    assert_eq!(host_path("/c/Users/you", host), None);
    assert_eq!(host_path("/d/Users/you/app", host), None);
    assert_eq!(host_path("/c/Users/you/app", "/Users/you/app"), None);
    assert_eq!(host_path("/c/Users/you/app", ""), None);
}
