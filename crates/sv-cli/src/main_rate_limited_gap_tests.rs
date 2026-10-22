use super::*;

#[test]
fn questions_the_limiter_answered_are_named_and_nothing_else_is_said() {
    assert!(rate_limited_gap(&[]).is_none(), "no limiter, no gap");
    let gap = rate_limited_gap(&["home (429)".to_owned(), "git-head (429)".to_owned()])
        .expect("a gap when the limiter answered");
    assert!(gap.what.contains("2 of the questions"), "{}", gap.what);
    assert!(
        gap.why.contains("home (429), git-head (429)")
            && gap.why.contains("neither a finding nor a pass"),
        "{}",
        gap.why
    );
}
