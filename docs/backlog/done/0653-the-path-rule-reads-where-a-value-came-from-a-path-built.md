# The path rule reads where a value came from: a path built from a request is a thing to fix, and any other value is listed apart

**Status:** done, 25 October 2026

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

**Built, 23 October 2026** (ADR-138, accepted). `ast.file-path-from-request` reports a path built from a request's input
in eleven languages, followed through up to three names; a path rule finding from any other value is listed apart as
worth a look in those languages, still counted. On StackVet's own code the things to fix fell from 151 to 1 (the bill
of materials, backlog 0656), with the same 31 requirements needing attention. Tests:
`crates/sv-check/src/ast/path_from_request_tests.rs` and `crates/sv-check/src/finding/path_apart_tests.rs`. What remains
is Rust's safe forms, which matter less now that Rust's other-value path findings are listed apart.

**Claimed 25 October 2026 for what remained:** Rust's safe forms for `ast.file-path-from-value`, as ADR-138 left them
("Not built"): a path given as a name the same function or file sets only to fixed text, and a join under a constant
(`Path::new(ROOT).join("data")`, `PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("x")`). Each form found by a test
first, built only where the rule still reports it today; recorded as a dated Later entry on ADR-138.

**Built 25 October 2026** (ADR-138, Later): Rust's safe forms, in `crates/sv-check/src/ast/followed_rust.rs`. Each form
was first shown still reported by a test, then built. On StackVet's own code, 68 Rust path findings spared and none
newly reported. Four tests in `ast/rust_safe_path_tests.rs`; six guards broken on purpose, each turning a test red.

