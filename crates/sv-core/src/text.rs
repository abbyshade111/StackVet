//! Text helpers a moved type needs: a cut that says how much it left out (backlog 0461), moved from `sv-check`'s
//! `text` on 20 November 2026 for `Secret::redact` (backlog 0686).

/// `text`, or its first `kept` characters and how many more there were: `abcd… (6 more characters)`. A text no
/// longer than `kept` comes back whole.
pub fn cut_with_count(text: &str, kept: usize) -> String {
    let total = text.chars().count();
    if total <= kept {
        return text.to_owned();
    }
    let head: String = text.chars().take(kept).collect();
    format!("{head}… ({} more characters)", total - kept)
}
