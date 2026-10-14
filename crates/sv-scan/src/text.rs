//! A line number from a byte offset, written once (backlog 0458): nine copies in two conventions, 1-based and 0-based,
//! made it one wrong neighbor away from a finding one line off.

/// The line of `text` that byte `at` falls on, counting the first line as 1, as a finding names it. `at` is a byte
/// offset at a character boundary, no greater than `text.len()`. A caller that wants the lines before `at` instead (a
/// fragment's offset in its page) writes `line_of(..) - 1` where it uses it, so the convention shows there.
pub fn line_of(text: &str, at: usize) -> usize {
    text.as_bytes()[..at]
        .iter()
        .filter(|b| **b == b'\n')
        .count()
        + 1
}

#[cfg(test)]
#[path = "text_tests.rs"]
mod tests;
