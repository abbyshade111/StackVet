# More Rust-specific tests of what StackVet finds in a Rust app

**Status:** claimed by claude/0649, 24 October 2026

Filed 23 October 2026 at the owner's word, after the triage of StackVet's findings on its own repository (backlog
0633): "look at adding additional rust-specific testing". That triage found three rules reading Rust wrongly: the
shell-command rule's `Command::new` (backlog 0643), Rust test modules that are files of their own (backlog 0644), and
checks reading comments and strings (backlog 0645). Of the 41 rules in `data/ast-rules.json`, 17 have a Rust query, and
none of the examples under `examples/` is a Rust app.

To do: a small Rust web app among the examples (Axum or Actix), with the faults the Rust rules look for and the safe
forms of each beside them, and its tests holding each rule to firing on the one and not the other; the rules without a
Rust query listed, each with whether Rust can have that fault and how it is written there; and the Rust-specific parts
of the reading (`#[cfg(test)]`, `mod` files, `unsafe`, `Command`) tested on that app end to end.

**Order,** at the owner's word of 23 October 2026: after 0648.
