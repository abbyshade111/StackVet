# A Rust module or item under cfg(all(test, ...)) is not read as test code

**Status:** done, 23 October 2026

Found 23 October 2026, reading the things to fix in StackVet's report on itself after backlog 0644. `sv` reads Rust test
code by the exact attribute `#[cfg(test)]` (`crates/sv-check/src/finding.rs`, `marks_test`, and
`crates/sv-check/src/finding/rust_modules.rs`, ADR-136). A condition that holds only in a test build is test-only too:
`#[cfg(all(test, unix))]`, which this repository puts on 12 test modules (`adapters/stand_in_tests.rs`,
`adapters/fence_tests.rs`, and others). Their findings, about 20 of the 151 things to fix on 23 October, read as
StackVet's own code.

To do: read a `cfg` whose condition is `test`, or `all(...)` with `test` among its own arguments, as test-only, in
both places; and not `any(test, ...)`, `not(test)`, or one where `test` sits under `not` or `any`, which can hold in a
build that is not a test. Test each form, both ways. A Later entry on ADR-136.

**Built within 0654,** at the owner's answer of 23 October 2026 to backlog 0647: one reading of test code for every
check, which reads `cfg(all(test, ...))` too.

**Done, 23 October 2026,** within backlog 0654 (ADR-139): `#[cfg(all(test, unix))]` and every `cfg` that holds only in a test
build reads as test code, on a declaring line and on lines inside a file; `any(test, ...)` and `not(test)` do not.
