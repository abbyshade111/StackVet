# A pull request that changes only backlog items runs only the tests that read them

**Status:** done, 12 October 2026

The owner's request of 12 October 2026: a claim waits on the whole suite before it can merge, and every build waits on
its claim. The required checks on `main` (the ruleset "Main - ADR Check") are "Decision records", "test", and code
scanning. "test" is `rust.yml`'s whole suite, about 15 minutes, whatever the pull request changes. #1519, a claim of
one new file, took 21 minutes to merge; #1508, claims and done notes only, took 30.

**Plan (a new decision record, proposed here and numbered, then accepted, in the pull request that builds it, since a record number cited before its record exists fails `decision_records.rs`).** The `test` job keeps its name, since a
required check that never reports blocks a pull request for good. Its first step reads which files the pull request
changes, against the commit it merges into (`HEAD^1` of the merge commit GitHub checks out).

- **Only files under `docs/backlog/`, on a pull request:** it runs the tests that read those files and no others:
  `backlog_board.rs`, `docs_page.rs`, `docs_page_tabs.rs`, and `decision_records.rs` in `sv-cli`. This claim's own first run found the fourth: `decision_records.rs` reads every Markdown file under `docs/` for the record numbers it cites. They need `sv-cli` built, but not Docker,
  formatting, lints, the rest of the suite, or the credit log, so the job should take a few minutes instead of 15.
- **Anything else, or any push or nightly run on `main`:** everything, as now.

The whole suite still runs on every commit to `main` (ADR-051). Anything the short path misses therefore shows up on
`main` within the quarter hour, and is fixed forward. Only `docs/backlog/` gets the short path, because other
documents (`docs/adr`, the design entries, `CLAUDE.md`, `data/`) are read by tests across the suite. The top of
`rust.yml` says "nothing is filtered out by path"; the new record says why there is one exception.

**To measure:** the time from opening to merge of the first claim after it lands, beside #1519's 21 minutes. Also how
long code scanning (CodeQL) takes on a claim, since it is also required and this item does not shorten it.

**Broken on purpose, each put back:** a pull request with one file outside `docs/backlog/` must take the long path; a
push to `main` must take the long path; a backlog file the tools cannot read must fail the short path.

**Measured, 12 October 2026.** The first pull request on the short path, #1534 (a backlog scrub, backlog files only), went from opening (16:08:39 UTC) to merged (16:14:20) in 5 minutes 41 seconds, beside #1519's 21 minutes. Its checks: `test` 2 minutes 22 seconds on the short path (about 15 on the whole suite); CodeQL's Rust analysis 5 minutes 22 seconds, which is now what a claim waits on; CodeQL's Python and JavaScript analyses about a minute each; Decision records 9 seconds. CodeQL took 6 to 6½ minutes on each of the eight pull requests before it that day, so this item does not shorten it. Whether CodeQL could skip a pull request of backlog files only, and still satisfy the code-scanning rule on `main`, is not looked into here.
