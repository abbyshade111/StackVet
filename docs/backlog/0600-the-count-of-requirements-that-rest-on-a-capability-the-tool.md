# The count of requirements that rest on a capability the tool answered `false` is not shown, and this run had thirteen such answers

**Status:** claimed by documentation, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the clean Cursor + Flash run of 15 October 2026 (backlog 0577). Severity: medium (honesty of the summary).

**What happened.** The tool answered `false` on 13 capability lines it did not check (the 14 October text does not ask
it to confirm them); not assessed fell to 2 and not applicable rose to 79 on the manifest's word. The summary's counts
show neither how many "does not apply" rest on an unverified `false` nor which.

**Fix.** A line in the summary, "N requirements do not apply because stackvet.toml says the app has no X, Y, Z; sv did
not check those answers", and in the applicability table the `by` of each capability answer beside it (0580's
proposals mechanism gives a way to confirm them in sv review).

**Record.** ADR-117 Later (how an unanswered or tool-answered capability is read).
