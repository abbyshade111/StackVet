//! Folders the owner leaves out of the checks, `[[left-out]]` in stackvet.toml (ADR-099).
//!
//! Every usable entry's folder is read apart from the app (`Manifest::set_apart`), so nothing in it
//! is evidence of what the app uses. An entry a person confirmed through `sv review` also leaves the
//! folder out of the count: a finding there is set aside unless it holds a key, which counts
//! wherever it sits. An entry nobody confirmed waits, and its folder is read as `not-the-app`:
//! checked, its findings listed apart and counted. stackvet.toml is written by the AI coding tool,
//! and a line in it that took findings out of the count by itself would hide them.

use sv_check::Finding;
use sv_check::seal::Checker;
use sv_manifest::Manifest;

/// One usable `[[left-out]]` entry, and whether it counts.
#[derive(Debug, Clone)]
pub struct Entry {
    /// The folder, as a path from the app folder.
    pub folder: String,
    /// The owner's reason, as written.
    pub why: String,
    /// Who confirmed it and when (`owner on 2026-10-12`, with where it was sealed), or why it does
    /// not count yet.
    pub confirmed: Result<String, String>,
}

/// The `[[left-out]]` entries of one app, as this computer reads them.
#[derive(Debug, Clone, Default)]
pub struct LeftOut {
    pub entries: Vec<Entry>,
    /// Entries that cannot be used, each with why.
    pub refused: Vec<String>,
}

impl LeftOut {
    /// The entries of `manifest`, each checked against what this computer can verify seals with.
    pub fn of(manifest: &Manifest, seals: &Checker) -> LeftOut {
        let (used, refused) = manifest.left_out();
        let entries = used
            .into_iter()
            .map(|(i, folder)| {
                let entry = &manifest.left_out[i];
                Entry {
                    folder,
                    why: entry.why.trim().to_owned(),
                    confirmed: confirmed(entry, seals),
                }
            })
            .collect();
        LeftOut { entries, refused }
    }

    /// The folders left out of the count: those confirmed, when the scan used the folders set apart
    /// (`used`, the scan's own list). None when the scan did not, since a list that would set apart
    /// all the app's code is not used at all (ADR-031).
    pub fn counted_out(&self, used: &[String]) -> Vec<String> {
        (self.entries.iter())
            .filter(|e| e.confirmed.is_ok() && used.contains(&e.folder))
            .map(|e| e.folder.clone())
            .collect()
    }

    /// The report's lines for these folders: one per entry, saying its reason, whether it counts,
    /// how many files it holds, and how many findings it set aside; then the refused entries.
    /// `used` is the scan's own list, `files` the app's file paths, `set_aside` the findings left out.
    pub fn gaps(
        &self,
        used: &[String],
        files: &[&str],
        set_aside: &[Finding],
    ) -> Vec<sv_report::Gap> {
        let mut gaps = Vec::new();
        for e in &self.entries {
            let held = (files.iter())
                .filter(|f| sv_scan::under_any(f, std::slice::from_ref(&e.folder)))
                .count();
            let held = format!("{held} file{}", if held == 1 { "" } else { "s" });
            let why = match &e.confirmed {
                Ok(by) if used.contains(&e.folder) => {
                    let out = (set_aside.iter())
                        .filter(|f| {
                            sv_scan::under_any(&f.location.file, std::slice::from_ref(&e.folder))
                        })
                        .count();
                    format!(
                        "Left out by the owner's word, confirmed by {by}: \"{}\". It holds {held}. \
                         {out} finding{} there {} not counted; keys and passwords are still looked \
                         for there, and one found counts as anywhere else.",
                        e.why,
                        if out == 1 { "" } else { "s" },
                        if out == 1 { "is" } else { "are" },
                    )
                }
                Ok(by) => format!(
                    "Confirmed by {by} (\"{}\"), but not left out: together with the other folders \
                     set apart it would leave none of the app's code, so every folder is read as \
                     the app.",
                    e.why
                ),
                Err(not_yet) => format!(
                    "stackvet.toml leaves it out (\"{}\"), but {not_yet}, so it is not left out yet: \
                     its {held} {} checked and its findings count, listed with test and sample code. \
                     Run `sv review` in your own terminal to confirm it.",
                    e.why,
                    if held.starts_with("1 ") { "is" } else { "are" },
                ),
            };
            gaps.push(sv_report::Gap {
                what: format!("the folder `{}`, left out of the checks", e.folder),
                why,
                reason: sv_report::GapReason::LeftOut,
                requirements: Vec::new(),
            });
        }
        if !self.refused.is_empty() {
            gaps.push(sv_report::Gap {
                what: "entries in `[[left-out]]` that were refused".to_owned(),
                why: format!(
                    "{}. Everything they would have named is checked as the app.",
                    self.refused.join("; ")
                ),
                reason: sv_report::GapReason::CouldNotRead,
                requirements: Vec::new(),
            });
        }
        gaps
    }
}

/// Splits `findings` into those that count and those in a folder left out of the count (`folders`).
/// A finding that holds a key or password counts wherever it is (ADR-099): the owner's answer of
/// 12 October 2026, that an exposed key is serious enough to report wherever it sits.
pub fn split(findings: Vec<Finding>, folders: &[String]) -> (Vec<Finding>, Vec<Finding>) {
    if folders.is_empty() {
        return (findings, Vec::new());
    }
    findings
        .into_iter()
        .partition(|f| f.secret.is_some() || !sv_scan::under_any(&f.location.file, folders))
}

/// Who confirmed `entry` and when, or why it does not count: it must say it is the owner's, and
/// carry a seal `sv review` made that holds on this computer.
fn confirmed(entry: &sv_manifest::LeftOut, seals: &Checker) -> Result<String, String> {
    if entry.seal.is_none() {
        return Err("nobody has confirmed it through `sv review`".to_owned());
    }
    if entry.by.as_deref().map(str::trim) != Some(sv_check::design::OWNER) {
        return Err("it is not marked as the owner's (`by = \"owner\"`)".to_owned());
    }
    let fields = sv_check::seal::left_out_fields(entry);
    match sv_check::seal::owner_recorded(seals, entry.seal.as_deref(), &fields) {
        Ok(sealed) => Ok(format!(
            "the owner on {}{}",
            entry.on.as_deref().unwrap_or("").trim(),
            match sealed {
                sv_check::seal::Sealed::Here => " on this computer".to_owned(),
                sv_check::seal::Sealed::Signed { key, from, lock } => {
                    format!(", {}", sv_check::seal::signed_with(&key, from, lock))
                }
            }
        )),
        Err(why) => Err(format!("its confirmation does not count here: {why}")),
    }
}

#[cfg(test)]
mod tests;
