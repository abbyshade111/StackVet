# The decision-record index and ADR-057's status line disagree with the records' own status lines

**Status:** claimed by builder-none, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: documentation. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** The index is what a reader uses to find a record's standing, and it is wrong for two of them; the README points at a file that no longer holds what it says.

**Where.** `docs/adr/README.md:109` (ADR-062 "(proposed)") against `docs/adr/ADR-062.md:3` (accepted, 9 October 2026); `docs/adr/README.md:104` and `docs/adr/ADR-057.md`'s status line (still "proposed" although all three parts were accepted in dated Later entries on 8 October); `docs/adr/README.md:44-45` and `:55-56` cite `DESIGN.md` for ADR-012, which `docs/DESIGN.md` no longer holds (the citations are in `docs/design/0001-*`, `0003-*`, `0007-*`).

**Evidence.** `grep -c ADR-012 docs/DESIGN.md` is 0; the status lines quoted.

**How it bites.** A session reads a record as proposed, builds as if the decision were open, or looks for a citation where none is.

**Fix.** Drop "(proposed)" from ADR-062's row; a one-line Later note on ADR-057 making its status line accepted; write "the design record (`docs/design/`, "What carries over unchanged")" in the README. The index test (`the_index_lists_every_record`) could hold each row's status word to the record's.

**Record.** ADR-057 and ADR-062 are corrected to themselves; ADR-060 governs the design folder: "unchanged, because" lines.
