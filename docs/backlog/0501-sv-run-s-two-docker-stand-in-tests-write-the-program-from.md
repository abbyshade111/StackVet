# sv-run's two docker stand-in tests write the program from the test process, so a parallel test hits "Text file busy" (the race sv-check's executable helper guards)

**Status:** claimed by honesty, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Found by session honesty on 14 October 2026, when its pull request for 0421 (#1762, no Rust changed) failed `suite
(rest)` on `a_teardown_names_what_it_could_not_remove_and_not_what_was_already_gone` with "Text file busy"; read
against the code by the orchestrating session before filing. Not the same fault as 0276 (the same test naming another
container), though it is the same test.

**What is wrong.** `crates/sv-run/src/docker/container_record_tests.rs:14` and
`crates/sv-run/src/docker/resource_reading_tests.rs:14` write their stand-in `docker` with `std::fs::write` and
`set_permissions(0o755)` from the test process itself. While the file is open for writing, any other test thread that
starts a program hands the open file to its child until the child's own program begins, and running the script then
fails with "Text file busy" (ETXTBSY). `sv-check` met this on 9 October 2026 and wrote `test_support::executable`,
which has a child `sh` write and `chmod` the script instead (its comment: eight threads writing and running a script
150 times each met the race 45 times in 1,200 when written in the process, and never when written by a child).
`sv-run`'s tests do not use it, so every pull request that runs the suite can fail at random here.

**Fix.** Write the two stand-ins through the same helper: `sv-run` depends on `sv-check`, so either make the helper
reachable from there (it must not be behind `cfg(test)` of the other crate; a small `pub` module, or a dev-only
feature) or give `sv-run` its own copy of the fifteen lines with a comment naming the original, and use it in both
files. Then look for any other test in `sv-run` and `sv-cli` that writes a program it runs (`grep -rn "0o755"
crates/`) and give it the same. Must not lose: the tests still make the file in their own scratch folder.

**Record.** None: test code only; `sv` itself does not change.
