//! Backlog 0500, V9.2.4: the app's own token's audience, as a signed-in run saw it, beside the audience question in the
//! questions given to the AI tool and `sv questions`, the name the app wrote inside the fence. (V9.2.4 is a question
//! for the AI tool, not one of report.html's "only you can check".)

use crate::Report;
use crate::next_step_tests::empty_report;
use sv_check::human::{Item, Route};
use sv_check::personal_fields::TokenFields;

fn asked(audience: &[&str]) -> Report {
    let mut report = empty_report(Vec::new());
    report.questions_for_you.push(Item {
        id: "V9.2.4".into(),
        title: "Each token names the service it is for, when one key signs for several".into(),
        how: "Find out whether the key signs for anything else.".into(),
        plain: None,
        where_to_look: None,
        route: Route::GoAndLook,
        where_means: None,
        rests_on: None,
        in_the_packages: None,
        why_not_checked: None,
        routes: None,
        pathways: None,
        personal_fields: None,
        crypto: None,
        token_audience: Some(TokenFields {
            carried: "the `Authorization` header".into(),
            fields: vec!["sub".into(), "aud".into()],
            cookie: false,
            audience: audience.iter().map(|a| (*a).to_owned()).collect(),
        }),
        outbound: None,
        measured_by: Vec::new(),
    });
    report
}

#[test]
fn the_audience_is_shown_to_the_ai_tool_inside_the_fence() {
    let report = asked(&["<b>notes-api</b>"]);
    let fence = crate::fence::Fence::for_draft(&crate::interview::text(&report));
    let text = crate::interview::text_with(&report, &fence);
    let tag = fence.tag().expect("a fence for the app's text");
    assert!(
        text.contains(&format!("it is for <{tag}><b>notes-api</b></{tag}>.")),
        "{text}"
    );
}

#[test]
fn a_token_with_no_aud_says_so() {
    let report = asked(&[]);
    assert!(crate::interview::text(&report).contains("had no `aud` field"));
}
