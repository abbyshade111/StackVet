//! What the spec tells the AI coding tool about a limit on sign-in attempts (backlog 0383; the owner's answer of 14
//! October 2026, A). It once said to let the copy `sv` runs allow 60 sign-ins, and twelve of 90 builds in the second
//! loop trial raised the wrong-password limit itself for that copy, so the guessing check reported a limit the shipped
//! app kept as missing. Now it says to keep the wrong-password limit, and to raise only a limit that counts every
//! sign-in.

/// The spec's words as one line, without the `#` that starts each of its lines.
fn spec_words() -> String {
    sv_manifest::spec::STARTER_MANIFEST
        .lines()
        .map(|l| l.trim_start_matches('#').trim())
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn the_spec_says_to_keep_stopping_wrong_passwords_and_to_raise_only_a_limit_on_every_sign_in() {
    let words = spec_words();
    assert!(
        words.contains(
            "The copy `sv` runs must keep stopping wrong passwords at the number `failed-sign-ins` declares, \
             as the real app does: never raise that limit for it"
        ),
        "{words}"
    );
    assert!(
        words.contains(
            "Only if the app also has a limit that counts every sign-in, right passwords too"
        ),
        "{words}"
    );
    // The old advice, which asked for the whole limit to be raised, is gone.
    assert!(!words.contains("allow that many"), "{words}");
}
