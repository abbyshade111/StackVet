# Prompt 6's commit check clones the repository into a temporary folder without asking, and the design questions arrive with a "(Recommended)" answer pre-chosen

**Status:** done, 15 October 2026. Prompt 5's step 6 (and the guide's copy): ask each question as StackVet words
it, with no answer marked as recommended; if asked what it would choose, say so then, with the lines it rests on.
Prompt 6's step 2: read the latest commit on the page itself, and clone, download, or fetch nothing to compare; after
installing or updating, compare `sv --version` with the commit the install printed. Restamped. To keep the guide under
its 4,500 words, two of its own sentences were tightened. Held by `crates/sv-cli/tests/setup_prompt_no_steering.rs`.

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the Cursor + Flash run of 15 October 2026 (backlog 0577). For the prompt library (0370). Severity: low.

**What happened.** For prompt 6's "the installed commit must match", the tool cloned the StackVet repository into /tmp
to compare, which the prompt neither asked for nor forbade (the person had not been asked). Cursor's question dialogs
offered "(Recommended)" answers on every design question, which the person then took; the prompts say to ask, not to
steer.

**What to change.** Prompt 6: "compare the commit sv --version prints with the one the install printed; do not fetch
anything to do it." Prompt 5: "ask the questions without a recommended answer; if I ask what you would choose, say so
then, with the lines it rests on."

**Record.** None.
