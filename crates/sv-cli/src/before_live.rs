//! `sv before-live [PATH]`: the checklist before going live from the app's last report, printed again (backlog 0589,
//! ADR-121).
//!
//! The checklist is written with each report, as `before-going-live.md` in the report folder. Printed from there, it
//! is the list of the report it came with, never one made up afresh from a folder that may have changed since, and
//! never one the AI coding tool could have written: the folder's seal must hold and the bytes printed must be the ones
//! sealed, as `sv explain --app` asks of `report.json`. It checks nothing and writes nothing.

use std::io::Read;
use std::path::{Path, PathBuf};

/// The checklist of the app at `rest`'s one word (this folder when there is none), or why it cannot be shown.
pub fn command(rest: &[String]) -> anyhow::Result<String> {
    let app_dir = rest
        .iter()
        .find(|a| !a.starts_with("--"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    checklist_of(&app_dir).map_err(|why| anyhow::anyhow!(why))
}

/// The checklist kept with the last report of the app in `app_dir`.
pub fn checklist_of(app_dir: &Path) -> Result<String, String> {
    let folder = sv_scan::ecosystems::default_report_dir_in(app_dir);
    let cannot = |why: String| {
        format!(
            "There is no checklist to print from {}: sv cannot show it wrote the report there ({why}). Write the \
             report with `sv report`; the checklist is written with it.",
            folder.display()
        )
    };
    let sealed = crate::report_seal::proven(&folder).map_err(cannot)?;
    let path = folder.join(sv_report::checklist::FILE);
    if !std::fs::symlink_metadata(&path).is_ok_and(|m| m.is_file()) {
        return Err(cannot(format!(
            "{} is not a file",
            sv_report::checklist::FILE
        )));
    }
    let mut bytes = Vec::new();
    std::fs::File::open(&path)
        .and_then(|file| {
            file.take(crate::mcp::MAX_RESOURCE_BYTES + 1)
                .read_to_end(&mut bytes)
        })
        .map_err(|e| {
            cannot(format!(
                "{} could not be read: {e}",
                sv_report::checklist::FILE
            ))
        })?;
    match sealed.get(sv_report::checklist::FILE) {
        Some(digest) if *digest == crate::bundle::sha256(&bytes) => {}
        Some(_) => {
            return Err(cannot(format!(
                "{} changed after it was sealed",
                sv_report::checklist::FILE
            )));
        }
        None => {
            return Err(cannot(format!(
                "its seal does not cover {}",
                sv_report::checklist::FILE
            )));
        }
    }
    String::from_utf8(bytes)
        .map_err(|_| cannot(format!("{} is not text", sv_report::checklist::FILE)))
}
