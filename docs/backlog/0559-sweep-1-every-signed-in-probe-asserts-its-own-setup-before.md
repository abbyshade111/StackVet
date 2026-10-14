# Sweep 1: every signed-in probe asserts its own setup before concluding, and reports "not assessed" with the reason when it failed

**Status:** claimed by honesty, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (`docs/research/SETUP-PROMPT-TRIALS-2026-10-14.md`); the owner's
choice the same day: fix the pattern across the probes, not one item at a time. The record is ADR-118 (proposed in this
claim; accepted by the pull request that builds this sweep). Severity: high.

**What to build.** For every probe under `crates/sv-check/src/signed_in/` that makes an account, a record, or a session
of its own (the guessing-limit probe in `signin.rs`, the validation and upload probes in `uploads.rs`, the forged-form
and other-user probes in `forgery.rs` and `admin.rs`, the burst and limit probes, the change-password and activation
flows): read the answer that made the setup and compare it with the run's control (the ordinary sign-up, the ordinary
create). When the setup did not work, the probe's requirements are not assessed, with the reason in plain words (what
was sent, what answered, what the manifest may be missing, such as an invite code), and no finding. The wording "an
account this check made for it" only when it was.

**First case.** Backlog 0522 (the Cursor trial: sign-up 403, seven guesses at an address with no account, a high
finding). It closes with this sweep.

**How to know it works.** One fake-app case per probe where the setup answers 403 or 500; before the fix, read which
findings and credits still appear; after it, each is "not assessed" with its reason. A list of the probes swept, in
the pull request, with the ones that already asserted their setup named too.

**Record.** ADR-118, accepted in this pull request.
