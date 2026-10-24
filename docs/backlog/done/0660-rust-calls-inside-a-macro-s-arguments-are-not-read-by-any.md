# Rust calls inside a macro's arguments are not read by any rule

**Status:** done, 24 October 2026

Found 24 October 2026, building backlog 0649's Rust example. `format!("{:x}", md5::compute(b))` is not found by the
weak-hash rule: tree-sitter's Rust grammar keeps a macro's arguments as an unparsed run of tokens (`token_tree`), so
no rule's query sees a call written inside `format!(…)`, `println!(…)`, `vec![…]`, `assert!(…)`, or a framework's own
macros. Every rule that reads Rust is affected, and Rust puts many calls inside macros. `html-from-value`'s Rust form,
`Html(format!(…))`, depends on it too (backlog 0661).

Held as known in `examples/rust-notes/src/notes.rs`, on a line marked `// sv:missed weak-hash-function`; the test in
`crates/sv-check/tests/rust_example.rs` fails the day it is found, and says to make it `sv:fault`.

To do: read the arguments of the common macros whose arguments are expressions (`format!`, `print!`, `println!`,
`eprintln!`, `write!`, `writeln!`, `vec!`, `assert!`, `assert_eq!`, `panic!`, and the like) as Rust, by parsing the
token tree's text as an expression list and running the rules on it, at the line it is on; and say in the rules' Rust
`looksForIn` what is read. A macro of the app's own, or one whose arguments are not expressions, stays unread, and is
said to be.

**Built, 24 October 2026** (ADR-142). `crates/sv-check/src/ast/macro_args.rs` rewrites a copy of each Rust file, byte
for byte, with the standard macros whose arguments are expressions written as calls, and `read_file` runs the rules on
it, keeping only what lies inside their arguments, never the rewritten macro itself. A macro of the app's own, or
`vec![value; count]`, stays unread, and a clean result in Rust says so. The example's held line is a `sv:fault` now,
with a nested case beside it and `assert!(std::write!(…))` as a safe form. Broken on purpose: with the rewrite turned
off, two unit tests and the example's test went red, naming both lines; with the exact-span guard off, the test that
holds it directly went red. No rule today reads a rewritten macro's own head as a call, which is why that guard is
held by a test of its own rather than by a finding.
