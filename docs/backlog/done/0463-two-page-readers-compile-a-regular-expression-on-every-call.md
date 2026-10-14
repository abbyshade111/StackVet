# Two page readers compile a regular expression on every call

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: code quality and duplication. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-check/src/signed_in/mod.rs:407-412` (`attribute(tag, name)` builds `Regex::new(&format!(...,
  regex::escape(name)))` per call; thirteen call sites, per tag per page), `crates/sv-check/src/signed_in/owner_field.rs:26-32`
  (`named_owner` builds up to seven per record read).
- What is wrong: 0335's "Measured and found efficient" paragraph records every regex outside tests as behind
  `LazyLock`/`OnceLock` or compiled once at load; these two are not. Both escape their input, so there is no
  pattern-from-app-text fault, only cost (tens to hundreds of microseconds per compile, on the running-app path).
- What to do: a hand parser for the three quoting forms (as `csrf_token` and `read_markup` already do by hand), or a
  `OnceLock<HashMap<&'static str, Regex>>` keyed by the fixed attribute names; the same for `OWNER_FIELDS`.
- How verified: read.
- Overlap: 0369 (small run-speed tidiness; does not name these), 0335 (contradicted).
- Needs a decision record?: no.
