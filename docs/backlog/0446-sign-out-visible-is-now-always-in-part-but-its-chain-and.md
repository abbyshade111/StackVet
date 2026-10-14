# `sign_out_visible` is now always in part, but its chain and comment still say "one drawn is one sample"

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: the last 48 hours' code, `sv-check`. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-check/src/browser.rs:797-799` (`.in_part()` then `.in_part_if(visits.len() == 1)` with the
  comment "Each private page listed is drawn; one drawn is one sample"); `verified.rs:110-112` (`in_part_if` only
  ever sets the flag).
- What is wrong: the second call and its comment are dead; a reader takes the credit for whole when two pages were
  drawn. The test at `:1585-1590` asserts in part for both, so the behavior is deliberate (group H, backlog 0006
  part 4) and only the code says otherwise.
- What to do: drop the `in_part_if` and say why it is always in part.
- How verified: read.
- Overlap: none found.
- Needs a decision record?: no.
