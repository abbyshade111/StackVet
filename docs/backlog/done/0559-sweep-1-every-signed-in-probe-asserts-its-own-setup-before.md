# Sweep 1: every signed-in probe asserts its own setup before concluding, and reports "not assessed" with the reason when it failed

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (`docs/research/SETUP-PROMPT-TRIALS-2026-10-14.md`); the owner's
choice the same day: fix the pattern across the probes, not one item at a time. The record is ADR-119 (proposed in this
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

**Record.** ADR-119, accepted in this pull request.

**Built 15 October 2026, by honesty** (ADR-119, accepted, with its Later entry listing every check read).
- **Fixed:** the password-guessing check (0522) and the emailed code's first check now sign in as the account they
  made, and open the first private page, before concluding anything; without it, they say what the sign-up answered
  and that the form may need a value stackvet.toml does not carry, such as an invite code.
- **Already asserted:** the password rules, change, and email change; re-sign-up; deletion; the reset; the roles
  written into sign-up and into the email change; the changes beside the email change; the log marker; the cross-site
  and simple-request checks; the burst; the uploads.
- **Safe without it:** whether a failed sign-in or a sign-up tells accounts apart, which are only ever a finding.
- **Held by** `crates/sv-check/src/signed_in/setup_sweep_tests.rs` (A and B seeded, a sign-up that is refused or makes
  nothing: no finding, and each check's line says why) and `signed_in/signin/own_account_tests.rs` (0522's case).
  Undoing either fix turns them red.
- The spec's sign-ins in a run go from 60 to 62 (ADR-028, Later).
