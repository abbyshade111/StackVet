# After sv review, writing the report again without --run overwrites the report that had the running-app evidence

**Status:** claimed by wording, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Found by the owner on 15 October 2026, running prompt 7 (`sv review`) in the follow-up model trial (backlog 0577): the
first time any trial reached the review. Severity: high (usability; the one step a person does by hand loses the
evidence they paid for).

**What is wrong.** The full report is made with `sv report --run`. `sv review` records the person's answers, and the
report must then be written again for them to count. Written again with plain `sv report`, it replaces
`stackvet-report/` wholesale, so the running-app checks, the test run, and the outside tools' results from the --run
report are gone, and the person either does the whole run twice or ends with a weaker report than before the review.

**Fix.** Three parts, the first the one that matters:
- `sv review` ends by writing the report again with the same options as the report it reviewed (the report records what
  it was made with: `run_record.inputs`, the tools that ran, `--run`, `--advisories`), saying "writing the report again
  with the options of the last one: --run ..." and running them; with `--no-report` to skip. The person never types
  the second command.
- `sv report` without `--run`, in a folder whose last report had `--run`, says so before writing: "the last report had
  the running-app checks; this one will not, and will replace it; add --run, or run sv review which keeps them", and
  goes on (never refuses: a person may want a quick one).
- The setup page's prompt 7 and the guide say which happens, in one sentence, so the person knows the review does not
  cost a second run.
The owner's two alternatives (default `--run` when committing review input; or order the review before the --run
report) are weaker: the first makes every review a run, and the second asks a person to answer questions before
seeing the report that raises them.

**Record.** A Later entry on the record that governs `sv review` and what it writes (ADR-026 or the review record;
check), since `sv review` would start a run.
