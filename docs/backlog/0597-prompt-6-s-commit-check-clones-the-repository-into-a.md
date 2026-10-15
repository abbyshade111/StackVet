# Prompt 6's commit check clones the repository into a temporary folder without asking, and the design questions arrive with a "(Recommended)" answer pre-chosen

**Status:** claimed by wording, 15 October 2026

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
