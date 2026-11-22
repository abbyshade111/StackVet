//! The pass-through's record under the six groups (backlog 0716, part 2): a call turned down and made again, a tool
//! called that the task file does not name, a record that was changed, and what is said when the record was not asked
//! for or could not be read.

use super::*;
use crate::proxy_seen::{Again, Seen, Tool};

fn gathered(proxy: Option<&Seen>, held: Option<&Held>) -> Vec<Gathered> {
    gather(&Read {
        build_loop: &BuildLoop::default(),
        hook: None,
        behavior: &ToolBehavior::default(),
        held,
        commits: None,
        checkpoints: &[],
        noted: &crate::outside_note::Standing::None,
        proxy,
    })
}

fn group(all: &[Gathered], group: Group) -> &Gathered {
    all.iter()
        .find(|g| g.group == group)
        .expect("every group is there")
}

fn again() -> Again {
    Again {
        tool: "delete".to_owned(),
        summary: r#"{"path":"data"}"#.to_owned(),
        at: "2026-11-22T00:00:00Z".to_owned(),
        answer: "refused".to_owned(),
        run: 1,
        times: 2,
        later: 0,
        last_answer: "ok".to_owned(),
    }
}

#[test]
fn a_call_turned_down_and_made_again_is_a_line_and_the_owner_s_own_no_is_still_said_to_be_unseen() {
    let seen = Seen {
        calls: 3,
        again: vec![again()],
        again_more: 4,
        ..Default::default()
    };
    let all = gathered(Some(&seen), None);
    let g = group(&all, Group::AgainAfterNo);
    assert_eq!(g.lines.len(), 1, "{g:?}");
    let line = &g.lines[0];
    assert!(
        line.what
            .contains("delete was turned down by the server (refused)")
            && line
                .what
                .contains("called again with the same arguments 2 times in the same session")
            && line.what.contains("the last was answered ok")
            && !line.what.contains("later session")
            && line.what.contains(r#"{"path":"data"}"#),
        "{}",
        line.what
    );
    assert_eq!(line.from, "the pass-through's record");
    // What the pass-through can never see is said first, whatever it did see.
    assert_eq!(g.not_shown[0], NO_AT_THE_PROMPT);
    assert!(
        g.not_shown
            .iter()
            .any(|n| n == "4 more such calls, not listed.")
    );
    assert!(
        g.not_shown
            .iter()
            .any(|n| n.contains("a tool that ran and failed") && n.contains("is often right")),
        "{:?}",
        g.not_shown
    );
}

#[test]
fn a_record_not_asked_for_or_not_readable_puts_no_line_and_says_which() {
    let all = gathered(None, None);
    let g = group(&all, Group::AgainAfterNo);
    assert_eq!(g.lines, Vec::<Line>::new());
    assert!(g.not_shown.iter().any(|n| n.contains("--proxy-record")));
    assert!(group(&all, Group::Oversight).not_shown.iter().any(|n| {
        n.contains("the pass-through's record was changed") && n.contains("--proxy-record")
    }));

    // A record that could not be read is not one with nothing in it, even if it carries lines.
    let unread = Seen {
        not_read: Some("there is no pass-through record for this app".to_owned()),
        again: vec![again()],
        broken_at: Some(1),
        ..Default::default()
    };
    let all = gathered(Some(&unread), None);
    let g = group(&all, Group::AgainAfterNo);
    assert_eq!(g.lines, Vec::<Line>::new());
    assert!(
        g.not_shown
            .iter()
            .any(|n| n.contains("there is no pass-through record for this app")),
        "{:?}",
        g.not_shown
    );
    assert_eq!(group(&all, Group::Oversight).lines, Vec::<Line>::new());
}

#[test]
fn a_tool_called_outside_the_task_file_is_a_line_only_when_the_file_says_which_may_be_called() {
    let held = Held::default();
    let tool = Tool {
        name: "delete".to_owned(),
        calls: 1,
        turned_down: 0,
    };
    let said = Seen {
        calls: 1,
        outside: Some(vec![tool]),
        ..Default::default()
    };
    let all = gathered(Some(&said), Some(&held));
    let g = group(&all, Group::Outside);
    assert_eq!(g.lines.len(), 1, "{g:?}");
    assert_eq!(
        g.lines[0].what,
        "delete was called once, and [may] call does not name it"
    );
    assert!(
        g.not_shown
            .iter()
            .any(|n| n.contains("some other way than through av proxy"))
    );

    // The file does not say: a question, and no line.
    let unsaid = Seen {
        calls: 1,
        outside: None,
        ..Default::default()
    };
    let all = gathered(Some(&unsaid), Some(&held));
    let g = group(&all, Group::Outside);
    assert_eq!(g.lines, Vec::<Line>::new());
    assert!(
        g.not_shown
            .iter()
            .any(|n| n.contains("[may] call is left out"))
    );

    // Not asked for: said so.
    let all = gathered(None, Some(&held));
    assert!(
        group(&all, Group::Outside)
            .not_shown
            .iter()
            .any(|n| n.contains("--proxy-record"))
    );
}

#[test]
fn a_pass_through_record_whose_chain_does_not_check_out_is_a_line_under_oversight() {
    let seen = Seen {
        calls: 2,
        broken_at: Some(2),
        ..Default::default()
    };
    let all = gathered(Some(&seen), None);
    let g = group(&all, Group::Oversight);
    assert!(
        g.lines
            .iter()
            .any(|l| l.from == "the pass-through's record" && l.place == "line 2 of the record"),
        "{g:?}"
    );
}

#[test]
fn a_call_that_came_back_in_a_later_session_is_said_as_that_and_not_as_made_again_after_its_no() {
    let seen = Seen {
        calls: 2,
        again: vec![Again {
            times: 0,
            later: 1,
            ..again()
        }],
        runs: vec![crate::proxy_seen::Run::default()],
        ..Default::default()
    };
    let all = gathered(Some(&seen), None);
    let g = group(&all, Group::AgainAfterNo);
    let what = &g.lines[0].what;
    assert!(
        what.contains(
            "the same call was made once in a later session, which may not have known of the no"
        ) && !what.contains("in the same session"),
        "{what}"
    );
    // A run with no start line is from before av proxy marked its starts, and the group says what that costs.
    assert!(
        g.not_shown
            .iter()
            .any(|n| n.contains("read as one session")),
        "{:?}",
        g.not_shown
    );
}
