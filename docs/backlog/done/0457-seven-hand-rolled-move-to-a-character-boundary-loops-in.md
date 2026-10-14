# Seven hand-rolled "move to a character boundary" loops in three files

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: code quality and duplication. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-check/src/bundled.rs:119` (`fn floor_char_boundary`), `crates/sv-scan/src/files.rs:316` and
  `:332` (`while !text.is_char_boundary(keep_to) { keep_to -= 1; }`), `crates/sv-run/src/docker.rs:3265`, `:3279`,
  `:3283`, `:3296` (`kept_body`: two forward, two backward loops).
- What is wrong: std's `floor_char_boundary` is unstable, so each file wrote its own; the backward loops in
  `files.rs` would underflow on a `cut` of 0 that is not a boundary (impossible for 0, so safe today, but each copy
  must reason it out). This is exactly the class of byte-offset slicing on app text that 0278 crashed on.
- What to do: `floor_char_boundary` and `ceil_char_boundary` in `sv-scan` (or `sv-frameworks::text`), both saturating,
  with a test on a multi-byte boundary; the seven sites call them.
- How verified: read.
- Overlap: 0278 (done; a different site), 0369 (double UTF-8 validation; different).
- Needs a decision record?: no.

**What was built (14 October 2026, session documentation).** The item's premise had moved: `str::floor_char_boundary`
and `str::ceil_char_boundary` are stable since Rust 1.91, and CI and the Windows workflow install the newest stable, so
no helper of `sv`'s own was needed. The seven sites call std's: `bundled.rs` (its own `floor_char_boundary` deleted),
`sv-scan`'s `files.rs` (both backward loops), and `sv-run`'s `kept_body` (two forward, two backward). Both of std's
functions clamp an index past the end to the end, so the `.min(body.len())` beside two of them went too. Breaking each
site (the raw byte index, still clamped to the length) turned one test red in each place:
`a_character_cut_by_a_piece_boundary_is_not_read_as_binary` (files.rs) and
`a_value_across_the_cut_is_kept_whole_and_no_character_is_split` (docker.rs); `bundled.rs` had no test that put a cut
inside a character, so `crates/sv-check/src/bundled_boundary_tests.rs` is new and goes red the same way.
