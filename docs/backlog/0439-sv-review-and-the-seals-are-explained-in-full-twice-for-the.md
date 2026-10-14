# `sv review` and the seals are explained in full twice for the same reader, with different details in each

**Status:** claimed by documentation, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: documentation. Severity: medium. Verified against the code by the reviewing session before filing.

- Where: `README.md:246-312` (859 words: the key, the passphrase, `allowed_signers`, `SV_TRUSTED_SEALS` with the
  GitHub path, the container invocation, the pre-6-October migration); `docs/GETTING-STARTED.md:234-305` (789 words:
  the `touch allowed_signers` step, the container invocation again, the `.mcp.json` with the read-only mount, "CI is
  the same, by another route: give it the one line ... as the variable `SV_TRUSTED_SEALS` (the README says where)");
  `docs/prompts/setup.md:289` (a sentence).
- What is wrong: The two texts are for the same person (the owner at a terminal), overlap by about two thirds, and
  each has a step the other lacks (the README's GitHub Variables path; the guide's `touch` so Docker does not make a
  folder in the file's place). The reader of either is sent to the other ("The README says where"; "the guide, step
  5, gives the whole `.mcp.json`"). Each explanation is a sequence of five actors and seven files told in prose.
- What to do: One sequence diagram, four lanes: **the AI coding tool** writes an entry as a proposal (`by =
  "ai-tool"`) → **the owner** runs `sv review` at a terminal → **`sv review`** signs the entry with the key in
  `~/.config/stackvet/` and adds the key's public half to `allowed_signers` → **wherever the report is made** (this
  computer, the container with the one file mounted read-only, CI with `SV_TRUSTED_SEALS`) the signature is checked
  against the list, and the entry counts as the owner's; a side note for what a passphrase adds and what the seal
  cannot show. One text home for the full procedure (a `docs/REVIEW.md`, or a section of the reports page), with
  the GitHub path and the `touch` step both in it. The README keeps about 150 words (why the step exists, what it
  signs, where the key lives) and the diagram; the guide keeps its three commands and the diagram. Saves about 600
  words across the two. Must not lose: "What it cannot show" (README:298-300), and that a `Written by: owner` line
  never recorded counts as the tool's (guide:272-274).
- How verified: read.
- Overlap: none found (grep "sv review", "allowed_signers", "SV_TRUSTED_SEALS" in `docs/backlog/`: only the done
  items that built them).
- Needs a decision record?: no; ADR-026 and ADR-034 are unchanged.
