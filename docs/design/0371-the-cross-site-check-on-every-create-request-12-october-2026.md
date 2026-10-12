# The cross-site check on every create request (12 October 2026)

Backlog 0232, part 3 (group C of the one-sample inventory, in part), and ADR-053, "Later, 12 October 2026" (the second).

**The problem.** `forgery_check` sent one forged request, the `owned` create, with the signed-in user's cookies, a
foreign Origin and Referer, and no anti-forgery token, and credited V3.5.1 in part when it was refused. V3.5.1 asks it
of requests to sensitive functionality, which a second create route may handle with other code.

**What was built.** Every create request stackvet.toml names is forged the same way: the `owned` create (`forged-create`)
and each `creates` entry (`forged-create-1` and on). An accepted one is a finding naming each route accepted; the
credit names every route refused. Changing requests that are not creates are not forged (`not_forged`): the password
and email changes, account deletion, the upload, the action done once, the admin actions, and changing or deleting a
record. The credit is whole only when two or more creates were refused and none of those is named; otherwise in part,
naming them as "not sent this way". `RESTS_ON_A_REFUSAL` lists `forged-create-` too, so a crash at a later create is
never read as a refusal (ADR-021).

**The stand-in app.** `/comments` now refuses a request whose Origin is another site, as `/notes` does, unless the new
flaw `comments_take_foreign` is set.

**Tests.** `crates/sv-check/src/signed_in/forgery/creates_tests.rs` (four), and a crash scenario in
`crates/sv-check/src/signed_in/mod.rs` with a second create taken from another site.

**Broken on purpose, each put back:** only the `owned` create forged (2 red); whole from one request (1 red); whole with
a changing request not forged (1 red); the crash guard missing the new ids (1 red, the crash sweep).
