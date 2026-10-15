//! The report written again after `sv review` with the options of the last one, and `sv report` saying so before it
//! drops the running-app checks (backlog 0578).

use crate::report_again::*;
use std::path::{Path, PathBuf};
use sv_check::test_support::scratch;

/// A copy of the Flask example, with a report sealed in its default report folder whose run record keeps `options`
/// (`None`: no list, as an older report).
fn app_with_report(name: &str, options: Option<Vec<String>>) -> (PathBuf, PathBuf) {
    // Keys of this test process's own, so no key is read or made on the computer running it.
    sv_check::seal::key_folder_for_tests(
        std::env::temp_dir().join(format!("sv-report-again-test-keys-{}", std::process::id())),
    );
    let example = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/flask-booking");
    let app = scratch(name);
    for entry in std::fs::read_dir(&example).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            std::fs::copy(entry.path(), app.join(entry.file_name())).unwrap();
        }
    }
    let folder = sv_scan::ecosystems::default_report_dir_in(&app);
    crate::report_folder::write_report_folder(
        &app,
        &folder,
        "a test",
        "elsewhere",
        false,
        || {
            let mut report = crate::assemble_report(
                &app,
                &crate::ReportOptions::reading_only("a test"),
                &crate::Loaded::load()?,
            )?;
            report.run_record.as_mut().unwrap().options = options.clone();
            Ok(report)
        },
        &mut |_| {},
    )
    .unwrap();
    assert!(
        crate::report_seal::proven(&folder).is_ok(),
        "the setup: a report sealed where it was written"
    );
    (app, folder)
}

fn strings(words: &[&str]) -> Vec<String> {
    words.iter().map(|w| (*w).to_owned()).collect()
}

#[test]
fn the_options_kept_are_those_that_change_the_report_with_paths_made_absolute() {
    // The outside tools run unless --no-tools (ADR-111): nothing to say when they ran.
    assert_eq!(
        options_of(false, false, true, false, None, None),
        Vec::<String>::new()
    );
    // --slow means something only with --run.
    assert_eq!(
        options_of(false, true, false, false, None, None),
        strings(&["--no-tools"])
    );
    let kept = options_of(
        true,
        true,
        true,
        true,
        Some(Path::new("advisories")),
        Some(Path::new("codeql.sarif")),
    );
    assert_eq!(
        kept[..4],
        strings(&["--run", "--slow", "--keep-tool-output", "--advisories"])[..]
    );
    assert!(Path::new(&kept[4]).is_absolute(), "{kept:?}");
    assert!(kept[4].ends_with("advisories"), "{kept:?}");
    assert_eq!(kept[5], "--codeql-results");
    assert!(Path::new(&kept[6]).is_absolute(), "{kept:?}");
}

#[test]
fn review_writes_the_report_again_with_the_options_of_the_last_one() {
    let advisories = std::env::temp_dir().join("advisories");
    let kept = strings(&[
        "--run",
        "--no-tools",
        "--advisories",
        &advisories.display().to_string(),
    ]);
    let (app, _) = app_with_report("again", Some(kept.clone()));
    assert_eq!(
        last_options(&sv_scan::ecosystems::default_report_dir_in(&app)),
        Ok(kept.clone())
    );

    let mut ran = Vec::new();
    let mut said = Vec::new();
    write_again(&app, false, &mut said, &|| Ok(()), &mut |args| {
        ran.push(args.to_vec());
        Ok(crate::exit::NOT_ASSESSED)
    })
    .unwrap();
    let said = String::from_utf8(said).unwrap();
    let mut expected = vec![app.display().to_string()];
    expected.extend(kept);
    assert_eq!(ran, vec![expected]);
    assert!(
        said.contains("Writing the report again with the options of the last one"),
        "{said}"
    );
    assert!(said.contains("--run --no-tools --advisories"), "{said}");

    // --no-report: nothing run, and the command said.
    let mut said = Vec::new();
    write_again(&app, true, &mut said, &|| Ok(()), &mut |_| {
        panic!("--no-report runs nothing")
    })
    .unwrap();
    let said = String::from_utf8(said).unwrap();
    assert!(
        said.contains("With --no-report it is not written here"),
        "{said}"
    );
    assert!(said.contains("--run --no-tools"), "{said}");

    // Where the app cannot be started, as inside the container: nothing run, since that report would have the
    // running-app checks not assessed and replace this one.
    let mut said = Vec::new();
    write_again(
        &app,
        false,
        &mut said,
        &|| Err("no Docker here".to_owned()),
        &mut |_| panic!("a --run report is not written where the app cannot start"),
    )
    .unwrap();
    let said = String::from_utf8(said).unwrap();
    assert!(said.contains("no Docker here"), "{said}");
    assert!(
        said.contains("Run this where you wrote the last one"),
        "{said}"
    );
    // Without --run in the last one, whether containers start does not matter.
    let (plain, _) = app_with_report("again-plain", Some(Vec::new()));
    let mut ran = 0;
    write_again(
        &plain,
        false,
        &mut Vec::new(),
        &|| Err("no Docker here".to_owned()),
        &mut |_| {
            ran += 1;
            Ok(crate::exit::CLEAN)
        },
    )
    .unwrap();
    std::fs::remove_dir_all(&plain).ok();
    assert_eq!(ran, 1);

    // A report that did not get written again is said, with what to type.
    let failed = write_again(&app, false, &mut Vec::new(), &|| Ok(()), &mut |_| {
        Ok(crate::exit::FAILED)
    });
    std::fs::remove_dir_all(&app).ok();
    let failed = format!("{:#}", failed.unwrap_err());
    assert!(failed.contains("exit status 3"), "{failed}");
    assert!(failed.contains("--run --no-tools"), "{failed}");
}

#[test]
fn a_report_sv_cannot_show_it_wrote_is_not_run_from() {
    let (app, folder) = app_with_report("changed", Some(strings(&["--run", "--no-tools"])));
    // Changed after its seal: what it asks for is not run, as the AI coding tool could have written it.
    let text = std::fs::read_to_string(folder.join("report.json")).unwrap();
    let mut report: serde_json::Value = serde_json::from_str(&text).unwrap();
    report["run_record"]["options"] =
        serde_json::json!(["--run", "--no-tools", "--advisories", "/elsewhere"]);
    std::fs::write(folder.join("report.json"), report.to_string()).unwrap();
    assert!(last_options(&folder).is_err());
    let mut said = Vec::new();
    write_again(&app, false, &mut said, &|| Ok(()), &mut |_| {
        panic!("a changed report runs nothing")
    })
    .unwrap();
    std::fs::remove_dir_all(&app).ok();
    let said = String::from_utf8(said).unwrap();
    assert!(said.contains("is not written again here"), "{said}");
    assert!(said.contains("run `sv report` yourself"), "{said}");
}

#[test]
fn a_sealed_report_with_no_options_or_an_unknown_one_is_not_run_from() {
    let (app, folder) = app_with_report("older", None);
    let older = last_options(&folder);
    std::fs::remove_dir_all(&app).ok();
    assert!(older.unwrap_err().contains("does not say which options"));

    let (app, folder) =
        app_with_report("unknown", Some(strings(&["--run", "--out", "/elsewhere"])));
    let unknown = last_options(&folder);
    let mut said = Vec::new();
    write_again(&app, false, &mut said, &|| Ok(()), &mut |_| {
        panic!("an unknown option runs nothing")
    })
    .unwrap();
    std::fs::remove_dir_all(&app).ok();
    assert!(unknown.unwrap_err().contains("`--out`"));

    let (app, folder) = app_with_report("no-value", Some(strings(&["--run", "--advisories"])));
    let no_value = last_options(&folder);
    std::fs::remove_dir_all(&app).ok();
    assert!(no_value.unwrap_err().contains("no value"));
}

#[test]
fn a_report_without_run_over_one_with_it_is_said_and_never_refused() {
    let folder = scratch("losing");
    let write = |report: serde_json::Value| {
        std::fs::write(folder.join("report.json"), report.to_string()).unwrap()
    };
    // No report there yet: nothing to lose.
    assert_eq!(losing_the_run(&folder, false), None);

    write(serde_json::json!({"run_record": {"options": ["--run", "--no-tools"]}}));
    let said = losing_the_run(&folder, false).expect("the running-app checks would be lost");
    assert!(said.contains("Add --run"), "{said}");
    assert!(said.contains("sv review"), "{said}");
    assert_eq!(
        losing_the_run(&folder, true),
        None,
        "with --run nothing is lost"
    );

    write(serde_json::json!({"run_record": {"options": []}}));
    assert_eq!(losing_the_run(&folder, false), None);

    // A report from before the options were kept: its run status says whether the app was asked to start.
    write(serde_json::json!({"run_status": {"state": "started"}}));
    assert!(losing_the_run(&folder, false).is_some());
    write(serde_json::json!({"run_status": {"state": "could-not-start", "why": "x"}}));
    assert!(losing_the_run(&folder, false).is_some());
    write(serde_json::json!({"run_status": {"state": "not-asked", "why": "x"}}));
    assert_eq!(losing_the_run(&folder, false), None);
    std::fs::remove_dir_all(&folder).ok();
}
