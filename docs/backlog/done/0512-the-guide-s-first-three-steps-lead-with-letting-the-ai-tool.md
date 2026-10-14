# The guide's first three steps lead with letting the AI tool do them, the typed steps kept as the fallback

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Left from backlog 0473 (14 October 2026), whose fix said the guide's sections 1 to 3 should "lead with 'let your AI
tool do steps 1 to 3' once F5 is honest, with the typed steps kept as the fallback they are". "F5" is a finding in
the deep review of 13 October 2026 (backlog 0417) that no committed file names, so whether it is honest yet could not
be checked. 0473 built the rest: the glossary's Terminal entry says how to open one, and one in a folder, on each
system; the connect line takes Docker's path from `which docker` rather than a shell substitution; and the long
`sv review` line for Docker alone says it becomes `sv review .` once StackVet is installed.

- What to do: find which finding F5 is (the review's own write-up, or ask the reviewing session), check it is
  settled, then rewrite sections 1 to 3 of `docs/GETTING-STARTED.md` to lead with the setup prompt
  (`docs/prompts/setup.md`) doing those steps, the typed commands kept as the way to do them by hand. The guide's word
  budget (4,200, `crates/sv-cli/tests/guide_first_afternoon.rs`) is full: the change must take out as many words as it
  adds.
- How verified: read.
- Needs a decision record?: no.

**Note from the orchestrating session, 14 October 2026:** "F5" is the usability reviewer's fifth finding, filed as backlog 0428 (the guide said the setup prompts were untried while the setup page said they were tried), done on 14 October 2026 (#1761) with a test holding the two pages together. So the condition is met and this can be built: the guide's steps 1 to 3 lead with the prompts, the typed steps kept as the fallback.

**Done, 14 October 2026.** F5 was settled (backlog 0428, per the orchestrating session's note). The guide's offer of
the setup prompts, already before step 1, now leads as the easiest way ("**Easiest: let your AI coding tool do steps 1
to 3.**") and ends by calling steps 1 to 3 the way to do it by hand, if the person prefers or a prompt stops. Step 1
opens by sending someone the prompts already set up on to step 4. Step 2's "(Or ask your AI tool to do it.)" and the
intro's "The steps below set this up, one at a time." went, since the lead says it; the guide is at 4,192 of its
4,200 words. `setup_prompts_guide.rs` reads the paragraph by its new opening; the new
`tests/guide_leads_with_prompts.rs` holds the order, broken on purpose twice (the step 1 line dropped, the offer moved
after step 1), each turning one test red.

Not done: the typed steps themselves are unchanged; the prompts are still tried only in Antigravity (backlog 0502).
