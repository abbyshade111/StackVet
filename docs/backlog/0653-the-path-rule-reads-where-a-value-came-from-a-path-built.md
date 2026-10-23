# The path rule reads where a value came from: a path built from a request is a thing to fix, and any other value is listed apart

**Status:** partly done: Rust's safe forms: a name followed back to fixed text, and a join under a constant (ADR-138, Not built)

From the owner's answer of 23 October 2026 to backlog 0647's choices ("A: read the source", and all three test-code fixes); the design entry is `docs/design/0394-the-path-rule-s-false-alarms-and-how-test-code-is-read-23.md`.

`ast.file-path-from-value` (V5.3.2) reports every file call whose path is not written out, and does not ask where the
value came from; on StackVet it listed 150, none from a request. To do:

- **A path built from a request's input is a thing to fix,** at a higher confidence than today: a framework's request
  value reaching the file call in the same function, directly or through names (as `ast/followed.rs` follows them).
  What counts as a request's input is a list per framework in `data/`, starting with those the trial apps used: Flask
  and Django, Express, Go's `net/http`, PHP's superglobals, and Axum's and Actix's extractors.
- **Any other value is still a finding, still counted and shown in full, but listed apart as worth a look,** with what
  the rule could not tell said beside it.
- **Rust gains the safe forms the other languages have:** names followed back to fixed text, and joins under a
  constant.
- Tests: each framework's request input found and raised; a value from elsewhere listed apart; a safe form spared; and
  V5.3.2's status unchanged by which list a finding is on. ADR-138, proposed.
