# Every sv sign-up was refused by the app's own username rule, and the gap quotes nothing: nine requirements stopped

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the clean Cursor + Flash run of 15 October 2026 (backlog 0577). Severity: medium (the same family as 0527: the
one line a person needs).

**What happened.** The app limits usernames to 3 to 30 characters; sv's stand-in names are 38-character addresses, so
every sign-up answered with "Username must be between 3 and 30 characters", and nine requirements stopped with a gap
that does not say why.

**Fix.** Quote the page's visible refusal in the gap (as 0527 does for sign-in); and let the stand-in names fit a stated
rule: when the manifest's sign-up form names a `username` field, sv makes a short name (a `{user}` under 20
characters) and keeps the address for the email field. Fake-app case: a sign-up that refuses long names.

**Record.** None.
