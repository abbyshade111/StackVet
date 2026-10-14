# The guide's first three steps lead with letting the AI tool do them, the typed steps kept as the fallback

**Status:** open

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
