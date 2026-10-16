# V7.1.1: the session timeout the code sets, read beside the policy's, and a disagreement reported (the rest of 0492)

**Status:** open

Split from backlog 0492 on 16 October 2026: 0492 built the two ways of ending sessions from the running app (V7.5.2,
V7.4.5, and V7.1.2's observation, ADR-125); this is the part it left, which reads the code instead.

**What to build** (0492's own words for V7.1.1, L2, idle timeout and absolute lifetime documented and appropriate):
`[policy] idle-timeout-minutes` and `session-lifetime-minutes` already exist and are what `sv run --slow` holds the app
to (V7.3.1, V7.3.2). Route 1, finding-only, the `resolve` pattern: a locator for the session configuration (`maxAge`,
`cookie.expires`, `SESSION_COOKIE_AGE`, `PERMANENT_SESSION_LIFETIME`, `expire_after`, `session.timeout`, Devise
`timeout_in`, `rememberFor`) and, when the number read disagrees with the policy number,
`config.session-timeout-differs-from-policy` citing V7.1.1 ("the documented timeout is 15 minutes; the code sets 7 days
at `config/session.js:9`"). Agreement credits nothing (the documentation half is the owner's). Route 4: the same lines
shown beside the notes question when no policy number is set. Read V7.1.1's text again before citing it.
