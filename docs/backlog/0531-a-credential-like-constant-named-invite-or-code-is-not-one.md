# A credential-like constant named `INVITE_*` or `*_CODE` is not one the secrets check knows

**Status:** claimed by documentation, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the VS Code app. Severity: low.

**What is wrong.** `app.py:42` `INVITE_FOR_STACKVET = "stackvet-test-invite"` is a fixed invite code in production code
(harmless only because `create_app` erases it at every start, `app.py:190`, and `seed.py:30` re-inserts it); no finding
names it. Invite, access, and join codes are credentials a family app depends on.

**Fix.** Add invite, access-code, and join-code names to the credential names, at low severity when the value is used
only by a seed script, with the test case.

**Record.** The secrets check's record: a Later entry if the names are listed there.
