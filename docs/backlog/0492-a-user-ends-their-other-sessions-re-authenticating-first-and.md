# A user ends their other sessions, re-authenticating first, and an admin ends a user's, tried from the manifest

**Status:** open

From the deep review of 13 October 2026 (backlog 0417), area: coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2). A proposal, not a fault: how these could be assessed, in the order of preference the owner set (a static read; the running app behind the fence; an outside tool already adapted; showing the person the lines of their own code with a plain question; a hand check with a reason), each at its honest tier (ADR-022, ADR-050). Written by a read-only reviewer and read, not built, by the reviewing session; a requirement's text must be read again before any citation is written (a citation is a claim). The cross-cutting item it assumes is "A question to the owner carries the lines it rests on".

**Section: V7 Session Management (V7.1.1, V7.1.2, V7.4.5, V7.5.2)**

- **V7.1.1** (L2), idle timeout and absolute lifetime documented and appropriate. Notes question exists, and
  `[policy] idle-timeout-minutes` and `session-lifetime-minutes` already exist in `stackvet.toml` (`spec.rs:323-324`)
  and are what `sv run --slow` holds the app to (V7.3.1, V7.3.2). Route 1, finding-only, the `resolve` pattern (a claim
  the code contradicts is reported): a locator for the session configuration (`maxAge`, `cookie.expires`,
  `SESSION_COOKIE_AGE`, `PERMANENT_SESSION_LIFETIME`, `expire_after`, `session.timeout`, Devise `timeout_in`,
  `rememberFor`) and, when the number read disagrees with the policy number, `config.session-timeout-differs-from-policy`
  citing V7.1.1 ("the documented timeout is 15 minutes; the code sets 7 days at `config/session.js:9`"). Agreement
  credits nothing (the documentation half is the owner's). Route 4: the same lines shown beside the notes question
  when no policy number is set. Tier: `sv`'s own (finding-only); *documented by the owner* for the documentation.
- **V7.1.2** (L2), how many parallel sessions are allowed, documented. Notes question exists. Route 2, as an
  observation: sign user A in twice and report whether the first session still works ("StackVet signed in twice; both
  sessions stayed valid"); no finding either way, since either is a legitimate design. No code to point at in most apps.
  Tier: *documented by the owner*, with the observation shown. Same as partial-checks.
- **V7.4.5** (L2), administrators can end a user's sessions. No question today. Route 2: an optional manifest entry
  `admin.end-sessions = { path = "/admin/users/{id}/sessions", form = {...} }`; the admin account ends A's sessions
  while A is signed in; A's next private request still answered is a finding citing V7.4.5; refused is *checked in part*
  ("one user; 'all users' not tried"). Tier: `sv`'s own, in part. Route 4 without the entry: the admin routes found
  (`/admin/...`) shown with "is there a control here that signs a user out everywhere?", *checked by hand*. Same as
  partial-checks; the manifest shape and the in-part scope are the addition.
- **V7.5.2** (L2), users can view and, after re-authenticating, end their own sessions. No question today. Route 2:
  `account.sessions = { path = "/account/sessions", ends = { path = "...", form = { password = "{password}", ... } } }`
  (the `{password}` placeholder is the account-change probes' pattern, `spec.rs:182-187`). A signs in twice; from the
  first session the sessions page should show two (the second's marker, if the page shows anything; otherwise skip the
  view half and say so); A ends "all other sessions"; the second session's next request still answered is a finding;
  refused is *checked* for the ending half. The re-authentication half: if the end action succeeds with the password
  field left out, `probe.sessions-ended-without-password` is a finding citing V7.5.2, exactly as
  `probe.email-change-without-password` does for V7.5.1. Tier: `sv`'s own; *checked in part* when the view half could
  not be read.
