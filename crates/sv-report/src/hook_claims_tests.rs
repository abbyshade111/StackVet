//! What a report says of a claim that the tests pass that the hook record shows otherwise (backlog 0565).

use crate::hook::{HookSeen, Mismatch, Said, sentences};

#[test]
fn each_claim_the_record_shows_otherwise_is_said_with_its_time_and_never_the_tool_s_words() {
    let seen = HookSeen {
        claims: 3,
        mismatches: vec![
            Said {
                at: "T2".into(),
                what: Mismatch::LastTestFailed { at: "T1".into() },
            },
            Said {
                at: "T3".into(),
                what: Mismatch::NoTestRan,
            },
        ],
        ..Default::default()
    };
    let (lines, _) = sentences(&seen);
    let all = lines.join("\n");
    assert!(
        all.contains("The tool ended 3 turns saying the app's tests pass"),
        "{all}"
    );
    assert!(all.contains("never its words"), "{all}");
    assert!(
        all.contains("in 2 of them the record shows otherwise"),
        "{all}"
    );
    assert!(
        all.contains(
            "At T2, the tool said the tests pass: the last test command before it, at T1, failed."
        ),
        "{all}"
    );
    assert!(
        all.contains(
            "At T3, the tool said the tests pass: no test command had run in that session."
        ),
        "{all}"
    );
}

#[test]
fn claims_the_record_bears_out_are_counted_and_nothing_more() {
    let (lines, _) = sentences(&HookSeen {
        claims: 1,
        ..Default::default()
    });
    let all = lines.join("\n");
    assert!(all.contains("1 turn saying the app's tests pass"), "{all}");
    assert!(
        all.contains("had finished without an error, with no file written since"),
        "{all}"
    );
    assert!(!all.contains("At "), "{all}");
    let (none, _) = sentences(&HookSeen::default());
    assert!(!none.join("\n").contains("tests pass"));
}
