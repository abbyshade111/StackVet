# Prompt 5: what to do when sign-up needs an invite code or another secret

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502): Cursor and VS Code. For the prompt library (0370).

**What was seen.** Both apps gate sign-up on an invite code. Cursor kept it out of the manifest, so sv's sign-up was
refused and every path that needed an account stopped (and one wrong high finding followed). VS Code hard-coded a
test code into the manifest and the seed, which worked and is itself a finding.

**What to change.** Until sv can be given such a value (the sign-up-secret proposal from this trial), prompt 5 says:
"If sign-up needs an invite code or similar, tell me that sv's run will not be able to sign up on its own, and ask
whether to seed a test code for the run and where it would live."

**Done, 14 October 2026:** rule 1 of the build prompt (the guide's step 4 and the setup page's step 5, word for word)
now says, after the line on naming the routes: "If signing up needs an invite code or another secret, tell me that
StackVet's run cannot sign up on its own, and ask whether to seed a test code for the run, and where it would live."
It holds until sv can be given such a value (the sign-up-secret proposal). A test code written as a fixed value in
`[stack.run.users] signup`, as VS Code did, does let the run sign up, which is why the prompt asks rather than forbids:
where the code lives is the person's call. `build_prompt_rules.rs` holds the sentence; with it taken out of the setup
page, it and the word-for-word test went red. Four sentences in the guide's step 3 were shortened, saying the same,
to keep it within its 4,200 words.
