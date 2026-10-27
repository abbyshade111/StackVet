//! The proof that a report folder is one `sv` wrote (deep review R9, ADR-034).
//!
//! The MCP server offers the reports below its root as resources, "a report sv wrote". It took any
//! folder holding the marker, `.stackvet-report`, as one, and anything can write that marker: the
//! review offered the AI coding tool a forged report that way. H6 of the same review made the walk of
//! the app believe the marker only in a folder holding nothing but the files `sv` writes, which keeps a
//! marker from hiding code, but a forged report is exactly such a folder.
//!
//! So `sv report` and `stackvet_write_report` seal what they wrote: the marker gains a line
//! `seal: v1:<key id>:<mac>`, an HMAC-SHA-256 of the folder it is in and each report file's name and
//! SHA-256, under a key of this computer's kept beside the review key (`sv_check::seal::REPORT_KEY_FILE`),
//! outside every app, made the first time a report is written. A report copied or moved to another
//! folder is not `sv`'s report of that folder (backlog 0286). A folder is offered as `sv`'s only when it holds nothing but
//! `sv`'s files (H6) and its seal holds for the files as they are now, and a file is handed over only
//! when what was read is what was sealed.
//!
//! What it shows and what it cannot, as for the review seal (ADR-026): that `sv` on this computer
//! wrote these files and nothing has changed them since. A report written on another computer, or in
//! a container since gone, cannot be checked here, and is not offered: a made-up seal would look the
//! same. Something running as the person could read the key and forge a seal; what the seal stops is
//! a report shipped in the app's own files, or written by anything that only writes into the app.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use sv_check::seal::{Key, REPORT_KEY_FILE};

/// The files a seal covers, in the order they are sealed: every report file `sv` writes, from the
/// one list of them.
pub const SEALED: [&str; crate::report_files::NAMES.len()] = crate::report_files::NAMES;

/// How the seal's line in the marker starts.
pub(crate) const SEAL_LINE: &str = "seal: ";

/// How the marker's line naming the folder the report was sealed in starts: the id `seal::App` gives
/// that folder, sealed with the files, so a report copied or moved to another folder is refused
/// (backlog 0286; the review seal was bound to its app for the same reason, ADR-026 item 11).
const FOLDER_LINE: &str = "sealed-in: ";

/// The largest file sealed or checked. A report of a very large app is a few megabytes.
const MAX_BYTES: u64 = 16 * 1024 * 1024;

/// The largest marker read: a sentence and a seal.
const MAX_MARKER_BYTES: u64 = 4096;

/// The SHA-256 of each sealed file, by name.
type Digests = BTreeMap<&'static str, String>;

/// What the marker says once the folder is sealed: the sentence it always said, the folder it was
/// sealed in, then the seal.
pub fn marker_text(sentence: &str, folder: &str, seal: &str) -> String {
    format!("{sentence}{FOLDER_LINE}{folder}\n{SEAL_LINE}{seal}\n")
}

/// The id of the folder `dir` is, as the seal names it: the same folder reached by another path is
/// the same folder.
fn folder_id(dir: &Path) -> Result<String, String> {
    Ok(sv_check::seal::App::of(dir)?.id().to_owned())
}

fn fields(folder: &str, digests: &BTreeMap<&'static str, String>) -> Vec<String> {
    let mut fields = vec!["report-folder".to_owned(), folder.to_owned()];
    for name in SEALED {
        fields.push(name.to_owned());
        fields.push(digests.get(name).cloned().unwrap_or_default());
    }
    fields
}

/// The SHA-256 of every sealed file in `dir`, each a plain file no larger than any report.
/// The digest of each sealed file from the bytes `sv` wrote to it, every one of them required.
fn digests_of(written: &[(&str, &[u8])]) -> Result<BTreeMap<&'static str, String>, String> {
    let mut found = BTreeMap::new();
    for name in SEALED {
        let (_, bytes) = written
            .iter()
            .find(|(n, _)| *n == name)
            .ok_or_else(|| format!("{name} was not written"))?;
        found.insert(name, crate::bundle::sha256(bytes));
    }
    Ok(found)
}

/// The SHA-256 of each sealed file in `dir`, with the bytes of `report.json` as they were read for its
/// digest, so a caller can use the very bytes the seal was checked against rather than read the file
/// again (backlog 0454).
fn digests_keeping_report(dir: &Path) -> Result<(Digests, Vec<u8>), String> {
    let mut found = BTreeMap::new();
    let mut report = Vec::new();
    for name in SEALED {
        let path = dir.join(name);
        let meta = std::fs::symlink_metadata(&path).map_err(|_| missing(name))?;
        if !meta.is_file() {
            return Err(format!("{name} is a link or a folder, not a report file"));
        }
        if meta.len() > MAX_BYTES {
            return Err(format!("{name} is larger than any report sv writes"));
        }
        let bytes = std::fs::read(&path).map_err(|_| format!("{name} cannot be read"))?;
        found.insert(name, crate::bundle::sha256(&bytes));
        if name == "report.json" {
            report = bytes;
        }
    }
    Ok((found, report))
}

/// Why a sealed file is not there. `seen.json` joined the report files with ADR-082, so a report
/// written before then has none, and its seal was made over the five before it: writing the report
/// again seals all six.
fn missing(name: &str) -> String {
    if name == sv_report::seen::FILE {
        format!(
            "{name} is missing, as it is from a report written by an sv from before it kept what the \
             running app answered, whose seal covers five files and not the six sv seals now; write \
             the report again"
        )
    } else if name == sv_report::checklist::FILE
        || sv_report::pages::PAGES.iter().any(|p| p.file == name)
    {
        // Backlog 0589, ADR-121.
        format!(
            "{name} is missing, as it is from a report written by an sv from before it wrote that page, whose \
             seal does not cover it; write the report again"
        )
    } else {
        format!("{name} is missing")
    }
}

/// Where this computer's report key is, or why there is nowhere for it.
fn key_folder() -> Result<PathBuf, String> {
    Key::folder().ok_or_else(|| {
        "this computer has no folder for sv's keys (neither HOME nor XDG_CONFIG_HOME is set)"
            .to_owned()
    })
}

/// What sealing a report folder came to.
pub struct Sealed {
    /// Where the report key is, when it was made by this run.
    pub made_key: Option<PathBuf>,
}

/// Seals the report `sv` has just written in `dir`, the marker last, while the run still holds the
/// folder. `written` is each sealed file's name and the bytes `sv` wrote to it: the seal is made from
/// those, not from the files read back, so anything that changes a file between the writing and the
/// sealing is not sealed as `sv`'s (the review of 6 October, item 17); the next read compares the
/// files with the seal and finds the change. `sentence` is what the marker says above the seal.
/// `Err` says why it could not be sealed, for the caller to say; the report itself stands either way.
pub fn seal(dir: &Path, sentence: &str, written: &[(&str, &[u8])]) -> Result<Sealed, String> {
    seal_in(&key_folder()?, dir, sentence, written)
}

/// `seal`, with the report key kept in `folder`.
fn seal_in(
    folder: &Path,
    dir: &Path,
    sentence: &str,
    written: &[(&str, &[u8])],
) -> Result<Sealed, String> {
    let (key, made) = Key::load_or_make_named(folder, REPORT_KEY_FILE)?;
    let digests = digests_of(written)?;
    let here = folder_id(dir)?;
    let fields = fields(&here, &digests);
    let seal = key.report_seal(&fields.iter().map(String::as_str).collect::<Vec<_>>());
    crate::write_without_following(
        dir,
        sv_scan::ecosystems::REPORT_MARKER,
        marker_text(sentence, &here, &seal).as_bytes(),
    )
    .map_err(|e| format!("{e:#}"))?;
    // A marker under the old name (ADR-062) is `sv`'s own file, replaced by the one just written.
    let old = dir.join(sv_frameworks::names::OLD_REPORT_MARKER);
    if std::fs::symlink_metadata(&old).is_ok_and(|m| m.is_file()) {
        std::fs::remove_file(&old).map_err(|e| format!("removing the old marker: {e}"))?;
    }
    Ok(Sealed {
        made_key: made.then(|| folder.join(REPORT_KEY_FILE)),
    })
}

/// The SHA-256 of each file of the report in `dir`, when `sv` can show it wrote them on this
/// computer and nothing has changed them since; otherwise why not, as the end of a sentence
/// beginning "sv cannot show it wrote this report:".
pub fn proven(dir: &Path) -> Result<BTreeMap<&'static str, String>, String> {
    holds_only_svs(dir)?;
    sealed_here(dir)
}

/// `proven`, giving back the bytes of the report's `report.json` that its seal was checked against:
/// what the MCP server reads the report's date from, with no second read of a file that could have
/// been replaced since (backlog 0454).
pub fn proven_report(dir: &Path) -> Result<Vec<u8>, String> {
    holds_only_svs(dir)?;
    match check_keeping_report(dir)? {
        (_, Place::Here, report) => Ok(report),
        (_, Place::Elsewhere, _) => Err(
            "it was sealed for another folder: it was copied or moved here from where sv wrote it"
                .to_owned(),
        ),
    }
}

/// `proven_wherever`, giving back the bytes of `report.json` its seal was checked against: what the OSCAL export reads
/// (ADR-156), with no second read of a file that could have been replaced since, as `proven_report` does.
pub fn proven_report_wherever(dir: &Path) -> Result<(Vec<u8>, Place), String> {
    holds_only_svs(dir)?;
    check_keeping_report(dir).map(|(_, place, report)| (report, place))
}

/// Where a report whose seal holds was sealed, as against where its folder is now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Place {
    /// In the folder it is in.
    Here,
    /// In another folder, and copied or moved here since: still `sv`'s report and unchanged, but of
    /// the folder it was written in, at the time its run record says.
    Elsewhere,
}

/// `proven`, for a report the person names by its place (`sv explain --report`, `sv compare`): a
/// report kept somewhere on purpose was sealed where it was written, and is still `sv`'s and
/// unchanged. It says which, for the caller to say. What is offered as an app's current report, by
/// the MCP server or from the app's own report folder, asks `proven` (backlog 0286).
pub fn proven_wherever(dir: &Path) -> Result<(BTreeMap<&'static str, String>, Place), String> {
    holds_only_svs(dir)?;
    check(dir)
}

/// The marker is one `sv` writes and the folder holds nothing else (H6).
fn holds_only_svs(dir: &Path) -> Result<(), String> {
    let marker = sv_scan::ecosystems::report_marker_in(dir)
        .ok_or_else(|| "the folder holds no marker of sv's".to_owned())?;
    let meta = std::fs::symlink_metadata(&marker)
        .map_err(|_| "the folder holds no marker of sv's".to_owned())?;
    if !meta.is_file() || meta.len() > MAX_MARKER_BYTES {
        return Err("its marker is not one sv writes".to_owned());
    }
    // H6: a folder holding anything but what `sv` writes there is not a report folder of `sv`'s.
    if !sv_scan::ecosystems::is_sv_output(dir) {
        return Err("the folder holds files sv does not write in a report folder".to_owned());
    }
    Ok(())
}

/// The SHA-256 of each sealed file in `dir`, when its marker's seal holds for them under this
/// computer's report key; otherwise why not, as `proven` says it. Whatever else the folder holds is
/// not looked at: this says `sv` wrote the report there, not that the folder holds nothing else.
///
/// What `sv report` asks before it writes beside files it did not write (the review of 8 October,
/// item 4): a marker can be planted in any of the app's folders, and a seal cannot be made without
/// the key.
pub fn sealed_here(dir: &Path) -> Result<BTreeMap<&'static str, String>, String> {
    match check(dir)? {
        (digests, Place::Here) => Ok(digests),
        (_, Place::Elsewhere) => Err(
            "it was sealed for another folder: it was copied or moved here from where sv wrote it"
                .to_owned(),
        ),
    }
}

/// The SHA-256 of each sealed file in `dir` when its seal holds under this computer's report key,
/// and whether it was sealed in this folder or another.
fn check(dir: &Path) -> Result<(BTreeMap<&'static str, String>, Place), String> {
    check_keeping_report(dir).map(|(digests, place, _)| (digests, place))
}

/// `check`, with the bytes of `report.json` its seal was checked against.
fn check_keeping_report(dir: &Path) -> Result<(Digests, Place, Vec<u8>), String> {
    let marker = sv_scan::ecosystems::report_marker_in(dir)
        .ok_or_else(|| "the folder holds no marker of sv's".to_owned())?;
    let meta = std::fs::symlink_metadata(&marker)
        .map_err(|_| "the folder holds no marker of sv's".to_owned())?;
    if !meta.is_file() || meta.len() > MAX_MARKER_BYTES {
        return Err("its marker is not one sv writes".to_owned());
    }
    let text = std::fs::read_to_string(&marker)
        .map_err(|_| "its marker is not one sv writes".to_owned())?;
    let seal = text
        .lines()
        .find_map(|line| line.strip_prefix(SEAL_LINE))
        .ok_or_else(|| {
            "its marker holds no seal, so it was written by something other than sv, or by an sv \
             from before reports were sealed"
                .to_owned()
        })?;
    let key = match Key::load_named(&key_folder()?, REPORT_KEY_FILE)? {
        Some(key) => key,
        None => {
            return Err(
                "this computer has no report key to check its seal with, so it was written on \
                 another computer, or not by sv"
                    .to_owned(),
            );
        }
    };
    // The folder it was sealed in, said in the marker and sealed with the files: a report copied
    // from another app, or put back from somewhere it was kept, was sealed for another folder.
    let sealed_in = text
        .lines()
        .find_map(|line| line.strip_prefix(FOLDER_LINE))
        .ok_or_else(|| {
            "its seal does not say which folder it was written in, as a report written by an sv \
             from before reports were sealed to their folder; write the report again"
                .to_owned()
        })?;
    // The seal is checked against the folder the marker names first, so a seal made with another
    // key, or files changed since, are said as such; then a seal that holds, for another folder.
    let sealed_in = sealed_in.trim();
    let (digests, report) = digests_keeping_report(dir)?;
    let fields = fields(sealed_in, &digests);
    key.report_seal_holds(seal, &fields.iter().map(String::as_str).collect::<Vec<_>>())?;
    let place = if sealed_in == folder_id(dir)? {
        Place::Here
    } else {
        Place::Elsewhere
    };
    Ok((digests, place, report))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_seal_is_made_from_the_bytes_written_not_from_the_files_read_back() {
        // The review of 6 October, item 17: a file changed between the writing and the sealing was
        // sealed as sv's. The digests come from what sv wrote, whatever the folder holds by then.
        let dir = std::env::temp_dir().join(format!("sv-seal-bytes-{}", std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).unwrap();
        let written: Vec<(&str, &[u8])> =
            SEALED.iter().map(|n| (*n, &b"sv wrote this"[..])).collect();
        for name in SEALED {
            std::fs::write(dir.join(name), "changed after").unwrap();
        }
        let from_bytes = digests_of(&written).unwrap();
        let from_disk = digests_keeping_report(&dir).unwrap().0;
        std::fs::remove_dir_all(&dir).ok();
        for name in SEALED {
            assert_eq!(from_bytes[name], crate::bundle::sha256(b"sv wrote this"));
            assert_ne!(
                from_bytes[name], from_disk[name],
                "the setup: the files differ"
            );
        }
        // Every sealed file must be among what was written.
        assert!(digests_of(&written[1..]).is_err());
    }

    #[test]
    fn the_seal_written_holds_for_the_bytes_and_not_for_a_file_changed_before_sealing() {
        let base = std::env::temp_dir().join(format!("sv-seal-in-{}", std::process::id()));
        std::fs::remove_dir_all(&base).ok();
        let (keys, dir) = (base.join("keys"), base.join("report"));
        std::fs::create_dir_all(&dir).unwrap();
        let written: Vec<(&str, &[u8])> =
            SEALED.iter().map(|n| (*n, &b"sv wrote this"[..])).collect();
        // Changed on disk after the writing, before the sealing.
        for name in SEALED {
            std::fs::write(dir.join(name), "changed after").unwrap();
        }
        seal_in(&keys, &dir, "a report\n", &written).expect("sealed");
        let marker = std::fs::read_to_string(dir.join(sv_scan::ecosystems::REPORT_MARKER)).unwrap();
        let seal = marker
            .lines()
            .find_map(|l| l.strip_prefix(SEAL_LINE))
            .expect("the marker holds a seal")
            .to_owned();
        let key = Key::load_named(&keys, REPORT_KEY_FILE)
            .unwrap()
            .expect("the key");
        let here = folder_id(&dir).unwrap();
        let holds = |digests: &BTreeMap<&'static str, String>| {
            let fields = fields(&here, digests);
            key.report_seal_holds(
                &seal,
                &fields.iter().map(String::as_str).collect::<Vec<_>>(),
            )
        };
        let (bytes, disk) = (
            digests_of(&written).unwrap(),
            digests_keeping_report(&dir).unwrap().0,
        );
        std::fs::remove_dir_all(&base).ok();
        assert!(holds(&bytes).is_ok(), "the seal is of what sv wrote");
        assert!(
            holds(&disk).is_err(),
            "the files changed before sealing are not sealed as sv's"
        );
    }
}
