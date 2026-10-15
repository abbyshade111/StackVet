# A review's seal is bound to the app folder it was made in, so a checkout elsewhere (CI, a moved folder) loses the owner's confirmations

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From today's like-for-like rerun of the Opus app (backlog 0577), 15 October 2026: a question for the owner, not a
fault. The rerun on a copy of the app reported each confirmation as "sealed through sv review for an app in another
folder, and a seal counts only in the app it was made for", with the remedy to run `sv review` again there.

**The choice.** (A) Keep it: a review belongs to one folder, and copying a folder means reviewing again (the honest line
is already there). (B) Bind the seal to the app's identity rather than its path (the manifest's name plus the repository
when there is one), so a checkout of the same app keeps its confirmations, with ADR-076's reasoning (the record travels
with the app) applied to the seal. The orchestrating session recommends (B) only if the paper or the person's workflow
needs reports from CI with the owner's confirmations counted; otherwise (A).

**Record.** The seal's record (ADR-026 or the one that names the app in seals) Later, if (B).
