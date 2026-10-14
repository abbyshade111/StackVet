# The exit codes are explained three times, in three scopes, and never as a table

**Status:** claimed by documentation, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: documentation. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `README.md:57-79` (one paragraph, about 330 words, `sv check`, `sv report`, `sv audit`, and `sv run`);
  `docs/ARCHITECTURE.md:108-114` (`sv check`, `sv report`, `sv audit`; `sv run` left out, which 0325(c) files);
  `docs/GETTING-STARTED.md:349-356` (the four numbers in plain words, then "The README says exactly what each number
  covers").
- What is wrong: A CI user wanting "what does 2 mean for `sv audit`" reads a paragraph whose sentences alternate
  between commands. The three texts also carry different lists of what 2 covers (the README names seven causes; the
  guide two), so a reader of the guide who then reads the README thinks the guide was wrong.
- What to do: One table, in a reference page (section 3's `docs/reference/exit-codes.md`, or a section of the
  "How it works" page): rows `sv check` / `sv report` / `sv audit` / `sv run` / every other command; columns 0, 1, 2,
  3; cells one phrase each; two lines under it for `--fail-on` and `--baseline`. README keeps two sentences and the
  link; the guide keeps its plain-words version and links the table. Saves about 250 words in the README. Must not
  lose: "what a static run cannot see by its nature does not count, or every run would exit 2" (README:66-68),
  which is the reason the design is what it is.
- How verified: read.
- Overlap: 0325(c) (open): ARCHITECTURE leaves out `sv run`; a line fix. This is the three-way duplication and the
  table; build on 0325 (give the table to whoever takes 0325, or file as its sibling).
- Needs a decision record?: no; ADR-029 (findings alone do not fail a run) is unchanged.
