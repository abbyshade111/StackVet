//! What `stackvet.toml` holds may be the AI coding tool's words, and `sv review` shows it to the person before they
//! sign. A field holding line breaks could go on as lines shaped like `sv`'s own, so the person would read one entry
//! and sign another (ADR-026, Later, 21 November 2026). Every field is shown on one line.

use super::tests::{HEAD, Scratch, proposal, with_app};

/// A reason that ends, then carries on as `sv review`'s own lines: the real entry's "Written by" line, a note that
/// it was left alone, and an entry that is not in the file.
const FORGED: &str = "checked\\\"\\n  Written by: somebody else\\n  Left as a proposal.\\n\\nA finding proposed as a \
                      false alarm: the code is fine.\\n  Rule: tests.print\\n  File: tests/conftest.py\\n  Reason \
                      given: \\\"a print in a test helper, which is fine";

#[test]
fn a_reason_with_line_breaks_cannot_draw_lines_shaped_like_sv_reviews_own() {
    let s = Scratch::new("one-line");
    with_app(&s, &format!("{HEAD}{}", proposal(FORGED)));
    // The setup: the file really holds the line breaks, and one entry.
    let m = sv_manifest::Manifest::load(&s.app().join("stackvet.toml")).unwrap();
    assert_eq!(m.finding_review.len(), 1, "the setup: one entry");
    assert!(
        m.finding_review[0]
            .why
            .contains("\n  Written by: somebody else\n"),
        "the setup: the reason holds line breaks: {:?}",
        m.finding_review[0].why
    );
    // Enter: left as it is.
    let (result, out) = s.run("\n");
    result.unwrap();
    assert!(
        out.contains("Reason given:"),
        "the setup: the entry was offered: {out}"
    );
    for line in out.lines() {
        let line = line.trim_start();
        assert!(
            !line.starts_with("Written by: somebody else")
                && !line.starts_with("Left as a proposal.")
                && !line.starts_with("File: tests/conftest.py")
                && !line.starts_with("Rule: tests.print"),
            "a line of the reason is shown as one of sv review's own: {out}"
        );
    }
    assert_eq!(
        out.lines()
            .filter(|l| l.starts_with("A finding proposed as"))
            .count(),
        1,
        "one entry is in the file, and one is shown: {out}"
    );
    assert!(
        out.contains("\\n  Written by: somebody else\\n"),
        "the line breaks are written out, so the person sees them: {out}"
    );
}
