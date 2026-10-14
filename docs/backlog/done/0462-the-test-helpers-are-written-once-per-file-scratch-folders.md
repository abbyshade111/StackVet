# The test helpers are written once per file: scratch folders in 52 files, the binary runner in 28, and five more

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: code quality and duplication. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `fn scratch(` in 52 files across all seven crates (list: `grep -rl 'fn scratch(' crates`); `fn sv(` in 28
  `crates/sv-cli/tests/*.rs` with three signatures (`brief.rs:6`, `notes_confirmed.rs:12`, `exit_codes.rs:12`, ...)
  while 121 of the 142 files there spawn `env!("CARGO_BIN_EXE_sv")`; `fn status_of(compliance, id)` in seven
  (`owner_answers.rs:9`, `notes_confirmed.rs:30`, `confirmations.rs:13`, `decisions_file.rs:9`,
  `notes_authorship.rs:33`, `planned_decisions.rs:9`, six identical, `notes_headings.rs:36` a `String` variant);
  `fn real_adapters()` in seven `crates/sv-check/tests/` files; `fn docker_ok()` in four (`sv_run::detect().is_ok()`);
  `fn secret_rules()` in nine (F6). The only shared module is `crates/sv-check/src/test_support.rs`, holding one
  function.
- What is wrong: 0020 made every scratch name unique per run, in 52 places; the next rule about scratch folders (a
  private `TMPDIR`, removing on drop, a name that says which test) is 52 edits again. The binary runner's three
  signatures mean three ways to read an exit code.
- What to do: a `tests/common/mod.rs` in `sv-cli` and `sv-check` (`mod common;` per test file; it links nothing
  extra), or a `sv-test-support` dev-dependency crate, holding `scratch(name)`, `sv(args, dir)`, `status_of`,
  `real_adapters()`, `docker_ok()`, `secret_rules()`. Must not lose: unique names per run and per call (0020), and
  `CARGO_BIN_EXE_sv`, which only resolves in `sv-cli`'s own integration tests.
- How verified: read; counts by `grep`.
- Overlap: 0020 (done; the names, not the copies), 0335 (link cost of the 118 binary-driving files; different), 0339
  (container tests one per file; different).
- Needs a decision record?: no.

**What was built (14 October 2026, session documentation).** `sv_check::test_support` (built always and hidden, as
backlog 0501 made it, so the tests of `sv-cli` and `sv-run` already reach it) holds two of the helpers:
`scratch(name)`, an empty folder named `sv-`, the calling file's name (read with `#[track_caller]`), the name, the
process id, and a number counted up on each call, so no two calls share one (0020's rule, now per call as well as per
run); and `status_of(compliance, id)`, the status column of a `compliance.md` row. 28 files' own `scratch` and six
files' `status_of` are gone; their tests use the shared ones. Held by `crates/sv-check/src/test_support/scratch_tests.rs`;
with the per-call number taken out, its first test went red ("two calls with one name share a folder"). The whole
workspace's tests pass.

Not done, and why: 23 files keep a `scratch` of their own because each does more or other than the shared one (a
`Scratch` that removes its folder when dropped, in six `sv-check/tests` files; a subfolder made with it; a thread id or
the time in the name; a folder left unmade), and `sv-scan`, `sv-frameworks`, and `sv-manifest` cannot reach `sv-check`,
which depends on them. The `sv` runner in 28 `sv-cli` tests has 18 different bodies (whether a failed run fails the
test, and whether it returns the output, the text, or the exit code); one helper for them is a change of each file's
calls, not a move, and is left for an item of its own. `real_adapters`, `secret_rules`, and `docker_ok` are one line
each, and a shared copy would cost each file the same line to import it.
