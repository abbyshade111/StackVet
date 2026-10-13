//! `sv report` writes the app's bill of materials beside the report, `sbom.cdx.json` (backlog 0382): the same document
//! `sv sbom` prints, sealed with the other report files. Before, the report built it to check it and wrote it nowhere.

use std::process::Command;

fn sv(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(args)
        .env_remove("SV_CONNECT_IN_CONTAINER")
        .output()
        .expect("sv runs")
}

#[test]
fn the_report_folder_holds_the_bill_of_materials_sv_sbom_prints() {
    let dir = std::env::temp_dir().join(format!("sv-sbom-file-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("stackvet.toml"),
        "manifest-version = 1\n[app]\nname = \"x\"\n[stack.run]\nimage = \"python:3.12-slim\"\n",
    )
    .unwrap();
    std::fs::write(dir.join("app.py"), "import flask\nimport sqlite3\n").unwrap();
    std::fs::write(dir.join("requirements.txt"), "flask==3.0.0\n").unwrap();
    let run = sv(&["report", dir.to_str().unwrap()]);
    assert!(
        matches!(run.status.code(), Some(0 | 2)),
        "sv report failed: {}",
        String::from_utf8_lossy(&run.stderr)
    );
    let written = std::fs::read_to_string(dir.join("stackvet-report/sbom.cdx.json"))
        .expect("the bill beside the report");
    let printed = sv(&["sbom", dir.to_str().unwrap()]);
    assert!(
        printed.status.success(),
        "{}",
        String::from_utf8_lossy(&printed.stderr)
    );
    let written: serde_json::Value = serde_json::from_str(&written).expect("CycloneDX JSON");
    let printed: serde_json::Value =
        serde_json::from_slice(&printed.stdout).expect("CycloneDX JSON");
    // The setup: the bill lists the one package the app declares, so the two agreeing is about something.
    assert_eq!(written["bomFormat"], "CycloneDX");
    assert!(
        written["components"].to_string().contains("flask"),
        "{written}"
    );
    assert_eq!(
        written, printed,
        "the report's bill is the one `sv sbom` prints"
    );
    // And what the app runs on beside its packages (backlog 0402): the app, its image, Python, and sqlite3 under it.
    assert_eq!(written["metadata"]["component"]["name"], "x", "{written}");
    let components = written["components"].as_array().unwrap();
    let python = components
        .iter()
        .find(|c| c["type"] == "platform" && c["name"] == "python")
        .expect("python in the report's bill");
    assert_eq!(python["version"], "3.12", "{python}");
    assert_eq!(python["components"][0]["name"], "sqlite3", "{python}");
    assert!(
        components
            .iter()
            .any(|c| c["type"] == "container" && c["version"] == "3.12-slim")
    );
    // Named near the top of the page, and linked.
    let page = std::fs::read_to_string(dir.join("stackvet-report/report.html")).unwrap();
    assert!(
        page.contains("<a href=\"sbom.cdx.json\">sbom.cdx.json</a>"),
        "the page links the bill"
    );
    std::fs::remove_dir_all(&dir).ok();
}
