# Three wildcard matchers, two of them in the same crate

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: code quality and duplication. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-check/src/config.rs:570` (`glob_matches`, git's rules: `*`, `?`, `**`, `[...]`, `\`), `crates/sv-check/src/adapters.rs:263`
  (`glob` over bytes: `*`, `?`, `**`, `**/`), `crates/sv-scan/src/ecosystems.rs:639` (`glob_matches` over path
  segments: `*` and `**` only).
- What is wrong: the two in `sv-check` overlap on `*`, `?`, and `**` and each lacks what the other has (`config.rs`
  has no `**/` case, so `**/x` works only through its generic `**` then `strip_prefix(&['/'])`; `adapters.rs` has no
  `[...]` or `\`). A pattern that works in `.gitignore` reading may not in `adapters.json` and the reader has to find
  out which matcher a file uses.
- What to do: one `glob` module in `sv-scan` (both crates depend on it) with the superset, `config.rs`'s semantics,
  and the three sets of tests moved under it; `ecosystems.rs`'s segment matcher can stay if its callers want `*` to
  mean one folder name, but say so at its top.
- How verified: read.
- Overlap: none found (the "glob" hits in the backlog are about gitignore reading, 0054 and 0010).
- Needs a decision record?: no.
