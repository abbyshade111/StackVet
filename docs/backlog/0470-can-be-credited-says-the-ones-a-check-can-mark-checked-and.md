# "Can be credited" says "the ones a check can mark *checked*", and counts three that can only ever be *checked in part*

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: coverage accuracy. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `docs/COVERAGE.md:9-10` ("**Can be credited**: of those, the ones a check can mark *checked*");
  `tools/coverage.py:1043-1049` (`credited()`: any crediting check, `IN_PART` not consulted); `tools/coverage.py:424`
  (`RUST_IN_PART`, and `RUST_IN_PART |= {"config.versions-pinned", "sbom"}` at `:593`); `docs/REQUIREMENTS.md:208`,
  `:414`, `:438` (V1.3.11, V15.1.2, V16.3.4, each marked "only ever in part").
- What is wrong: for V1.3.11 (`probe.mail-header-injected`), V15.1.2 (`config.versions-pinned`, `sbom`), and V16.3.4
  (`probe.ai-service-failure-logged`) every check that can credit is in `IN_PART`, so the strongest status they can
  reach is *checked in part* (ADR-053), never *checked*. The Summary counts them among the 122 "can be credited" under
  a definition that says *checked*. REQUIREMENTS.md gets it right per requirement ("only ever in part"). I ran
  `coverage.py`'s own `evidence()` and `IN_PART` to find them; the list is exactly those three for ASVS (for AISVS,
  C2.1.2, C9.1.1, C9.6.1, C11.4.2, C12.1.2, C12.2.5, C12.4.2 are the same case, seven of the 27).
- What to do: either change the definition to "*checked* or *checked in part*" and add the count of in-part-only ones
  in the sentence under the level table ("3 of the 122 can only ever be *checked in part*"), or count them apart.
  Do not lose: the per-requirement "only ever in part" marks.
- How verified: ran (the script's functions, in Python, read-only).
- Overlap: 0307 (done; made V15.1.2 in part) and ADR-053; neither touched the summary's definition. `grep -rn "in
  part" docs/backlog/*.md` finds no open item on the coverage counts. None found.
- Needs a decision record?: no.
