//! A file's real place, written the way people and Docker read it (backlog 0120).
//!
//! On Windows the standard library gives a real place in the long form, `\\?\C:\Users\me\app`.
//! Windows itself reads that form; Docker does not (`docker run -v \\?\C:\...:/app` is refused as an
//! "invalid spec"), and in a report or a settings file it is a puzzle to anybody who has not met it.
//! Every real place `sv` works out goes through `canonical`, which gives the short form, `C:\Users\me\app`,
//! wherever that names the same place. On macOS and Linux it is exactly `std::fs::canonicalize`.

use std::path::{Path, PathBuf};

/// The real place of `path`: links followed, `..` and `.` gone, and on Windows without the `\\?\`
/// prefix wherever the short form names the same place.
pub fn canonical(path: impl AsRef<Path>) -> std::io::Result<PathBuf> {
    let real = std::fs::canonicalize(path)?;
    if cfg!(windows)
        && let Some(short) = real.to_str().and_then(short_form)
    {
        return Ok(PathBuf::from(short));
    }
    Ok(real)
}

/// The same as `canonical`, called as a method: `folder.canonical()`.
pub trait Canonical {
    fn canonical(&self) -> std::io::Result<PathBuf>;
}

impl Canonical for Path {
    fn canonical(&self) -> std::io::Result<PathBuf> {
        canonical(self)
    }
}

/// Windows's longest path in the short form, counting the end: past it only the long form reaches.
const SHORT_LIMIT: usize = 260;

/// The short form of a Windows long-form path, or `None` where there is none that names the same
/// place: a drive path (`\\?\C:\a` to `C:\a`) or a network share (`\\?\UNC\server\share` to
/// `\\server\share`), short enough, and with no part Windows would read differently once the prefix
/// is gone (`/`, a name that ends in a dot or a space, or a device name such as `CON` or `NUL`).
/// Pure, so it is tested on every system.
pub fn short_form(long: &str) -> Option<String> {
    let short = if let Some(rest) = long.strip_prefix(r"\\?\UNC\") {
        format!(r"\\{rest}")
    } else {
        let rest = long.strip_prefix(r"\\?\")?;
        let drive = rest.as_bytes();
        if drive.len() < 3
            || !drive[0].is_ascii_alphabetic()
            || drive[1] != b':'
            || drive[2] != b'\\'
        {
            return None;
        }
        rest.to_owned()
    };
    if short.len() >= SHORT_LIMIT || short.contains('/') {
        return None;
    }
    let parts = short.trim_start_matches('\\').split('\\').skip(1);
    for part in parts.filter(|p| !p.is_empty()) {
        if part.ends_with('.') || part.ends_with(' ') || device(part) {
            return None;
        }
    }
    Some(short)
}

/// Whether Windows reads `part` as a device rather than a file: `CON`, `PRN`, `AUX`, `NUL`, `COM1`
/// to `COM9`, and `LPT1` to `LPT9`, in any case and with any extension.
fn device(part: &str) -> bool {
    let stem = part
        .split('.')
        .next()
        .unwrap_or(part)
        .trim_end()
        .to_ascii_uppercase();
    matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || ((stem.starts_with("COM") || stem.starts_with("LPT"))
            && stem.len() == 4
            && matches!(stem.as_bytes()[3], b'1'..=b'9'))
}

/// What `sv connect --docker` tells the container the app's folder is on the person's computer, when that is a
/// Windows folder (backlog 0375, ADR-153): `C:\Users\you\app`, shown inside at `/c/Users/you/app`.
pub const HOST_FOLDER_ENV: &str = "SV_HOST_FOLDER";

/// Whether `path` starts at a Windows drive's root: `C:\` or `C:/`.
pub fn is_drive_path(path: &str) -> bool {
    let b = path.as_bytes();
    b.len() >= 3 && b[0].is_ascii_alphabetic() && b[1] == b':' && (b[2] == b'\\' || b[2] == b'/')
}

/// A Windows folder as a Linux container shows it: the drive letter in lower case, then the rest with `/` for `\`
/// (`C:\Users\you\app` is `/c/Users/you/app`). `None` for a path that does not start at a drive's root.
pub fn inside_path(folder: &str) -> Option<String> {
    if !is_drive_path(folder) {
        return None;
    }
    let drive = folder[..1].to_ascii_lowercase();
    let rest = folder[2..].replace('\\', "/");
    let rest = rest.trim_end_matches('/');
    Some(format!("/{drive}{rest}"))
}

/// The person's own path of `inside`, a path in a Linux container whose app's folder is the Windows folder `host`:
/// `/c/Users/you/app/sub` is `C:\Users\you\app\sub` (backlog 0728). `None` when `inside` is not under the folder
/// `host` is shown at, or `host` is not a Windows folder.
pub fn host_path(inside: &str, host: &str) -> Option<String> {
    let host = if host.len() > 3 {
        host.trim_end_matches(['\\', '/'])
    } else {
        host
    };
    let shown = inside_path(host)?;
    let inside = inside.trim_end_matches('/');
    let head = inside.get(..shown.len())?;
    let rest = inside.get(shown.len()..)?;
    if !head.eq_ignore_ascii_case(&shown) || !(rest.is_empty() || rest.starts_with('/')) {
        return None;
    }
    let host = host.replace('/', "\\");
    let rest = rest.replace('/', "\\");
    Some(if host.ends_with('\\') {
        format!("{host}{}", rest.trim_start_matches('\\'))
    } else {
        format!("{host}{rest}")
    })
}

/// One form of a folder's full path, for naming what is kept about it outside it, so the same folder gets one name
/// whichever program writes it down (backlog 0728). A Windows path loses the `\\?\` before its drive, gets its drive
/// letter in upper case and `\` between its parts, and loses a separator at its end; any other path is as it was. The
/// rest of a Windows path keeps its case: both sides take it from the same `canonical`, which gives the case the
/// folder has on the disk, and a path lowered here would rename every note already kept on Windows.
pub fn kept_form(path: &str) -> String {
    let path = path
        .strip_prefix(r"\\?\")
        .filter(|rest| is_drive_path(rest))
        .unwrap_or(path);
    if !is_drive_path(path) {
        return path.to_owned();
    }
    let mut out = path[..1].to_ascii_uppercase();
    out.push_str(&path[1..].replace('/', "\\"));
    while out.len() > 3 && out.ends_with('\\') {
        out.pop();
    }
    out
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod kept_form_tests;
