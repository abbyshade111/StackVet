# The open-redirect rule flags a redirect to a fixed address

**Status:** claimed by honesty, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the second loop trial (backlog 0257, `docs/prompts/loop-2/results.md`), 13 October 2026.

`ast.open-redirect` (medium, low confidence) flagged `haiku-b2-9`'s `app.py:289`, `redirect(req, "/login")`: a redirect
to a fixed address, through the app's own helper `redirect(req, location)`. The rule seems to read the helper's
`location` parameter as the destination "built from a value". A call whose argument is a string literal is not one; the
rule should follow the argument at the call, not the parameter inside the helper. One of the ten findings read by hand.
