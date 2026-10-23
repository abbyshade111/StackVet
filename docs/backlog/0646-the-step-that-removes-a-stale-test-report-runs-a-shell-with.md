# The step that removes a stale test report runs a shell with the report path in quotes

**Status:** claimed by claude/0633-triage, 23 October 2026

From the triage of StackVet's findings on its own repository (backlog 0633, 23 October 2026). Not one of the 312: found reading the code beside them. Before running the app's tests, `sv` removes a stale
test report in the app's container with `sh -c "rm -f -- '<path>'"` (`crates/sv-run/src/docker.rs`), the path from
`[stack.run] test_report` in stackvet.toml, unchecked. A `'` in the path ends the quoting, and what follows runs as a
command in the app's container.

It gives no one anything new: whoever writes `test_report` also writes `test`, which runs as a shell line in the same
container by design. But a path with a quote in it breaks the step, and there is no need for a shell. To do: run
`docker exec <app> rm -f -- <path>` with the path as an argument of its own, and test that a path holding a quote is
removed and runs nothing. No decision changes.
