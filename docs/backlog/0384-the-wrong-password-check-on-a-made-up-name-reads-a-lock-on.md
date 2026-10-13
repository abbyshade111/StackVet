# The wrong-password check on a made-up name reads a lock on real accounts as no limit

**Status:** open

From the second loop trial (backlog 0257, `docs/prompts/loop-2/results.md`), 13 October 2026.

With no `signup` in `stackvet.toml`, the wrong-password check tries its wrong passwords against "a user name no account
has" (`crates/sv-check/src/signed_in/signin.rs`), so as not to lock the seeded users out. An app that counts wrong
passwords per real account (a sound design, and the one most builds chose) never locks a name that has no account, and
the check reports "Wrong passwords can be tried without limit". Read by hand in `haiku-a-1`: the app locks a real
account after five wrong passwords and checks a dummy hash for unknown names; `sv` reported no limit.

A fix might use an account `sv` makes or seeds for this check alone, or say "not assessed" when only a made-up name was
tried and the app answered it as any wrong password.
