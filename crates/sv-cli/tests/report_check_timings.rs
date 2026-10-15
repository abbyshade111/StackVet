//! Backlog 0526: `sv report --run` times each check inside the signed-in suite in `report.json`'s `timings`, named as
//! inside the suites. Real containers and the real binary. With no container backend, the app is not run and no
//! check is timed; where CI says one must be here (`SV_REQUIRE_BACKEND=1`), its absence is a failure.

use serde_json::Value;
use std::path::Path;
use std::process::Command;

#[test]
fn each_signed_in_check_is_timed_in_the_report() {
    let app = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/notes-with-users");
    let out = sv_check::test_support::scratch("report");
    let ran = Command::new(env!("CARGO_BIN_EXE_sv"))
        .args([
            "report",
            app.to_str().unwrap(),
            "--run",
            "--no-tools",
            "--out",
            out.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    let json: Value = serde_json::from_str(
        &std::fs::read_to_string(out.join("report.json")).expect("report.json is written"),
    )
    .unwrap();
    std::fs::remove_dir_all(&out).ok();
    let checks: Vec<&str> = json["timings"]
        .as_array()
        .expect("timings")
        .iter()
        .filter_map(|t| t["what"].as_str())
        .filter_map(|what| what.strip_prefix(sv_report::CHECK_TIMING))
        .collect();
    if sv_run::detect().is_err() {
        assert_ne!(
            std::env::var("SV_REQUIRE_BACKEND").as_deref(),
            Ok("1"),
            "SV_REQUIRE_BACKEND=1 and there is no container backend here"
        );
        println!("no container backend here; checking that no check is said to be timed");
        assert!(checks.is_empty(), "{checks:?}");
        return;
    }
    // The setup: the app was started, so the times stand for a real run.
    assert_eq!(
        json["run_status"]["state"],
        "started",
        "the app was not started: {}\n{}",
        json["run_status"],
        String::from_utf8_lossy(&ran.stderr)
    );
    assert!(
        checks.contains(&"the check `owned_checks`"),
        "the signed-in suite's checks are timed: {}",
        json["timings"]
    );
}
