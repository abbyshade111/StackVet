# The refusals locator shows the first 403 in a file, which in notes-with-users refuses a forged form, not a person

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Filed 14 October 2026 by honesty, from the second reading of 0516. `locate.refusals` (0516) finds every 403 an app
sends, and a question shows the first few in file order. In `notes-with-users` the first is `app.py:133`, the refusal
of a form whose anti-forgery token is wrong, so V8.1.1 and V8.3.1 show that line, which is about forged requests and
not about what a person may do. The admin page's refusal at `:209` and `:210`, the real check, is found but not shown:
a question shows three places, one from each locator first, and the owner query at `:241` and the next owner line
take the rest. The sentence beside them says they are where to look, not evidence, but this one points at the wrong
kind of check.

**What to do:** order a locator's places by how near they are to what the question is about before file order, or
leave out a 403 whose condition is about a token or a password, by shape (a call or comparison whose names say
`csrf`, `forged`, `token`, or `password` in the `if` that leads to it). Hold it to `notes-with-users`: V8.3.1 must
show `:209` or `:210`, and not `:133`. Where to look, never evidence, as now (ADR-022).

**Built 14 October 2026, by honesty**
(`docs/design/0388-a-refusal-for-a-forged-form-or-a-wrong-password-is-not.md`). A locator may say `unless`, and
`locate.refusals` leaves out a 403 whose nearest `if` names a token, a forged form, or a password. On
`notes-with-users`, V8.1.1 and V8.3.1 now show `app.py:241`, `:209`, and `:316`, and never `:133`; the other
four example apps still get no pointer. Ordering by nearness was not built: leaving out the other checks was enough.
The pointer's second reading can go ahead.
