# The failed-sign-ins probe credits a lockout that still lets the right password in

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the VS Code app. Severity: medium (a credit in part
for a limit that limits nothing). Verified against the app's code by the reviewing subagent and read by the
orchestrating session.

**What is wrong.** The VS Code app checks the password first (`app.py:374-378`) and applies its lockout only on a wrong
one (`:403-418`); its own test asserts that the right password signs in during the lockout (`tests/test_app.py:329-342`).
sv saw the push-back after five wrong passwords and credited V6.3.1 in part. The probe knows the account's password
and stops after the push-back.

**Fix.** After the push-back, send the account's real password once: if it signs in, the limit is cosmetic, report it
("after the limit, the right password still signed in"), and credit nothing. One request more.

**Record.** The probe's record: a Later entry (what counts as evidence for V6.3.1).

**Built by the sweep:** this is the first case of backlog 0561 (ADR-119); it closes with it.
