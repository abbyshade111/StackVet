# One reading of test code for every check, Rust's test modules in src/ included

**Status:** done, 23 October 2026

From the owner's answer of 23 October 2026 to backlog 0647's choices ("A: read the source", and all three test-code fixes); the design entry is `docs/design/0394-the-path-rule-s-false-alarms-and-how-test-code-is-read-23.md`. Builds backlog 0652 (`#[cfg(all(test, ...))]`) within it.

Test code is read three ways: the code rules mark it and list it apart; nine checks skip test files by name alone
(`sv_scan::files::is_test_path`); and Rust's test modules in `src/` are known only to the code rules
(`crates/sv-check/src/finding.rs`, `crates/sv-check/src/finding/rust_modules.rs`). To do: one reading in `sv-scan` that
every check uses, by name and folder as now, by `#[cfg(test)]` lines and the line that declares a module, and by a `cfg`
whose condition holds only in a test build (`test`, or `all(...)` with `test` among its own arguments; never `any`, and
never under `not`). The six checks that gather evidence of what the app uses keep skipping test code, now including
Rust's. Tests of each form both ways, and of a check that skips. ADR-139, proposed.

**Order,** at the owner's word of 23 October 2026: after 0653.

**Done, 23 October 2026** (ADR-139, accepted; ADR-136, Later). One reading of Rust test code, in `sv-scan`
(`crates/sv-scan/src/rust_tests.rs`): a file is test code by its folder or name, by its own `#![cfg(test)]`, or by the
line declaring it as a module, under a `cfg` that holds only in a test build (`test`, or `all(...)` with such a part;
never `any` or `not`). Every check reads it through `Listing::is_test_code`: the eight that gather evidence or skip test
files, the import reading, and the code rules' marking, which drops its own parser for it. Read from the text, so `sv-scan`
needs no Rust grammar. Tests: `crates/sv-scan/src/rust_tests_tests.rs`, and 0644's tests, unchanged, on the shared
reader.
