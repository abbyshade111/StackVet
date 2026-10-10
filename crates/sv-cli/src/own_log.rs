//! The log of `sv`'s own stages, kept only when the owner asks for it with `SV_LOG` (backlog 0237). It holds the time, and
//! the names of the stages and of the command, never the arguments, the paths the owner typed, or what a report says.
//! A line that cannot be written is dropped: the log never ends a run.

use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// The file `SV_LOG` names, when it is set and not empty.
fn path() -> Option<PathBuf> {
    std::env::var_os("SV_LOG")
        .filter(|p| !p.is_empty())
        .map(PathBuf::from)
}

/// Appends one line, with the time, to the `SV_LOG` file. Does nothing when `SV_LOG` is not set.
pub fn line(text: &str) {
    let Some(path) = path() else {
        return;
    };
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
    {
        let _ = writeln!(file, "{secs} {text}");
    }
}
