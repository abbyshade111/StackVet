use super::one_line;

#[test]
fn what_could_start_a_line_or_hide_text_is_shown_and_the_rest_is_kept() {
    assert_eq!(one_line("a\nb\rc\td"), "a\\nb\\rc\\td");
    assert_eq!(
        one_line("bell\u{7}esc\u{1b}[31m"),
        "bell\\u{0007}esc\\u{001b}[31m"
    );
    assert_eq!(one_line("x\u{2028}y\u{2029}z"), "x\\u{2028}y\\u{2029}z");
    assert_eq!(one_line("rl\u{202e}o"), "rl\\u{202e}o");
    assert_eq!(
        one_line("iso\u{2066}late\u{2069}"),
        "iso\\u{2066}late\\u{2069}"
    );
    assert_eq!(
        one_line("zero\u{200b}width\u{feff}"),
        "zero\\u{200b}width\\u{feff}"
    );
    assert_eq!(one_line("next\u{85}line"), "next\\u{0085}line");
    // Ordinary names, other scripts and emoji included, pass through untouched.
    for kept in [
        "src/app.py",
        "Clinic booking",
        "café/日本語/Ünïcødé.rs",
        "notes 📝.md",
        "a b",
    ] {
        assert_eq!(one_line(kept), kept);
    }
    // Nothing it returns can end a line, whatever it was given.
    let every: String = (0u32..0x3000).filter_map(char::from_u32).collect();
    let out = one_line(&every);
    assert_eq!(out.lines().count(), 1, "{out:?}");
}
