//! ADR-118 (backlog 0232, part 3): every private WebSocket path stackvet.toml names is opened, and each credit is whole
//! only when two or more paths were named, each could be judged, and every one held.

use super::super::fake_app::*;
use super::*;

/// The users section with two private WebSocket paths, `/ws` and `/chat`, the stand-in app answering both.
fn two_paths() -> UsersSection {
    let mut u = users();
    u.private_websockets = vec!["/ws".into(), "/chat".into()];
    u
}

fn credit<'o>(o: &'o Outcome, rule: &Rule) -> Option<&'o crate::Verified> {
    o.verified.iter().find(|v| v.check_id == rule.rule_id)
}

/// Both paths were opened signed in, or nothing below means anything.
fn assert_both_opened(o: &Outcome) {
    for path in ["/ws", "/chat"] {
        assert!(
            o.steps.iter().any(|s| s.starts_with(&format!(
                "opened the WebSocket at {path} signed in: accepted"
            ))),
            "{path} was not opened signed in: {:?}",
            o.steps
        );
    }
}

#[test]
fn two_paths_that_both_hold_are_checked_whole() {
    let o = run_against(Flaws::default(), &two_paths());
    assert_both_opened(&o);
    for rule in [&WS_WITHOUT_SESSION, &WS_FOREIGN_ORIGIN] {
        let credit = credit(&o, rule).unwrap_or_else(|| panic!("{} not credited", rule.rule_id));
        assert!(!credit.in_part, "{credit:?}");
        assert!(
            credit.scope.contains("/ws") && credit.scope.contains("/chat"),
            "{}",
            credit.scope
        );
    }
    // Signing out was tried on each path.
    assert!(
        o.steps
            .iter()
            .any(|s| s.contains("opened the WebSocket at /chat with its old cookie: refused")),
        "{:?}",
        o.steps
    );
}

#[test]
fn a_second_path_left_open_is_found_naming_it_and_credits_neither() {
    let o = run_against(
        Flaws {
            chat_ws_open: true,
            ..Default::default()
        },
        &two_paths(),
    );
    assert_both_opened(&o);
    for (rule, ids) in [
        (
            &WS_WITHOUT_SESSION,
            &["websocket-no-session-1", "websocket-invented-session-1"][..],
        ),
        (&WS_FOREIGN_ORIGIN, &["websocket-foreign-origin-1"][..]),
    ] {
        let found = o
            .findings
            .iter()
            .find(|f| f.rule_id == rule.rule_id)
            .unwrap_or_else(|| panic!("no {} finding", rule.rule_id));
        assert_eq!(found.evidence, ids);
        assert!(found.description.contains("/chat"), "{}", found.description);
        assert!(
            credit(&o, rule).is_none(),
            "credited beside its own finding"
        );
    }
}

#[test]
fn one_path_is_one_sample() {
    let mut u = users();
    u.private_websockets = vec!["/chat".into()];
    let o = run_against(Flaws::default(), &u);
    for rule in [&WS_WITHOUT_SESSION, &WS_FOREIGN_ORIGIN] {
        assert!(
            credit(&o, rule).is_some_and(|c| c.in_part),
            "{:?}",
            o.verified
        );
    }
}

#[test]
fn a_path_that_does_not_open_signed_in_keeps_both_credits_in_part_and_is_named() {
    let mut u = two_paths();
    u.private_websockets.push("/nowhere".into());
    let o = run_against(Flaws::default(), &u);
    assert_both_opened(&o);
    for rule in [&WS_WITHOUT_SESSION, &WS_FOREIGN_ORIGIN] {
        let credit = credit(&o, rule).expect("credited for the two that held");
        assert!(credit.in_part, "{credit:?}");
        assert!(
            credit.scope.contains("not judged: /nowhere"),
            "{}",
            credit.scope
        );
    }
}

#[test]
fn both_private_websocket_and_private_websockets_is_a_problem_in_the_settings() {
    let mut u = two_paths();
    u.private_websocket = Some("/ws".into());
    assert!(
        u.problems()
            .iter()
            .any(|p| p.contains("both `private-websocket` and `private-websockets`")),
        "{:?}",
        u.problems()
    );
    let mut u = two_paths();
    u.private_websockets.push("chat".into());
    assert!(
        u.problems()
            .iter()
            .any(|p| p.contains("(chat) is not a path")),
        "{:?}",
        u.problems()
    );
}
