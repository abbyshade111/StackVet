//! Small things done to a piece of text in more than one place, written once (backlog 0461): the first line worth
//! showing, and a cut that says how much it left out.

/// The first line of `text` that is not blank, trimmed, or `no detail` when there is none, so a message built from
/// it never ends in nothing.
pub fn first_line(text: &str) -> String {
    text.lines()
        .map(str::trim)
        .find(|l| !l.is_empty())
        .unwrap_or("no detail")
        .to_owned()
}

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

#[cfg(test)]
#[path = "text_tests.rs"]
mod tests;
