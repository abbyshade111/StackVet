//! Backlog 0500, V9.2.4: the app's own token, as a signed-in run saw it, is given to the audience question and to no
//! other, and only when the run read a JWT.

use super::*;
use sv_check::personal_fields::TokenFields;

fn report() -> sv_report::Report {
    let dir = std::env::temp_dir().join(format!("sv-audience-question-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("app.py"), "def home():\n    return 'hi'\n").unwrap();
    std::fs::write(
        dir.join("stackvet.toml"),
        "manifest-version = 1\n\n[app]\nname = \"Tokens\"\naudience = \"customers\"\n\
         deployment = \"local-only\"\n\n[stack]\nlanguages = [\"python\"]\n\n[capabilities]\nauth = true\njwt = true\n",
    )
    .unwrap();
    let report = crate::assemble_report(
        &dir,
        &crate::ReportOptions::reading_only("a test"),
        &crate::Loaded::load().unwrap(),
    )
    .unwrap();
    std::fs::remove_dir_all(&dir).ok();
    report
}

fn token(cookie: bool) -> TokenFields {
    TokenFields {
        carried: "the `Authorization` header".into(),
        fields: vec!["sub".into(), "aud".into()],
        cookie,
        audience: vec!["notes-api".into()],
    }
}

/// The questions, on both lists, that carry the token.
fn given(report: &sv_report::Report) -> Vec<String> {
    report
        .only_you_can_check
        .iter()
        .chain(&report.questions_for_you)
        .filter(|i| i.token_audience.is_some())
        .map(|i| i.id.clone())
        .collect()
}

#[test]
fn the_token_goes_to_the_audience_question_alone_and_only_when_it_is_a_jwt() {
    let mut asked = report();
    assert!(
        asked.questions_for_you.iter().any(|i| i.id == "V9.2.4"),
        "the setup: V9.2.4 is asked"
    );
    audience_questions(Some(&token(false)), &mut asked);
    assert!(!given(&asked).is_empty());
    assert!(
        given(&asked).iter().all(|id| id == "V9.2.4"),
        "{:?}",
        given(&asked)
    );

    let mut cookie = report();
    audience_questions(Some(&token(true)), &mut cookie);
    assert!(
        given(&cookie).is_empty(),
        "a cookie has no `aud` field to show"
    );

    let mut none = report();
    audience_questions(None, &mut none);
    assert!(given(&none).is_empty());
}
