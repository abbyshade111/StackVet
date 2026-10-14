# A "not verified" row never says what would verify it, though sv knows and the website's example promises it

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: usability. Severity: medium (every reader; data already
loaded). Verified against the code by the reviewing session before filing.

**What is wrong.** For the person, "not verified" on eighty rows reads as one thing, and it is four: a check of the
running app could settle it (`--run`), an outside tool could (`--tools`), a test the tool writes could, or only they
can. The report counts them ("N of the requirements that apply can only be checked that way", `not_run_this_time`
reading `data/reach.json`) but puts the answer on no row, so the person cannot see which rows the full run would move
and which are theirs whatever they run. 0191 part 5 chose a terminal command (`sv explain`) over a column, which is
right for a programmer and wrong for this reader, who has no terminal open beside the page. The website's settled
example rows already show a per-row reason ("the app was not started, so this could not be tried").

**Where.** `crates/sv-report/src/html.rs:541-600` (the status column), `markdown.rs:156-165` (`status_cell`),
`lib.rs:1188-1238` (`not_run_this_time`); `crates/sv-cli/src/explain.rs:8,22-34` (`checks_for`); `mcp/catalog.rs:732-769`
(`stackvet_explain`); `docs/website/home-page.md:85-91`.

**Fix.** One short phrase beside each `not verified` row, from `reach.json`'s kinds for that id: "a check of the
running app could settle it (`sv report --run`)", "an outside tool could (`--tools`)", "the app's own tests could",
"only you can (it is in "What only you can check")". Give `stackvet_explain` the same line, so the tool can answer
"what would it take?" without the terminal. Must not lose: the phrase is about what *could* credit the row, never a
credit; a kind of run that already happened and credited nothing says so ("the app ran, and no check reached it")
rather than inviting the same run again. Sibling of 0372 (what each requirement *asks*): one pull request could do
both.

**Record.** None; `ADR-110: unchanged, because` lines for `html.rs` and `markdown.rs`.

**Done, 14 October 2026:** each row left not verified says what could settle it, after its status in report.html and
compliance.md (`sv_report::could_settle`, from `data/reach.json` and how the report ran, carried as
`Report::could_settle`). A kind of run that did not happen is named with how to run it; one that happened says it
settled nothing; a test of the app's, or only the person, otherwise. `stackvet_explain` says the same from the kinds
alone (`couldSettle`), and `sv explain --app` repeats the report's phrase. Held by
`crates/sv-report/src/could_settle_tests.rs` and `crates/sv-cli/src/mcp/explain_settle_tests.rs`.
