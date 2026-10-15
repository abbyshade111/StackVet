//! A report keeps the options it was written with, for `sv review` to write it again the same way, and `sv report`
//! says so before it writes a report without `--run` over one that had it (backlog 0578: after `sv review`, the report
//! written again with plain `sv report` lost the running-app checks the person had run).

use serde_json::Value;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn a_report_keeps_its_options_and_says_when_it_drops_the_running_app_checks() {
    let sv = env!("CARGO_BIN_EXE_sv");
    let app = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/flask-booking");
    let out = std::env::temp_dir().join(format!("sv-options-kept-{}", std::process::id()));
    std::fs::remove_dir_all(&out).ok();
    let report = || {
        Command::new(sv)
            .args([
                "report",
                app.to_str().unwrap(),
                "--no-tools",
                "--out",
                out.to_str().unwrap(),
            ])
            .output()
            .expect("sv runs")
    };
    let warning = "had the running-app checks";

    let first = report();
    let stderr = String::from_utf8_lossy(&first.stderr).into_owned();
    assert!(first.status.code().is_some_and(|c| c <= 2), "{stderr}");
    assert!(
        !stderr.contains(warning),
        "nothing was there to lose: {stderr}"
    );
    let path = out.join("report.json");
    let mut json: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(
        json["run_record"]["options"],
        serde_json::json!(["--no-tools"]),
        "{}",
        json["run_record"]
    );

    // The last report had --run: this one is still written, and says what it drops first.
    json["run_record"]["options"] = serde_json::json!(["--run", "--no-tools"]);
    std::fs::write(&path, json.to_string()).unwrap();
    let second = report();
    let stderr = String::from_utf8_lossy(&second.stderr).into_owned();
    std::fs::remove_dir_all(&out).ok();
    assert!(second.status.code().is_some_and(|c| c <= 2), "{stderr}");
    assert!(stderr.contains(warning), "{stderr}");
    assert!(stderr.contains("Add --run"), "{stderr}");
}
