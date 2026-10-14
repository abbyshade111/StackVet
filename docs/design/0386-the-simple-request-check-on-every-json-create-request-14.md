# The simple-request check on every JSON create request (14 October 2026)


Backlog 0232, part 3 (group C of the one-sample inventory, in part), and ADR-053, "Later, 14 October 2026: the
simple-request check on every JSON create request".

**The problem.** `simple_request_check` sent the `owned` create request's fields from another site in the three forms a
browser sends without asking the app first (`text/plain`, a form, and multipart), and credited V3.5.2 in part when all
three were refused. V3.5.2 asks it of sensitive functionality, which a second create route may handle with other code.
The cross-site check (V3.5.1) already sent every create request named (0371); this one did not.

**What was built.** Every JSON create request stackvet.toml names is sent the same way: the `owned` create
(`simple-request-…`) and each `creates` entry (`simple-request-1-…` and on). A form create request is left out, as the
`owned` one always was: a browser sends a form across sites without asking first anyway, so no preflight is relied on,
and whether it is refused is V3.5.1's question. Each `creates` entry is first sent as the app's own page would send it,
and judged only when that was accepted, as the cross-site check does since 0420. A form taken at any request is a
finding naming each path and the forms it took; the credit names every path refused in all three. It is whole only when
two or more requests were refused, none was left unjudged (its control not accepted, or an answer that neither took nor
refused it), and the manifest names no changing request the check does not send (`not_forged`, shared with the
cross-site check); otherwise it is in part and names what was not sent or not judged. When no create request is JSON,
nothing is judged, as before.

**The stand-in app.** The JSON API answers at `/api/comments` as at `/api/notes`, keeping comments apart, and the new
flaw `api_comments_take_forms` leaves only the comments open to forms.

**Tests.** `crates/sv-check/src/signed_in/forgery/simple_request_tests.rs` (seven); the earlier simple-request tests
in `forgery.rs` pass unchanged.

**Broken on purpose, each put back:** only the `owned` create sent (3 red); whole with a changing request not sent or a
request not judged (1 red, and 1 more once a test with two judged and one unjudged was added for the second
condition); no control for a `creates` entry (1 red).
