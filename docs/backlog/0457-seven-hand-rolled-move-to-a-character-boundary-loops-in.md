# Seven hand-rolled "move to a character boundary" loops in three files

**Status:** open

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
