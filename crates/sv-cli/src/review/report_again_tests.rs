//! `sv review` ends by writing the report again when it recorded something, and only then (backlog 0578). Writing it
//! with the last report's options, and refusing a report `sv` cannot show it wrote: `report_again_tests.rs` beside
//! `report_again.rs`.

use super::tests::{Scratch, with_app};

const APP: &str = "manifest-version = 1\n[app]\nname = \"R\"\naudience = \"just-me\"\n[stack]\nlanguages = [\"python\"]\n[data]\ncategories = [\"contact\"]\n";
const AGAIN: &str = "What you recorded counts once the report is written again.";

#[test]
fn something_recorded_brings_the_report_to_be_written_again() {
    let s = Scratch::new("again-recorded");
    with_app(&s, APP);
    let (result, out) = s.run_then_report("owner\n", false, &mut |_| {
        panic!("no report sv wrote is there to take options from, so nothing is run")
    });
    result.unwrap();
    assert!(
        out.contains("Confirmed as your answers."),
        "the setup: {out}"
    );
    assert!(out.contains(AGAIN), "{out}");
    assert!(out.contains("run `sv report` yourself"), "{out}");
}

#[test]
fn nothing_recorded_writes_nothing_again() {
    let s = Scratch::new("again-nothing");
    with_app(&s, APP);
    let (result, out) = s.run_then_report("\n", false, &mut |_| panic!("nothing was recorded"));
    result.unwrap();
    assert!(out.contains("Left unconfirmed."), "the setup: {out}");
    assert!(!out.contains(AGAIN), "{out}");
}
