//! Backlog 0641, ADR-133: AC.4.1, a person reviewing the code an AI coding tool wrote, counts as checked by hand only
//! when the reviewer was someone other than the person who asked for the code. The owner's own look is kept and said
//! as a gap; the requirement stays not verified.

use std::path::PathBuf;
use std::process::Command;
use sv_check::advisories::Day;
use sv_check::test_support::status_of;

/// compliance.md and report.json of a report on a small app whose owner recorded AC.4.1 with `reviewer`.
fn report_with(tag: &str, reviewer: Option<&str>) -> (String, serde_json::Value) {
    let today = Day::today().expect("a clock after 1970").show();
    let dir: PathBuf =
        std::env::temp_dir().join(format!("sv-separate-review-{tag}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("app.py"), "def home():\n    return 'hi'\n").unwrap();
    let example =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/tested-notes/stackvet.toml");
    let mut manifest = std::fs::read_to_string(example).unwrap();
    let config = dir.join("config");
    let (key, _) =
        sv_check::seal::Key::load_or_make_in(&config.join(sv_frameworks::names::CONFIG_DIR))
            .unwrap();
    let key = key.for_app(&sv_check::seal::App::of(&dir).unwrap());
    let how = "Read every file the AI tool changed.";
    let hand = sv_manifest::HandCheck {
        reviewer: reviewer.map(str::to_owned),
        result: "done".into(),
        on: Some(today.clone()),
        by: Some("owner".into()),
        how: Some(how.into()),
        ..Default::default()
    };
    let seal = key.seal(&sv_check::seal::as_strs(
        &sv_check::seal::hand_check_fields("AC.4.1", &hand),
    ));
    let reviewer_line = reviewer
        .map(|r| format!(", reviewer = \"{r}\""))
        .unwrap_or_default();
    assert!(
        !manifest.contains("[checked-by-hand]"),
        "the example grew checks by hand of its own"
    );
    manifest.push_str(&format!(
        "\n[checked-by-hand]\n\"AC.4.1\" = {{ result = \"done\", on = \"{today}\", by = \"owner\", how = \"{how}\"{reviewer_line}, seal = \"{seal}\" }}\n"
    ));
    std::fs::write(dir.join("stackvet.toml"), &manifest).unwrap();
    let out_dir = dir.join("report");
    let out = Command::new(env!("CARGO_BIN_EXE_sv"))
        .env("XDG_CONFIG_HOME", &config)
        .args(["report", "--no-tools"])
        .arg(&dir)
        .arg("--out")
        .arg(&out_dir)
        .output()
        .expect("sv runs");
    let compliance = std::fs::read_to_string(out_dir.join("compliance.md")).unwrap_or_default();
    let json = std::fs::read_to_string(out_dir.join("report.json")).unwrap_or_default();
    std::fs::remove_dir_all(&dir).ok();
    assert!(
        out.status.success() || out.status.code() == Some(2),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    (
        compliance,
        serde_json::from_str(&json).expect("report.json"),
    )
}

fn gap_names_ac41(json: &serde_json::Value) -> bool {
    json["gaps"].as_array().into_iter().flatten().any(|g| {
        g["requirements"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|r| r == "AC.4.1")
    })
}

#[test]
fn a_review_by_someone_else_counts_as_checked_by_hand() {
    let (compliance, json) = report_with("else", Some("someone-else"));
    let status = status_of(&compliance, "AC.4.1");
    assert!(
        status.starts_with("checked by hand by the owner"),
        "AC.4.1: {status}"
    );
    assert!(!gap_names_ac41(&json), "no gap for a review that counts");
}

#[test]
fn the_requester_s_own_review_is_kept_as_a_gap_and_credits_nothing() {
    for (tag, reviewer) in [("self", Some("the-requester")), ("unasked", None)] {
        let (compliance, json) = report_with(tag, reviewer);
        let status = status_of(&compliance, "AC.4.1");
        assert!(
            !status.starts_with("checked by hand"),
            "{tag}: AC.4.1 credited: {status}"
        );
        assert!(gap_names_ac41(&json), "{tag}: no gap says why");
        let said = json["gaps"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|g| {
                g["requirements"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .any(|r| r == "AC.4.1")
            })
            .map(|g| g["why"].as_str().unwrap_or_default().to_owned())
            .unwrap_or_default();
        assert!(
            said.contains("asks that the reviewer is not the person who asked for the code"),
            "{tag}: {said}"
        );
    }
}
