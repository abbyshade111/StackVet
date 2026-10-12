//! `sv report` says each stage on stderr, and `--quiet` leaves the progress and timing lines out while the report and
//! its counts stay (backlog 0247).

use std::path::PathBuf;
use std::process::Command;

fn report(tag: &str, quiet: bool) -> std::process::Output {
    let sv = env!("CARGO_BIN_EXE_sv");
    let app = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/flask-booking");
    let out = std::env::temp_dir().join(format!("sv-quiet-{tag}-{}", std::process::id()));
    std::fs::remove_dir_all(&out).ok();
    let mut args = vec![
        "report".to_owned(),
        app.to_str().unwrap().to_owned(),
        "--out".to_owned(),
        out.to_str().unwrap().to_owned(),
    ];
    if quiet {
        args.push("--quiet".to_owned());
    }
    let run = Command::new(sv).args(&args).output().expect("sv runs");
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    run
}

#[test]
fn quiet_leaves_out_the_progress_and_keeps_the_report() {
    // The control: without --quiet the stages are said, so their absence below is the flag's doing.
    let loud = report("loud", false);
    let said = String::from_utf8_lossy(&loud.stderr).to_string();
    assert!(
        said.contains("sv report: 1 of 10"),
        "the stages are said without --quiet: {said}"
    );
    let quiet = report("quiet", true);
    let said = String::from_utf8_lossy(&quiet.stderr).to_string();
    assert!(
        !said.contains("sv report: 1 of 10"),
        "no stage line with --quiet: {said}"
    );
    assert!(
        !said.contains("This run took"),
        "no timing line with --quiet: {said}"
    );
    let printed = String::from_utf8_lossy(&quiet.stdout).to_string();
    assert!(
        printed.contains("Wrote "),
        "the report is still written and said: {printed}"
    );
}
