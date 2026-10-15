# More samples for the checks that rest on one sample (groups A to H)

**Status:** claimed by honesty, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

The owner's word, 10 October 2026, asked with the three other open decisions of that day: "go with your recommendation and add to the backlog to add more checks to each of these."

The running checks that rest on one sample are being marked *checked in part* (ADR-053, Later, accepted the same
day; backlog 0006 part 4). This item is the other half: try more of each, so a check can earn plain *checked* again
by speaking for what its requirement names. One part per group of the inventory in
`docs/design/0359-one-action-one-race-one-redirect-one-svg-checked-in-part-10.md`; each is claimed on its own.

1. **A. Every response, not one or two.** Headers, cookies, content types, and error pages asked of every page the
   app answers and every route `stackvet.toml` names, not one or two anonymous responses.
   **Record, `Status: proposed`: ADR-096.** What group A asks, when its credit is whole, and what stays out.
   **Done the same day** (ADR-096; `docs/design/0369-group-a-the-header-checks-ask-every-route-stackvet-toml.md`):
   each route stackvet.toml names is asked as a stranger, and the header, cookie, and content-type credits are whole
   only when every one answered and passed. The error-page check stays in part.
   **Part status:** done, 11 October 2026
2. **B. Every private page and session path.** Session handling tried on each private page the owner lists and
   after each way of signing in, not one.
   **Scope of this claim (12 October 2026):** the made-up session check (`probe.session-token-unverified`, V7.2.1,
   whose requirement says *all* session-token verification) tried on every private page the owner lists, not one;
   whole only when two or more were tried and every one refused. The other session credits marked in part (the
   cookies, renewal, and fixed session of one sign-in, and the one sign-out) wait for "each way of signing in", which
   needs the manifest to say what the ways are; idle timeout and lifetime were left out of the inventory as app-wide.
   Recorded as a Later entry on ADR-053 in the pull request that builds it.
   **Built 12 October 2026** (ADR-053, "Later, 12 October 2026";
   `docs/design/0370-the-made-up-session-on-every-listed-private-page-12-october.md`): the made-up session is tried on
   every listed private page, and its credit is whole only when two or more were tried and all refused.
   **Part status:** partly done: the cookies, renewal, and fixed session of one sign-in, and the one sign-out, wait for the manifest to name each way of signing in
3. **C. Each form, flow, and connection.** Cross-site requests tried on each changing route the app answers, each
   step of a flow, and each WebSocket path, not one request.
   **Scope of this claim (12 October 2026):** the cross-site check (`probe.cross-site-request-accepted`, V3.5.1,
   which asks it of requests to sensitive functionality) sent for every create request stackvet.toml names, the
   `owned` create and each `creates` entry, not one. Those alone, because a forged create that is accepted only makes
   a record, where a forged password or email change, account deletion, or admin action would change the test
   accounts for the rest of the run. So the credit is whole only when two or more were tried, all refused, and the
   manifest names no other changing request; otherwise in part, naming what was not tried. The simple-request and
   null-Origin checks, the flow, and the WebSocket are not in this claim. Recorded as a Later entry on ADR-053 in the
   pull request that builds it.
   **Built 12 October 2026** (ADR-053, "Later, 12 October 2026";
   `docs/design/0371-the-cross-site-check-on-every-create-request-12-october-2026.md`): every create request named is
   forged; whole only when two or more were refused and no other changing request is named.
   **Scope of the next claim (14 October 2026, by honesty):** the simple-request check (`probe.preflight-skipped`,
   V3.5.2, which asks it of sensitive functionality) sent for every JSON create request stackvet.toml names, the
   `owned` create and each `creates` entry, each `creates` entry beside a control as the cross-site check's are (0420).
   A form create request is left to V3.5.1, since no preflight guards a form. Whole only when two or more refused all
   three forms, none was left unjudged, and the manifest names no other changing request.
   **Built 14 October 2026** (ADR-053, "Later, 14 October 2026: the simple-request check on every JSON create
   request"; `docs/design/0386-the-simple-request-check-on-every-json-create-request-14.md`).
   **Scope of the next claim (14 October 2026, by honesty; the owner's word the same day):** several flows and
   WebSocket paths in stackvet.toml, `flows`, `private-websockets`, and `websockets` beside the single keys, each tried
   as the one is now, and each credit whole only when two or more were named, judged, and held.
   **Record, `Status: proposed`: ADR-118.** The null-Origin check is left out: it credits nothing (it reports an app
   refusing its own forms, which no requirement asks), so it has no sample to add.
   **Built 15 October 2026** (ADR-118, accepted; ADR-053, "Later, 15 October 2026: every flow and WebSocket path
   named"; `docs/design/0389-every-flow-and-websocket-path-named-15-october-2026.md`).
   **Part status:** done, 15 October 2026
4. **D. Each upload route, more than one file.** Every upload route the manifest names, with a small set of files
   per check rather than one.
   **Scope of this claim (12 October 2026):** a small set of files for the two served-file checks, on the one
   upload route the manifest can name: `.php`, `.phtml`, and `.php5` for `probe.uploaded-file-executed` (V5.3.1), and
   `.html`, `.htm`, and `.xhtml` for `probe.uploaded-file-rendered` (V3.2.1), each whole only when every file was
   accepted, fetched back with its mark, and judged safe. `.xhtml` served as `application/xhtml+xml` is rendered by a
   browser, so the render check counts that type as a page too. More than one upload route needs the manifest's
   `upload` to become a list, which is not in this claim; nor are the checks about one specific file (the size, the
   antivirus test file, the SVG) or the archives. Recorded as a Later entry on ADR-053 in the pull request that
   builds it.
   **Built 12 October 2026** (ADR-053, "Later, 12 October 2026";
   `docs/design/0372-the-served-file-checks-judge-three-files-each-12-october-2026.md`): the two served-file checks judge
   three files each, whole only when all three are safe.
   **Part status:** partly done: more than one upload route (the manifest names one); the one-file checks and the archives
5. **E. Each kind of event.** Each event the requirement names made to happen and looked for in the log, not one
   line.
   **Scope of this claim (12 October 2026):** the authorization-failure log check (`probe.authorization-failure-logged`,
   V16.3.2) planted on every private page the owner lists, not the first: each page asked for by a stranger with a
   marker in its address, and the credit whole only when two or more pages were refused and every one's refusal was
   found in the app's output with its status. The markers are made so that none is part of another, so one line
   cannot stand for two pages. The other log credits (the sign-ins, the line's metadata, format, and timestamp) are
   not in this claim. Recorded as a Later entry on ADR-053 in the pull request that builds it.
   **Built 12 October 2026** (ADR-053, "Later, 12 October 2026";
   `docs/design/0373-the-authorization-failure-log-check-on-every-listed-page-12-october.md`): a marker on every listed
   private page, whole only when two or more were refused and each refusal was found with its status.
   **Scope of the next claim (15 October 2026, by honesty):** the sign-in log check (`probe.authentication-logged`,
   V16.3.1, which asks that *all* authentication operations be logged, successful and unsuccessful). Each way of
   signing in stackvet.toml names (the password; `email-code`; `totp`) is made to succeed once and fail once, each
   between two marked requests, and looked for in the app's output. The password's failure is tried for a real
   account (a wrong password for the account the check made) as well as for a name no account has. The credit is
   whole only when every one of these was found and the manifest names no other authentication operation the check
   did not try (a password change, a reset, or a change to the account's sign-in details), which mirrors part C;
   otherwise it is in part and names what was not tried. The line's metadata, format, and timestamp are not in this
   claim. **Record, `Status: proposed`:** a Later entry on ADR-053, accepted in the pull request that builds it.
   **Built, first batch, 15 October 2026 (honesty)** (ADR-053, "Later, 15 October 2026: the sign-ins in the log";
   `docs/design/0390-the-sign-ins-in-the-log-whole-when-every-one-is-found-15.md`): the password-guessing check's
   first wrong password, at an account that exists, is bracketed and read, and the credit is whole when all three
   password sign-ins were found and stackvet.toml names no other way to authenticate. Next in this claim: the
   emailed-code and two-factor sign-ins.
   **Part status:** partly done: the emailed-code and two-factor sign-ins, and the line's metadata, format, and timestamp
6. **F. Each browser page and form.** The browser-driven checks run on each page and form the owner lists.
   **Scope of this claim (12 October 2026):** the typed-text check (`probe.text-rendered-as-markup`, V3.2.2) on every
   text form the owner names. **Record, `Status: proposed`: ADR-097**, since it adds a key to stackvet.toml.
   **Built 12 October 2026** (ADR-097; `docs/design/0375-the-typed-text-check-on-every-text-form-12-october-2026.md`):
   `browser.text-forms` names several text forms; each is typed into, and the credit is whole only when every one is
   shown as text.
   **Part status:** partly done: the storage check after signing out (one sign-in, one sign-out)
7. **G. Each operation the requirement names.** Phone, two-factor, and recovery changes beside the email change
   (V7.5.1); password change beside sign-up for the password checks; both TOTP and an emailed code where both
   exist (V6.5.1, V6.5.5); one manifest key per operation.
   **Assessed 12 October 2026, not built:** the phone, two-factor, and recovery changes each need a key of their own in
   stackvet.toml for features few apps have, a change of format for a decision of its own; and the password checks
   tried at a password change as well as sign-up stay in part either way, since ADR-090 keeps every password rule probe
   in part (one password is one sample of every password). Left open for the owner to weigh.
   **Built 12 October 2026, at the owner's word the same day (ADR-105):** `change-phone`, `change-two-factor`, and
   `change-recovery` beside `change-email`, each tried with a wrong password and then the right one on an account made
   through `signup`, seen on the page `shows` names; and `not-offered`, confirmed by the owner in `sv review`. V7.5.1 is
   whole only when the email change and each of the other three is refused or confirmed not offered. The password
   change beside sign-up, and TOTP beside an emailed code, stay out, as the owner agreed (ADR-090).
   **Part status:** done, 12 October 2026
8. **H. More than one listed page.** The owner is asked, where only one private or admin page is listed, to list
   more; `sv init` and the prompts say why.
   **Scope of this claim (12 October 2026), taken before F and G:** F and G each need the manifest to name more than
   it can now (several text forms; phone, two-factor, and recovery changes), which is a change of format with a
   decision of its own, while H is wording, and is what groups B and E now rest on: their credits are whole only when
   two or more pages are listed. `sv preflight` says, when only one private page or one admin page is listed, that the
   checks asking each listed page can then only credit in part, and asks for more; the spec `sv init` writes says the
   same beside `private` and `admin`. No check changes.
   **Done the same day** (`docs/design/0374-ask-for-more-than-one-private-page-12-october-2026.md`): `sv preflight`
   says when one private page is listed and why it caps the credits; the spec says to list every private and admin page.
   **Part status:** done, 12 October 2026

**Claim released, 14 October 2026** (the owner's word): session securevibe-trial-prep was archived after the coordination trial with no branch pushed for this item, so the claim was stale. Open again, queued on the board.
