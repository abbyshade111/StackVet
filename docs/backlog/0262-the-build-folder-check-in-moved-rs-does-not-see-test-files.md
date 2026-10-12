# The build-folder check in moved.rs does not see test files declared in lib.rs or main.rs

**Status:** claimed by backlog-org-0262, 12 October 2026

Found on 11 October 2026, when it failed pull request #1472. Low priority: it errs on the strict side, so it can
flag test code wrongly but never lets program code through.

**What the check is for.** `nothing_but_the_data_finder_reads_from_the_build_folder_at_run_time` in
`crates/sv-cli/tests/moved.rs` fails on any `env!("CARGO_MANIFEST_DIR")` outside test code, since program code that
names the build folder reads from it wherever `sv` is installed (ADR-036). Test code may name it, because tests run
where the code was built.

**The gap.** A whole file of tests (`#[cfg(test)] mod name;` pointing at `name.rs`) is skipped only when that
declaration is found in the folder's `mod.rs` or in the file named after the folder (`moved.rs`, lines 148 and
149). For a file at the top of a crate's `src/`, the declaration is in `src/lib.rs` or `src/main.rs`, which the
check does not read. So `crates/sv-cli/src/resources_beside_quotas_tests.rs`, declared in `lib.rs`, was reported
as program code. #1472 was mended by having that test write its manifest inline instead; the check was left as it is.

**The fix.** Also read `lib.rs` and `main.rs` when the file sits directly in a crate's `src/`. Then break it both
ways, as `CLAUDE.md` asks: a top-level test file declared in `lib.rs` that names the build folder passes, and the
same line in a top-level file that is not declared for tests still fails. Keep the second case as a fixture, so the
guard's strict side is tested on purpose rather than by accident. A test-only change: no decision record is owed
beyond an `ADR-036: unchanged` line, since what counts as reading from the build folder does not change.
