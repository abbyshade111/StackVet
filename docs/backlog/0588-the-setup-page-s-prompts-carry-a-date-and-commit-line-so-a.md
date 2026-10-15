# The setup page's prompts carry a date and commit line, so a trial record says which text the tool read

**Status:** claimed by wording, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the Opus run's reading (backlog 0577), 15 October 2026. Severity: low, for the prompt library (0370).

**What was seen.** The prompts pasted in the Opus run match the page's text of 14 October, before 0545 to 0557 landed:
none of this week's rules (the audience asked, no `confirmed`, silencing includes rewriting, the `.gitignore` line, the
sandbox clause, "until I run sv review") was in what the tool read, which a reader could tell only by diffing the
pasted text against the page's history.

**Fix.** Each prompt block on the page ends with one quiet line, "StackVet setup prompts, <date>, <short commit>",
written by the test that holds the page to the guide, so a pasted prompt carries its version; the trial-reading
scripts read it.

**Record.** None.
