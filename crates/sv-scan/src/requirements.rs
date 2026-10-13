//! The logical lines of a pip requirements file, read as pip reads them (backlog 0291).
//!
//! Four readers in `sv` read `requirements.txt` (whether it is hash-pinned, the bill of materials, what the
//! manifest asks for, and what the install step would leave unpinned), and until 0291 each joined lines and
//! cut comments its own way: one ended a comment at a tab as well as a space, one cut comments before joining
//! continued lines and another after, one did not join them at all. They now all start from these lines.
//!
//! pip (`req_file.py`, `join_lines` then `ignore_comments`): a physical line ending in `\` is joined to the
//! next, with the `\` dropped and nothing added, unless the line is itself a comment; a comment is a `#` at
//! the start of a line or after white space, and runs to the end of the logical line; what is left is
//! trimmed, and empty lines are dropped.

/// Whether `line` starts with a comment: a `#` with nothing but white space before it.
fn is_comment_line(line: &str) -> bool {
    line.trim_start().starts_with('#')
}

/// `line` with its comment cut off: from the first `#` that starts the line or follows white space,
/// together with the white space before it.
fn without_comment(line: &str) -> &str {
    let bytes = line.as_bytes();
    for (at, &b) in bytes.iter().enumerate() {
        if b == b'#' && (at == 0 || bytes[at - 1].is_ascii_whitespace()) {
            return line[..at].trim_end();
        }
    }
    line
}

/// Every requirement or option line of `text`, continuations joined and comments gone, as pip reads them.
pub fn logical_lines(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut joined = String::new();
    let mut joining = false;
    for line in text.lines() {
        let line = line.strip_suffix('\r').unwrap_or(line);
        match line.strip_suffix('\\') {
            Some(head) if !is_comment_line(line) => {
                joined.push_str(head);
                joining = true;
            }
            _ => {
                // A comment line ends a continuation as a line of its own would, and adds nothing.
                if !is_comment_line(line) {
                    joined.push_str(line);
                }
                push(&mut out, &joined);
                joined.clear();
                joining = false;
            }
        }
    }
    if joining {
        push(&mut out, &joined);
    }
    out
}

fn push(out: &mut Vec<String>, line: &str) {
    let line = without_comment(line).trim();
    if !line.is_empty() {
        out.push(line.to_owned());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_are_joined_and_comments_cut_as_pip_does() {
        let text = "# a whole-line comment\n\
                    flask==3.0.0 \\\n    --hash=sha256:aaaa  # after the hash\n\
                    requests==2.31.0\t# a tab before it\n\
                    Jinja2 \\\n  ==3.1.4 \\\n  --hash=sha256:bbbb\n\
                    \n\
                    pkg#not-a-comment==1.0\n\
                    tail==1 \\\n# a comment line ends the continuation\n\
                    crlf==2\r\n";
        assert_eq!(
            logical_lines(text),
            [
                "flask==3.0.0     --hash=sha256:aaaa",
                "requests==2.31.0",
                "Jinja2   ==3.1.4   --hash=sha256:bbbb",
                "pkg#not-a-comment==1.0",
                "tail==1",
                "crlf==2",
            ]
        );
    }

    #[test]
    fn a_continuation_at_the_end_of_the_file_is_kept() {
        assert_eq!(logical_lines("a==1 \\"), ["a==1"]);
        assert!(logical_lines("").is_empty());
        assert!(logical_lines("# only a comment\n  \n").is_empty());
    }
}
