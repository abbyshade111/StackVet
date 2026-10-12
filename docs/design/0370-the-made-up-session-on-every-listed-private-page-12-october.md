# The made-up session on every listed private page (12 October 2026)


Backlog 0232, part 2 (group B of the one-sample inventory, in part), and ADR-053, "Later, 12 October 2026".

**The problem.** `probe.session-token-unverified` gave the session cookies set at sign-in made-up values of the same
length and asked one private page. V7.2.1 asks that *all* session-token verification happen on the server; one page
refusing says nothing of the next, which may be served by other code.

**What was built.** `invented_session_check` takes the private pages stackvet.toml lists (`users.private`), the
confirmed page first, leaving out a path with a placeholder. Each is asked with the real session as its control and
then with the made-up one. A page the made-up session opens is a finding naming it (evidence `invented-session`, then
`invented-session-1` and on); the credit names every page refused and is whole only when two or more were and none
was skipped. The rule about cookies that may not carry the session is unchanged.

**Not built.** The other group B credits (session cookies, renewal, and the fixed session of one sign-in; the one
sign-out) stay in part: "each way of signing in" needs the manifest to name the ways, which it does not yet.

**Tests.** `crates/sv-check/src/signed_in/sessions/invented_pages_tests.rs` (five), including one through the whole
signed-in run with a listed page the real session cannot open. The 53 existing session tests pass unchanged.

**Broken on purpose, each put back:** only the first page tried (4 red); whole from one page (1 red); whole with a page
skipped (1 red); the run passing no pages (1 red).
