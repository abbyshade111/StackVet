//! Backlog 0605: the environment files a start command itself reads are named, and each is read for its format before
//! anything in it is judged.

use super::*;

#[test]
fn the_files_a_start_command_reads_are_named() {
    assert_eq!(
        env_files_named_in("node --env-file=.env server.js"),
        [".env"]
    );
    assert_eq!(
        env_files_named_in("uvicorn app:app --env-file config/run.env"),
        ["config/run.env"]
    );
    assert_eq!(
        env_files_named_in("set -a; . ./.env.run; set +a; python app.py"),
        ["./.env.run"]
    );
    assert_eq!(
        env_files_named_in("source .env && source .env && gunicorn app:app"),
        [".env"]
    );
}

#[test]
fn a_start_command_that_reads_no_file_names_none() {
    assert!(env_files_named_in("python app.py --port 8000").is_empty());
    // `.` here is a folder, not the shell reading a file.
    assert!(env_files_named_in("cd . && python app.py").is_empty());
    // A flag after `--env-file` is not a file.
    assert!(env_files_named_in("node --env-file --watch app.js").is_empty());
}

#[test]
fn an_env_file_gives_its_settings_and_hides_long_values() {
    let text = "# for the run\n\nexport SESSION_COOKIE_SECURE=false\nCSRF_ENABLED=\"0\"\nGREETING=hello there\n";
    let settings = env_file_settings(text).expect("a plain env file is read");
    assert_eq!(
        settings,
        "SESSION_COOKIE_SECURE=false\nCSRF_ENABLED=0\nGREETING"
    );
    // Judged as the start command's settings are.
    let loose: Vec<String> = loosened(&settings).into_iter().map(|(s, _)| s).collect();
    assert_eq!(loose, ["SESSION_COOKIE_SECURE=false", "CSRF_ENABLED=0"]);
}

#[test]
fn a_file_in_another_form_is_not_judged_and_says_which_line() {
    let why = env_file_settings("[app]\nCOOKIE_SECURE=false\n").unwrap_err();
    assert!(why.contains("line 1"), "{why}");
    let why = env_file_settings("A=1\nmy-setting=off\n").unwrap_err();
    assert!(why.contains("line 2"), "{why}");
}
