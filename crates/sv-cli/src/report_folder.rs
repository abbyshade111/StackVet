//! The one way a report folder is written, for `sv report` and the MCP server's
//! `stackvet_write_report` alike.
//!
//! Until 8 October 2026 each of the two kept its own copy of the sequence: the folder claimed, the
//! report built, a changed manifest noted, an older report refused, the files written, the folder
//! sealed, the claim released. Each guard added to one (the lock, ADR-041; the seal, ADR-034; the
//! refusal of an older report) had to be remembered for the other, and the architecture assessment of
//! that day listed the pair as a cost (BACKLOG, "From the architecture assessment of 8 October 2026",
//! item 5). Now there is one sequence, and what differs between the two callers is passed in: how
//! the report is built, what the run is called in the lock, where a second run is told to write
//! instead, and what is done with what `sv` has to say on the way.

use crate::{Written, claim_report_folder, report_lock, seal_report_folder, write_report};
use anyhow::Result;
use std::path::Path;

/// What writing a report folder came to.
pub struct ReportFolder {
    /// The report as written, a changed manifest noted in its gaps.
    pub report: sv_report::Report,
    /// The files written, with their text.
    pub written: Written,
    /// Whether the folder was sealed, so the MCP server will offer it as a report `sv` wrote.
    pub sealed: bool,
}

/// Writes a report of the app at `app_dir` into `out_dir`, in the order every writer of a report
/// folder keeps:
///
/// 1. The folder is claimed (`claim_report_folder`: the lock, the marker, the refusals) before
///    anything slow happens, so a second run is refused at once rather than after the wait, and
///    `command` is what the lock names this run as. `elsewhere` is what that second run is told.
/// 2. The report is built, which may start the app and take as long as it takes.
/// 3. A manifest that changed while it was built is said, and goes into the report as a gap.
/// 4. A report already in the folder from a newer run is refused, so this one does not replace it.
/// 5. The six report files are written (`write_report`), each under a new name and renamed into place.
/// 6. The folder is sealed (`seal_report_folder`), so the MCP server can tell the report is `sv`'s.
/// 7. The claim is released as written: the lock goes, the folder and the report stay.
///
/// Whatever `sv` has to say on the way (the lock's notes, the manifest, the seal) goes to `say` as
/// it happens: at a terminal it is printed before the wait, and the MCP server collects it for its
/// reply. A step that fails leaves the folder as the claim found it: the marker goes if the claim
/// wrote it, and the folder if the claim or the caller (`made_by_caller`) made it and it holds
/// nothing else.
pub fn write_report_folder(
    app_dir: &Path,
    out_dir: &Path,
    command: &str,
    elsewhere: &str,
    made_by_caller: bool,
    build: impl FnOnce() -> Result<sv_report::Report>,
    say: &mut dyn FnMut(&str),
) -> Result<ReportFolder> {
    let held = claim_report_folder(out_dir, command, elsewhere, made_by_caller)?;
    for note in &held.notes {
        say(note);
    }
    let mut report = build()?;
    // What the record of the build loop shows, read now and carried under this report's seal
    // (ADR-076).
    let mut build_loop = crate::build_loop::read(app_dir);
    let false_alarms: Vec<String> = report
        .set_aside
        .iter()
        .filter(|s| {
            !report
                .findings
                .iter()
                .any(|f| f.fingerprint == s.finding.fingerprint)
        })
        .map(|s| s.finding.fingerprint.clone())
        .collect();
    crate::build_loop::settle(&mut build_loop, &false_alarms);
    crate::build_loop::look_again(&mut build_loop, app_dir, &false_alarms);
    report.build_loop = Some(build_loop);
    // A manifest read under its old name is said at the terminal as the report says it (ADR-062).
    if let Some(note) = sv_manifest::locate(app_dir)
        .ok()
        .flatten()
        .and_then(|l| l.note())
    {
        say(&format!("Note: {note}"));
    }
    if let Some((note, gap)) = report_lock::manifest_changed(&report, app_dir) {
        say(&note);
        report.gaps.push(gap);
    }
    report_lock::refuse_older(&report, out_dir, elsewhere)?;
    let written = write_report(&report, out_dir)?;
    let (sealed, notes) = seal_report_folder(out_dir, &written);
    for note in &notes {
        say(note);
    }
    held.written();
    drop(held);
    Ok(ReportFolder {
        report,
        written,
        sealed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "sv-report-folder-{name}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn report_of(app: &Path) -> sv_report::Report {
        crate::assemble_report(
            app,
            &crate::ReportOptions::reading_only("a test"),
            &crate::Loaded::load().unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn the_folder_is_claimed_before_the_report_is_built_and_released_sealed_after() {
        // Keys of this test process's own, so no key is read or made on the computer running it.
        sv_check::seal::key_folder_for_tests(
            std::env::temp_dir().join(format!("sv-report-folder-test-keys-{}", std::process::id())),
        );
        let app = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/flask-booking");
        let dir = scratch("order");
        let out = dir.join("stackvet-report");
        let lock = out.join(report_lock::LOCK_NAME);
        let marker = out.join(sv_scan::ecosystems::REPORT_MARKER);
        let mut said = Vec::new();
        let mut seen_while_building = None;
        let folder = write_report_folder(
            &app,
            &out,
            "a test",
            "elsewhere",
            false,
            || {
                // The claim is already on the folder while the report is being built.
                seen_while_building = Some((lock.is_file(), marker.is_file()));
                Ok(report_of(&app))
            },
            &mut |note| said.push(note.to_owned()),
        )
        .unwrap();
        assert_eq!(seen_while_building, Some((true, true)), "{said:?}");
        assert!(
            !lock.exists(),
            "the lock stayed after the report was written"
        );
        assert!(marker.is_file(), "the marker went with the lock");
        assert_eq!(folder.written.names(), crate::report_files::NAMES);
        for name in crate::report_files::NAMES {
            assert!(out.join(name).is_file(), "{name} was not written");
        }
        assert!(folder.sealed, "{said:?}");
        assert!(
            crate::report_seal::proven(&out).is_ok(),
            "the folder is not sealed as sv's"
        );
        assert!(
            !folder
                .report
                .gaps
                .iter()
                .any(|g| g.why.contains("changed while")),
            "an unchanged manifest was reported as changed"
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_report_from_an_older_run_does_not_replace_a_newer_one_already_there() {
        sv_check::seal::key_folder_for_tests(std::env::temp_dir().join(format!(
            "sv-report-folder-older-test-keys-{}",
            std::process::id()
        )));
        let app = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/flask-booking");
        let dir = scratch("older");
        let out = dir.join("stackvet-report");
        let newer = report_of(&app);
        assert!(
            newer.run_record.is_some(),
            "the report carries no run record"
        );
        let mut quiet = |_: &str| {};
        write_report_folder(
            &app,
            &out,
            "a test",
            "elsewhere",
            false,
            || Ok(newer.clone()),
            &mut quiet,
        )
        .unwrap();
        let before = std::fs::read_to_string(out.join("report.json")).unwrap();
        // The same report, from a run that started ten seconds before the one already written.
        let mut older = newer.clone();
        older.run_record.as_mut().unwrap().started_unix_ms -= 10_000;
        let result = write_report_folder(
            &app,
            &out,
            "a test",
            "elsewhere",
            false,
            || Ok(older.clone()),
            &mut quiet,
        );
        let err = match result {
            Ok(_) => panic!("the older run's report replaced the newer one"),
            Err(e) => format!("{e:#}"),
        };
        assert!(
            err.contains("already holds a report from a run that started"),
            "{err}"
        );
        assert!(err.contains("elsewhere"), "{err}");
        assert_eq!(
            std::fs::read_to_string(out.join("report.json")).unwrap(),
            before,
            "the newer report was changed"
        );
        assert!(
            !out.join(report_lock::LOCK_NAME).exists(),
            "the refused run kept the lock"
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_report_that_cannot_be_built_leaves_no_folder_behind() {
        let app = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/flask-booking");
        let dir = scratch("undone");
        let out = dir.join("stackvet-report");
        let mut said = Vec::new();
        let err = match write_report_folder(
            &app,
            &out,
            "a test",
            "elsewhere",
            false,
            || anyhow::bail!("the app could not be read"),
            &mut |note| said.push(note.to_owned()),
        ) {
            Ok(_) => panic!("a build that fails is a write that fails"),
            Err(e) => e,
        };
        assert!(format!("{err:#}").contains("could not be read"), "{err:#}");
        assert!(
            !out.exists(),
            "the folder made for the report stayed after nothing was written"
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_sealed_report_copied_or_moved_to_another_folder_is_not_proven_sv_s() {
        // Backlog 0286. The seal covered the files and nothing of where they were, so a clean report
        // kept from an earlier run, or another app's, put into `stackvet-report/` was offered by the
        // MCP server as `sv`'s current report.
        sv_check::seal::key_folder_for_tests(std::env::temp_dir().join(format!(
            "sv-report-folder-copied-test-keys-{}",
            std::process::id()
        )));
        let app = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/flask-booking");
        let dir = scratch("copied");
        let out = dir.join("stackvet-report");
        let mut quiet = |_: &str| {};
        let report = report_of(&app);
        write_report_folder(
            &app,
            &out,
            "a test",
            "elsewhere",
            false,
            || Ok(report),
            &mut quiet,
        )
        .unwrap();
        assert!(
            crate::report_seal::proven(&out).is_ok(),
            "the setup: sealed where it was written"
        );

        // Every file as it is, in another app's report folder.
        let elsewhere = dir.join("another-app").join("stackvet-report");
        std::fs::create_dir_all(&elsewhere).unwrap();
        let marker_name = sv_scan::ecosystems::REPORT_MARKER;
        for name in crate::report_files::NAMES.iter().chain([&marker_name]) {
            std::fs::copy(out.join(name), elsewhere.join(name)).unwrap();
        }
        let why = crate::report_seal::proven(&elsewhere).expect_err("a copy is not proven sv's");
        assert!(why.contains("sealed for another folder"), "{why}");

        // Nor does naming the new folder in the marker help: the folder is sealed with the files.
        let marker = std::fs::read_to_string(elsewhere.join(marker_name)).unwrap();
        let here = sv_check::seal::App::of(&elsewhere).unwrap().id().to_owned();
        let renamed: String = marker
            .lines()
            .map(|l| match l.strip_prefix("sealed-in: ") {
                Some(_) => format!("sealed-in: {here}\n"),
                None => format!("{l}\n"),
            })
            .collect();
        assert_ne!(renamed, marker, "the setup: the marker names a folder");
        std::fs::write(elsewhere.join(marker_name), renamed).unwrap();
        let why = crate::report_seal::proven(&elsewhere).expect_err("a renamed copy is not proven");
        assert!(why.contains("does not match"), "{why}");
        // A seal that names the folder it was made in is not edited to name this one.
        assert!(!why.contains("another folder"), "{why}");

        // A report sealed before seals named their folder is asked to be written again.
        let unnamed: String = marker
            .lines()
            .filter(|l| !l.starts_with("sealed-in: "))
            .map(|l| format!("{l}\n"))
            .collect();
        std::fs::write(out.join(marker_name), unnamed).unwrap();
        let why = crate::report_seal::proven(&out).expect_err("an unnamed seal is not proven");
        assert!(why.contains("write the report again"), "{why}");
        std::fs::remove_dir_all(&dir).ok();
    }
}
