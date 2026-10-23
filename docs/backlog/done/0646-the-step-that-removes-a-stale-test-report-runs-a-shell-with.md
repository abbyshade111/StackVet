# The step that removes a stale test report runs a shell with the report path in quotes

**Status:** done, 23 October 2026

From the triage of StackVet's findings on its own repository (backlog 0633, 23 October 2026). Not one of the 312: found reading the code beside them. Before running the app's tests, `sv` removes a stale
test report in the app's container with `sh -c "rm -f -- '<path>'"` (`crates/sv-run/src/docker.rs`), the path from
`[stack.run] test_report` in stackvet.toml, unchecked. A `'` in the path ends the quoting, and what follows runs as a
command in the app's container.

It gives no one anything new: whoever writes `test_report` also writes `test`, which runs as a shell line in the same
container by design. But a path with a quote in it breaks the step, and there is no need for a shell. To do: run
`docker exec <app> rm -f -- <path>` with the path as an argument of its own, and test that a path holding a quote is
removed and runs nothing. No decision changes.

**Done, 23 October 2026.** The stale test report is removed with `docker exec <app> rm -f -- <path>`, the path an
argument of its own (`DockerBackend::remove_stale_report` in `crates/sv-run/src/docker.rs`), so no shell reads it. Test:
`crates/sv-run/src/docker/stale_report_tests.rs`, which runs in a read-only `busybox` container with the report folder
the app's run gets. With the old `sh -c` removal put back, its path `x'; touch …; rm -f …; '` ran both commands and the
test failed. Where no container backend answers it says so and tries nothing, and CI, which must have one, fails
`a_container_backend_is_here_where_ci_says_it_must_be` instead.
