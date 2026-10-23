//! Backlog 0638: `{seed:NAME}`, the value `sv run` fills in from what an app's seed printed (ADR-124), is a blank and
//! not a code written into the code; the same with text of its own, or a name `sv run` would not read, is still judged.

use super::*;

fn judged(line: &str) -> bool {
    !assignment_findings("stackvet.toml", line, 1, &(0..usize::MAX)).is_empty()
}

#[test]
fn sv_s_own_seed_placeholder_is_not_a_code_in_the_code() {
    // The line from the owner's Cursor build of 22 October 2026, where it was a medium finding.
    assert!(!judged(r#"invite = "{seed:INVITE}""#));
    assert!(!judged(r#"invite_code = "{seed:INVITE_CODE_2}""#));
    assert!(!judged(r#"token = "{seed:_first}""#));
}

#[test]
fn a_seed_placeholder_with_text_of_its_own_or_a_name_sv_would_not_read_is_judged() {
    // The control, as the line above's: a code beside the placeholder is still a code. Named, not printed.
    let with_text = ["{seed:INVITE}", "x9Q2vL7kPq"].concat();
    assert!(
        judged(&format!(r#"invite = "{with_text}""#)),
        "a value around a blank was passed over"
    );
    // Names `sv run` does not read from a seed are not its placeholder, and the form is the whole value only.
    for not_a_seed_placeholder in [
        "{seed:9INVITE}",
        "{seed:INVITE-CODE}",
        "{seed:}",
        "x{seed:INVITE}",
    ] {
        assert!(
            !looks_like_placeholder(not_a_seed_placeholder),
            "{not_a_seed_placeholder}"
        );
    }
    assert!(
        looks_like_placeholder("{seed:INVITE}"),
        "the setup: the form itself is a blank"
    );
}
