# The report's opening: one count of things to fix, no "worst first" in the headline, and the build record broken up

**Status:** done, 22 October 2026

Found by the owner on 22 October 2026, reading `report.html` of the Cursor build made for the walk-through (backlog
0373), held to level 3, with `--run`. The owner: "The worst first doesn't make sense and the number of things to fix
is different there than further down. The huge wall of text right under the initial BLUF statement needs to be broken
up to make it more readable."

**What the opening says now, and why it reads wrong** (`crates/sv-report/src/bluf.rs`, `html.rs`, `build_loop.rs`):

- **Three numbers for one list.** The headline says "19 things were found, worst first. 9 of them are in test or
   sample code, or from rules written for a framework this app does not use, listed after the app's own." The list
   under it ends "… and 14 more, in security.md, worst first" (19 less the 5 it names). "What to do next" says "Fix
   the 10 things that were found", and the section below is "10 things to fix". The 10 is the app's own; the 19 counts
   findings the report itself sets apart; the 14 is neither.
- **"Worst first" in the headline** describes the order of a list that comes several paragraphs later, after the
   build record, so it reads as a claim about nothing in sight.
- **Rule ids in the headline:** "48 things in test code are information, not counted as findings: bandit.B101,
   bandit.B105." The ids mean nothing to the people the report is for.
- **The wall of text:** right under the headline, the build-loop record (ADR-076, ADR-084) is one paragraph of about
   220 words: how many times the AI tool called sv, the first and last check's counts, what changed between them, the
   sections it asked about, the tool's own name, the hash chain, and why the chain cannot catch a rewrite, then where
   the record is kept.
- **The same counts twice:** the bars of what stands behind each requirement, and right under them "Of the
   requirements that apply to this app" with the same seven numbers as a list.

**To do** (the honesty rules at the top of `bluf.rs` hold throughout: no score, nothing reads as a pass, and nothing
left out is hidden; a number moves, it is never dropped):

- **One count of things to fix, the same everywhere.** The headline leads with the app's own ("10 things to fix in
  the app's own code."), then, as a sentence of its own, the ones set apart ("9 more are in test or sample code, or
  come from rules for a framework the app does not use; they are listed after."). The named list says "… and N more
  of the app's own, in fix-now.md", with N from the app's own, and "What to do next" and the section heading keep the
  same number. "Most serious first" goes with the list it describes, not the headline.
- **No rule ids in the headline:** "48 notes on test code are information, not findings; information.md lists them."
  The ids stay in information.md and security.md.
- **The build record in short lines, details folded away.** Its first lines say what a person needs: whether StackVet
  was used while the app was built, how many checks, and what changed between the first and the last. The tool's
  name, the sections it asked about, the hash, and how the record is kept go under a closed "How this record is kept"
  (`<details>` in report.html; a paragraph of its own in the Markdown files). Every fact it says today is still there.
- **The tally once:** the bars' legend already carries every count with its words, so the list under it goes from
  report.html, or the two are made one. The Markdown files, which have no bars, keep the list.
- Tests: the headline's count, the list's "more", "What to do next", and the section heading agree on a report with
  findings set apart (the walk-through build's shape); no rule id in the headline; the build record's first block is
  short, and every fact the old paragraph held is in the report. `bluf.rs`'s tests that pin "worst first" change with
  it.

Its record is a Later entry on ADR-084 (what the report says of the build record), in the pull request that builds it.

**Done, 22 October 2026** (ADR-084, Later). The headline now leads with the app's own things to fix ("9 things to fix
were found in the app's own code."), the same number as "What to do next" and the section heading, and says what is
listed apart in a sentence of its own; the list of the most serious follows it at once under "Most serious first:",
its "… and N more" counted from the same things; no rule ids in the headline; the build record's paragraph is in two
parts, the details folded under "How this record is kept, and what else it says" in report.html, with readable times;
and report.html lists the counts under its bars only when there are no bars. Seen on a copy of the owner's build, made
without `--run`, in report.html and compliance.md. Tests: `bluf.rs` (one count from the headline to what to do next,
no rule in the headline), `build_loop_span_tests.rs`, `html.rs` (the counts not listed twice), and the report tests
whose wording changed.
