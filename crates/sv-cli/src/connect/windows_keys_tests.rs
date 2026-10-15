//! `sv connect --keys` takes a Windows folder's full path (`C:\Users\…`), whose drive's colon Docker Desktop reads as
//! part of the path, and still refuses a colon or comma past it, which Docker reads as a separator. The Windows run
//! of 15 October 2026 refused the first as not a full path.

use super::check_keys_on;

fn said(keys: &str, windows: bool) -> String {
    match check_keys_on(keys, windows) {
        Ok(kept) => format!("kept {kept}"),
        Err(e) => e.to_string(),
    }
}

#[test]
fn a_windows_drive_path_is_a_full_path_there_and_nowhere_else() {
    // Past the path checks, the list is looked for: not there, so said, and the path was taken as a full one.
    for keys in [r"C:\Users\someone\no-such-sv-keys", "D:/no-such-sv-keys/"] {
        let windows = said(keys, true);
        assert!(windows.contains("is not there yet"), "{keys}: {windows}");
        let elsewhere = said(keys, false);
        assert!(
            elsewhere.contains("needs your key folder's full path"),
            "{keys}: {elsewhere}"
        );
    }
}

#[test]
fn past_the_drive_a_colon_or_comma_is_still_refused() {
    for keys in [r"C:\keys:x", r"C:\a,b", r"\\?\C:\keys"] {
        let windows = said(keys, true);
        assert!(
            windows.contains("Docker reads as separators") || windows.contains("full path"),
            "{keys}: {windows}"
        );
        assert!(!windows.starts_with("kept"), "{keys}: {windows}");
    }
}

#[test]
fn a_relative_path_is_refused_on_windows_too() {
    for keys in [r"keys\here", "C:keys", "~/.config/stackvet"] {
        let windows = said(keys, true);
        assert!(
            windows.contains("needs your key folder's full path"),
            "{keys}: {windows}"
        );
    }
}
