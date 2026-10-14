# Notes for people who used StackVet before 9 October sit in the first-time reader's path

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: documentation. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `README.md:302-304` (entries sealed before 6 October with `~/.config/securevibe/review-key`); `:466-468`
  ("Until 9 October 2026 there were thirteen" tools); `:533` (images before 9 October carry no signature);
  `:536-539` (the product was called SecureVibe, the old image name, the old file names); `docs/GETTING-STARTED.md:566-568`
  ("Before 5 October 2026 the guide had you use the program in the build folder"); and `:16-20` of the README (v1 and
  the 26 September move).
- What is wrong: About 300 words of migration notes, each true and each needed by perhaps a dozen people, interrupt
  the explanation of the current behavior for everyone who arrives after them. The README's third paragraph, before
  anything about what `sv` does, is about v1's branch and DOI.
- What to do: One section at the end of the README, "If you used StackVet before 9 October 2026" (old names still
  read; the one-time re-sign offer; the unsigned early images; the two renamed MCP tools), and move the v1 paragraph
  to "Building" or that section; the guide's note the same. The dates and the ADR numbers stay, in that section.
- How verified: read; grep for "Until <date>", "before <date>".
- Overlap: 0325 (open) is wording fixes; none moves text.
- Needs a decision record?: no; ADR-062 (the rename) is unchanged.
