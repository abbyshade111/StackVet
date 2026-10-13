//! What the report says of a build-loop record whose chain holds (backlog 0310): lines cut from the
//! end leave the chain valid, so the sentence says that, and that only an earlier report's hash shows it.

use crate::BuildLoop;
use crate::next_step_tests::empty_report;

#[test]
fn a_chain_that_holds_does_not_claim_lines_cut_from_the_end_would_show() {
    let mut report = empty_report(Vec::new());
    report.build_loop = Some(BuildLoop {
        calls: 2,
        chained: 2,
        chain_head: Some("0123456789abcdef".to_owned()),
        ..BuildLoop::default()
    });
    let line = crate::build_loop_line(&report).expect("the setup: a record to describe");
    assert!(
        line.contains("checks out against the one before it"),
        "{line}"
    );
    assert!(line.contains("Lines cut from the end"), "{line}");
    assert!(line.contains("earlier report's"), "{line}");
}
