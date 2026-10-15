# The wrong-password check with no sign-up credits a push-back without trying the right password

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the inventory of sweep 3 (backlog 0561, ADR-119); more than two requests, so filed rather than built there.

**What is wrong.** With no `signup` in stackvet.toml, the wrong-password check (V6.3.1, `signin.rs`) guesses at a name
no account has, and a push-back by address is credited. It never tries a right password afterwards, as it now does
with an account of its own (0561's first batch), so a limit that refuses only wrong passwords is credited here.

**What to build.** After the push-back, sign A in from the same address (the sign-in page, the post, the first
private page, about three requests). If A signs in, the limit lets a right password through. Mind the sign-in budget
(62 in a run, `SIGN_INS_IN_A_RUN`): it comes after the guesses, so it is counted with them.

**Built, 15 October 2026 (honesty).** After the push-back, A signs in once from the same address (`guess-right`, then
the first private page). A refused: credited, and the credit says A was refused too. A let in: not credited and no
finding, because a sound limit counting each name lets A in exactly as a limit refusing only wrong passwords does
(ADR-119, Later). Tests: `crates/sv-check/src/signed_in/no_signup_right_password_tests.rs`. Left open for the board
to close.
