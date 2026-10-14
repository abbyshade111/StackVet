//! Backlog 0401, through the server: the record of the build loop keeps where each finding was (its rule, and a hash
//! of its file), and the report looks for each one that went again in today's code.

use super::tests::{call, scratch_app, text};
use super::*;

const MANIFEST: &str =
    "manifest-version = 1\n[app]\nname = \"Looped\"\n[stack]\nlanguages = [\"python\"]\n";
const APP: &str = "from flask import Flask, redirect, request\napp = Flask(__name__)\n\n@app.route(\"/go\")\ndef go():\n    return redirect(request.args.get(\"next\"))\n";

fn report(app: &Path) -> Value {
    serde_json::from_str(
        &std::fs::read_to_string(
            sv_scan::ecosystems::default_report_dir_in(app).join("report.json"),
        )
        .unwrap(),
    )
    .unwrap()
}

#[test]
fn a_finding_still_found_under_a_look_away_comment_did_not_go() {
    let root = scratch_app("build-loop-went-same", "tested-notes");
    let app = root.join("app");
    for entry in std::fs::read_dir(&app).unwrap().flatten() {
        std::fs::remove_file(entry.path()).ok();
    }
    std::fs::write(app.join("stackvet.toml"), MANIFEST).unwrap();
    std::fs::write(app.join("app.py"), APP).unwrap();
    let server = Server::new(&root).unwrap().recording(true);

    let first = call(&server, "stackvet_check", json!({ "path": "app" }));
    assert_eq!(first["isError"], false, "{}", text(&first));
    let record = std::fs::read_to_string(sv_scan::ecosystems::build_loop_record_in(&app)).unwrap();
    // Where each finding was: its rule, and a hash of its file, never the file's name.
    assert!(record.contains("\"placed\""), "{record}");
    assert!(record.contains("ast.open-redirect"), "{record}");
    assert!(
        record.contains(&sv_check::review::file_key("app.py")),
        "{record}"
    );
    assert!(!record.contains("app.py"), "{record}");

    // The AI tool adds a comment telling Bandit to look away. `sv`'s own check still finds the redirect, under a new
    // fingerprint since its line now reads differently.
    std::fs::write(
        app.join("app.py"),
        APP.replace("get(\"next\"))\n", "get(\"next\"))  # nosec\n"),
    )
    .unwrap();
    let checked = call(&server, "stackvet_check", json!({ "path": "app" }));
    assert_eq!(checked["isError"], false, "{}", text(&checked));
    let last = call(&server, "stackvet_write_report", json!({ "path": "app" }));
    assert_eq!(last["isError"], false, "{}", text(&last));
    let build_loop = &report(&app)["build_loop"];
    // The setup: the redirect went by its old fingerprint, and came back under a new one.
    assert_eq!(
        build_loop["findings_moved"]["no_longer_found"], 1,
        "{build_loop:#}"
    );
    // Found again on its line, it is neither named as gone nor counted as looked for.
    assert_eq!(build_loop["went_same"], Value::Null, "{build_loop:#}");
    assert_eq!(build_loop["looked_again"], Value::Null, "{build_loop:#}");
    let page = std::fs::read_to_string(
        sv_scan::ecosystems::default_report_dir_in(&app).join("report.html"),
    )
    .unwrap();
    assert!(!page.contains("looked for again"), "{page}");
    std::fs::remove_dir_all(&root).ok();
}
