# More Rust-specific tests of what StackVet finds in a Rust app

**Status:** done, 24 October 2026

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

**Done, 24 October 2026.** `examples/rust-notes` is a small Axum-shaped notes service, never built or run: for each of
the 18 rules that read Rust, a fault and its safe form side by side, marked on their lines (`// sv:fault <rule>`,
`// sv:safe <rule>`), a module of its own (`notes.rs`), and its test code in the three shapes `sv` reads (an inline
`#[cfg(test)]` module, a `#[path]` module, and one under `cfg(all(test, unix))`, each fault there in a helper outside
`#[test]` so only the declaring line marks it). `crates/sv-check/tests/rust_example.rs` holds each rule to its fault
and never its safe form, the test code to being marked, and every rule that reads Rust to having a fault there; it
finds the lines by their markers, not by number. Of the 24 rules that do not read Rust, `sv`'s data already says why
for each; twelve are gaps, filed as backlog 0661. Building it found that a Rust call inside a macro's arguments is
read by no rule: filed as backlog 0660, and held as known in the example (`// sv:missed`). The workspace leaves the
example out (`Cargo.toml`, `exclude`).
