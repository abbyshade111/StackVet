//! The comments that tell a scanner to look away, end to end (backlog 0400): `sv report` on an app with `# nosec` on
//! a line lists the comment in a section of its own, in compliance.md, report.html, and report.json, and the
//! requirements and findings are the same as the same app's without it. The same app without one has no such
//! section.

use std::path::PathBuf;
use std::process::Command;

/// `sv report --no-tools` on a small app whose `app.py` is `code`: compliance.md, report.html, and report.json.
fn report(name: &str, code: &str) -> (String, String, serde_json::Value) {
    let app = std::env::temp_dir().join(format!("sv-look-away-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&app).ok();
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(app.join("app.py"), code).unwrap();
    std::fs::write(
        app.join("stackvet.toml"),
        "manifest-version = 1\n[app]\nname = \"Looks\"\naudience = \"customers\"\n\
         deployment = \"local-only\"\n[stack]\nlanguages = [\"python\"]\n",
    )
    .unwrap();
    let out: PathBuf = app.join("report");
    let run = Command::new(env!("CARGO_BIN_EXE_sv"))
        .arg("report")
        .arg(&app)
        .arg("--no-tools")
        .arg("--out")
        .arg(&out)
        .output()
        .expect("sv runs");
    let markdown = std::fs::read_to_string(out.join("compliance.md")).unwrap_or_default();
    let html = std::fs::read_to_string(out.join("report.html")).unwrap_or_default();
    let json = std::fs::read_to_string(out.join("report.json")).unwrap_or_default();
    std::fs::remove_dir_all(&app).ok();
    assert!(
        run.status.code().is_some_and(|c| c == 0 || c == 2),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(
        markdown.contains("## Requirements that apply"),
        "the report was written: {markdown}"
    );
    (
        markdown,
        html,
        serde_json::from_str(&json).expect("report.json"),
    )
}

const PLAIN: &str = "def serve(app):\n    app.run(host='0.0.0.0')\n";
const MARKED: &str = "def serve(app):\n    app.run(host='0.0.0.0')  # nosec B104 <ok>\n";

#[test]
fn a_comment_that_tells_a_scanner_to_look_away_is_listed_and_changes_nothing_else() {
    let (markdown, html, json) = report("marked", MARKED);
    assert!(
        markdown.contains("## Comments that tell a scanner to look away"),
        "{markdown}"
    );
    assert!(
        markdown.contains("| `app.py` | 2 | Bandit | `nosec B104 <ok>` |"),
        "{markdown}"
    );
    assert!(
        html.contains("<h2>Comments that tell a scanner to look away</h2>"),
        "{html}"
    );
    // What the comment says is the app's text, escaped in the page.
    assert!(html.contains("nosec B104 &lt;ok&gt;"), "{html}");
    assert_eq!(
        json["look_away"]["comments"],
        serde_json::json!([{ "file": "app.py", "line": 2, "scanner": "Bandit", "says": "nosec B104 <ok>" }])
    );
    // It credits nothing and counts against nothing: the requirements and the findings are those of the same app
    // without the comment.
    let (_, _, plain) = report("plain", PLAIN);
    assert_eq!(json["requirements"], plain["requirements"]);
    let rules = |j: &serde_json::Value| -> Vec<String> {
        j["findings"]
            .as_array()
            .expect("findings")
            .iter()
            .map(|f| format!("{} {}", f["rule_id"], f["location"]["line"]))
            .collect()
    };
    assert_eq!(rules(&json), rules(&plain));
}

#[test]
fn without_such_a_comment_there_is_no_such_section() {
    let (markdown, html, json) = report("without", PLAIN);
    assert!(
        !markdown.contains("tell a scanner to look away"),
        "{markdown}"
    );
    assert!(!html.contains("tell a scanner to look away"), "{html}");
    assert_eq!(json["look_away"]["comments"], serde_json::json!([]));
}
