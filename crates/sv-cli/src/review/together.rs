//! The answers given as the owner's, shown together and recorded at one `owner` (backlog 0618; ADR-026, Later, 20
//! October 2026).
//!
//! An answer a person gives their AI coding tool in the chat is written as theirs (`by = "owner"`, or a section
//! marked `Written by: owner`), and counts as theirs only once `sv review` records it. Asked about one at a time, a
//! person with a dozen of them handed the rest back to the tool (backlog 0550). Here every one is listed, numbered,
//! under the warning each was given alone, and one prompt takes `owner` for all of them, numbers to leave some out, or
//! Enter to go through them one at a time as before. Nothing about what counts changes: each is still sealed over its
//! own fields, and saved only if it then counts.

use super::*;

/// What the person chose.
pub(super) enum Together {
    /// Record these, by their place in the list; the rest are left as they are. Empty when every one was left out,
    /// or the input ended.
    Record(Vec<usize>),
    /// Go through them one at a time instead.
    OneByOne,
}

/// Shows `whats` (what each answer says, as `sv review` words it) and asks once.
pub(super) fn ask(
    whats: &[String],
    input: &mut dyn BufRead,
    out: &mut dyn Write,
) -> Result<Together> {
    writeln!(
        out,
        "\n{} of these are answers given as yours, most likely ones you gave your AI coding tool in the chat. None \
         was recorded through `sv review`, so for now each counts as your AI coding tool's word. Your AI coding tool \
         can write `by = \"owner\"` as well as you can, so any of them, and any `how` beside it, may be the tool's \
         words, not yours. Record only the ones you gave, or made, yourself. If one is wrong, change it in the file \
         first and run `sv review` again.",
        whats.len()
    )?;
    let mut left: Vec<usize> = (0..whats.len()).collect();
    loop {
        writeln!(out)?;
        for &i in &left {
            writeln!(out, "  {}. {}", i + 1, whats[i])?;
        }
        let Some(typed) = super::ask(
            input,
            out,
            &format!(
                "\nType `owner` to record {} as your own. To leave some out, type their numbers (for example `{}`), \
                 and you will see the list again without them. Press Enter to go through them one at a time instead.\n> ",
                if left.len() == 1 {
                    "it".to_owned()
                } else {
                    format!("all {}", left.len())
                },
                left.last().map_or(1, |i| i + 1)
            ),
        )?
        else {
            return Ok(Together::Record(Vec::new()));
        };
        if typed.is_empty() {
            return Ok(Together::OneByOne);
        }
        if typed.eq_ignore_ascii_case(sv_check::design::OWNER) {
            return Ok(Together::Record(left));
        }
        match numbers(&typed, &left) {
            Some(out_of_it) => {
                left.retain(|i| !out_of_it.contains(i));
                if left.is_empty() {
                    writeln!(out, "  Every one is left as it is.")?;
                    return Ok(Together::Record(Vec::new()));
                }
            }
            None => writeln!(
                out,
                "  Type `owner`, the numbers of the ones to leave out, or press Enter. Only the app's owner gives \
                 these answers; someone else who has looked can confirm the AI coding tool's answer instead."
            )?,
        }
    }
}

/// The places in the list that `typed` names, when it is nothing but numbers of answers still in it (`2 5`, `2, 5`).
fn numbers(typed: &str, left: &[usize]) -> Option<Vec<usize>> {
    let named: Vec<usize> = typed
        .split(|c: char| c == ',' || c.is_whitespace())
        .filter(|w| !w.is_empty())
        .map(|w| w.parse::<usize>().ok()?.checked_sub(1))
        .collect::<Option<_>>()?;
    (!named.is_empty() && named.iter().all(|i| left.contains(i))).then_some(named)
}

#[cfg(test)]
#[path = "together_tests.rs"]
mod tests;
