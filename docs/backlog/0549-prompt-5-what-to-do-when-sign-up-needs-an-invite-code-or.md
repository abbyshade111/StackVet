# Prompt 5: what to do when sign-up needs an invite code or another secret

**Status:** claimed by wording, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502): Cursor and VS Code. For the prompt library (0370).

**What was seen.** Both apps gate sign-up on an invite code. Cursor kept it out of the manifest, so sv's sign-up was
refused and every path that needed an account stopped (and one wrong high finding followed). VS Code hard-coded a
test code into the manifest and the seed, which worked and is itself a finding.

**What to change.** Until sv can be given such a value (the sign-up-secret proposal from this trial), prompt 5 says:
"If sign-up needs an invite code or similar, tell me that sv's run will not be able to sign up on its own, and ask
whether to seed a test code for the run and where it would live."
