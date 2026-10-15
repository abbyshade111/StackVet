# Sweep 3: a probe that holds the evidence to finish sends the one more request, starting with the lockout and the anonymous token

**Status:** claimed by honesty, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026; the owner's choice the same day. The record is ADR-119. Severity:
medium.

**What to build.** After the lockout's push-back, try the account's real password once: if it signs in, report "after
the limit, the right password still signed in" and credit nothing for V6.3.1 (read its text first). Fetch the sign-in
page twice as two strangers and once more within one session, and report an anti-forgery token that is the same for
two strangers (V3.5.1, read its text first). Then read every probe under `signed_in/` for the same shape, a stop one
request short of what the requirement asks, and list them in the pull request with what the one more request would be
(built here when it is one request; filed as an item when it is more).

**First cases.** Backlog 0524 (the VS Code app verifies every guess and lets the right one in during the lockout) and
backlog 0537 (the Antigravity app's anonymous token is one value per process). Both close with this sweep.

**How to know it works.** Fake-app cases: a lockout that checks the password first; a token that is a constant. Before
the fix the credit stands; after it, the finding appears and the credit goes.

**Record.** ADR-119: a Later line if the one-more-request list changes what counts as evidence for a requirement the
decision does not name.
