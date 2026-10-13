# A "not verified" row never says what would verify it, though sv knows and the website's example promises it

**Status:** open

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

**The owner's decision, 13 October 2026** (asked by session stackvet-deep-review): yes, build it. The third answer's word must not be `owner`, which today confirms the tool's word; a confirmation stays shown as the tool's word confirmed by a person, never as the owner's own (the owner's rule of 27 September 2026, 0115). The record is a Later entry on ADR-026, proposed in the claim and accepted in the pull request that builds it.
