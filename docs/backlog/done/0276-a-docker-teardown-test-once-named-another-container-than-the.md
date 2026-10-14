# A Docker teardown test once named another container than the one it made (sv-run, 12 October 2026)

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Seen on 12 October 2026 (session backlog-org-0275) in CI's run of pull request #1543, a test for backlog 0275 that
changed only `.github/workflows/codeql.yml`.
`docker::container_record_tests::a_teardown_names_what_it_could_not_remove_and_not_what_was_already_gone`
(`crates/sv-run/src/docker/container_record_tests.rs`, line 48) expected `container app1 (Error response from daemon:
container app1 is busy)` and was given `container sv-1-1-test-app (…)`. A re-run of the same commit passed, and
`main`'s runs of the same hour passed.

Look first at whether the test reads a container list, or a stand-in's output, that another test running at the same
time can add to: the name it was given looks like one a run gives its own container, which was not checked. If so, the
test's own setup should name only what it made, so it cannot be handed another test's container.

Seen again on 13 October 2026: Rust (suite (rest)), [run 38347894447](https://github.com/abbyshade111/StackVet/actions/runs/38347894447), commit e723bf0 (the merge of #1658); the same line 48, given `container sv-1-1-test-app (…)` where it expected `container app1 (…)`. The next push's run (#1662, `0dd38a2`) passed.

**Done 14 October 2026 (session documentation): overtaken by backlog 0501.** The name the test was given,
`sv-1-1-test-app`, is the one `to_remove` falls back to (`crates/sv-run/src/docker.rs`) when the stand-in `docker`'s
`ps -aq` does not answer; the `rm -f` that followed did answer, as the script says for any name but `gone1`. So the
first call to the stand-in failed and the second ran: the "text file busy" race, in which a script written by the
test process is held open by another test thread's child for a moment. No other test's container was read; the
container list here is the stand-in's own fixed output. Until 14 October 2026 this test wrote its stand-in with
`fs::write`; 0501 (commit `ae419f87`, 14 October 2026 01:10 UTC, after both failures here) changed it, and every
other stand-in, to `sv_check::test_support::executable`, which writes the script from a child `sh`, and whose own test
met the race 45 times in 1,200 written in the test process and never written by a child. Nothing more is built here.
