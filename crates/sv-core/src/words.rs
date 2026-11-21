//! Whether a value reads as words, moved from `sv-check`'s `secrets` on 20 November 2026 for `Secret::redact`
//! (backlog 0686).

/// Whether a value is made of words, as a passphrase or a test fixture is (`violet-canoe-marble-ledger`,
/// `TestPassword`), rather than random characters (backlog 0529). Each word, split at `-`, `_`, `.`, a space, or a
/// capital, is three letters or more with a vowel in it; a digit or any other character, or a word with no vowel
/// (`Qz`, `kqzvmw`), says otherwise. Two words at least, so a single random-looking run of letters is not one.
pub fn reads_as_words(value: &str) -> bool {
    let mut words = Vec::new();
    for part in value.split(['-', '_', '.', ' ']) {
        if part.is_empty() {
            continue;
        }
        if !part.chars().all(|c| c.is_ascii_alphabetic()) {
            return false;
        }
        let mut word = String::new();
        for c in part.chars() {
            if c.is_ascii_uppercase()
                && !word.is_empty()
                && !word.chars().all(|w: char| w.is_ascii_uppercase())
            {
                words.push(std::mem::take(&mut word));
            }
            word.push(c);
        }
        words.push(word);
    }
    words.len() >= 2
        && words.iter().all(|w| {
            w.len() >= 3
                && w.chars()
                    .any(|c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u' | 'y'))
        })
}
