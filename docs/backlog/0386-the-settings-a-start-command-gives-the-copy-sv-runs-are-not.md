# The settings a start command gives the copy sv runs are not shown in the report

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the second loop trial (backlog 0257, `docs/prompts/loop-2/results.md`), 13 October 2026.

Twelve of the 90 builds scored (the trial's 70 and the drafts trial's 20 controls) gave the copy `sv` runs settings through their `start` command that the shipped app would not have:
sign-in limits raised (0383), and in four builds the session cookie's `Secure` flag turned off
(`CLUB_SECURE_COOKIES=0`, `CLUB_INSECURE_COOKIES=1`, `COOKIE_INSECURE=1`). The report says nothing of them, so an owner reads findings and credits
about a copy that differs from what they ship. A line in the report naming each setting the start command gives
(names only, never values that could be secrets), beside the checks it could affect, would let the owner see it.
