//! Backlog 0605 (ADR-114, Later; the owner's choice (b)): the environment file the start command itself reads is read,
//! for its format first, and a setting there that loosens the copy takes out the credits it bears on, as one in the
//! start command does. A file outside the app's folder, missing, or in another form is not judged, and the report says
//! that no other settings file is read.

use super::*;

fn app(name: &str, files: &[(&str, &str)]) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-env-file-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    for (file, text) in files {
        let path = dir.join(file);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }
    dir
}

fn credit(rule: &str, ids: &[&str]) -> sv_check::Verified {
    sv_check::Verified::new(rule, ids, "what the run saw".into()).in_part()
}

#[test]
fn a_setting_loosened_in_the_sourced_env_file_withholds_its_credit_and_names_the_file() {
    let dir = app(
        "sourced",
        &[(
            ".env",
            "# the run\nexport SESSION_COOKIE_SECURE=false\nPORT=8000\n",
        )],
    );
    let (read, not_read) = env_files_for_the_run(&dir, "set -a; . ./.env; set +a; python app.py");
    assert!(not_read.is_empty(), "{not_read:?}");
    assert_eq!(read.len(), 1, "the setup: the file was read");
    assert_eq!(read[0].0, "./.env");

    let mut verified = vec![
        credit("probe.session-cookie-attributes", &["V3.3.2", "V3.3.4"]),
        credit("probe.cross-site-request-accepted", &["V3.5.1"]),
    ];
    let gaps = loosened_in_env_file(&mut [], &mut verified, &read[0].1, &read[0].0);
    let left: Vec<&str> = verified.iter().map(|v| v.check_id.as_str()).collect();
    assert_eq!(left, ["probe.cross-site-request-accepted"]);
    assert_eq!(gaps.len(), 1, "{gaps:?}");
    let why = &gaps[0].why;
    assert!(why.contains("`SESSION_COOKIE_SECURE=false`"), "{why}");
    assert!(why.contains("`./.env`, the environment file"), "{why}");
    assert!(!why.contains("start command in stackvet.toml"), "{why}");
}

#[test]
fn the_env_file_flag_is_read_and_a_path_under_app_is_the_app_folder() {
    let dir = app("flag", &[("config/run.env", "CSRF_ENABLED=0\n")]);
    let (read, not_read) =
        env_files_for_the_run(&dir, "uvicorn main:app --env-file /app/config/run.env");
    assert!(not_read.is_empty(), "{not_read:?}");
    assert_eq!(
        read,
        [(
            "/app/config/run.env".to_owned(),
            "CSRF_ENABLED=0".to_owned()
        )]
    );
}

#[test]
fn a_file_outside_the_folder_missing_or_in_another_form_is_not_judged_and_said() {
    let dir = app(
        "refused",
        &[("settings.toml", "[app]\nCOOKIE_SECURE = false\n")],
    );
    let (read, not_read) = env_files_for_the_run(
        &dir,
        "source ../outside.env; source /etc/run.env; source missing.env; source settings.toml; python app.py",
    );
    assert!(read.is_empty(), "{read:?}");
    assert_eq!(not_read.len(), 4, "{not_read:?}");
    assert!(
        not_read[0].contains("`../outside.env`")
            && not_read[0].contains("outside the app's folder")
    );
    assert!(
        not_read[1].contains("`/etc/run.env`") && not_read[1].contains("outside the app's folder")
    );
    assert!(
        not_read[2].contains("`missing.env`") && not_read[2].contains("not in the app's folder")
    );
    assert!(
        not_read[3].contains("line 1") && not_read[3].contains("form"),
        "{}",
        not_read[3]
    );
}

#[cfg(unix)]
#[test]
fn a_link_out_of_the_folder_is_not_followed() {
    let outside = app("link-target", &[("secret.env", "INSECURE_COOKIES=1\n")]);
    let dir = app("link", &[]);
    std::os::unix::fs::symlink(outside.join("secret.env"), dir.join(".env")).unwrap();
    let (read, not_read) = env_files_for_the_run(&dir, "source .env && python app.py");
    assert!(read.is_empty(), "{read:?}");
    assert!(
        not_read[0].contains("outside the app's folder"),
        "{not_read:?}"
    );
}

#[test]
fn the_report_says_no_other_settings_file_is_read() {
    let none = settings_files_sentence(&[], &[]);
    assert!(
        none.contains("a settings file the app reads on its own is not read"),
        "{none}"
    );
    let one = settings_files_sentence(&[(".env".to_owned(), String::new())], &["The start command reads `x`, which was not read for settings that loosen the copy: it is outside the app's folder.".to_owned()]);
    assert!(
        one.contains("the environment file it reads (`.env`)"),
        "{one}"
    );
    assert!(
        one.contains("any other settings file the app reads is not read"),
        "{one}"
    );
    assert!(one.ends_with("outside the app's folder."), "{one}");
}

#[test]
fn the_copy_is_judged_by_its_start_command_and_the_file_it_reads_together() {
    let dir = app("together", &[(".env", "CSRF_ENABLED=0\n")]);
    let mut verified = vec![
        credit("probe.session-cookie-attributes", &["V3.3.2", "V3.3.4"]),
        credit("probe.cross-site-request-accepted", &["V3.5.1"]),
        credit("probe.no-hsts", &["V3.4.1"]),
    ];
    let (gaps, sentence) = loosened_for_the_copy(
        &mut [],
        &mut verified,
        &dir,
        "SESSION_COOKIE_SECURE=false node --env-file=.env server.js",
    );
    let left: Vec<&str> = verified.iter().map(|v| v.check_id.as_str()).collect();
    assert_eq!(
        left,
        ["probe.no-hsts"],
        "both the command's and the file's setting withhold"
    );
    assert_eq!(gaps.len(), 2, "{gaps:?}");
    assert!(
        gaps[0].why.contains("start command in stackvet.toml"),
        "{}",
        gaps[0].why
    );
    assert!(
        gaps[1].why.contains("`CSRF_ENABLED=0` set in `.env`"),
        "{}",
        gaps[1].why
    );
    assert!(sentence.contains("(`.env`)"), "{sentence}");
}
