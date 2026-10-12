# The authorization-failure log check on every listed page (12 October 2026)

Backlog 0232, part 5 (group E of the one-sample inventory, in part), and ADR-053, "Later, 12 October 2026" (the fourth).

**The problem.** V16.3.2 asks that authorization failures be logged. The run asked the first listed private page, as a
stranger, with a marker in its address, and credited the requirement in part when the app's output had that marker on
a line with the refusal's status. The next page may be guarded, and logged, by other code.

**What was built.** `signin.rs` plants the two markers (after `?`, and as the last part of the path) on every listed
private page. Page 0 keeps `sv-log-refused-<tag>`; page `i` is `sv-log-refused<i>-<tag>`, so the number sits before
the hyphen and no marker is part of another: one line cannot stand for two pages. `Markers::refused_requests` carries
the page each marker was planted on. `logs::evaluate` counts the pages planted and the pages whose refusal was found
with its status, and gives the credit whole only when there were two or more and every one was found.

**Tests.** `crates/sv-check/src/logs_pages_tests.rs` (five, including the marker rule over several tag shapes) and
`crates/sv-check/src/signed_in/signin/log_pages_tests.rs` (one, through the whole signed-in run).

**Broken on purpose, each put back:** markers on the first page only (1 red); whole from one page (2 red); whole with a
page's refusal not found (2 red); no page number on the markers (1 red).
