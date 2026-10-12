# More samples for the checks that rest on one sample (groups A to H)

**Status:** partly done: groups B, C, and D partly done; groups E to H open

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
   **Part status:** partly done: the simple-request and null-Origin checks, the flow, and the WebSocket
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
   **Part status:** claimed by backlog-org-0232, 12 October 2026
6. **F. Each browser page and form.** The browser-driven checks run on each page and form the owner lists.
   **Part status:** open
7. **G. Each operation the requirement names.** Phone, two-factor, and recovery changes beside the email change
   (V7.5.1); password change beside sign-up for the password checks; both TOTP and an emailed code where both
   exist (V6.5.1, V6.5.5); one manifest key per operation.
   **Part status:** open
8. **H. More than one listed page.** The owner is asked, where only one private or admin page is listed, to list
   more; `sv init` and the prompts say why.
   **Part status:** open
