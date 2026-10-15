# Sweep 3: a probe that holds the evidence to finish sends the one more request, starting with the lockout and the anonymous token

**Status:** claimed by honesty, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026; the owner's choice the same day. The record is ADR-119. Severity:
medium.

**What to build.** After the lockout's push-back, try the account's real password once: if it signs in, report "after
the limit, the right password still signed in" and credit nothing for V6.3.1 (read its text first). Fetch the sign-in
page twice as two strangers and once more within one session, and report an anti-forgery token that is the same for
two strangers (V3.5.1, read its text first). Then read every probe under `signed_in/` for the same shape, a stop one
request short of what the requirement asks, and list them in the pull request with what the one more request would be
(built here when it is one request; filed as an item when it is more).

**First cases.** Backlog 0524 (the VS Code app verifies every guess and lets the right one in during the lockout) and
backlog 0537 (the Antigravity app's anonymous token is one value per process). Both close with this sweep.

**How to know it works.** Fake-app cases: a lockout that checks the password first; a token that is a constant. Before
the fix the credit stands; after it, the finding appears and the credit goes.

**Record.** ADR-119: a Later line if the one-more-request list changes what counts as evidence for a requirement the
decision does not name.

**First batch, built (the lockout, backlog 0524):** after a push-back by a refusal or another status, the account's own
password is sent once, straight after, under a `guess-` id: signed in, it is the finding "After the limit, the right
password still signs in" (high) and nothing is credited; refused, the credit stands in part and says so. A limit that
only slows attempts is not asked. 0524 is built by this, and left for the board to close.

**Second batch, built (the anonymous token, backlog 0537):** the sign-in page is asked for twice as two strangers,
before signing in; the same anti-forgery token both times is the finding `probe.csrf-token-shared` (V3.5.1, medium,
only ever a finding). The item's third request, once more within one session, is not sent: it would only matter to a
credit, and different tokens credit nothing. 0537 is built by this, and left for the board to close.

**The inventory (15 October 2026, honesty):** the probes under `signed_in/` that stop one request short, from a
read of every file. Items 1, 2, and 7 are checked against the code and by a test that fails without their fix; the
rest are from the read alone, and each is checked again when it is built:

1. `admin.rs` `owned_checks` (V8.2.2): the second user never shown signed in. One request. Built in the third batch.
2. `codes.rs` `email_code_guessing` (V6.6.3): the right code already sent after the guesses, and its answer ignored
   when the wrong ones were refused. No request. Built in the third batch.
3. `passwords.rs` `change_password_checks` (V6.2.3) and `change_email_check` (V7.5.1), `account_changes.rs`
   `one_change` (V7.5.1): the change never sent with the password field left out, only with a wrong one; an app that
   checks the password only when the field is there takes it. One request each.
4. `mod.rs` `run_checks` (V8.2.1): only the first private page is opened as A; the others are only known refused to a
   stranger, so a listed path that refuses everybody counts as private. One request per page.
5. `sessions.rs` `websocket_session_checks` (V4.4.3): the made-up session is built from the first cookie, which may
   not be the session cookie. One handshake.
6. `forgery.rs` `forgery_check` and `simple_request_check` (V3.5.1, V3.5.2): `Origin: null` without a token is never
   sent; an allow-list that takes `null` lets a sandboxed frame through. One request.
7. `flows.rs` `try_flow` (V2.3.1): the second user's fresh sign-ins never shown. One request each. Built in the
   third batch.
8. `uploads.rs` `download_name_checks` (V5.4.2): a name with a quote in it is not tried. Two requests (upload, fetch).

More than two requests, to be filed as items of their own: the wrong-password check with no `signup` (the right
password needs A, about three); the burst limit per session or per user (about three); the once check's solo try; a
token from another session sent with A's cookies (two or three); an SVG named `.gif` and fetched back (two or more).

**Third batch, built (inventory items 1, 2, and 7):** the second user shown signed in before the other-users'-data and
skipped-step checks read refusals as theirs; and the wrong-code check's right code, already sent, read: taken after a
limit that refused the wrong ones, it is the finding "After the limit, the right emailed code still signs in".
