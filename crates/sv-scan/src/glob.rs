//! The one wildcard matcher `sv` reads paths with (backlog 0456): a `.gitignore` line (`sv-check`'s `config.rs`), and a
//! semgrep rule's `paths` (`sv-check`'s `adapters.rs`). Each had a matcher of its own, and each lacked what the other
//! had: the `.gitignore` one read `**/x` as `x` anywhere, so `**/.env` matched `my.env`; the semgrep one read `[` and
//! `\` as themselves. `ecosystems.rs` keeps a matcher of its own for workspace members, which name whole folders.

/// Whether `text`, a path with `/` between its parts, matches `pattern`, with git's wildcards:
/// - `*` is any run of characters within one part, and `?` one character, never `/`;
/// - `**/` is no folders or any number of whole ones, and `**` elsewhere any run of characters, `/` included;
/// - `[...]` is one character of a set, with `a-z` for a range and a first `!` or `^` for its opposite, never `/`;
/// - `\` takes the next character as it is.
pub fn matches(pattern: &str, text: &str) -> bool {
    let p: Vec<char> = pattern.chars().collect();
    let t: Vec<char> = text.chars().collect();
    go(&p, &t)
}

fn go(p: &[char], t: &[char]) -> bool {
    match p {
        [] => t.is_empty(),
        // No folders, or any number of whole ones.
        ['*', '*', '/', rest @ ..] => {
            go(rest, t) || (0..t.len()).any(|i| t[i] == '/' && go(rest, &t[i + 1..]))
        }
        ['*', '*', rest @ ..] => (0..=t.len()).any(|i| go(rest, &t[i..])),
        ['*', rest @ ..] => (0..=t.len())
            .take_while(|i| *i == 0 || t[i - 1] != '/')
            .any(|i| go(rest, &t[i..])),
        ['?', rest @ ..] => t.first().is_some_and(|c| *c != '/') && go(rest, &t[1..]),
        ['[', rest @ ..] => {
            let Some(close) = rest.iter().skip(1).position(|c| *c == ']').map(|i| i + 1) else {
                return t.first() == Some(&'[') && go(rest, &t[1..]);
            };
            let (set, after) = (&rest[..close], &rest[close + 1..]);
            let (negated, set) = match set.first() {
                Some('!' | '^') => (true, &set[1..]),
                _ => (false, set),
            };
            let Some(c) = t.first() else { return false };
            let mut inside = false;
            let mut i = 0;
            while i < set.len() {
                if i + 2 < set.len() && set[i + 1] == '-' {
                    inside |= set[i] <= *c && *c <= set[i + 2];
                    i += 3;
                } else {
                    inside |= set[i] == *c;
                    i += 1;
                }
            }
            inside != negated && *c != '/' && go(after, &t[1..])
        }
        ['\\', next, rest @ ..] => t.first() == Some(next) && go(rest, &t[1..]),
        [c, rest @ ..] => t.first() == Some(c) && go(rest, &t[1..]),
    }
}

#[cfg(test)]
#[path = "glob_tests.rs"]
mod tests;
