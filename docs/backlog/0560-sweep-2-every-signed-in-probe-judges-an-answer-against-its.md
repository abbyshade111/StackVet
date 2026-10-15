# Sweep 2: every signed-in probe judges an answer against its control, never against a status threshold alone

**Status:** claimed by honesty, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026; the owner's choice the same day. The record is ADR-119. Severity:
high.

**What to build.** For every probe under `crates/sv-check/src/signed_in/` that has a control request in hand: accepted
means "answered the way the control did" (same status class, same redirect target or page shape); an answer that
differs from the control in a way the probe cannot read is "could not tell", and the step says what was compared.
Where a second request settles it and the manifest allows it (signing in as the account the broken value would have
made; reading the record back), make it. Start by listing every `status < 400`, `status == 200`, and `accepted()` read
in `signed_in/` and saying for each what the control is.

**First case.** Backlog 0523 (the VS Code trial: the control sign-up 302, the over-long email 200 with "Enter a valid
email address", read as accepted). It closes with this sweep.

**How to know it works.** A fake app that re-renders the form with 200 on a bad value and redirects on a good one: the
finding goes, and the step reads "could not tell: 200 where the control redirected" or, with the second request, the
right answer.

**Record.** ADR-119 (accepted by sweep 1's pull request; this one says `ADR-119: unchanged, because ...` or adds a Later
line if a rule is refined).

## The inventory (15 October 2026, honesty)

Every read under `signed_in/` that decides on a status class or threshold, and its control. Crashes and a limiter's
answers are already set aside per request (`RESTS_ON_A_REFUSAL`, `RAISED_ON_A_REFUSAL`), so this is about 2xx, 3xx,
and 4xx answers read wrongly.

**Already read against the control or settled by a second request:** the reads of a marker written and read back
(`admin.rs` `shows`, `holds_marker`, the change and its control; `uploads.rs` traversal, SVG, scan, and served
uploads; `owner_field.rs`; `stored_markup.rs`), the flows and once checks (the `completed` text), the WebSocket
handshakes (exactly 101), the account checks (`one_change` reads the value back), the emailed and TOTP codes (the
right code as control), whether sign-in or sign-up tells accounts apart (status and page shape), the open redirect
(3xx and a target off the app), and every check that confirms an account through `passwords::account_works`.

**Decided on status alone while a control of the same kind is in hand** (each is a batch of this sweep, in the order
a real app is likely to trip it):

1. `forgery.rs` `forgery_check` and `simple_request_check`: a create without the token, or as a simple request, read
   as `accepted` (any 2xx or 3xx); Flask-WTF shows the form again with 200, or redirects back to it. The control's
   answer is in hand for `creates`; reading the marker back settles both.
2. `uploads.rs` the oversized file and the content mismatch, and `refusal_stands`; `archives.rs` the archive check:
   a refusal flashed on a 200 page where the ordinary file was redirected.
3. `burst.rs` `burst_one`: a limit shown as a 200 flash or a redirect back to the form; counting the markers on the
   list page settles it.
4. `signin.rs` `brute_force_check` through `limit.rs` `past_the_limit`: status codes only, so a lockout shown on a 200
   page with other words is missed. Belongs with sweep 3 (0561), which tries the right password after the guesses.
5. `admin.rs` `admin_checks` and `role_field_check`, `email_role.rs`: the ordinary user's 200 beside the admin's 200,
   class only; a 403 template served with 200 is a finding. Compare the page's shape with the admin's.
6. `admin.rs` `owned_checks`, the delete: a 404 after B's change is read as deleted; A reading the record again just
   before B's delete settles it.
7. `signin.rs` `logout_check` and `sessions.rs` the WebSocket after sign-out: a sign-out refused on a 200 page passes
   as accepted.
8. `mod.rs` `run_checks`, a private page opened without signing in: a sign-in form served with 200 at the private
   address. A's own read of the page is the control; it is sent after and never compared.
9. `sessions.rs` `invented_session_check`: pages already served anonymously are asked again with an invented cookie.
10. `uploads.rs` `download_name_checks`: a 2xx or 3xx that is not the file is read as the download.
11. `sessions.rs` `identity_header_check`, idle and lifetime timeouts: class only beside A's read.
12. `admin.rs` `admin_action_checks` with no `check` page: no control is sent at all.
13. `sql.rs`: two conditions compared with each other, with no unmodified request as a baseline.

**First batch, built:** the shared comparison (`signed_in/control.rs`: the same status class, the same redirect
target, and for two successes JSON with the same names or no form where the control had none) and the browser-
validation probe (0523).

**Second batch, built (forgery and simple requests, list item 1):** a create sent as another site would send it is
taken when the record it made holds its marker when the signed-in user reads it back. A read-back is believed only
when the same way of reading found the control's own record (the `owned` record its owner read back; a `creates`
entry's control, read back the same way). When the record cannot be read back, the answer is read beside the
control's, and an answer unlike it is not judged. The simple-request check gives each of its three forms a marker of
its own, and judges a 3xx the same way rather than calling it unclear.

**Third batch, built (uploads and archives, list item 2):** the oversized file, the mismatched file, and each
compressed file past the limits are read beside the ordinary file's answer. Where `serves-at` found the ordinary GIF
by its name, the file is fetched back by its own name: there, it was kept; not found, it was not. An answer unlike the
ordinary file's that nothing fetched back settles is not assessed and says what was compared. `refusal_stands` counts
the GIF sent after a refusal as taken only when it was answered as the first ordinary GIF was, so a quota shown on a
200 page no longer lets a refusal stand. Still open from item 2's note: the first ordinary GIF itself is taken on
"under 400" (a 200 re-render of the form there would pass the gate), which a `serves-at` fetch could settle.

**Fourth batch, built (the burst, list item 3):** a burst's answers are read beside its first, and only one answered
as the first was counts as going through; a 429, or 503 with `Retry-After`, is still read as the limit. Where its
answer says where the first record went, that record is read back first, and a burst whose first record is not
there shows nothing either way. A limit shown on a 200 page or as a redirect elsewhere is not assessed, and says what
was compared.

**Fifth batch, built (admin pages, part of list item 5):** an ordinary user's 200 for an admin page is read beside
the admin's own page there (`control::shows_page`: half of its pieces of text or more): the page shown is a finding as
before; a 200 page showing little of it (a 403 template served with 200) is neither a finding nor a refusal, says so,
and keeps the credit from resting on the pages that were judged. Without the admin's page, a 2xx is a finding as
before. Still open from item 5: the role written into sign-up and into the email change, which have no admin page of
their own to read beside, since they do not sign the admin in.

**Sixth batch, built (sign-out, part of list item 7):** a sign-out answered with a page (200), or with a redirect
straight back to a private page, after which the browser's own session still opens that page, is not judged: it may
be a sign-out refused on a page of its own. A sign-out that sends the browser elsewhere, or takes its cookie away, is
done, and the old session is asked again as before. Still open from item 7: the WebSocket after sign-out
(`sessions.rs`), which reads its own sign-out the same old way.

**Seventh batch, built (a private page asked without signing in, list item 8):** each private page served with a 2xx
to somebody not signed in is read beside the same page served to the first test user once signed in
(`control::shows_page`): the page shown is a finding as before; a page showing little of it (a sign-in form served
with 200) is neither a finding nor a refusal, says so, and is not credited. Where the user's own page is not in hand
(signing in got no answer, or the page did not open for the user), a 2xx is a finding as before.

**Eighth batch, built (another user's delete, list item 6):** the first user reads the record just before the second
user's delete is sent, and a 404 or 410 afterwards is the delete only when the record was there then; already gone (a
change above sent it away), the delete is not judged, and says so. A crash of that read withholds the credit, as the
table of answers a credit rests on says (`RESTS_ON_A_REFUSAL`, ADR-021).

**Ninth batch, built (the made-up session, list item 9):** a private page that opened to somebody not signed in is
not asked with a made-up session (it would open, and be blamed on the session), and says so in a step; a made-up
session answered with a page that shows little of the real session's (a sign-in form served with 200) is neither the
page opening nor a refusal, says so, and leaves the credit in part. `control::shows_page` now compares only the words
between tags, so two pages built from the same `<form>` and `<button>` are not taken for one, and counts the same
body, even an empty one, as the same page; the admin-page and private-page checks that use it are unchanged in their
tests.

**Tenth batch, built (the download name, list item 10):** a file fetched back to read the name it is served under is
the file only when the answer is a 2xx holding its own contents; a redirect to where it is kept, or a page of the
app's own, is not assessed and says so, for the ordinary file (V5.4.1) and the file with the hostile name (V5.4.2).

**Eleventh batch, built (the identity header, part of list item 11):** each private page asked for with a header that
names a user, and no session, is read beside the same page served to the signed-in user: the page shown is a finding
as before; a 200 showing little of it ("unknown user", a sign-in form served with 200) is neither a finding nor a
refusal, and says so. Without the user's page, a 2xx is a finding as before. Still open from item 11: the idle and
lifetime timeouts, read by status class beside the busy and fresh sessions.

**Twelfth batch, built (an admin action with no `check` page, list item 12):** the ordinary user's success status is
read beside the admin's own same request, sent as the control: answered alike, it is the finding by the status
alone, at medium confidence, as before; answered unlike (a refusal shown on a 200 page where the admin was sent on)
it is not judged, says so, and keeps the credit from resting on the actions that were. When the admin's own request
was not taken either, there is nothing to read it beside, and it is read as before. The existing test that held a
refusal answered 200 to be a status-only finding now holds it not judged, and a case answered alike keeps the
status-only finding covered.

