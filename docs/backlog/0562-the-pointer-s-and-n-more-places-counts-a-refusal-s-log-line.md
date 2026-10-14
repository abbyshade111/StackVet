# The pointer's "and N more places" counts a refusal's log line and its response line as two places

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the pointer's second reading (backlog 0516, 14 October 2026 evening) on `examples/notes-with-users`. Severity: low.

**What is wrong.** V8.1.1 and V8.3.1 show three places (app.py 241, 209, 316) "and 2 more places like these"; one of
the two is app.py:210, the `send(403, ...)` right under the `event(..., status=403)` at 209 that is already shown:
one refusal counted twice. (Checked by changing line 210 in a copy: the count fell to 1.) The other was not
identified; the six forged-form and wrong-password refusals are correctly left out.

**Fix.** Collapse places within two lines of each other in the same file into one, keeping the first; and say in the
pull request what the remaining "more" place is.

**Record.** None.
