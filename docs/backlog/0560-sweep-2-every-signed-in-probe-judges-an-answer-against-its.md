# Sweep 2: every signed-in probe judges an answer against its control, never against a status threshold alone

**Status:** claimed by honesty, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026; the owner's choice the same day. The record is ADR-119. Severity:
high.

**What to build.** For every probe under `crates/sv-check/src/signed_in/` that has a control request in hand: accepted
means "answered the way the control did" (same status class, same redirect target or page shape); an answer that
differs from the control in a way the probe cannot read is "could not tell", and the step says what was compared.
Where a second request settles it and the manifest allows it (signing in as the account the broken value would have
made; reading the record back), make it. Start by listing every `status < 400`, `status == 200`, and `accepted()` read
in `signed_in/` and saying for each what the control is.

**First case.** Backlog 0523 (the VS Code trial: the control sign-up 302, the over-long email 200 with "Enter a valid
email address", read as accepted). It closes with this sweep.

**How to know it works.** A fake app that re-renders the form with 200 on a bad value and redirects on a good one: the
finding goes, and the step reads "could not tell: 200 where the control redirected" or, with the second request, the
right answer.

**Record.** ADR-119 (accepted by sweep 1's pull request; this one says `ADR-119: unchanged, because ...` or adds a Later
line if a rule is refined).
