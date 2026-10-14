# `allowed + 2` in the sign-in guessing guard can wrap in a release build

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: code quality and duplication. Severity: low. Verified against the code by the reviewing session before filing. Filed on its own so the one-line fix is not deferred with the refactor of the duplicated reading (the item before it).

- Where: `crates/sv-check/src/signed_in/signin.rs:299-300` (`const MOST_ATTEMPTS: u32 = 26; if allowed + 2 >
  MOST_ATTEMPTS {`); compare `crates/sv-check/src/signed_in/codes.rs:278` (`(1..MOST_ATTEMPTS - 1).contains(n)`).
- What is wrong: `Cargo.toml`'s release profile sets only `strip = true`, so overflow checks are off: with
  `failed-sign-ins = 4294967295`, `allowed + 2` is 1, the guard passes, `attempts` is 1, one request is sent, and
  `answers[allowed as usize]` at `:398` indexes out of bounds. The panic is caught by `guarded("running_app")`
  (`crates/sv-cli/src/assemble.rs:98`, `:384`) and reported as the stage having crashed, which is honest, but the whole
  stage's evidence is lost for one number. A debug build panics at the addition instead.
- What to do: `allowed.saturating_add(2)` or the range form `codes.rs` uses; and `sv-manifest` refusing
  `failed-sign-ins` and `failed-codes` above a plain bound (say 1000) with a sentence, so the limit is said once at
  load rather than discovered in the suite.
- How verified: read (`Cargo.toml:41-55` for the profile).
- Overlap: 0278 (done; another crash of the stage), 0234 (the guard). None for the wrap.
- Needs a decision record?: no (a refusal of a number no app can mean, with a message; nothing a report concludes changes).
