//! The one reading of answers past a limit that the wrong-password (V6.3.1) and wrong-code (V6.6.3)
//! checks share (backlog 0423): each answer list in the table is read once, and both checks word
//! only what it says.

use super::limit::{PastTheLimit, Reading, past_the_limit};
use super::signin::Slowing;

/// Statuses for a limit of 2 (four attempts), each answered in 100ms unless `slow`.
fn answers(statuses: [u16; 4], slow: bool) -> Vec<(u16, u128)> {
    statuses
        .iter()
        .enumerate()
        .map(|(n, &s)| (s, if slow && n >= 2 { 2_000 } else { 100 }))
        .collect()
}

#[test]
fn every_kind_of_answer_list_reads_one_way() {
    let read = |first, last, refused, status_changed, slowing| {
        PastTheLimit::Read(Reading {
            first,
            last,
            refused,
            status_changed,
            slowing,
            past: [100, 100],
            quickest: 100,
        })
    };
    let table: Vec<([u16; 4], PastTheLimit)> = vec![
        // Refused before the first attempt: a limit something earlier tripped.
        (
            [429, 429, 429, 429],
            PastTheLimit::AlreadyRefusing { first: 429 },
        ),
        (
            [423, 200, 200, 200],
            PastTheLimit::AlreadyRefusing { first: 423 },
        ),
        (
            [0, 200, 200, 200],
            PastTheLimit::AlreadyRefusing { first: 0 },
        ),
        // A crash or silence past the limit is not pushing back (backlog 0280).
        (
            [401, 401, 500, 500],
            PastTheLimit::NotReal {
                first: 401,
                last: 500,
                next: 500,
            },
        ),
        (
            [401, 401, 401, 0],
            PastTheLimit::NotReal {
                first: 401,
                last: 401,
                next: 0,
            },
        ),
        // Answered alike up to and including the first past the limit, and differently only after.
        (
            [401, 401, 401, 429],
            PastTheLimit::OneMoreLetThrough {
                first: 401,
                next: 429,
            },
        ),
        // Refused outright at the first past the limit.
        (
            [401, 401, 429, 429],
            read(401, 429, true, true, Slowing::Not),
        ),
        // A different status that is not 423 or 429 is a change, not an outright refusal: the
        // wrong-password check once also called a first 4xx after a 2xx refused, the code check
        // did not, and the two readings disagreed only in how the credit was worded.
        (
            [200, 200, 401, 401],
            read(200, 401, false, true, Slowing::Not),
        ),
        // The same answer every time, and no delay.
        (
            [401, 401, 401, 401],
            read(401, 401, false, false, Slowing::Not),
        ),
    ];
    for (statuses, expected) in table {
        assert_eq!(
            past_the_limit(&answers(statuses, false), 2, &[]),
            expected,
            "{statuses:?}"
        );
    }
}

#[test]
fn a_delay_on_both_attempts_past_the_limit_is_read_as_slowed_unless_a_control_was_as_slow() {
    let PastTheLimit::Read(slowed) = past_the_limit(&answers([401; 4], true), 2, &[]) else {
        panic!("the setup: real answers, alike throughout");
    };
    assert_eq!(slowed.slowing, Slowing::Slowed);
    assert_eq!(slowed.past, [2_000, 2_000]);
    assert!(!slowed.answered_differently());
    let PastTheLimit::Read(unclear) = past_the_limit(&answers([401; 4], true), 2, &[2_000]) else {
        panic!("the setup: real answers, alike throughout");
    };
    assert!(
        matches!(unclear.slowing, Slowing::Unclear(_)),
        "{unclear:?}"
    );
}
