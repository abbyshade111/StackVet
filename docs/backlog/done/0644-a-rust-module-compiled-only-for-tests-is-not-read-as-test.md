# A Rust module compiled only for tests is not read as test code when it is a file of its own

**Status:** done, 23 October 2026

From the triage of StackVet's findings on its own repository (backlog 0633, 23 October 2026). 142 of the 269 path findings, and the 9 test passwords in `crates/sv-check/src/signed_in/fake_app.rs`, are in
files Rust compiles only for its tests, and the report did not say so beside them.

`sv` reads Rust test code two ways (`crates/sv-check/src/finding.rs`, `mark_rust_test_code`): a file under a `tests`
folder, by name (`sv_scan::files::is_test_path`), and lines under `#[cfg(test)]` or `#[test]` inside a file. A module
that is a file of its own, declared `#[cfg(test)] mod fake_app;` or `#[cfg(test)] #[path = "x_tests.rs"] mod tests;`,
is neither: the file carries no attribute of its own, and its name says nothing a test runner reads. This repository
keeps a module's tests that way (108 of its 109 `#[path]` modules), and so do many Rust projects.

The declaring line is the evidence: Rust compiles that file only for tests. To do: mark a finding as in test code when
its file is a module some `#[cfg(test)]` declaration in the app pulls in (by `#[path]`, or `name.rs` or `name/mod.rs`
beside the declaring file), so the report says so beside it, and the rules that are information in test code
(ADR-091, backlog 0529) read it so. Test code still counts: this only says where the finding is. Test that each form
is read, and that a module without `#[cfg(test)]` is not. ADR-136, proposed.

**Done, 23 October 2026** (ADR-136, accepted). A finding in a Rust file is marked as in test code when a file beside it
or above it declares that file as a module under `#[cfg(test)]`, by `#[path]` or as `name.rs` or `name/mod.rs`, as Rust
finds them, or declares it inside a module that is already test code. The reports then say so beside it, and the rules
that are information in test code read it that way; it still counts. Tests:
`crates/sv-check/src/finding/rust_modules_tests.rs`.
