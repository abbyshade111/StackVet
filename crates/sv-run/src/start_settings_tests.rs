//! The settings a start command gives, named in the report (backlog 0386), held to the second loop trial's own: what
//! twelve of its 90 builds set in their start commands (`docs/prompts/loop-2/scores.json`, `start_sets`).

use super::*;

#[test]
fn every_setting_the_trial_s_builds_gave_is_named() {
    for (start, names) in [
        (
            "CLUB_LOGIN_ATTEMPTS=100 CLUB_SECURE_COOKIES=0 python app.py",
            &["CLUB_LOGIN_ATTEMPTS", "CLUB_SECURE_COOKIES"][..],
        ),
        (
            "env AUTH_IP_LIMIT=1000 FORGOT_IP_LIMIT=1000 SIGNUP_IP_LIMIT=1000 node server.js",
            &["AUTH_IP_LIMIT", "FORGOT_IP_LIMIT", "SIGNUP_IP_LIMIT"][..],
        ),
        (
            "export COOKIE_INSECURE=1 && npm start",
            &["COOKIE_INSECURE"][..],
        ),
        (
            "sh -c 'GARDEN_SIGNIN_LIMIT=\"60\" exec gunicorn app:app'",
            &["GARDEN_SIGNIN_LIMIT"][..],
        ),
    ] {
        assert_eq!(named_in(start), names, "{start}");
    }
}

#[test]
fn a_flag_or_a_command_with_no_setting_names_nothing() {
    for start in [
        "python app.py --port=8000",
        "npm start",
        "gunicorn -b 0.0.0.0:8000 app:app",
        "node server.js 2>&1",
        "",
    ] {
        assert!(named_in(start).is_empty(), "{start}: {:?}", named_in(start));
        assert_eq!(note(start), None, "{start}");
    }
}

#[test]
fn the_note_names_the_settings_and_never_a_value() {
    // Built from pieces, so no key is written here; the value must not reach the note.
    let value = format!("{}{}", "sk_live_", "0123456789abcdefghijklmn");
    let start = format!("STRIPE_SECRET_KEY={value} CLUB_LOGIN_LIMIT=100 python app.py");
    let said = note(&start).expect("two settings");
    assert!(
        said.contains("`STRIPE_SECRET_KEY` and `CLUB_LOGIN_LIMIT`"),
        "{said}"
    );
    assert!(
        !said.contains(&value[..10]) && !said.contains("100"),
        "a value is shown"
    );
    assert!(said.contains("may not have"), "{said}");
}

#[test]
fn a_sign_in_limit_is_told_from_the_trial_s_other_settings() {
    // Every sign-in limit the trial's builds set, and every other setting they set.
    let limits = [
        "CLUB_LOGIN_ATTEMPTS",
        "CLUB_LOGIN_MAX_FAILURES",
        "CLUB_LOGIN_LIMIT",
        "GARDEN_LOGIN_LIMIT",
        "GARDEN_SIGNIN_LIMIT",
        "GARDEN_SIGNIN_FAILURES",
        "SIGNIN_FAILURE_LIMIT",
        "AUTH_IP_LIMIT",
        "CLUB_LOGIN_FAILURE_LIMIT",
        "SIGN_IN_MAX_TRIES",
    ];
    let others = [
        "CLUB_SECURE_COOKIES",
        "CLUB_INSECURE_COOKIES",
        "COOKIE_INSECURE",
        "GARDEN_COOKIE_SECURE",
        "FORGOT_IP_LIMIT",
        "SIGNUP_IP_LIMIT",
        "PT_TOOLS",
        "PORT",
    ];
    for name in limits {
        assert_eq!(
            sign_in_limits_in(&format!("{name}=100 python app.py")),
            [name],
            "{name} is a sign-in limit"
        );
    }
    for name in others {
        assert!(
            sign_in_limits_in(&format!("{name}=1 python app.py")).is_empty(),
            "{name} is not a sign-in limit"
        );
    }
}

#[test]
fn the_finding_s_note_names_only_the_sign_in_limit() {
    let said = sign_in_limit_note("CLUB_LOGIN_ATTEMPTS=100 CLUB_SECURE_COOKIES=0 python app.py")
        .expect("one sign-in limit");
    assert!(said.contains("`CLUB_LOGIN_ATTEMPTS`"), "{said}");
    assert!(!said.contains("COOKIES"), "{said}");
    assert_eq!(
        sign_in_limit_note("CLUB_SECURE_COOKIES=0 python app.py"),
        None
    );
}
