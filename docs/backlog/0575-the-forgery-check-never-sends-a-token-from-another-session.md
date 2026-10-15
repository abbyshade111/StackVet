# The forgery check never sends a token from another session with the signed-in user's cookies

**Status:** open

From the inventory of sweep 3 (backlog 0561, ADR-119); two or three requests, filed rather than built there.

**What is wrong.** The forgery check (V3.5.1) sends no token at all. An app that checks a token is present and well
formed, but not that it is the session's own, refuses that and is credited, though another site can fetch a token of
its own and send it.

**What to build.** Fetch a token in another session (a stranger's page, or B's) and send the forged create with it and
A's cookies; taken, the token is not tied to the session. The anonymous-token check (0537) is the sign-in form's half
of this.
