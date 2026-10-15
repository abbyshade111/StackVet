# The failed-sign-ins probe reports "an account this check made for it" after its own sign-up was refused

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the Cursor app. Severity: high (a high finding that
is false, and a setup that fails quietly). Verified against `seen.json` and the code by the orchestrating session.

**What is wrong.** The Cursor app's sign-up needs an invite code the manifest does not carry, so every sign-up sv sent
answered 403 (`seen.json`: `signup-control` 403, `signup-guessed` 403). The probe then sent seven wrong passwords to
`guessed.sv-a-...`, an address with no account, got 400 each time, and wrote the high finding "Wrong passwords can be
tried without limit ... against an account this check made for it": the app locks a real account after five
(`familygames/db.py:89-100`, and its test `tests/test_auth.py:211-231`). The probe never looks at what its sign-up
answered: `crates/sv-check/src/signed_in/signin.rs:327` discards `sign_up`'s result, and `:335` sets `real_account`
from whether the manifest has a `signup` section, not from whether the sign-up worked. This is the rule in CLAUDE.md:
a test whose setup can fail quietly must first assert the setup worked.

**Where.** `crates/sv-check/src/signed_in/signin.rs:318-335` (the target account), `:476` (the wording "an account this
check made for it").

**Fix.** Read `sign_up`'s answer against the control sign-up: when the account was not made, report V6.3.1 as not
assessed with the reason ("the sign-up this check needs answered 403; the form may need a value stackvet.toml does not
carry, such as an invite code"), never as a finding; when the manifest has no `signup`, keep the "name no account can
have" path and its honest wording. Add the fake-app case: a sign-up that answers 403, and watch the high finding go.

**Record.** The probe's record (ADR-085 or the one that governs `signed_in/`): a Later entry, since what counts as
evidence for V6.3.1 changes.

**Built by the sweep:** this is the first case of backlog 0559 (ADR-119); it closes with it.

**Built 15 October 2026, by honesty, in the setup sweep (0559, ADR-119):** the guessing check signs in as its own
account before guessing, and says the question is not assessed, and why, when the account was not made.
