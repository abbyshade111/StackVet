# Windows is red since 0510: the MCP status test expects five topics, and sv doctor on Windows now answers a sixth

**Status:** claimed by documentation, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Found by the orchestrating session at the 20:13 UTC check-in on 14 October 2026: `main`'s Windows workflow has failed
on every merge since b39c7da1 (#1904, backlog 0510, session documentation), one test each time:
`mcp::status_tests::the_status_answers_each_question_and_credits_nothing` (`crates/sv-cli/src/mcp/status_tests.rs`).
Rust and CodeQL are green; Windows is not a required check, which is why nothing stopped the merge (ADR-051).

**What is wrong.** 0510 gave `doctor::Asked` a `windows` field, set from `cfg!(windows)` in the status tool
(`crates/sv-cli/src/mcp/tools.rs:286`), and `doctor::answers` adds a line on Windows saying the checks that start the
app are untried there (`crates/sv-cli/src/doctor.rs:208`). The status test holds the answer to exactly five topics
(`TOPICS`: sv, git, stackvet.toml, how to start the app, Docker), so on a Windows runner the sixth topic fails
`assert_eq!(topics, TOPICS)`. The new line is right; the test did not learn about it.

**Fix.** In the test, expect the sixth topic when `cfg!(windows)` (and its state), or build the expected list from
the platform; keep the five on Mac and Linux. `crates/sv-cli/src/doctor/windows_tests.rs` already tests the line
itself. Then watch the next Windows run on `main` go green. Documentation's, as 0510's owner (CLAUDE.md: a red `main`
after your merge is yours).
