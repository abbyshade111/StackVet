//! Backlog 0660: the calls written inside Rust's standard macros, read by the same rules as every other call.
//!
//! Tree-sitter's Rust grammar keeps a macro's arguments as an unparsed run of tokens (`token_tree`), so a rule's
//! query never sees `md5::compute(b)` inside `format!("{:x}", md5::compute(b))`. The macros named in `READ` take
//! expressions, so their arguments are read here as Rust: a copy of the file has each such `name!(…)` rewritten,
//! byte for byte, as the call `name (…)`, and the rules run on the copy. Nothing moves, so a line in the copy is the
//! same line in the file.
//!
//! Only what lies wholly inside a rewritten macro's arguments is kept from the copy, and never the rewritten macro
//! itself: `write!(f, "…")` must not read as `std::fs::write(f, …)`. A macro of the app's own, a framework's, or one
//! of these whose arguments are not expressions (`vec![0; n]`) stays unread, and the clean-run wording says so.

use std::ops::Range;
use tree_sitter::{Node, Parser};

/// The standard macros whose arguments are expressions, read as calls.
pub(crate) const READ: &[&str] = &[
    "assert",
    "assert_eq",
    "assert_ne",
    "debug_assert",
    "debug_assert_eq",
    "debug_assert_ne",
    "eprint",
    "eprintln",
    "format",
    "format_args",
    "panic",
    "print",
    "println",
    "todo",
    "unimplemented",
    "unreachable",
    "vec",
    "write",
    "writeln",
];

/// What a clean run in Rust says it read, beside the files.
pub(crate) const SAID: &str = "inside the standard macros' arguments too, such as `format!`, `println!`, and \
     `vec!`, but not inside a macro of the app's own or a library's";

/// Nested macros are found a round at a time, since a macro's arguments are tokens until its own round rewrites
/// them; no code nests the standard macros deeper than this.
const ROUNDS: usize = 8;

/// The copy of a Rust file with its standard macros read as calls, and where their arguments are.
pub(crate) struct Rewritten {
    pub source: String,
    inside: Vec<Range<usize>>,
    /// Each rewritten macro's own span, the call it became in the copy.
    heads: Vec<Range<usize>>,
}

impl Rewritten {
    /// A match in the copy is kept when its node lies wholly inside a rewritten macro's arguments and is not
    /// itself a rewritten macro; a call made on one's result (`format!(…).len()`) is code written there, and kept.
    pub(crate) fn holds(&self, node: Node) -> bool {
        let (start, end) = (node.start_byte(), node.end_byte());
        !self.heads.contains(&(start..end))
            && self.inside.iter().any(|r| r.start <= start && end <= r.end)
    }
}

/// The copy, or `None` when the file has no standard macro to read.
pub(crate) fn rewrite(source: &str) -> Option<Rewritten> {
    let mut parser = Parser::new();
    parser.set_language(&super::grammar("rust")?).ok()?;
    let mut bytes = source.as_bytes().to_vec();
    let (mut inside, mut heads) = (Vec::new(), Vec::new());
    for _ in 0..ROUNDS {
        // Every byte changed is ASCII put in place of ASCII, so the copy stays valid UTF-8.
        let text = std::str::from_utf8(&bytes).ok()?;
        let tree = parser.parse(text, None)?;
        let mut found = Vec::new();
        macros(tree.root_node(), text.as_bytes(), &mut found);
        if found.is_empty() {
            break;
        }
        for (head, bang, open, close) in found {
            bytes[bang] = b' ';
            bytes[open] = b'(';
            bytes[close] = b')';
            inside.push(open + 1..close);
            heads.push(head..close + 1);
        }
    }
    if heads.is_empty() {
        return None;
    }
    Some(Rewritten {
        source: String::from_utf8(bytes).ok()?,
        inside,
        heads,
    })
}

/// Each standard macro in the tree whose arguments are expressions: where it starts, its `!`, and its delimiters.
fn macros(node: Node, source: &[u8], out: &mut Vec<(usize, usize, usize, usize)>) {
    if node.kind() == "macro_invocation"
        && let Some(found) = standard(node, source)
    {
        out.push(found);
        // Its arguments are tokens in this round; a macro among them is found in the next.
        return;
    }
    let mut walk = node.walk();
    for child in node.children(&mut walk) {
        macros(child, source, out);
    }
}

fn standard(node: Node, source: &[u8]) -> Option<(usize, usize, usize, usize)> {
    let name = node.child_by_field_name("macro")?;
    // `std::println!` as well as `println!`.
    let last = match name.kind() {
        "scoped_identifier" => name.child_by_field_name("name")?,
        _ => name,
    };
    if !READ.contains(&last.utf8_text(source).ok()?) {
        return None;
    }
    let mut walk = node.walk();
    let tokens = node
        .children(&mut walk)
        .find(|c| c.kind() == "token_tree")?;
    // `vec![0; n]` is a value and a count, not a list of expressions.
    let mut walk = tokens.walk();
    if tokens.children(&mut walk).any(|c| c.kind() == ";") {
        return None;
    }
    let bang = (name.end_byte()..tokens.start_byte()).find(|&at| source[at] == b'!')?;
    Some((
        node.start_byte(),
        bang,
        tokens.start_byte(),
        tokens.end_byte() - 1,
    ))
}
