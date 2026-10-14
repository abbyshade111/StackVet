# A tab before `--hash` in requirements.txt makes the whole file read as unpinned, since the 0291 reader splits on a space only

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: the last 48 hours' code, `sv-run`, `sv-report`, `sv-scan`. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-scan/src/ecosystems.rs:551-561`: `let (requirement, rest) = line.find(" --").map_or((line, ""), |at| (&line[..at], &line[at..]));` then `.split_whitespace().collect()`.
- What is wrong: Until 0291 the requirement ended at the first whitespace (`split_once(char::is_whitespace)`), so
  `flask==3.0.0<TAB>--hash=sha256:...` was read as `flask==3.0.0` with the hash in `rest`. Now the split looks for
  `" --"` (a space) only; with a tab, `rest` is empty, the whitespace is removed from the whole line, and the name
  becomes `flask` with version `3.0.0--hash=sha256:...`, which passes `version_ok` but fails `hashed`, so the function
  returns `false` for the whole file. The same happens for a continuation written as `name==1 \` then
  `<TAB>--hash=...`, since `logical_lines` joins with no added space. `pip` accepts both. The effect is honest in
  direction (the file is not taken as a lockfile, so `complete` is false and the V15.1.2 credit, now in part after
  0307, is not given), but the report says the app's packages were not all read when they were.
- What to do: Split the requirement from its options at the first whitespace followed by `--` (`find` over
  `char::is_whitespace` then check the next two bytes), or tokenise with `split_whitespace` and take tokens until the
  first that starts with `--`. Keep the `Jinja2 ==3.1.4` case 0291 added (spaces inside the requirement), and add a
  tab case to the test beside it.
- How verified: read (the function, and `crates/sv-scan/src/requirements.rs` for what `logical_lines` yields).
- Overlap: 0291 (done) is the change that introduced it; no item names tabs. `grep -ril 'tab' docs/backlog/` finds
  nothing about requirements files.
- Needs a decision record?: no.
