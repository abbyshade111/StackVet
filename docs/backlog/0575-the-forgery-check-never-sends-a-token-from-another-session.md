# The forgery check never sends a token from another session with the signed-in user's cookies

**Status:** claimed by documentation, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the inventory of sweep 3 (backlog 0561, ADR-119); two or three requests, filed rather than built there.

**What is wrong.** The forgery check (V3.5.1) sends no token at all. An app that checks a token is present and well
formed, but not that it is the session's own, refuses that and is credited, though another site can fetch a token of
its own and send it.

**What to build.** Fetch a token in another session (a stranger's page, or B's) and send the forged create with it and
A's cookies; taken, the token is not tied to the session. The anonymous-token check (0537) is the sign-in form's half
of this.

**Done, 15 October 2026** (documentation). The first create request refused without a token is sent again from
another site with a token a stranger was handed on the sign-in page (`forgery.rs`, `stranger_token`, `OtherToken`),
in the request's own fields. Taken, it is the forgery finding; refused, the credit says so; with no token to be had
from the sign-in page, the credit is in part and says it was not shown. Two requests. The scripted app now takes a
sign-in page's token only from its own session, with `csrf_token_any_session` for one that takes it from any, relying
on the token alone. ADR-119, Later. Tests: `forgery/other_token_tests.rs` (3); with the try switched off, all three
fail.
