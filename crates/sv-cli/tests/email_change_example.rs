//! The email-change probe against a real app (backlog 0244, from 0029 part 14): `examples/notes-with-users`
//! changes an account's address only with its password, so V7.5.1 is credited in part; a copy with that password
//! check taken out is found. Real containers and the real binary. With no container backend, the report must not
//! claim the probe ran, and that is what is checked instead (a failure under `SV_REQUIRE_BACKEND=1`).

#![cfg(unix)]

use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;

const PROBE: &str = "probe.email-change-without-password";
const ROLE_PROBE: &str = "probe.email-change-role-trusted";

/// The password check of the email change, exactly as the example has it; the broken copy loses it.
const PASSWORD_CHECK: &str = "                return self.send(403, page(\"No\", \"That is not your password.\"))\n            new_email = form.get(\"email\", \"\")";

/// The example, copied into Cargo's scratch folder beside the build, which every Mac backend shares (see
/// `killed_run.rs`), and changed by `change` first.
fn app(tag: &str, change: impl Fn(String) -> String) -> PathBuf {
    let example = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/notes-with-users");
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("sv-email-change-{tag}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    for file in ["app.py", "seed.py", "common-passwords.txt", "stackvet.toml"] {
        std::fs::copy(example.join(file), dir.join(file)).unwrap();
    }
    let code = std::fs::read_to_string(dir.join("app.py")).unwrap();
    std::fs::write(dir.join("app.py"), change(code)).unwrap();
    dir
}

/// `sv report --run` on the app, and its report file; `None` when there is no container backend.
fn reported(app: &Path) -> Option<Value> {
    let out = Command::new(env!("CARGO_BIN_EXE_sv"))
        .arg("report")
        .arg(app)
        .arg("--run")
        .output()
        .expect("sv runs");
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let text = std::fs::read_to_string(
        app.join(sv_scan::ecosystems::DEFAULT_REPORT_DIR)
            .join("report.json"),
    )
    .unwrap_or_else(|e| panic!("no report ({e}): {stderr}"));
    let json: Value = serde_json::from_str(&text).unwrap();
    if sv_run::detect().is_err() {
        assert_ne!(
            std::env::var("SV_REQUIRE_BACKEND").as_deref(),
            Ok("1"),
            "SV_REQUIRE_BACKEND=1 and there is no container backend here"
        );
        println!("no container backend here; checking the honest-absence path instead");
        assert!(
            !credited(&json).contains(&PROBE.to_owned()),
            "credited with no app run: {text}"
        );
        assert!(found(&json).is_empty(), "a finding with no app run: {text}");
        return None;
    }
    println!("container backend present; running the app for real");
    Some(json)
}

fn credited(json: &Value) -> Vec<String> {
    json["requirements"]
        .as_array()
        .into_iter()
        .flatten()
        .flat_map(|r| r["checked_by"].as_array().into_iter().flatten())
        .filter_map(|c| c["check_id"].as_str())
        .map(str::to_owned)
        .collect()
}

/// The email change's own findings in the report.
fn found(json: &Value) -> Vec<String> {
    json["findings"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|f| f["rule_id"].as_str())
        .filter(|id| *id == PROBE || *id == ROLE_PROBE)
        .map(str::to_owned)
        .collect()
}

#[test]
fn the_example_asks_for_the_password_and_is_credited() {
    let app = app("kept", |code| code);
    let Some(json) = reported(&app) else { return };
    assert!(
        credited(&json).contains(&PROBE.to_owned()),
        "the email change was not credited; the probe may not have reached it: {json:#}"
    );
    assert!(
        found(&json).is_empty(),
        "the example's email change was found at fault: {:?}",
        found(&json)
    );
    std::fs::remove_dir_all(&app).ok();
}

#[test]
fn a_copy_without_the_password_check_is_found() {
    let app = app("broken", |code| {
        // The setup has to have worked: the check is taken out exactly where it is.
        assert_eq!(
            code.matches(PASSWORD_CHECK).count(),
            1,
            "the example no longer has the check as written here"
        );
        code.replace(
            PASSWORD_CHECK,
            "                pass\n            new_email = form.get(\"email\", \"\")",
        )
    });
    let Some(json) = reported(&app) else { return };
    assert_eq!(found(&json), [PROBE], "{json:#}");
    assert!(
        !credited(&json).contains(&PROBE.to_owned()),
        "found and credited at once: {json:#}"
    );
    std::fs::remove_dir_all(&app).ok();
}
