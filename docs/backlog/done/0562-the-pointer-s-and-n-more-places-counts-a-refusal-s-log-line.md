# The pointer's "and N more places" counts a refusal's log line and its response line as two places

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the pointer's second reading (backlog 0516, 14 October 2026 evening) on `examples/notes-with-users`. Severity: low.

**What is wrong.** V8.1.1 and V8.3.1 show three places (app.py 241, 209, 316) "and 2 more places like these"; one of
the two is app.py:210, the `send(403, ...)` right under the `event(..., status=403)` at 209 that is already shown:
one refusal counted twice. (Checked by changing line 210 in a copy: the count fell to 1.) The other was not
identified; the six forged-form and wrong-password refusals are correctly left out.

**Fix.** Collapse places within two lines of each other in the same file into one, keeping the first; and say in the
pull request what the remaining "more" place is.

**Record.** None.

**Done, 15 October 2026** (documentation). The places a question points at are now merged per question before any
is shown or counted: a place within two lines of one already taken, in the same file, is the same place, and the first
is kept (`point_questions` in `crates/sv-cli/src/code_lines.rs`). This holds across locators too. On
`examples/notes-with-users`, V8.1.1 and V8.3.1 still show app.py 241, 209, and 316, and now say "and 1 more place"
where they said 2: app.py:210, the 403 answer under the log line at 209, is no longer counted. **The remaining more
place** is app.py:336, `update notes set owner = ? where owner = ?`, which moves a person's notes to their new email
address: a query by owner, rightly counted. Read from the places each locator found (owner checks at 241, 316, 336;
refusals at 209, 210). Not merged: places past each locator's own cap (`locate::MOST_PLACES`), which are counted
without their lines. Test: `crates/sv-cli/tests/pointer_same_place.rs` (2), with the example app as the item's own
case; with the distance set to 0 both went red.
